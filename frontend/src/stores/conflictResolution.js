/**
 * Intelligent Conflict Resolution System
 * Handles file conflicts with automatic and manual resolution strategies
 */

import { writable } from 'svelte/store';

// Store for active conflicts
export const activeConflicts = writable([]);

// Store for conflict resolution history
export const conflictHistory = writable([]);

// Conflict types
export const CONFLICT_TYPES = {
  CONCURRENT_EDIT: 'concurrent_edit',
  VERSION_MISMATCH: 'version_mismatch',
  DUPLICATE_NAME: 'duplicate_name',
  PERMISSION_DENIED: 'permission_denied',
  LOCK_CONFLICT: 'lock_conflict'
};

// Resolution strategies
export const RESOLUTION_STRATEGIES = {
  AUTO_MERGE: 'auto_merge',
  MANUAL_MERGE: 'manual_merge',
  KEEP_BOTH: 'keep_both',
  OVERWRITE: 'overwrite',
  CANCEL: 'cancel',
  RENAME: 'rename'
};

class ConflictResolver {
  constructor() {
    this.conflicts = new Map();
    this.resolutionQueue = [];
  }

  // Detect and handle different types of conflicts
  async detectConflict(operation, filePath, metadata = {}) {
    try {
      const conflictCheck = await this.performConflictCheck(operation, filePath, metadata);
      
      if (conflictCheck.hasConflict) {
        return await this.handleConflict(conflictCheck);
      }
      
      return { success: true, canProceed: true };
    } catch (error) {
      console.error('Conflict detection failed:', error);
      return { success: false, error: error.message };
    }
  }

  // Perform comprehensive conflict check
  async performConflictCheck(operation, filePath, metadata) {
    const checks = await Promise.allSettled([
      this.checkConcurrentEdit(filePath),
      this.checkVersionMismatch(filePath, metadata.lastKnownVersion),
      this.checkDuplicateName(filePath),
      this.checkPermissions(operation, filePath),
      this.checkFileLocks(filePath)
    ]);

    const conflicts = checks
      .map((result, index) => {
        if (result.status === 'fulfilled' && result.value.hasConflict) {
          return result.value;
        }
        return null;
      })
      .filter(Boolean);

    return {
      hasConflict: conflicts.length > 0,
      conflicts: conflicts,
      operation,
      filePath,
      metadata
    };
  }

  // Check for concurrent editing conflicts
  async checkConcurrentEdit(filePath) {
    try {
      const response = await fetch(`/api/collaboration/locks?file_path=${encodeURIComponent(filePath)}`, {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });
      
      if (response.ok) {
        const locks = await response.json();
        const otherUserLocks = locks.filter(lock => 
          lock.locked_by_user_id !== localStorage.getItem('userId')
        );
        
        if (otherUserLocks.length > 0) {
          return {
            hasConflict: true,
            type: CONFLICT_TYPES.CONCURRENT_EDIT,
            description: `File is being edited by ${otherUserLocks[0].locked_by_username}`,
            details: { locks: otherUserLocks },
            suggestedStrategy: RESOLUTION_STRATEGIES.MANUAL_MERGE
          };
        }
      }
      
      return { hasConflict: false };
    } catch (error) {
      console.error('Failed to check concurrent edit:', error);
      return { hasConflict: false };
    }
  }

  // Check for version mismatches
  async checkVersionMismatch(filePath, lastKnownVersion) {
    if (!lastKnownVersion) return { hasConflict: false };

    try {
      // Get current file version/hash
      const response = await fetch(`/api/files/version/${encodeURIComponent(filePath)}`, {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });
      
      if (response.ok) {
        const versionInfo = await response.json();
        
        if (versionInfo.version !== lastKnownVersion) {
          return {
            hasConflict: true,
            type: CONFLICT_TYPES.VERSION_MISMATCH,
            description: 'File has been modified by another user',
            details: { 
              currentVersion: versionInfo.version, 
              lastKnownVersion,
              modifiedBy: versionInfo.last_modified_by
            },
            suggestedStrategy: RESOLUTION_STRATEGIES.AUTO_MERGE
          };
        }
      }
      
      return { hasConflict: false };
    } catch (error) {
      console.error('Failed to check version mismatch:', error);
      return { hasConflict: false };
    }
  }

  // Check for duplicate names
  async checkDuplicateName(filePath) {
    try {
      const fileName = filePath.split('/').pop();
      const directory = filePath.split('/').slice(0, -1).join('/');
      
      const response = await fetch(`/api/files/${encodeURIComponent(directory)}`, {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });
      
      if (response.ok) {
        const files = await response.json();
        const duplicate = files.find(file => 
          file.name === fileName && file.path !== filePath
        );
        
        if (duplicate) {
          return {
            hasConflict: true,
            type: CONFLICT_TYPES.DUPLICATE_NAME,
            description: `A file named "${fileName}" already exists`,
            details: { duplicate },
            suggestedStrategy: RESOLUTION_STRATEGIES.RENAME
          };
        }
      }
      
      return { hasConflict: false };
    } catch (error) {
      console.error('Failed to check duplicate name:', error);
      return { hasConflict: false };
    }
  }

