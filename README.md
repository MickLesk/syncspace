# SyncSpace Prototype

This repository contains a cross‑platform synchronisation tool called **SyncSpace**. It consists of a Rust backend and a browser‑based frontend built with [Lit](https://lit.dev/) and styled in a Material 3 expressive aesthetic. The system allows you to manage a local folder, synchronise files, manage peers and receive live updates via WebSockets.

## Directory structure

- `backend` – Rust backend exposing a REST API and WebSocket for file operations, peer management, search, rename and stats.
- `frontend` – Material‑inspired web UI built with Lit.  You can run it directly in a browser or embed it in Tauri or Electron.
- `desktop-app` – Placeholder for a Tauri configuration. A desktop app can embed the frontend here.
- `mobile-app` – Placeholder for a Flutter app. Use `flutter create` to generate the mobile client.

## Running locally

1. **Backend:** Navigate to `backend` and run the server with Cargo (requires Rust and Cargo installed):

   ```bash
   cd backend
   cargo run
   ```

   The backend listens on `http://localhost:8080`. It automatically creates a `data` folder for synchronised files and a `config.json` for peers and settings.

2. **Frontend:** Open `frontend/index.html` in a browser or serve the `frontend` folder with a static file server (e.g. using `python -m http.server`). The UI connects to the backend at `http://localhost:8080` and `ws://localhost:8080`.

## Features

- **File browsing:** Navigate through directories, download files, rename or delete entries and create new folders.
- **Upload:** Upload files to any subfolder using the upload widget.
- **Search:** Perform case‑insensitive searches across all files and directories.
- **Stats:** View the total number of files and their combined size.
- **Peers:** Add peers via the API; peer information is persisted in `config.json`.
- **Live updates:** The backend emits file system events via WebSocket. The UI automatically refreshes on changes.

