use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/metadata/{*path}", get(get_metadata))
        .route("/metadata-types", get(get_supported_types))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub metadata_type: String,
    pub metadata: HashMap<String, MetadataValue>,
    pub extracted_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MetadataValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Array(Vec<String>),
    Object(HashMap<String, String>),
}

#[derive(Debug, Serialize)]
pub struct SupportedTypes {
    pub image: Vec<String>,
    pub audio: Vec<String>,
    pub video: Vec<String>,
    pub document: Vec<String>,
}

/// Get supported metadata types
async fn get_supported_types() -> impl IntoResponse {
    let types = SupportedTypes {
        image: vec![
            "jpg".to_string(),
            "jpeg".to_string(),
            "tiff".to_string(),
            "tif".to_string(),
            "heic".to_string(),
            "heif".to_string(),
        ],
        audio: vec![
            "mp3".to_string(),
            "flac".to_string(),
            "ogg".to_string(),
            "m4a".to_string(),
            "wav".to_string(),
        ],
        video: vec!["mp4".to_string(), "m4v".to_string(), "mov".to_string()],
        document: vec!["pdf".to_string(), "docx".to_string()],
    };
    Json(types)
}

/// Extract metadata from a file
async fn get_metadata(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let data_dir = PathBuf::from("./data");
    let full_path = data_dir.join(&path);

    // Security check
    let canonical_path = full_path
        .canonicalize()
        .map_err(|e| (StatusCode::NOT_FOUND, format!("File not found: {}", e)))?;

    let canonical_data = data_dir.canonicalize().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Data directory error: {}", e),
        )
    })?;

    if !canonical_path.starts_with(&canonical_data) {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    if !canonical_path.exists() || !canonical_path.is_file() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let file_name = canonical_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let extension = canonical_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let file_size = std::fs::metadata(&canonical_path)
        .map(|m| m.len())
        .unwrap_or(0);

    let mime_type = mime_guess::from_path(&canonical_path)
        .first_or_octet_stream()
        .to_string();

    let (metadata_type, metadata) = match extension.as_str() {
        "jpg" | "jpeg" | "tiff" | "tif" | "heic" | "heif" => extract_exif_metadata(&canonical_path),
        "mp3" => extract_id3_metadata(&canonical_path),
        "pdf" => extract_pdf_metadata(&canonical_path),
        _ => (
            "generic".to_string(),
            extract_generic_metadata(&canonical_path),
        ),
    };

    let result = FileMetadata {
        file_path: path,
        file_name,
        file_size,
        mime_type,
        metadata_type,
        metadata,
        extracted_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(result))
}

/// Extract EXIF metadata from image files
fn extract_exif_metadata(path: &PathBuf) -> (String, HashMap<String, MetadataValue>) {
    let mut metadata = HashMap::new();

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return ("image".to_string(), metadata),
    };

    let mut reader = BufReader::new(file);

    match exif::Reader::new().read_from_container(&mut reader) {
        Ok(exif_data) => {
            for field in exif_data.fields() {
                let tag_name = field.tag.to_string();
                let value = field.display_value().with_unit(&exif_data).to_string();

                // Map common EXIF fields to readable names
                let key = match field.tag {
                    exif::Tag::Make => "camera_make",
                    exif::Tag::Model => "camera_model",
                    exif::Tag::DateTimeOriginal => "date_taken",
                    exif::Tag::ExposureTime => "exposure_time",
                    exif::Tag::FNumber => "f_number",
                    exif::Tag::ISOSpeedRatings => "iso",
                    exif::Tag::FocalLength => "focal_length",
                    exif::Tag::ImageWidth => "width",
                    exif::Tag::ImageLength => "height",
                    exif::Tag::GPSLatitude => "gps_latitude",
                    exif::Tag::GPSLongitude => "gps_longitude",
                    exif::Tag::GPSAltitude => "gps_altitude",
                    exif::Tag::Orientation => "orientation",
                    exif::Tag::Software => "software",
                    exif::Tag::Artist => "artist",
                    exif::Tag::Copyright => "copyright",
                    exif::Tag::Flash => "flash",
                    exif::Tag::WhiteBalance => "white_balance",
                    exif::Tag::LensModel => "lens_model",
                    _ => continue, // Skip unknown tags to reduce noise
                };

                metadata.insert(key.to_string(), MetadataValue::String(value));
            }
        }
        Err(_) => {
            // No EXIF data found
        }
    }

    ("image".to_string(), metadata)
}

