import { writable, derived } from 'svelte/store';
import { createWebSocket } from '../lib/api.js';
import { success, info, warning, error as toastError } from './toast.js';

// WebSocket connection states
export const WS_STATES = {
  DISCONNECTED: 'disconnected',
  CONNECTING: 'connecting',
  CONNECTED: 'connected',
  RECONNECTING: 'reconnecting',
  ERROR: 'error',
  MAX_RETRIES_REACHED: 'max_retries_reached'
};

// WebSocket connection state
export const wsState = writable(WS_STATES.DISCONNECTED);
export const wsConnected = derived(wsState, $state => $state === WS_STATES.CONNECTED);
export const wsEvents = writable([]);
export const lastFileEvent = writable(null);
export const wsReconnectAttempts = writable(0);

class WebSocketManager {
  constructor() {
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 10;
    this.baseReconnectDelay = 1000; // Start with 1 second
    this.maxReconnectDelay = 30000; // Max 30 seconds
    this.eventHandlers = new Map();
    this.reconnectTimeout = null;
    this.heartbeatInterval = null;
    this.heartbeatIntervalMs = 30000; // Send heartbeat every 30 seconds
    this.heartbeatTimeout = null;
    this.heartbeatTimeoutMs = 5000; // Expect pong within 5 seconds
    this.missedHeartbeats = 0;
    this.maxMissedHeartbeats = 3;
    this.manualDisconnect = false;
    
    // Don't auto-connect! Let components decide when to connect.
    // Call websocketManager.connect() explicitly when needed.
  }

  connect() {
    // Prevent multiple simultaneous connection attempts
    if (this.ws && (this.ws.readyState === WebSocket.CONNECTING || this.ws.readyState === WebSocket.OPEN)) {
      console.log('⚠️ WebSocket already connecting or connected');
      return;
    }

    // Clear any existing reconnect timeout
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }

    this.manualDisconnect = false;
    
    try {
      const isReconnecting = this.reconnectAttempts > 0;
      wsState.set(isReconnecting ? WS_STATES.RECONNECTING : WS_STATES.CONNECTING);
      
      console.log(`🔌 ${isReconnecting ? 'Reconnecting' : 'Connecting'} to WebSocket... (attempt ${this.reconnectAttempts + 1}/${this.maxReconnectAttempts})`);
      
      this.ws = createWebSocket();
      
      this.ws.onopen = () => {
        console.log('✅ WebSocket connected');
        wsState.set(WS_STATES.CONNECTED);
        this.reconnectAttempts = 0;
        wsReconnectAttempts.set(0);
        this.missedHeartbeats = 0;
        
        // Show success notification only on reconnect (not initial connect)
        if (isReconnecting) {
          success('Reconnected to server', 3000);
        }
        
        // Start heartbeat mechanism
        this.startHeartbeat();
        
        // Emit connection event
        this.emit('connected', { timestamp: new Date().toISOString() });
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          
          console.log('📨 WebSocket message received:', data);
          
          // Add to events store (keep last 50 events)
          wsEvents.update(events => [...events.slice(-49), data]);
          
          // Handle file system events
          if (data.kind && data.path) {
            lastFileEvent.set(data);
            this.handleFileEvent(data);
          }
          
          // Trigger specific event handlers
          const eventType = data.type || 'file_change';
          if (this.eventHandlers.has(eventType)) {
            this.eventHandlers.get(eventType).forEach(handler => {
              try {
                handler(data);
              } catch (error) {
                console.error(`Error in ${eventType} handler:`, error);
              }
            });
          }
        } catch (error) {
          console.error('❌ Failed to parse WebSocket message:', error);
        }
      };

