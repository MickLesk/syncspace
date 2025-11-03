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
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::auth::UserInfo;

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
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement peer discovery/listing
    
    Ok(Json(Vec::<Peer>::new()))
}

/// Add a new peer
async fn add_peer(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<AddPeerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let peer_id = Uuid::new_v4();
    
    // TODO: Implement peer registration
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "id": peer_id,
        "name": req.name,
        "address": req.address,
        "message": "Peer added successfully"
    }))))
}

/// Build peers router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/peers", get(list_peers).post(add_peer))
}
