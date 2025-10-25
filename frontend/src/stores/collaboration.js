/**
 * Real-time Collaboration Store
 * Handles file locking, user presence, and collaborative features
 */

import { writable, derived } from 'svelte/store';
import { createWebSocket } from '../lib/api.js';

// Store for file locks
export const fileLocks = writable([]);

// Store for user presence (who's online and what they're doing)
export const userPresence = writable([]);

// Store for collaboration activity
export const collaborationActivity = writable([]);

// Store for current user's locks
export const myLocks = writable([]);

// Derived store for files currently locked by others
export const lockedByOthers = derived(
  [fileLocks, myLocks],
  ([$fileLocks, $myLocks]) => {
    const myLockPaths = new Set($myLocks.map(lock => lock.file_path));
    return $fileLocks.filter(lock => !myLockPaths.has(lock.file_path));
  }
);

// Collaboration manager
class CollaborationManager {
  constructor() {
    this.websocket = null;
    this.currentFile = null;
    this.presenceUpdateInterval = null;
    this.lockRenewInterval = null;
  }

  // Initialize collaboration features
  async initialize() {
    try {
      await this.loadFileLocks();
      await this.loadUserPresence();
      await this.loadCollaborationActivity();
      this.setupWebSocketListeners();
      this.startPresenceUpdates();
    } catch (error) {
      console.error('Failed to initialize collaboration features:', error);
    }
  }

