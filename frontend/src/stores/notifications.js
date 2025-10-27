/**
 * Notifications Store (Backend-synchronized with WebSocket)
 * Real-time notification management for user alerts
 */

import { writable, derived } from 'svelte/store';
import { get as apiGet, put as apiPut, deleteRequest as apiDelete } from '../lib/api.js';

// Notifications store
const notificationsStore = writable([]);

// WebSocket connection for real-time updates
let ws = null;
let reconnectTimeout = null;
let reconnectAttempts = 0;
const MAX_RECONNECT_ATTEMPTS = 5;
const RECONNECT_DELAY = 3000;

// Create enhanced notifications store
const createNotificationsStore = () => {
  const { subscribe, set, update } = notificationsStore;
  
  return {
    subscribe,
    
    // Load notifications from backend
    async load(isRead = null) {
      try {
        const params = new URLSearchParams();
        if (isRead !== null) {
          params.append('is_read', isRead ? '1' : '0');
        }
        
        const url = `/notifications${params.toString() ? '?' + params.toString() : ''}`;
        const notifications = await apiGet(url);
        
        set(Array.isArray(notifications) ? notifications : []);
        return notifications;
      } catch (error) {
        console.error('Failed to load notifications:', error);
        return [];
      }
    },
    
    // Mark notification as read
    async markRead(notificationId) {
      try {
        await apiPut(`/notifications/${notificationId}/read`);
        
        // Update local state
        update(notifications => 
          notifications.map(n => 
            n.id === notificationId ? { ...n, is_read: true } : n
          )
        );
        
        return true;
      } catch (error) {
        console.error('Failed to mark notification as read:', error);
        return false;
      }
    },
    
    // Mark all notifications as read
    async markAllRead() {
      try {
        let currentNotifications = [];
        const unsubscribe = subscribe(n => { currentNotifications = n; });
        unsubscribe();
        
        const unreadIds = currentNotifications
          .filter(n => !n.is_read)
          .map(n => n.id);
        
        // Mark each as read (parallel)
        await Promise.all(
          unreadIds.map(id => this.markRead(id))
        );
        
        return true;
      } catch (error) {
        console.error('Failed to mark all as read:', error);
        return false;
      }
    },
    
    // Delete notification
    async delete(notificationId) {
      try {
        await api.delete(`/notifications/${notificationId}`);
        
        // Remove from local state
        update(notifications => 
          notifications.filter(n => n.id !== notificationId)
        );
        
        return true;
      } catch (error) {
        console.error('Failed to delete notification:', error);
        return false;
      }
    },
    
    // Clear all notifications
    async clearAll() {
      try {
        let currentNotifications = [];
        const unsubscribe = subscribe(n => { currentNotifications = n; });
        unsubscribe();
        
        // Delete each notification (parallel)
        await Promise.all(
          currentNotifications.map(n => this.delete(n.id))
        );
        
        set([]);
        return true;
      } catch (error) {
        console.error('Failed to clear notifications:', error);
        return false;
      }
    },
    
    // Connect to WebSocket for real-time updates
    connectWebSocket() {
      if (ws && ws.readyState === WebSocket.OPEN) {
        console.log('[Notifications] WebSocket already connected');
        return;
      }
      
      try {
        const token = localStorage.getItem('authToken');
        if (!token) {
          console.warn('[Notifications] No auth token, skipping WebSocket connection');
          return;
        }
        
        const wsUrl = `ws://localhost:8080/api/ws?token=${encodeURIComponent(token)}`;
        ws = new WebSocket(wsUrl);
        
        ws.onopen = () => {
          console.log('[Notifications] WebSocket connected');
          reconnectAttempts = 0;
          
          // Request notification updates
          ws.send(JSON.stringify({ type: 'subscribe', channel: 'notifications' }));
        };
        
        ws.onmessage = (event) => {
          try {
            const message = JSON.parse(event.data);
            
            if (message.type === 'notification') {
              // New notification received
              update(notifications => [message.data, ...notifications]);
              
              // Show browser notification if enabled
              if ('Notification' in window && Notification.permission === 'granted') {
                new Notification(message.data.title || 'SyncSpace', {
                  body: message.data.message,
                  icon: '/favicon.ico',
                  tag: message.data.id
                });
              }
            } else if (message.type === 'notification_update') {
              // Notification updated (e.g., marked as read)
              update(notifications => 
                notifications.map(n => 
                  n.id === message.data.id ? { ...n, ...message.data } : n
                )
              );
            } else if (message.type === 'notification_delete') {
              // Notification deleted
              update(notifications => 
                notifications.filter(n => n.id !== message.data.id)
              );
            }
          } catch (error) {
            console.error('[Notifications] Failed to parse WebSocket message:', error);
          }
        };
        
        ws.onerror = (error) => {
          console.error('[Notifications] WebSocket error:', error);
        };
        
        ws.onclose = () => {
          console.log('[Notifications] WebSocket closed');
          ws = null;
          
          // Auto-reconnect with exponential backoff
          if (reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
            const delay = RECONNECT_DELAY * Math.pow(2, reconnectAttempts);
            console.log(`[Notifications] Reconnecting in ${delay}ms (attempt ${reconnectAttempts + 1}/${MAX_RECONNECT_ATTEMPTS})`);
            
            reconnectTimeout = setTimeout(() => {
              reconnectAttempts++;
              this.connectWebSocket();
            }, delay);
          }
        };
      } catch (error) {
        console.error('[Notifications] Failed to connect WebSocket:', error);
      }
    },
    
    // Disconnect WebSocket
    disconnectWebSocket() {
      if (reconnectTimeout) {
        clearTimeout(reconnectTimeout);
        reconnectTimeout = null;
      }
      
      if (ws) {
        ws.close();
        ws = null;
      }
      
      reconnectAttempts = 0;
    },
    
    // Request browser notification permission
    async requestPermission() {
      if (!('Notification' in window)) {
        console.warn('Browser does not support notifications');
        return false;
      }
      
      if (Notification.permission === 'granted') {
        return true;
      }
      
      if (Notification.permission !== 'denied') {
        const permission = await Notification.requestPermission();
        return permission === 'granted';
      }
      
      return false;
    }
  };
};

export const notifications = createNotificationsStore();

// Derived stores for common queries
export const unreadNotifications = derived(
  notificationsStore,
  $notifications => $notifications.filter(n => !n.is_read)
);

export const unreadCount = derived(
  unreadNotifications,
  $unread => $unread.length
);

export const priorityNotifications = derived(
  notificationsStore,
  $notifications => $notifications.filter(n => n.priority === 'high' || n.priority === 'urgent')
);

// Helper to format notification time
export function formatNotificationTime(timestamp) {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);
  
  if (diffMins < 1) return 'Just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  
  return date.toLocaleDateString();
}