  // Check permissions
  async checkPermissions(operation, filePath) {
    try {
      const response = await fetch(`/api/files/permissions/${encodeURIComponent(filePath)}`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('authToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ operation })
      });
      
      if (response.status === 403) {
        return {
          hasConflict: true,
          type: CONFLICT_TYPES.PERMISSION_DENIED,
          description: `Insufficient permissions for ${operation}`,
          details: { operation },
          suggestedStrategy: RESOLUTION_STRATEGIES.CANCEL
        };
      }
      
      return { hasConflict: false };
    } catch (error) {
      console.error('Failed to check permissions:', error);
      return { hasConflict: false };
    }
  }

  // Check file locks
  async checkFileLocks(filePath) {
    try {
      const response = await fetch(`/api/collaboration/locks`, {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });
      
      if (response.ok) {
        const locks = await response.json();
        const fileLock = locks.find(lock => lock.file_path === filePath);
        
        if (fileLock && fileLock.locked_by_user_id !== localStorage.getItem('userId')) {
          return {
            hasConflict: true,
            type: CONFLICT_TYPES.LOCK_CONFLICT,
            description: `File is locked by ${fileLock.locked_by_username}`,
            details: { lock: fileLock },
            suggestedStrategy: RESOLUTION_STRATEGIES.CANCEL
          };
        }
      }
      
      return { hasConflict: false };
    } catch (error) {
      console.error('Failed to check file locks:', error);
      return { hasConflict: false };
    }
  }

