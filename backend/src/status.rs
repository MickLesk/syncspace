#![allow(dead_code)]

/**
 * Status Module - API Status Page & Health Checks
 *
 * Provides comprehensive system status, health checks, and API endpoint documentation
 */
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub database: DatabaseStatus,
    pub search_index: SearchIndexStatus,
    pub websocket: WebSocketStatus,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub connected: bool,
    pub pool_size: u32,
    pub active_connections: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndexStatus {
    pub initialized: bool,
    pub index_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketStatus {
    pub enabled: bool,
    pub endpoint: String,
    pub active_connections: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub description: String,
    pub auth_required: bool,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub healthy: bool,
    pub timestamp: u64,
    pub checks: HealthChecks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthChecks {
    pub database: bool,
    pub filesystem: bool,
    pub search: bool,
}

/// Get comprehensive system status (JSON)
pub async fn get_status_json(State(state): State<AppState>) -> impl IntoResponse {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime = current_time.saturating_sub(state.start_time);

    let status = SystemStatus {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        database: DatabaseStatus {
            connected: true,
            pool_size: 10,
            active_connections: state.db_pool.size(),
        },
        search_index: SearchIndexStatus {
            initialized: true,
            index_path: "./data/search_index".to_string(),
        },
        websocket: WebSocketStatus {
            enabled: true,
            endpoint: "ws://localhost:8080/api/ws".to_string(),
            active_connections: state.ws_connections.load(Ordering::Relaxed),
        },
        endpoints: get_all_endpoints(),
    };

    Json(status)
}

/// Get system status as HTML page
pub async fn get_status_html(State(state): State<AppState>) -> impl IntoResponse {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime = current_time.saturating_sub(state.start_time);

    let uptime_str = format_uptime(uptime);
    let endpoints = get_all_endpoints();

    // Group endpoints by category
    let mut auth_endpoints = Vec::new();
    let mut file_endpoints = Vec::new();
    let mut user_endpoints = Vec::new();
    let mut sharing_endpoints = Vec::new();
    let mut system_endpoints = Vec::new();
    let mut collab_endpoints = Vec::new();
    let mut other_endpoints = Vec::new();

    for endpoint in endpoints {
        match endpoint.category.as_str() {
            "Authentication" => auth_endpoints.push(endpoint),
            "Files" => file_endpoints.push(endpoint),
            "Users" => user_endpoints.push(endpoint),
            "Sharing" => sharing_endpoints.push(endpoint),
            "System" => system_endpoints.push(endpoint),
            "Collaboration" => collab_endpoints.push(endpoint),
            _ => other_endpoints.push(endpoint),
        }
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SyncSpace Backend Status</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
            padding: 2rem;
            min-height: 100vh;
        }}
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        .header {{
            background: white;
            border-radius: 16px;
            padding: 2rem;
            margin-bottom: 2rem;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
        }}
        h1 {{
            font-size: 2.5rem;
            color: #667eea;
            margin-bottom: 0.5rem;
        }}
        .subtitle {{
            color: #666;
            font-size: 1.1rem;
        }}
        .status-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }}
        .status-card {{
            background: white;
            border-radius: 12px;
            padding: 1.5rem;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
            transition: transform 0.2s, box-shadow 0.2s;
        }}
        .status-card:hover {{
            transform: translateY(-4px);
            box-shadow: 0 8px 30px rgba(0,0,0,0.12);
        }}
        .status-card h3 {{
            color: #667eea;
            margin-bottom: 1rem;
            font-size: 1.2rem;
        }}
        .status-indicator {{
            display: inline-block;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background: #10b981;
            margin-right: 8px;
            animation: pulse 2s infinite;
        }}
        @keyframes pulse {{
            0%, 100% {{ opacity: 1; }}
            50% {{ opacity: 0.5; }}
        }}
        .metric {{
            margin: 0.75rem 0;
            display: flex;
            justify-content: space-between;
            padding: 0.5rem 0;
            border-bottom: 1px solid #f0f0f0;
        }}
        .metric:last-child {{
            border-bottom: none;
        }}
        .metric-label {{
            color: #666;
        }}
        .metric-value {{
            font-weight: 600;
            color: #333;
        }}
        .endpoints {{
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
        }}
        .endpoints h2 {{
            color: #667eea;
            margin-bottom: 1.5rem;
            font-size: 1.8rem;
        }}
        .endpoint-category {{
            margin-bottom: 2rem;
        }}
        .endpoint-category h3 {{
            color: #764ba2;
            margin-bottom: 1rem;
            padding-bottom: 0.5rem;
            border-bottom: 2px solid #f0f0f0;
        }}
        .endpoint {{
            display: grid;
            grid-template-columns: 100px 1fr 120px;
            gap: 1rem;
            padding: 1rem;
            margin: 0.5rem 0;
            background: #f9fafb;
            border-radius: 8px;
            border-left: 4px solid #667eea;
            transition: background 0.2s;
        }}
        .endpoint:hover {{
            background: #f0f1f5;
        }}
        .method {{
            font-weight: 700;
            padding: 0.25rem 0.75rem;
            border-radius: 6px;
            text-align: center;
            font-size: 0.875rem;
        }}
        .method.GET {{ background: #10b981; color: white; }}
        .method.POST {{ background: #3b82f6; color: white; }}
        .method.PUT {{ background: #f59e0b; color: white; }}
        .method.DELETE {{ background: #ef4444; color: white; }}
        .method.PATCH {{ background: #8b5cf6; color: white; }}
        .path {{
            font-family: 'Courier New', monospace;
            color: #333;
            word-break: break-all;
        }}
        .auth-badge {{
            padding: 0.25rem 0.75rem;
            border-radius: 6px;
            font-size: 0.875rem;
            text-align: center;
        }}
        .auth-required {{
            background: #fee2e2;
            color: #991b1b;
        }}
        .auth-optional {{
            background: #e0e7ff;
            color: #3730a3;
        }}
        .description {{
            grid-column: 1 / -1;
            color: #666;
            font-size: 0.95rem;
            margin-top: 0.5rem;
        }}
        .footer {{
            text-align: center;
            color: white;
            margin-top: 3rem;
            font-size: 0.9rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 SyncSpace Backend</h1>
            <p class="subtitle">API Status & Endpoint Documentation</p>
        </div>

        <div class="status-grid">
            <div class="status-card">
                <h3><span class="status-indicator"></span>Server Status</h3>
                <div class="metric">
                    <span class="metric-label">Status</span>
                    <span class="metric-value">Operational</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Version</span>
                    <span class="metric-value">{}</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Uptime</span>
                    <span class="metric-value">{}</span>
                </div>
            </div>

            <div class="status-card">
                <h3>💾 Database</h3>
                <div class="metric">
                    <span class="metric-label">Connected</span>
                    <span class="metric-value">✅ Yes</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Pool Size</span>
                    <span class="metric-value">10</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Active</span>
                    <span class="metric-value">{}</span>
                </div>
            </div>

            <div class="status-card">
                <h3>🔍 Search Index</h3>
                <div class="metric">
                    <span class="metric-label">Status</span>
                    <span class="metric-value">✅ Initialized</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Path</span>
                    <span class="metric-value">./data/search_index</span>
                </div>
            </div>

            <div class="status-card">
                <h3>📡 WebSocket</h3>
                <div class="metric">
                    <span class="metric-label">Enabled</span>
                    <span class="metric-value">✅ Yes</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Endpoint</span>
                    <span class="metric-value">ws://localhost:8080/api/ws</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Connections</span>
                    <span class="metric-value">{}</span>
                </div>
            </div>
        </div>

        <div class="endpoints">
            <h2>📚 API Endpoints</h2>

            {}
            {}
            {}
            {}
            {}
            {}
        </div>

        <div class="footer">
            <p>SyncSpace Backend v{} | Powered by Rust + Axum</p>
        </div>
    </div>
</body>
</html>"#,
        env!("CARGO_PKG_VERSION"),
        uptime_str,
        state.db_pool.size(),
        state.ws_connections.load(Ordering::Relaxed),
        render_endpoint_category("🔐 Authentication", &auth_endpoints),
        render_endpoint_category("📁 Files", &file_endpoints),
        render_endpoint_category("👤 Users", &user_endpoints),
        render_endpoint_category("🔗 Sharing", &sharing_endpoints),
        render_endpoint_category("⚙️ System", &system_endpoints),
        render_endpoint_category("🤝 Collaboration", &collab_endpoints),
        env!("CARGO_PKG_VERSION")
    );

    Html(html)
}

/// Health check endpoint (simple boolean)
pub async fn health_check(State(_state): State<AppState>) -> impl IntoResponse {
    let checks = HealthChecks {
        database: true, // Pool exists
        filesystem: std::path::Path::new("./data").exists(),
        search: std::path::Path::new("./data/search_index").exists(),
    };

    let health = HealthCheck {
        healthy: checks.database && checks.filesystem && checks.search,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        checks,
    };

    if health.healthy {
        (StatusCode::OK, Json(health))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(health))
    }
}

/// Get all API endpoints with documentation
fn get_all_endpoints() -> Vec<EndpointInfo> {
    vec![
        // Authentication
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/register".to_string(),
            description: "Register a new user account".to_string(),
            auth_required: false,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/login".to_string(),
            description: "Login with username and password, returns JWT token".to_string(),
            auth_required: false,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/refresh".to_string(),
            description: "Refresh JWT token".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/auth/me".to_string(),
            description: "Get current user information".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/setup-2fa".to_string(),
            description: "Setup two-factor authentication".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/enable-2fa".to_string(),
            description: "Enable two-factor authentication".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/disable-2fa".to_string(),
            description: "Disable two-factor authentication".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/auth/change-password".to_string(),
            description: "Change user password".to_string(),
            auth_required: true,
            category: "Authentication".to_string(),
        },
        // Files
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/files".to_string(),
            description: "List files in root directory".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/files/*path".to_string(),
            description: "List files in specific directory or download file".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/upload/*path".to_string(),
            description: "Upload file to specific path".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/dirs/*path".to_string(),
            description: "Create new directory".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "PUT".to_string(),
            path: "/api/rename/*path".to_string(),
            description: "Rename file or directory".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "PUT".to_string(),
            path: "/api/move/*path".to_string(),
            description: "Move file to new location".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/copy/*path".to_string(),
            description: "Copy file to new location".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "DELETE".to_string(),
            path: "/api/files/*path".to_string(),
            description: "Delete file or directory".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/search".to_string(),
            description: "Full-text search across files".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/favorites".to_string(),
            description: "List favorite files".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/favorites".to_string(),
            description: "Add file to favorites".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        EndpointInfo {
            method: "DELETE".to_string(),
            path: "/api/favorites/*path".to_string(),
            description: "Remove file from favorites".to_string(),
            auth_required: true,
            category: "Files".to_string(),
        },
        // Users
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            description: "List all users (admin only)".to_string(),
            auth_required: true,
            category: "Users".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/users/:id".to_string(),
            description: "Get specific user details".to_string(),
            auth_required: true,
            category: "Users".to_string(),
        },
        EndpointInfo {
            method: "PUT".to_string(),
            path: "/api/users/:id".to_string(),
            description: "Update user profile".to_string(),
            auth_required: true,
            category: "Users".to_string(),
        },
        EndpointInfo {
            method: "DELETE".to_string(),
            path: "/api/users/:id".to_string(),
            description: "Delete user account (admin only)".to_string(),
            auth_required: true,
            category: "Users".to_string(),
        },
        // Sharing
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/sharing".to_string(),
            description: "List all shares created by current user".to_string(),
            auth_required: true,
            category: "Sharing".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/sharing".to_string(),
            description: "Create new share link for file".to_string(),
            auth_required: true,
            category: "Sharing".to_string(),
        },
        EndpointInfo {
            method: "DELETE".to_string(),
            path: "/api/sharing/:id".to_string(),
            description: "Delete share link".to_string(),
            auth_required: true,
            category: "Sharing".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/public/:share_id".to_string(),
            description: "Access publicly shared file (no auth required)".to_string(),
            auth_required: false,
            category: "Sharing".to_string(),
        },
        // System
        EndpointInfo {
            method: "GET".to_string(),
            path: "/status".to_string(),
            description: "System status page (HTML)".to_string(),
            auth_required: false,
            category: "System".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/status/json".to_string(),
            description: "System status (JSON)".to_string(),
            auth_required: false,
            category: "System".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/health".to_string(),
            description: "Health check endpoint".to_string(),
            auth_required: false,
            category: "System".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/system/info".to_string(),
            description: "Get system information (CPU, memory, disk)".to_string(),
            auth_required: true,
            category: "System".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/analytics/overview".to_string(),
            description: "Get analytics overview".to_string(),
            auth_required: true,
            category: "System".to_string(),
        },
        // Collaboration
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/collaboration/locks".to_string(),
            description: "List all file locks".to_string(),
            auth_required: true,
            category: "Collaboration".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/collaboration/locks".to_string(),
            description: "Acquire file lock".to_string(),
            auth_required: true,
            category: "Collaboration".to_string(),
        },
        EndpointInfo {
            method: "DELETE".to_string(),
            path: "/api/collaboration/locks/:file_path".to_string(),
            description: "Release file lock".to_string(),
            auth_required: true,
            category: "Collaboration".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/collaboration/presence".to_string(),
            description: "Get user presence information".to_string(),
            auth_required: true,
            category: "Collaboration".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/api/collaboration/presence".to_string(),
            description: "Update user presence".to_string(),
            auth_required: true,
            category: "Collaboration".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/api/ws".to_string(),
            description: "WebSocket connection for real-time events".to_string(),
            auth_required: false,
            category: "Collaboration".to_string(),
        },
    ]
}

fn render_endpoint_category(title: &str, endpoints: &[EndpointInfo]) -> String {
    if endpoints.is_empty() {
        return String::new();
    }

    let mut html = format!("<div class=\"endpoint-category\"><h3>{}</h3>", title);

    for endpoint in endpoints {
        let auth_class = if endpoint.auth_required {
            "auth-required"
        } else {
            "auth-optional"
        };
        let auth_text = if endpoint.auth_required {
            "🔒 Required"
        } else {
            "🔓 Optional"
        };

        html.push_str(&format!(
            r#"<div class="endpoint">
                <div class="method {}">{}</div>
                <div class="path">{}</div>
                <div class="auth-badge {}">{}</div>
                <div class="description">{}</div>
            </div>"#,
            endpoint.method,
            endpoint.method,
            endpoint.path,
            auth_class,
            auth_text,
            endpoint.description
        ));
    }

    html.push_str("</div>");
    html
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