/// Extract ID3 metadata from MP3 files
fn extract_id3_metadata(path: &PathBuf) -> (String, HashMap<String, MetadataValue>) {
    let mut metadata = HashMap::new();

    match id3::Tag::read_from_path(path) {
        Ok(tag) => {
            if let Some(title) = tag.title() {
                metadata.insert(
                    "title".to_string(),
                    MetadataValue::String(title.to_string()),
                );
            }
            if let Some(artist) = tag.artist() {
                metadata.insert(
                    "artist".to_string(),
                    MetadataValue::String(artist.to_string()),
                );
            }
            if let Some(album) = tag.album() {
                metadata.insert(
                    "album".to_string(),
                    MetadataValue::String(album.to_string()),
                );
            }
            if let Some(year) = tag.year() {
                metadata.insert("year".to_string(), MetadataValue::Integer(year as i64));
            }
            if let Some(genre) = tag.genre_parsed() {
                metadata.insert(
                    "genre".to_string(),
                    MetadataValue::String(genre.to_string()),
                );
            }
            if let Some(track) = tag.track() {
                metadata.insert("track".to_string(), MetadataValue::Integer(track as i64));
            }
            if let Some(total_tracks) = tag.total_tracks() {
                metadata.insert(
                    "total_tracks".to_string(),
                    MetadataValue::Integer(total_tracks as i64),
                );
            }
            if let Some(disc) = tag.disc() {
                metadata.insert("disc".to_string(), MetadataValue::Integer(disc as i64));
            }
            if let Some(album_artist) = tag.album_artist() {
                metadata.insert(
                    "album_artist".to_string(),
                    MetadataValue::String(album_artist.to_string()),
                );
            }

            // Check for duration in extended header
            if let Some(duration) = tag.duration() {
                metadata.insert(
                    "duration_ms".to_string(),
                    MetadataValue::Integer(duration as i64),
                );
            }

            // Comments
            for comment in tag.comments() {
                if !comment.text.is_empty() {
                    metadata.insert(
                        format!("comment_{}", comment.lang),
                        MetadataValue::String(comment.text.clone()),
                    );
                }
            }

            // Has cover art?
            let has_cover = tag.pictures().count() > 0;
            metadata.insert(
                "has_cover_art".to_string(),
                MetadataValue::Boolean(has_cover),
            );
        }
        Err(_) => {
            // No ID3 tags found
        }
    }

    ("audio".to_string(), metadata)
}

/// Extract PDF metadata
fn extract_pdf_metadata(path: &PathBuf) -> (String, HashMap<String, MetadataValue>) {
    let mut metadata = HashMap::new();

    match lopdf::Document::load(path) {
        Ok(doc) => {
            // Get document info dictionary
            if let Ok(info_dict) = doc.trailer.get(b"Info") {
                if let Ok(info_ref) = info_dict.as_reference() {
                    if let Ok(info) = doc.get_dictionary(info_ref) {
                        for (key, value) in info.iter() {
                            let key_str = String::from_utf8_lossy(key).to_string();
                            let value_str = match value {
                                lopdf::Object::String(s, _) => {
                                    String::from_utf8_lossy(s).to_string()
                                }
                                lopdf::Object::Name(n) => String::from_utf8_lossy(n).to_string(),
                                _ => continue,
                            };

                            let readable_key = match key_str.as_str() {
                                "Title" => "title",
                                "Author" => "author",
                                "Subject" => "subject",
                                "Keywords" => "keywords",
                                "Creator" => "creator",
                                "Producer" => "producer",
                                "CreationDate" => "creation_date",
                                "ModDate" => "modification_date",
                                _ => continue,
                            };

                            metadata
                                .insert(readable_key.to_string(), MetadataValue::String(value_str));
                        }
                    }
                }
            }

            // Page count
            let page_count = doc.get_pages().len();
            metadata.insert(
                "page_count".to_string(),
                MetadataValue::Integer(page_count as i64),
            );

            // PDF version
            metadata.insert(
                "pdf_version".to_string(),
                MetadataValue::String(format!("{}.{}", doc.version.0, doc.version.1)),
            );
        }
        Err(_) => {
            // Failed to parse PDF
        }
    }

    ("document".to_string(), metadata)
}

/// Extract generic file metadata
fn extract_generic_metadata(path: &PathBuf) -> HashMap<String, MetadataValue> {
    let mut metadata = HashMap::new();

    if let Ok(file_meta) = std::fs::metadata(path) {
        if let Ok(created) = file_meta.created() {
            if let Ok(duration) = created.duration_since(std::time::UNIX_EPOCH) {
                metadata.insert(
                    "created_timestamp".to_string(),
                    MetadataValue::Integer(duration.as_secs() as i64),
                );
            }
        }

        if let Ok(modified) = file_meta.modified() {
            if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                metadata.insert(
                    "modified_timestamp".to_string(),
                    MetadataValue::Integer(duration.as_secs() as i64),
                );
            }
        }

        if let Ok(accessed) = file_meta.accessed() {
            if let Ok(duration) = accessed.duration_since(std::time::UNIX_EPOCH) {
                metadata.insert(
                    "accessed_timestamp".to_string(),
                    MetadataValue::Integer(duration.as_secs() as i64),
                );
            }
        }

        metadata.insert(
            "is_readonly".to_string(),
            MetadataValue::Boolean(file_meta.permissions().readonly()),
        );
    }

    metadata
}
