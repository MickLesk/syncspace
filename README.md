# SyncApp-Prototyp

Dieses Repository enthält eine initiale Projektstruktur für eine plattformübergreifende Datei-Synchronisationsanwendung.

## Verzeichnisse

- `backend` – Rust-basierter Backend-Dienst (läuft auf `http://localhost:8080/hello`).  Er dient als Ausgangspunkt für die spätere Synchronisations-Engine.
- `frontend` – einfache Web-Oberfläche auf Basis von Web Components (Lit).  Die Dateien können über einen Webserver oder über Tauri/Electron geladen werden.
- `desktop-app` – Placeholder für eine Tauri-Konfiguration.  Hier wird später die Desktop-Anwendung gebaut, die die Web-Oberfläche einbettet.
- `mobile-app` – Platzhalter für die Flutter-App.  Nutzen Sie `flutter create`, um die mobile App zu generieren.

## Start

Zum Testen des Backends können Sie mit Rust (falls installiert) folgenden Befehl ausführen:

```bash
cd backend
rustc src/main.rs && ./main
```

Die Web-Oberfläche können Sie mit einem beliebigen Webserver hosten oder direkt als statische Dateien öffnen.

