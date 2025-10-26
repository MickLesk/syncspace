import { writable } from 'svelte/store';
import { createWebSocket } from '../lib/api.js';

// WebSocket connection state
export const wsConnected = writable(false);
export const wsEvents = writable([]);
export const lastFileEvent = writable(null);

class WebSocketManager {
  constructor() {
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000; // Start with 1 second
    this.maxReconnectDelay = 30000; // Max 30 seconds
    this.eventHandlers = new Map();
    this.isConnecting = false;
    
    // Don't auto-connect! Let components decide when to connect.
    // Call websocketManager.connect() explicitly when needed.
  }

  connect() {
    try {
      console.log('🔌 Connecting to WebSocket...');
      this.ws = createWebSocket();
      
      this.ws.onopen = () => {
        console.log('✅ WebSocket connected');
        wsConnected.set(true);
        this.reconnectAttempts = 0;
        this.reconnectDelay = 1000;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          console.log('📨 WebSocket message received:', data);
          
          // Add to events store
          wsEvents.update(events => [...events.slice(-49), data]);
          
          // Handle file system events
          if (data.kind && data.path) {
            lastFileEvent.set(data);
            this.handleFileEvent(data);
          }
          
          // Trigger specific event handlers
          if (this.eventHandlers.has(data.type || 'file_change')) {
            this.eventHandlers.get(data.type || 'file_change').forEach(handler => {
              try {
                handler(data);
              } catch (error) {
                console.error('Error in event handler:', error);
              }
            });
          }
        } catch (error) {
          console.error('❌ Failed to parse WebSocket message:', error);
        }
      };

      this.ws.onclose = (event) => {
        console.log('🔌 WebSocket disconnected:', event.code, event.reason);
        wsConnected.set(false);
        
        if (event.code !== 1000 && this.reconnectAttempts < this.maxReconnectAttempts) {
          this.scheduleReconnect();
        }
      };

      this.ws.onerror = (error) => {
        console.error('❌ WebSocket error:', error);
        wsConnected.set(false);
      };
      
    } catch (error) {
      console.error('❌ Failed to create WebSocket:', error);
      this.scheduleReconnect();
    }
  }

  scheduleReconnect() {
    this.reconnectAttempts++;
    const delay = Math.min(this.reconnectDelay * Math.pow(2, this.reconnectAttempts), this.maxReconnectDelay);
    
    console.log(`🔄 Scheduling reconnect attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts} in ${delay}ms`);
    
    setTimeout(() => {
      if (this.reconnectAttempts <= this.maxReconnectAttempts) {
        this.connect();
      } else {
        console.error('❌ Max reconnection attempts reached');
      }
    }, delay);
  }

  handleFileEvent(event) {
    console.log('📁 File system event:', event);
    
    // Broadcast to all file-related components
    this.emit('file_change', event);
    
    // Handle specific event types
    switch (event.kind) {
      case 'created':
        console.log(`📄 File created: ${event.path}`);
        this.emit('file_created', event);
        break;
      case 'modified':
        console.log(`✏️ File modified: ${event.path}`);
        this.emit('file_modified', event);
        break;
      case 'deleted':
        console.log(`🗑️ File deleted: ${event.path}`);
        this.emit('file_deleted', event);
        break;
      case 'renamed':
        console.log(`📝 File renamed: ${event.path}`);
        this.emit('file_renamed', event);
        break;
    }
  }

  // Event subscription system
  on(eventType, handler) {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, new Set());
    }
    this.eventHandlers.get(eventType).add(handler);
    
    // Return unsubscribe function
    return () => {
      const handlers = this.eventHandlers.get(eventType);
      if (handlers) {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.eventHandlers.delete(eventType);
        }
      }
    };
  }

  // Emit events to handlers
  emit(eventType, data) {
    if (this.eventHandlers.has(eventType)) {
      this.eventHandlers.get(eventType).forEach(handler => {
        try {
          handler(data);
        } catch (error) {
          console.error(`Error in ${eventType} handler:`, error);
        }
      });
    }
  }

  // Send message to server
  send(data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    } else {
      console.warn('⚠️ WebSocket not connected, cannot send message');
    }
  }

  // Disconnect
  disconnect() {
    if (this.ws) {
      this.ws.close(1000, 'Manual disconnect');
      this.ws = null;
    }
    wsConnected.set(false);
  }
}

// Create global WebSocket manager instance
export const websocketManager = new WebSocketManager();

// Convenience functions for components
export function onFileEvent(handler) {
  return websocketManager.on('file_change', handler);
}

export function onFileCreated(handler) {
  return websocketManager.on('file_created', handler);
}

export function onFileModified(handler) {
  return websocketManager.on('file_modified', handler);
}

export function onFileDeleted(handler) {
  return websocketManager.on('file_deleted', handler);
}

export function onFileRenamed(handler) {
  return websocketManager.on('file_renamed', handler);
}