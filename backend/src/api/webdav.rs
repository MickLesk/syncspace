//! WebDAV API endpoints
//! Provides WebDAV protocol support for mounting SyncSpace as network drive

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, Method, StatusCode},
    response::Response,
    routing::any,
    Router,
};

use crate::{auth::UserInfo, webdav, AppState};

/// Main WebDAV handler
async fn webdav_handler(
    state: State<AppState>,
    headers: HeaderMap,
    method: Method,
    path: Path<String>,
    user: UserInfo,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    match method.as_str() {
        "OPTIONS" => Ok(webdav::handle_options(headers).await),
        "PROPFIND" => webdav::handle_propfind(state, headers, path, user).await,
        "PROPPATCH" => webdav::handle_proppatch(state, path, user).await,
        "MKCOL" => webdav::handle_mkcol(state, path, user)
            .await
            .map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "COPY" => webdav::handle_copy(state, headers, path, user)
            .await
            .map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "MOVE" => webdav::handle_move(state, headers, path, user)
            .await
            .map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        "LOCK" => webdav::handle_lock(state, path, user).await,
        "UNLOCK" => Ok(Response::builder()
            .status(webdav::handle_unlock(state, path, user).await)
            .body(Body::empty())
            .unwrap()),
        "GET" | "HEAD" => handle_get(state, path, user, method == Method::HEAD).await,
        "PUT" => handle_put(state, path, user, body).await,
        "DELETE" => handle_delete(state, path, user)
            .await
            .map(|s| Response::builder().status(s).body(Body::empty()).unwrap()),
        _ => Err(StatusCode::METHOD_NOT_ALLOWED),
    }
}

/// Handle GET/HEAD request
async fn handle_get(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
    head_only: bool,
) -> Result<Response<Body>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    if full_path.is_dir() {
        // Directory listing not supported via GET
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

    let content = tokio::fs::read(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content_type = mime_guess::from_path(&full_path)
        .first_or_octet_stream()
        .to_string();

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, content.len());

    // Add DAV headers
    for (name, value) in webdav::get_dav_headers() {
        builder = builder.header(name, value);
    }

    if head_only {
        builder.body(Body::empty()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        builder.body(Body::from(content)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Handle PUT request
async fn handle_put(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&path);

    // Create parent directories
    if let Some(parent) = full_path.parent() {
        tokio::fs::create_dir_all(parent).await.ok();
    }

    let bytes = axum::body::to_bytes(body, 100 * 1024 * 1024)
        .await
        .map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)?;

    let existed = full_path.exists();

    tokio::fs::write(&full_path, &bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(if existed {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::CREATED
        })
        .body(Body::empty())
        .unwrap())
}

/// Handle DELETE request
async fn handle_delete(
    State(_state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    if full_path.is_dir() {
        tokio::fs::remove_dir_all(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        tokio::fs::remove_file(&full_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// WebDAV root OPTIONS handler
async fn webdav_options() -> Response<Body> {
    let mut builder = Response::builder().status(StatusCode::OK);

    for (name, value) in webdav::get_dav_headers() {
        builder = builder.header(name, value);
    }

    builder.body(Body::empty()).unwrap()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/webdav", any(webdav_options))
        .route("/webdav/{*path}", any(webdav_handler))
}
