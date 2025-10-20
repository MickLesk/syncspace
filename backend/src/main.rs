//! Synchronisations‑Backend für SyncSpace.
//!
//! Dieses kleine Backend nutzt Warp als HTTP‑Server und stellt einfache APIs
//! bereit, um Dateien in einem lokalen Verzeichnis aufzulisten und hochzuladen.
//! Die Struktur kann später erweitert werden, um syncthing‑ähnliche
//! Synchronisationsmechanismen zu integrieren.

use std::convert::Infallible;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;
use warp::{http::StatusCode, Filter};

/// Informationen über eine Datei oder ein Verzeichnis im Datenordner.
#[derive(Serialize)]
struct FileInfo {
    name: String,
    is_dir: bool,
    size: u64,
}

/// API‑Handler, der alle Dateien im `data`‑Verzeichnis auflistet.
async fn list_files_handler() -> Result<impl warp::Reply, Infallible> {
    let dir_path = Path::new("./data");
    // Stelle sicher, dass das Datenverzeichnis existiert
    if !dir_path.exists() {
        if let Err(e) = fs::create_dir_all(dir_path).await {
            eprintln!("Fehler beim Erstellen des data‑Verzeichnisses: {}", e);
        }
    }

    let mut entries = Vec::new();
    match fs::read_dir(dir_path).await {
        Ok(mut read_dir) => {
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                if let Ok(meta) = entry.metadata().await {
                    entries.push(FileInfo {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        is_dir: meta.is_dir(),
                        size: meta.len(),
                    });
                }
            }
        }
        Err(e) => {
            eprintln!("Fehler beim Lesen des data‑Verzeichnisses: {}", e);
        }
    }
    Ok(warp::reply::json(&entries))
}

/// API‑Handler zum Hochladen einer Datei. Der Dateiname wird aus der URL
/// entnommen, der Request‑Body enthält die Rohdaten.
async fn upload_file_handler(filename: String, bytes: bytes::Bytes) -> Result<impl warp::Reply, Infallible> {
    let dir_path = Path::new("./data");
    if !dir_path.exists() {
        if let Err(e) = fs::create_dir_all(dir_path).await {
            eprintln!("Fehler beim Erstellen des data‑Verzeichnisses: {}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    let file_path: PathBuf = dir_path.join(&filename);
    match fs::write(&file_path, &bytes).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Fehler beim Schreiben der Datei {}: {}", filename, e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() {
    // Route zum Auflisten der Dateien: GET /api/files
    let list_files = warp::path("api")
        .and(warp::path("files"))
        .and(warp::path::end())
        .and_then(list_files_handler);

    // Route zum Hochladen von Dateien: POST /api/upload/<filename>
    let upload = warp::path("api")
        .and(warp::path("upload"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(upload_file_handler);

    // Kombiniere die Routen und aktiviere CORS für Browserzugriffe
    let routes = list_files
        .or(upload)
        .with(warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST"])
            .allow_headers(vec!["Content-Type"]))
        .recover(|rej| async move {
            eprintln!("Unhandled rejection: {:?}", rej);
            Ok::<_, Infallible>(warp::reply::with_status(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    println!("SyncSpace Backend läuft auf http://localhost:8080/api/files");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}