  // Load all current file locks
  async loadFileLocks() {
    try {
      const locks = await fetch('/api/collaboration/locks', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      }).then(r => r.json());
      
      fileLocks.set(locks);
      
      // Filter locks owned by current user
      const currentUserId = localStorage.getItem('userId');
      myLocks.set(locks.filter(lock => lock.locked_by_user_id === currentUserId));
    } catch (error) {
      console.error('Failed to load file locks:', error);
    }
  }

  // Load user presence information
  async loadUserPresence() {
    try {
      const presence = await fetch('/api/collaboration/presence', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      }).then(r => r.json());
      
      userPresence.set(presence);
    } catch (error) {
      console.error('Failed to load user presence:', error);
    }
  }

  // Load collaboration activity
  async loadCollaborationActivity(filePath = null) {
    try {
      const url = filePath 
        ? `/api/collaboration/activity?file_path=${encodeURIComponent(filePath)}&limit=20`
        : '/api/collaboration/activity?limit=50';
        
      const activity = await fetch(url, {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      }).then(r => r.json());
      
      collaborationActivity.set(activity);
    } catch (error) {
      console.error('Failed to load collaboration activity:', error);
    }
  }

  // Acquire a file lock
  async acquireFileLock(filePath, lockType = 'write', durationMinutes = 30) {
    try {
      const response = await fetch('/api/collaboration/locks', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('authToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          file_path: filePath,
          lock_type: lockType,
          duration_minutes: durationMinutes,
          client_id: this.getClientId()
        })
      });

      if (!response.ok) {
        if (response.status === 409) {
          throw new Error('File is already locked by another user');
        }
        throw new Error(`Failed to acquire lock: ${response.status}`);
      }

      const lock = await response.json();
      
      // Update stores
      myLocks.update(locks => [...locks, lock]);
      fileLocks.update(locks => [...locks, lock]);
      
      // Setup auto-renewal
      this.setupLockRenewal(lock);
      
      return lock;
    } catch (error) {
      console.error('Failed to acquire file lock:', error);
      throw error;
    }
  }

  // Release a file lock
  async releaseFileLock(filePath) {
    try {
      const response = await fetch(`/api/collaboration/locks/${encodeURIComponent(filePath)}`, {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });

      if (response.ok) {
        // Update stores
        myLocks.update(locks => locks.filter(lock => lock.file_path !== filePath));
        fileLocks.update(locks => locks.filter(lock => lock.file_path !== filePath));
      }
    } catch (error) {
      console.error('Failed to release file lock:', error);
    }
  }

  // Update user presence
  async updatePresence(currentFile = null, status = 'active') {
    try {
      await fetch('/api/collaboration/presence', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('authToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          current_file: currentFile,
          status: status,
          client_info: this.getClientInfo()
        })
      });

      this.currentFile = currentFile;
    } catch (error) {
      console.error('Failed to update presence:', error);
    }
  }

  // Set current file being viewed/edited
  async setCurrentFile(filePath) {
    await this.updatePresence(filePath, 'active');
    
    // Load activity for this specific file
    if (filePath) {
      await this.loadCollaborationActivity(filePath);
    }
  }

  // Setup WebSocket listeners for real-time updates
  setupWebSocketListeners() {
    if (typeof createWebSocket === 'function') {
      this.websocket = createWebSocket();
      
      this.websocket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.handleCollaborationEvent(data);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };
    }
  }

  // Handle real-time collaboration events
  handleCollaborationEvent(event) {
    switch (event.kind) {
      case 'file_locked':
        this.handleFileLocked(event);
        break;
      case 'file_unlocked':
        this.handleFileUnlocked(event);
        break;
      case 'user_presence_updated':
        this.handlePresenceUpdate(event);
        break;
      case 'batch_progress':
        this.handleBatchProgress(event);
        break;
      case 'batch_complete':
        this.handleBatchComplete(event);
        break;
    }
  }

  handleFileLocked(event) {
    // Refresh file locks
    this.loadFileLocks();
    
    // Show notification if it affects current user
    if (event.path === this.currentFile && event.user_id !== localStorage.getItem('userId')) {
      this.showCollaborationNotification(
        `${event.metadata?.username || 'Someone'} is now editing this file`,
        'warning'
      );
    }
  }

  handleFileUnlocked(event) {
    // Refresh file locks
    this.loadFileLocks();
  }

  handlePresenceUpdate(event) {
    // Refresh user presence
    this.loadUserPresence();
  }

  handleBatchProgress(event) {
    // Parse progress info from path: "batch_copy_progress:job_id:completed:total"
    const [operation, jobId, completed, total] = event.path.split(':').slice(1);
    
    this.showCollaborationNotification(
      `${operation} operation: ${completed}/${total} files processed`,
      'info'
    );
  }

  handleBatchComplete(event) {
    // Parse completion info
    const [operation, jobId, completed, errors] = event.path.split(':').slice(1);
    const hasErrors = parseInt(errors || '0') > 0;
    
    this.showCollaborationNotification(
      `${operation} completed: ${completed} files ${hasErrors ? `(${errors} errors)` : 'successfully'}`,
      hasErrors ? 'warning' : 'success'
    );
  }

  // Start periodic presence updates
  startPresenceUpdates() {
    // Update presence every 30 seconds when active
    this.presenceUpdateInterval = setInterval(async () => {
      if (document.visibilityState === 'visible') {
        await this.updatePresence(this.currentFile, 'active');
      }
    }, 30000);

    // Update presence when tab becomes visible/hidden
    document.addEventListener('visibilitychange', async () => {
      const status = document.visibilityState === 'visible' ? 'active' : 'away';
      await this.updatePresence(this.currentFile, status);
    });
  }

  // Setup automatic lock renewal
  setupLockRenewal(lock) {
    // Renew lock every 20 minutes (locks expire after 30 minutes by default)
    const renewInterval = setInterval(async () => {
      try {
        await this.acquireFileLock(lock.file_path, lock.lock_type, 30);
      } catch (error) {
        console.warn('Failed to renew lock:', error);
        clearInterval(renewInterval);
      }
    }, 20 * 60 * 1000);

    // Store interval reference for cleanup
    lock._renewInterval = renewInterval;
  }

  // Utility functions
  getClientId() {
    let clientId = localStorage.getItem('collaborationClientId');
    if (!clientId) {
      clientId = 'client_' + Math.random().toString(36).substr(2, 9);
      localStorage.setItem('collaborationClientId', clientId);
    }
    return clientId;
  }

  getClientInfo() {
    return JSON.stringify({
      userAgent: navigator.userAgent,
      platform: navigator.platform,
      timestamp: Date.now()
    });
  }

  showCollaborationNotification(message, type = 'info') {
    // This would integrate with your notification system
    console.log(`[Collaboration ${type.toUpperCase()}]:`, message);
    
    // You can integrate this with your existing toast notification system
    // Example: dispatch custom event for toast notifications
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('collaboration-notification', {
        detail: { message, type }
      }));
    }
  }

  // Cleanup when component unmounts
  cleanup() {
    if (this.presenceUpdateInterval) {
      clearInterval(this.presenceUpdateInterval);
    }
    
    if (this.lockRenewInterval) {
      clearInterval(this.lockRenewInterval);
    }
    
    if (this.websocket) {
      this.websocket.close();
    }
    
    // Release all locks when leaving
    myLocks.update(locks => {
      locks.forEach(lock => this.releaseFileLock(lock.file_path));
      return [];
    });
  }
}

// Create global collaboration manager instance
export const collaborationManager = new CollaborationManager();

// Auto-initialize when module loads
if (typeof window !== 'undefined') {
  collaborationManager.initialize();
  
  // Cleanup on page unload
  window.addEventListener('beforeunload', () => {
    collaborationManager.cleanup();
  });
}

// Helper functions for components
export const collaborationHelpers = {
  // Check if a file is locked by someone else
  isFileLockedByOthers(filePath) {
    let locked = false;
    const unsubscribe = lockedByOthers.subscribe(locks => {
      locked = locks.some(lock => lock.file_path === filePath);
    });
    unsubscribe();
    return locked;
  },

  // Get who has locked a file
  getFileLockInfo(filePath) {
    let lockInfo = null;
    const unsubscribe = fileLocks.subscribe(locks => {
      lockInfo = locks.find(lock => lock.file_path === filePath);
    });
    unsubscribe();
    return lockInfo;
  },

  // Get users currently viewing a file
  getUsersViewingFile(filePath) {
    let users = [];
    const unsubscribe = userPresence.subscribe(presence => {
      users = presence.filter(p => p.current_file === filePath && p.status === 'active');
    });
    unsubscribe();
    return users;
  }
};