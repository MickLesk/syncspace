//! WebDAV Server Implementation
//! Allows mounting SyncSpace as a network drive via WebDAV protocol

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, Method, StatusCode},
    response::Response,
    routing::{any, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::{auth::UserInfo, AppState};

/// WebDAV resource representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDavResource {
    pub path: String,
    pub href: String,
    pub is_collection: bool,
    pub content_length: u64,
    pub content_type: String,
    pub last_modified: String,
    pub created: String,
    pub etag: Option<String>,
}

/// WebDAV lock information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDavLock {
    pub token: String,
    pub path: String,
    pub owner: String,
    pub depth: String,
    pub timeout: u64,
    pub lock_type: String, // "write"
    pub lock_scope: String, // "exclusive" or "shared"
    pub created_at: String,
}

/// Build DAV compliance headers
pub fn get_dav_headers() -> [(header::HeaderName, &'static str); 3] {
    [
        (header::HeaderName::from_static("dav"), "1, 2, 3"),
        (header::HeaderName::from_static("ms-author-via"), "DAV"),
        (header::ALLOW, "GET, HEAD, PUT, DELETE, OPTIONS, PROPFIND, PROPPATCH, MKCOL, COPY, MOVE, LOCK, UNLOCK"),
    ]
}

/// Handle WebDAV OPTIONS request
pub async fn handle_options(
    _headers: HeaderMap,
) -> Response<Body> {
    let mut response = Response::builder()
        .status(StatusCode::OK);
    
    for (name, value) in get_dav_headers() {
        response = response.header(name, value);
    }
    
    response.body(Body::empty()).unwrap()
}

/// Handle WebDAV PROPFIND request
pub async fn handle_propfind(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<Response<Body>, StatusCode> {
    let depth = headers
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("0");

    let full_path = PathBuf::from("./data").join(&path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut resources = Vec::new();
    
    // Add the requested resource
    resources.push(get_resource_info(&full_path, &path).await?);
    
    // If depth is not 0 and it's a directory, list children
    if depth != "0" && full_path.is_dir() {
        let mut entries = fs::read_dir(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        while let Ok(Some(entry)) = entries.next_entry().await {
            let child_path = format!("{}/{}", path, entry.file_name().to_string_lossy());
            if let Ok(resource) = get_resource_info(&entry.path(), &child_path).await {
                resources.push(resource);
            }
        }
    }

    // Build XML response
    let xml = build_multistatus_xml(&resources);

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(xml))
        .unwrap())
}

/// Handle WebDAV PROPPATCH request
pub async fn handle_proppatch(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<Response<Body>, StatusCode> {
    let full_path = PathBuf::from("./data").join(&path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Property updates are not fully implemented - return success for compatibility
    let xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<D:multistatus xmlns:D="DAV:">
  <D:response>
    <D:href>/{}</D:href>
    <D:propstat>
      <D:status>HTTP/1.1 200 OK</D:status>
    </D:propstat>
  </D:response>
</D:multistatus>"#,
        path
    );

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(xml))
        .unwrap())
}

/// Handle WebDAV MKCOL request (create directory)
pub async fn handle_mkcol(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let full_path = PathBuf::from("./data").join(&path);
    
    if full_path.exists() {
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

    fs::create_dir_all(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

/// Handle WebDAV COPY request
pub async fn handle_copy(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let destination = headers
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let source_path = PathBuf::from("./data").join(&path);
    let dest_path = extract_path_from_uri(destination);
    let dest_full = PathBuf::from("./data").join(&dest_path);

    if !source_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check overwrite header
    let overwrite = headers
        .get("Overwrite")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("T") == "T";

    if dest_full.exists() && !overwrite {
        return Err(StatusCode::PRECONDITION_FAILED);
    }

    if source_path.is_dir() {
        copy_dir_recursive(&source_path, &dest_full).await?;
    } else {
        if let Some(parent) = dest_full.parent() {
            fs::create_dir_all(parent).await.ok();
        }
        fs::copy(&source_path, &dest_full)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(if dest_full.exists() { StatusCode::NO_CONTENT } else { StatusCode::CREATED })
}

/// Handle WebDAV MOVE request
pub async fn handle_move(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let destination = headers
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let source_path = PathBuf::from("./data").join(&path);
    let dest_path = extract_path_from_uri(destination);
    let dest_full = PathBuf::from("./data").join(&dest_path);

    if !source_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check overwrite header
    let overwrite = headers
        .get("Overwrite")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("T") == "T";

    if dest_full.exists() {
        if !overwrite {
            return Err(StatusCode::PRECONDITION_FAILED);
        }
        if dest_full.is_dir() {
            fs::remove_dir_all(&dest_full).await.ok();
        } else {
            fs::remove_file(&dest_full).await.ok();
        }
    }

    if let Some(parent) = dest_full.parent() {
        fs::create_dir_all(parent).await.ok();
    }

    fs::rename(&source_path, &dest_full)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

/// Handle WebDAV LOCK request
pub async fn handle_lock(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    user: UserInfo,
) -> Result<Response<Body>, StatusCode> {
    let lock_token = format!("opaquelocktoken:{}", uuid::Uuid::new_v4());
    
    let xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<D:prop xmlns:D="DAV:">
  <D:lockdiscovery>
    <D:activelock>
      <D:locktype><D:write/></D:locktype>
      <D:lockscope><D:exclusive/></D:lockscope>
      <D:depth>0</D:depth>
      <D:owner><D:href>{}</D:href></D:owner>
      <D:timeout>Second-3600</D:timeout>
      <D:locktoken>
        <D:href>{}</D:href>
      </D:locktoken>
      <D:lockroot>
        <D:href>/{}</D:href>
      </D:lockroot>
    </D:activelock>
  </D:lockdiscovery>
</D:prop>"#,
        user.username, lock_token, path
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .header("Lock-Token", format!("<{}>", lock_token))
        .body(Body::from(xml))
        .unwrap())
}

/// Handle WebDAV UNLOCK request
pub async fn handle_unlock(
    State(_state): State<AppState>,
    Path(_path): Path<String>,
    _user: UserInfo,
) -> StatusCode {
    // Lock management is simplified - always succeed
    StatusCode::NO_CONTENT
}

// Helper functions

async fn get_resource_info(path: &std::path::Path, href: &str) -> Result<WebDavResource, StatusCode> {
    let metadata = fs::metadata(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let is_dir = metadata.is_dir();
    let content_length = if is_dir { 0 } else { metadata.len() };
    
    let last_modified = metadata
        .modified()
        .ok()
        .map(|t| httpdate::fmt_http_date(t))
        .unwrap_or_else(|| "Thu, 01 Jan 1970 00:00:00 GMT".to_string());

    let content_type = if is_dir {
        "httpd/unix-directory".to_string()
    } else {
        mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string()
    };

    Ok(WebDavResource {
        path: path.to_string_lossy().to_string(),
        href: format!("/{}", href.trim_start_matches('/')),
        is_collection: is_dir,
        content_length,
        content_type,
        last_modified,
        created: last_modified.clone(),
        etag: Some(format!("\"{}\"", content_length)),
    })
}

fn build_multistatus_xml(resources: &[WebDavResource]) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="utf-8"?>
<D:multistatus xmlns:D="DAV:">"#);

    for resource in resources {
        let resource_type = if resource.is_collection {
            "<D:resourcetype><D:collection/></D:resourcetype>"
        } else {
            "<D:resourcetype/>"
        };

        xml.push_str(&format!(
            r#"
  <D:response>
    <D:href>{}</D:href>
    <D:propstat>
      <D:prop>
        <D:displayname>{}</D:displayname>
        {}
        <D:getcontentlength>{}</D:getcontentlength>
        <D:getcontenttype>{}</D:getcontenttype>
        <D:getlastmodified>{}</D:getlastmodified>
        <D:getetag>{}</D:getetag>
      </D:prop>
      <D:status>HTTP/1.1 200 OK</D:status>
    </D:propstat>
  </D:response>"#,
            resource.href,
            resource.href.split('/').last().unwrap_or("root"),
            resource_type,
            resource.content_length,
            resource.content_type,
            resource.last_modified,
            resource.etag.as_deref().unwrap_or("\"\"")
        ));
    }

    xml.push_str("\n</D:multistatus>");
    xml
}

fn extract_path_from_uri(uri: &str) -> String {
    // Extract path from full URI like http://localhost:8080/webdav/path/to/file
    if let Some(pos) = uri.find("/webdav/") {
        uri[pos + 8..].to_string()
    } else if let Some(pos) = uri.find("/api/webdav/") {
        uri[pos + 12..].to_string()
    } else {
        uri.trim_start_matches('/').to_string()
    }
}

async fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), StatusCode> {
    fs::create_dir_all(dst)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut entries = fs::read_dir(src)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            Box::pin(copy_dir_recursive(&src_path, &dst_path)).await?;
        } else {
            fs::copy(&src_path, &dst_path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    Ok(())
}

/// WebDAV API Router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/webdav", any(handle_options))
        .route("/webdav/{*path}", any(webdav_handler))
}

/// Main WebDAV handler that routes based on HTTP method
async fn webdav_handler(
    state: State<AppState>,
    headers: HeaderMap,
    method: Method,
    path: Path<String>,
    user: UserInfo,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    match method.as_str() {
        "OPTIONS" => Ok(handle_options(headers).await),
        "PROPFIND" => handle_propfind(state, headers, path, user).await,
        "PROPPATCH" => handle_proppatch(state, path, user).await,
        "MKCOL" => handle_mkcol(state, path, user).await.map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "COPY" => handle_copy(state, headers, path, user).await.map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "MOVE" => handle_move(state, headers, path, user).await.map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "LOCK" => handle_lock(state, path, user).await,
        "UNLOCK" => Ok(Response::builder().status(handle_unlock(state, path, user).await).body(Body::empty()).unwrap()),
        "GET" => handle_get(state, path, user).await,
        "PUT" => handle_put(state, path, user, body).await,
        "DELETE" => handle_delete(state, path, user).await.map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        _ => Err(StatusCode::METHOD_NOT_ALLOWED),
    }
}

/// Handle GET request (download file)
async fn handle_get(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<Response<Body>, StatusCode> {
    let full_path = PathBuf::from("./data").join(&path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    if full_path.is_dir() {
        // Return directory listing as HTML for browsers
        return handle_propfind(
            State(_state),
            HeaderMap::new(),
            Path(path),
            _user,
        ).await;
    }

    let content = fs::read(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content_type = mime_guess::from_path(&full_path)
        .first_or_octet_stream()
        .to_string();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, content.len())
        .body(Body::from(content))
        .unwrap())
}

/// Handle PUT request (upload file)
async fn handle_put(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    let full_path = PathBuf::from("./data").join(&path);

    // Create parent directories if needed
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).await.ok();
    }

    // Collect body bytes
    let bytes = axum::body::to_bytes(body, 100 * 1024 * 1024) // 100MB limit
        .await
        .map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)?;

    let existed = full_path.exists();

    fs::write(&full_path, &bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(if existed { StatusCode::NO_CONTENT } else { StatusCode::CREATED })
        .body(Body::empty())
        .unwrap())
}

/// Handle DELETE request
async fn handle_delete(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let full_path = PathBuf::from("./data").join(&path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    if full_path.is_dir() {
        fs::remove_dir_all(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        fs::remove_file(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
