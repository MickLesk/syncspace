//! P2P Peers API Routes

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub address: String,
    pub status: String, // online, offline, syncing
    pub last_seen: String,
    pub sync_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddPeerRequest {
    pub name: String,
    pub address: String,
}

/// List all peers
async fn list_peers(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Log access for audit
    eprintln!("Peer list accessed by: {}", user_info.username);

    // Query peers table from database
    let peers: Vec<Peer> = sqlx::query_as(
        "SELECT id, name, address, status, last_seen, sync_enabled 
         FROM peers 
         WHERE user_id = ? 
         ORDER BY created_at DESC"
    )
    .bind(&user_info.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch peers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(peers))
}

/// Add a new peer
async fn add_peer(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<AddPeerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let peer_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    // Log peer addition
    eprintln!(
        "Peer added by {}: {} at {}",
        user_info.username, req.name, req.address
    );

    // Insert into peers table with user ownership
    sqlx::query(
        "INSERT INTO peers (id, user_id, name, address, status, last_seen, sync_enabled, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'offline', ?, 1, ?, ?)"
    )
    .bind(&peer_id)
    .bind(&user_info.id)
    .bind(&req.name)
    .bind(&req.address)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to add peer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": peer_id,
            "name": req.name,
            "address": req.address,
            "status": "offline",
            "sync_enabled": true,
            "message": "Peer added successfully"
        })),
    ))
}

/// Build peers router
pub fn router() -> Router<AppState> {
    Router::new().route("/peers", get(list_peers).post(add_peer))
}