      this.ws.onclose = (event) => {
        console.log('🔌 WebSocket disconnected:', event.code, event.reason);
        
        // Clean up heartbeat
        this.stopHeartbeat();
        
        // Don't reconnect if it was a manual disconnect (code 1000) or max retries reached
        if (this.manualDisconnect || event.code === 1000) {
          wsState.set(WS_STATES.DISCONNECTED);
          return;
        }
        
        wsState.set(WS_STATES.DISCONNECTED);
        
        // Attempt reconnection
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
          this.scheduleReconnect();
        } else {
          wsState.set(WS_STATES.MAX_RETRIES_REACHED);
          toastError('Connection lost. Please refresh the page.', 10000);
          console.error('❌ Max reconnection attempts reached');
          wsReconnectAttempts.set(this.reconnectAttempts);
        }
      };

      this.ws.onerror = (error) => {
        console.error('❌ WebSocket error:', error);
        wsState.set(WS_STATES.ERROR);
      };
      
    } catch (error) {
      console.error('❌ Failed to create WebSocket:', error);
      wsState.set(WS_STATES.ERROR);
      this.scheduleReconnect();
    }
  }

  scheduleReconnect() {
    // Clear any existing reconnect timeout
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }

    this.reconnectAttempts++;
    wsReconnectAttempts.set(this.reconnectAttempts);
    
    // Exponential backoff: 1s, 2s, 4s, 8s, 16s, 30s (max)
    const delay = Math.min(
      this.baseReconnectDelay * Math.pow(2, this.reconnectAttempts - 1),
      this.maxReconnectDelay
    );
    
    console.log(`🔄 Scheduling reconnect attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts} in ${delay}ms`);
    wsState.set(WS_STATES.RECONNECTING);
    
    this.reconnectTimeout = setTimeout(() => {
      if (this.reconnectAttempts <= this.maxReconnectAttempts && !this.manualDisconnect) {
        this.connect();
      } else {
        console.error('❌ Max reconnection attempts reached');
        wsState.set(WS_STATES.MAX_RETRIES_REACHED);
      }
    }, delay);
  }

  startHeartbeat() {
    // Clean up existing heartbeat
    this.stopHeartbeat();
    
    console.log('💓 Starting heartbeat mechanism');
    
    // Increase heartbeat interval to 45 seconds (less aggressive)
    this.heartbeatIntervalMs = 45000;
    // Increase timeout to 10 seconds (more lenient)
    this.heartbeatTimeoutMs = 10000;
    
    // Send ping every 45 seconds
    this.heartbeatInterval = setInterval(() => {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        console.log('💓 Checking connection (passive)');
        
        // Don't send JSON ping - WebSocket protocol handles this automatically
        // Just check if connection is still alive
        // If connection drops, onclose will handle reconnection
        
        // Reset missed heartbeats since connection is still open
        if (this.missedHeartbeats > 0) {
          console.log('✅ Connection recovered, resetting missed heartbeats');
          this.missedHeartbeats = 0;
        }
      } else {
        this.missedHeartbeats++;
        console.warn(`⚠️ Connection not open ${this.missedHeartbeats}/${this.maxMissedHeartbeats}`);
        
        // If connection is dead for too long, force reconnect
        if (this.missedHeartbeats >= this.maxMissedHeartbeats) {
          console.error('❌ Connection dead for too long, forcing reconnect');
          this.stopHeartbeat();
          this.reconnect();
        }
      }
    }, this.heartbeatIntervalMs);
  }

  stopHeartbeat() {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
    
    if (this.heartbeatTimeout) {
      clearTimeout(this.heartbeatTimeout);
      this.heartbeatTimeout = null;
    }
    
    this.missedHeartbeats = 0;
  }

  handleFileEvent(event) {
    console.log('📁 File system event:', event);
    
    // Broadcast to all file-related components
    this.emit('file_change', event);
    
    // Show toast notifications for file events
    const fileName = event.path?.split('/').pop() || event.path;
    
    // Handle specific event types
    switch (event.kind) {
      case 'created':
        console.log(`📄 File created: ${event.path}`);
        success(`File created: ${fileName}`, 2500);
        this.emit('file_created', event);
        break;
      case 'modified':
        console.log(`✏️ File modified: ${event.path}`);
        info(`File modified: ${fileName}`, 2000);
        this.emit('file_modified', event);
        break;
      case 'deleted':
        console.log(`🗑️ File deleted: ${event.path}`);
        warning(`File deleted: ${fileName}`, 2500);
        this.emit('file_deleted', event);
        break;
      case 'renamed':
        console.log(`📝 File renamed: ${event.path}`);
        info(`File renamed: ${fileName}`, 2500);
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
    console.log('🔌 Manually disconnecting WebSocket');
    this.manualDisconnect = true;
    
    // Clear reconnect attempts
    this.reconnectAttempts = 0;
    wsReconnectAttempts.set(0);
    
    // Clear reconnect timeout
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
    
    // Stop heartbeat
    this.stopHeartbeat();
    
    // Close WebSocket connection
    if (this.ws) {
      this.ws.close(1000, 'Manual disconnect');
      this.ws = null;
    }
    
    wsState.set(WS_STATES.DISCONNECTED);
  }

  // Get current connection state
  getState() {
    if (!this.ws) return WS_STATES.DISCONNECTED;
    
    switch (this.ws.readyState) {
      case WebSocket.CONNECTING:
        return WS_STATES.CONNECTING;
      case WebSocket.OPEN:
        return WS_STATES.CONNECTED;
      case WebSocket.CLOSING:
      case WebSocket.CLOSED:
        return this.reconnectAttempts > 0 ? WS_STATES.RECONNECTING : WS_STATES.DISCONNECTED;
      default:
        return WS_STATES.DISCONNECTED;
    }
  }

  // Force reconnect (reset retry counter)
  forceReconnect() {
    console.log('🔄 Forcing reconnect...');
    this.reconnectAttempts = 0;
    wsReconnectAttempts.set(0);
    
    if (this.ws) {
      this.ws.close(4001, 'Force reconnect');
      this.ws = null;
    }
    
    this.connect();
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