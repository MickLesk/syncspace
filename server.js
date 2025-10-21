#!/usr/bin/env node

const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 8080;
const FRONTEND_DIR = path.join(__dirname, 'frontend');

const server = http.createServer((req, res) => {
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  // Serve index.html for root
  if (req.url === '/' || req.url === '') {
    const filePath = path.join(FRONTEND_DIR, 'index.html');
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    res.writeHead(200);
    res.end(fs.readFileSync(filePath, 'utf-8'));
    return;
  }

  // Serve static files
  if (req.url.startsWith('/static/')) {
    const filePath = path.join(FRONTEND_DIR, req.url.substring(8)); // Remove /static/
    try {
      const content = fs.readFileSync(filePath);
      const ext = path.extname(filePath);
      const mimeTypes = {
        '.html': 'text/html',
        '.css': 'text/css',
        '.js': 'application/javascript',
        '.json': 'application/json',
        '.png': 'image/png',
        '.jpg': 'image/jpeg',
        '.gif': 'image/gif',
        '.svg': 'image/svg+xml'
      };
      res.setHeader('Content-Type', mimeTypes[ext] || 'application/octet-stream');
      res.writeHead(200);
      res.end(content);
    } catch (e) {
      res.writeHead(404);
      res.end('Not found');
    }
    return;
  }

  // 404
  res.writeHead(404);
  res.end('Not found');
});

server.listen(PORT, () => {
  console.log(`✅ SyncSpace Frontend Server läuft auf http://localhost:${PORT}`);
  console.log(`📁 Serving from: ${FRONTEND_DIR}`);
});