  // Handle detected conflicts
  async handleConflict(conflictInfo) {
    const conflict = {
      id: `conflict_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp: new Date().toISOString(),
      ...conflictInfo
    };

    // Add to active conflicts
    activeConflicts.update(conflicts => [...conflicts, conflict]);

    // Determine resolution strategy
    const resolution = await this.determineResolutionStrategy(conflict);
    
    if (resolution.strategy === RESOLUTION_STRATEGIES.AUTO_MERGE) {
      return await this.attemptAutoResolution(conflict);
    } else {
      // Require manual intervention
      return {
        success: false,
        requiresManualResolution: true,
        conflict,
        suggestedActions: this.getSuggestedActions(conflict)
      };
    }
  }

  // Determine the best resolution strategy
  async determineResolutionStrategy(conflict) {
    const strategies = conflict.conflicts.map(c => c.suggestedStrategy);
    
    // Priority order for automatic resolution
    if (strategies.includes(RESOLUTION_STRATEGIES.AUTO_MERGE)) {
      return { strategy: RESOLUTION_STRATEGIES.AUTO_MERGE, confidence: 0.8 };
    }
    
    if (strategies.includes(RESOLUTION_STRATEGIES.RENAME)) {
      return { strategy: RESOLUTION_STRATEGIES.RENAME, confidence: 0.9 };
    }
    
    // Default to manual intervention for complex conflicts
    return { strategy: RESOLUTION_STRATEGIES.MANUAL_MERGE, confidence: 0.5 };
  }

  // Attempt automatic resolution
  async attemptAutoResolution(conflict) {
    try {
      switch (conflict.conflicts[0].type) {
        case CONFLICT_TYPES.VERSION_MISMATCH:
          return await this.autoMergeVersionConflict(conflict);
        
        case CONFLICT_TYPES.DUPLICATE_NAME:
          return await this.autoRenameDuplicate(conflict);
        
        default:
          throw new Error('No automatic resolution available');
      }
    } catch (error) {
      console.error('Auto-resolution failed:', error);
      return {
        success: false,
        requiresManualResolution: true,
        conflict,
        error: error.message
      };
    }
  }

  // Auto-merge version conflicts for text files
  async autoMergeVersionConflict(conflict) {
    try {
      const filePath = conflict.filePath;
      
      // Get both versions
      const [currentVersion, userVersion] = await Promise.all([
        this.getFileContent(filePath),
        this.getUserVersionContent(filePath, conflict.metadata)
      ]);

      // Attempt three-way merge for text files
      if (this.isTextFile(filePath)) {
        const mergedContent = await this.performThreeWayMerge(
          currentVersion, 
          userVersion, 
          conflict.metadata.baseVersion
        );

        if (mergedContent.success) {
          // Save merged version
          await this.saveResolvedFile(filePath, mergedContent.content, {
            resolutionType: 'auto_merge',
            conflictId: conflict.id
          });

          this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.AUTO_MERGE);
          
          return {
            success: true,
            canProceed: true,
            resolution: 'Automatically merged changes'
          };
        }
      }

      // Fallback to manual resolution
      throw new Error('Automatic merge not possible');
      
    } catch (error) {
      console.error('Auto-merge failed:', error);
      throw error;
    }
  }

  // Auto-rename duplicate files
  async autoRenameDuplicate(conflict) {
    const originalPath = conflict.filePath;
    const directory = originalPath.split('/').slice(0, -1).join('/');
    const fileName = originalPath.split('/').pop();
    const [name, extension] = this.splitFileName(fileName);
    
    // Generate unique name
    let counter = 1;
    let newName;
    let newPath;
    
    do {
      newName = `${name} (${counter})${extension ? '.' + extension : ''}`;
      newPath = directory ? `${directory}/${newName}` : newName;
      counter++;
    } while (await this.fileExists(newPath));

    // Update the operation with new path
    conflict.metadata.resolvedPath = newPath;
    
    this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.RENAME);
    
    return {
      success: true,
      canProceed: true,
      resolution: `Renamed to "${newName}"`,
      newPath
    };
  }

  // Manual resolution methods
  async resolveConflictManually(conflictId, strategy, userInput = {}) {
    const conflict = this.conflicts.get(conflictId);
    if (!conflict) {
      throw new Error('Conflict not found');
    }

    switch (strategy) {
      case RESOLUTION_STRATEGIES.KEEP_BOTH:
        return await this.resolveBySavingBoth(conflict, userInput);
      
      case RESOLUTION_STRATEGIES.OVERWRITE:
        return await this.resolveByOverwriting(conflict);
      
      case RESOLUTION_STRATEGIES.MANUAL_MERGE:
        return await this.resolveByManualMerge(conflict, userInput.mergedContent);
      
      case RESOLUTION_STRATEGIES.CANCEL:
        return this.resolveByCancel(conflict);
      
      default:
        throw new Error('Unknown resolution strategy');
    }
  }

  // Get suggested actions for manual resolution
  getSuggestedActions(conflict) {
    const actions = [];
    
    conflict.conflicts.forEach(c => {
      switch (c.type) {
        case CONFLICT_TYPES.CONCURRENT_EDIT:
          actions.push({
            action: 'merge',
            description: 'Merge changes with other user',
            strategy: RESOLUTION_STRATEGIES.MANUAL_MERGE
          });
          actions.push({
            action: 'wait',
            description: 'Wait for other user to finish',
            strategy: RESOLUTION_STRATEGIES.CANCEL
          });
          break;
          
        case CONFLICT_TYPES.VERSION_MISMATCH:
          actions.push({
            action: 'merge',
            description: 'Merge your changes with current version',
            strategy: RESOLUTION_STRATEGIES.AUTO_MERGE
          });
          actions.push({
            action: 'overwrite',
            description: 'Overwrite with your version',
            strategy: RESOLUTION_STRATEGIES.OVERWRITE
          });
          break;
          
        case CONFLICT_TYPES.DUPLICATE_NAME:
          actions.push({
            action: 'rename',
            description: 'Save with a different name',
            strategy: RESOLUTION_STRATEGIES.RENAME
          });
          actions.push({
            action: 'replace',
            description: 'Replace existing file',
            strategy: RESOLUTION_STRATEGIES.OVERWRITE
          });
          break;
      }
    });
    
    return actions;
  }

  // Utility methods
  isTextFile(filePath) {
    const textExtensions = ['.txt', '.md', '.js', '.ts', '.css', '.html', '.json', '.xml', '.yml', '.yaml'];
    const ext = '.' + filePath.split('.').pop().toLowerCase();
    return textExtensions.includes(ext);
  }

  splitFileName(fileName) {
    const lastDot = fileName.lastIndexOf('.');
    if (lastDot === -1) {
      return [fileName, ''];
    }
    return [fileName.substring(0, lastDot), fileName.substring(lastDot + 1)];
  }

  async fileExists(filePath) {
    try {
      const response = await fetch(`/api/files/${encodeURIComponent(filePath)}`, {
        method: 'HEAD',
        headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
      });
      return response.ok;
    } catch {
      return false;
    }
  }

  markConflictResolved(conflictId, strategy) {
    // Remove from active conflicts
    activeConflicts.update(conflicts => 
      conflicts.filter(c => c.id !== conflictId)
    );
    
    // Add to history
    conflictHistory.update(history => [...history, {
      id: conflictId,
      resolvedAt: new Date().toISOString(),
      strategy,
      resolvedBy: localStorage.getItem('username')
    }]);
  }

  // Three-way merge implementation (simplified)
  async performThreeWayMerge(current, user, base) {
    // This is a simplified merge - in practice, you'd use a proper diff/merge library
    try {
      const currentLines = current.split('\n');
      const userLines = user.split('\n');
      const baseLines = base ? base.split('\n') : [];
      
      // For now, just concatenate with conflict markers
      const mergedLines = [
        '<<<<<<< Current Version',
        ...currentLines,
        '=======',
        ...userLines,
        '>>>>>>> Your Version'
      ];
      
      return {
        success: true,
        content: mergedLines.join('\n'),
        hasConflictMarkers: true
      };
    } catch (error) {
      return {
        success: false,
        error: error.message
      };
    }
  }

  // Helper methods for file operations
  async getFileContent(filePath) {
    const response = await fetch(`/api/file/${encodeURIComponent(filePath)}`, {
      headers: { 'Authorization': `Bearer ${localStorage.getItem('authToken')}` }
    });
    return await response.text();
  }

  async getUserVersionContent(filePath, metadata) {
    // Get the user's local version from metadata or cached data
    if (metadata.userContent) {
      return metadata.userContent;
    }
    
    // If no cached user content, attempt to retrieve from localStorage or session
    const cachedVersion = localStorage.getItem(`pending_edit_${filePath}`);
    if (cachedVersion) {
      return cachedVersion;
    }
    
    // Fall back to empty string if no local version available
    console.warn(`No user version content found for ${filePath}`);
    return '';
  }

  async saveResolvedFile(filePath, content, metadata) {
    const response = await fetch(`/api/upload/${encodeURIComponent(filePath)}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('authToken')}`,
        'Content-Type': 'application/octet-stream',
        'X-Conflict-Resolution': JSON.stringify(metadata)
      },
      body: content
    });
    return response.ok;
  }

  async resolveBySavingBoth(conflict, userInput) {
    // Implementation for keeping both versions
    const originalPath = conflict.filePath;
    const newPath = userInput.newPath || this.generateAlternatePath(originalPath);
    
    await this.saveResolvedFile(newPath, userInput.content, {
      resolutionType: 'keep_both',
      conflictId: conflict.id
    });
    
    this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.KEEP_BOTH);
    return { success: true, newPath };
  }

  async resolveByOverwriting(conflict) {
    // Implementation for overwriting
    await this.saveResolvedFile(conflict.filePath, conflict.metadata.userContent, {
      resolutionType: 'overwrite',
      conflictId: conflict.id
    });
    
    this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.OVERWRITE);
    return { success: true };
  }

  async resolveByManualMerge(conflict, mergedContent) {
    // Implementation for manual merge
    await this.saveResolvedFile(conflict.filePath, mergedContent, {
      resolutionType: 'manual_merge',
      conflictId: conflict.id
    });
    
    this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.MANUAL_MERGE);
    return { success: true };
  }

  resolveByCancel(conflict) {
    // Implementation for canceling operation
    this.markConflictResolved(conflict.id, RESOLUTION_STRATEGIES.CANCEL);
    return { success: true, cancelled: true };
  }

  generateAlternatePath(originalPath) {
    const directory = originalPath.split('/').slice(0, -1).join('/');
    const fileName = originalPath.split('/').pop();
    const [name, extension] = this.splitFileName(fileName);
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const newName = `${name}_${timestamp}${extension ? '.' + extension : ''}`;
    return directory ? `${directory}/${newName}` : newName;
  }
}

// Create global conflict resolver instance
export const conflictResolver = new ConflictResolver();

// Helper functions for components
export const conflictHelpers = {
  // Check if operation would cause conflicts
  async checkBeforeOperation(operation, filePath, metadata = {}) {
    return await conflictResolver.detectConflict(operation, filePath, metadata);
  },

  // Get active conflicts for a specific file
  getFileConflicts(filePath) {
    let conflicts = [];
    const unsubscribe = activeConflicts.subscribe(list => {
      conflicts = list.filter(c => c.filePath === filePath);
    });
    unsubscribe();
    return conflicts;
  },

  // Resolve conflict manually
  async resolveConflict(conflictId, strategy, userInput = {}) {
    return await conflictResolver.resolveConflictManually(conflictId, strategy, userInput);
  },

  // Get conflict resolution statistics
  getConflictStats() {
    let stats = { total: 0, resolved: 0, pending: 0 };
    
    const unsubscribeActive = activeConflicts.subscribe(active => {
      stats.pending = active.length;
    });
    
    const unsubscribeHistory = conflictHistory.subscribe(history => {
      stats.resolved = history.length;
    });
    
    unsubscribeActive();
    unsubscribeHistory();
    
    stats.total = stats.pending + stats.resolved;
    return stats;
  }
};