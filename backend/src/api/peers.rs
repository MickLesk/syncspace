//! P2P Peers API Routes

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub status: String, // online, offline, syncing
    pub last_seen: DateTime<Utc>,
    pub sync_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddPeerRequest {
    pub name: String,
    pub address: String,
}

/// List all peers
async fn list_peers(
    State(_state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Log access for audit
    eprintln!("Peer list accessed by: {}", user_info.username);

    // TODO: Query peers table from database
    // For now return empty list (P2P feature not yet fully implemented)

    Ok(Json(Vec::<Peer>::new()))
}

/// Add a new peer
async fn add_peer(
    State(_state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<AddPeerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let peer_id = Uuid::new_v4();

    // Log peer addition
    eprintln!(
        "Peer added by {}: {} at {}",
        user_info.username, req.name, req.address
    );

    // TODO: Insert into peers table with user ownership
    // sqlx::query("INSERT INTO peers (id, name, address, owner_id) VALUES (?, ?, ?, ?)")
    //     .bind(&peer_id.to_string())
    //     .bind(&req.name)
    //     .bind(&req.address)
    //     .bind(&user_info.id)
    //     .execute(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": peer_id,
            "name": req.name,
            "address": req.address,
            "message": "Peer added successfully"
        })),
    ))
}

/// Build peers router
pub fn router() -> Router<AppState> {
    Router::new().route("/peers", get(list_peers).post(add_peer))
}
