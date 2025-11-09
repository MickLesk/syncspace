/**
 * Batch Operations Service
 * Handles Move, Copy, Delete, and Compress operations
 * Integrates with WebSocket for real-time progress updates
 */

import { api } from './api';

/**
 * Operation status enum
 */
export const OPERATION_STATUS = {
  PENDING: 'pending',
  IN_PROGRESS: 'in-progress',
  COMPLETED: 'completed',
  FAILED: 'failed',
  CANCELLED: 'cancelled'
};

/**
 * Operation type enum
 */
export const OPERATION_TYPE = {
  COPY: 'copy',
  MOVE: 'move',
  DELETE: 'delete',
  COMPRESS: 'compress',
  EXTRACT: 'extract'
};

/**
 * Conflict resolution strategy enum
 */
export const CONFLICT_STRATEGY = {
  SKIP: 'skip',
  OVERWRITE: 'overwrite',
  RENAME: 'rename',
  AUTO_RENAME: 'auto_rename'
};

/**
 * Creates a batch operation manager
 * @returns {Object} Operation manager with methods to handle batch operations
 */
export function createBatchOperationManager() {
  let operations = new Map(); // Map<operationId, operation>
  let operationCounter = 0;

  /**
   * Generate unique operation ID
   */
  function generateOperationId() {
    return `op_${Date.now()}_${++operationCounter}`;
  }

  /**
   * Create operation object
   */
  function createOperation(type, items, destination, options = {}) {
    return {
      id: generateOperationId(),
      type,
      items,
      destination,
      options,
      status: OPERATION_STATUS.PENDING,
      progress: 0,
      processedItems: 0,
      skippedItems: 0,
      failedItems: [],
      conflicts: [],
      errors: {},
      startTime: null,
      endTime: null,
      estimatedTimeRemaining: null,
      createdAt: new Date()
    };
  }

  /**
   * Detect file conflicts in destination
   */
  async function detectConflicts(items, destination) {
    try {
      const destItems = await api.files.listDirectory(destination);
      const destNames = new Set(destItems.data?.map(i => i.name) || []);
      
      return items.filter(item => 
        destNames.has(item.name)
      ).map(item => ({
        source: item,
        existingFile: destItems.data.find(d => d.name === item.name),
        strategy: CONFLICT_STRATEGY.SKIP // Default strategy
      }));
    } catch (error) {
      console.error('Error detecting conflicts:', error);
      return [];
    }
  }

  /**
   * Validate copy/move operation
   */
  async function validateOperation(type, items, destination) {
    const errors = [];

    // Check if destination exists
    try {
      await api.files.stat(destination);
    } catch (error) {
      errors.push({
        code: 'DESTINATION_NOT_FOUND',
        message: `Destination path does not exist: ${destination}`
      });
    }

    // Check for circular operations (moving folder into itself)
    if (type === OPERATION_TYPE.MOVE) {
      for (const item of items) {
        if (destination.startsWith(item.path + '/')) {
          errors.push({
            code: 'CIRCULAR_OPERATION',
            message: `Cannot move folder into itself: ${item.name}`
          });
        }
      }
    }

    // Check if source items exist
    for (const item of items) {
      try {
        await api.files.stat(item.path);
      } catch (error) {
        errors.push({
          code: 'SOURCE_NOT_FOUND',
          message: `Source item not found: ${item.name}`
        });
      }
    }

    return errors;
  }

  /**
   * Get suggested filename for duplicate
   */
  function getSuggestedName(originalName) {
    const parts = originalName.split('.');
    const ext = parts.pop();
    const name = parts.join('.');
    const timestamp = Date.now().toString().slice(-6);
    return `${name} - copy (${timestamp}).${ext}`;
  }

  /**
   * Resolve filename conflict
   */
  function resolveConflict(filename, strategy, customName = null) {
    switch (strategy) {
      case CONFLICT_STRATEGY.SKIP:
        return null;
      case CONFLICT_STRATEGY.OVERWRITE:
        return filename;
      case CONFLICT_STRATEGY.RENAME:
      case CONFLICT_STRATEGY.AUTO_RENAME:
        return customName || getSuggestedName(filename);
      default:
        return filename;
    }
  }

  /**
   * Execute copy operation
   */
  async function executeCopy(operation) {
    operation.status = OPERATION_STATUS.IN_PROGRESS;
    operation.startTime = new Date();
    const startTime = Date.now();

    for (let i = 0; i < operation.items.length; i++) {
      const item = operation.items[i];
      const conflict = operation.conflicts.find(c => c.source.path === item.path);
      
      try {
        // Handle conflict resolution
        if (conflict) {
          const resolvedName = resolveConflict(
            item.name,
            conflict.strategy,
            conflict.customName
          );

          if (resolvedName === null) {
            operation.skippedItems++;
            operation.processedItems++;
            updateProgress(operation);
            continue;
          }

          const targetPath = `${operation.destination}/${resolvedName}`;
          await api.files.copy(item.path, targetPath);
        } else {
          const targetPath = `${operation.destination}/${item.name}`;
          await api.files.copy(item.path, targetPath);
        }

        operation.processedItems++;
      } catch (error) {
        console.error(`Error copying ${item.name}:`, error);
        operation.failedItems.push(item.name);
        operation.errors[item.path] = error.message;
        operation.processedItems++;
      }

      updateProgress(operation, startTime);
    }

    completeOperation(operation);
  }

  /**
   * Execute move operation
   */
  async function executeMove(operation) {
    operation.status = OPERATION_STATUS.IN_PROGRESS;
    operation.startTime = new Date();
    const startTime = Date.now();

    for (let i = 0; i < operation.items.length; i++) {
      const item = operation.items[i];
      const conflict = operation.conflicts.find(c => c.source.path === item.path);

      try {
        // Handle conflict resolution
        if (conflict) {
          const resolvedName = resolveConflict(
            item.name,
            conflict.strategy,
            conflict.customName
          );

          if (resolvedName === null) {
            operation.skippedItems++;
            operation.processedItems++;
            updateProgress(operation);
            continue;
          }

          const targetPath = `${operation.destination}/${resolvedName}`;
          await api.files.move(item.path, targetPath);
        } else {
          const targetPath = `${operation.destination}/${item.name}`;
          await api.files.move(item.path, targetPath);
        }

        operation.processedItems++;
      } catch (error) {
        console.error(`Error moving ${item.name}:`, error);
        operation.failedItems.push(item.name);
        operation.errors[item.path] = error.message;
        operation.processedItems++;
      }

      updateProgress(operation, startTime);
    }

    completeOperation(operation);
  }

  /**
   * Update operation progress
   */
  function updateProgress(operation, startTime = null) {
    operation.progress = Math.round(
      (operation.processedItems / operation.items.length) * 100
    );

    // Estimate time remaining
    if (startTime) {
      const elapsedMs = Date.now() - startTime;
      const itemsProcessed = operation.processedItems;
      const avgTimePerItem = elapsedMs / Math.max(itemsProcessed, 1);
      const remainingItems = operation.items.length - operation.processedItems;
      operation.estimatedTimeRemaining = Math.ceil(avgTimePerItem * remainingItems);
    }
  }

  /**
   * Complete operation
   */
  function completeOperation(operation) {
    operation.status = operation.failedItems.length === 0
      ? OPERATION_STATUS.COMPLETED
      : OPERATION_STATUS.FAILED;
    operation.endTime = new Date();
    operation.progress = 100;
  }

  /**
   * Start batch operation (public API)
   */
  async function startOperation(type, items, destination, options = {}) {
    // Validate operation
    const validationErrors = await validateOperation(type, items, destination);
    if (validationErrors.length > 0) {
      throw new Error(validationErrors.map(e => e.message).join(', '));
    }

    // Create operation
    const operation = createOperation(type, items, destination, options);

    // Detect conflicts
    operation.conflicts = await detectConflicts(items, destination);

    // If conflicts and no auto-resolve, return operation for UI conflict resolution
    if (operation.conflicts.length > 0 && !options.autoResolveConflicts) {
      operations.set(operation.id, operation);
      return operation;
    }

    // Apply auto-resolution if specified
    if (options.autoResolveConflicts) {
      operation.conflicts.forEach(conflict => {
        conflict.strategy = options.conflictStrategy || CONFLICT_STRATEGY.AUTO_RENAME;
        if (conflict.strategy === CONFLICT_STRATEGY.RENAME) {
          conflict.customName = options.customNames?.[conflict.source.path];
        }
      });
    }

    // Store and execute operation
    operations.set(operation.id, operation);
    executeOperationAsync(operation, type);

    return operation;
  }

  /**
   * Execute operation asynchronously
   */
  function executeOperationAsync(operation, type) {
    setImmediate(async () => {
      try {
        if (type === OPERATION_TYPE.COPY) {
          await executeCopy(operation);
        } else if (type === OPERATION_TYPE.MOVE) {
          await executeMove(operation);
        }
      } catch (error) {
        console.error(`Batch operation failed: ${error.message}`);
        operation.status = OPERATION_STATUS.FAILED;
        operation.errors['_general'] = error.message;
      }
    });
  }

  /**
   * Resolve conflict and resume operation
   */
  function resolveConflictAndResume(operationId, conflictIndex, resolution) {
    const operation = operations.get(operationId);
    if (!operation) return;

    operation.conflicts[conflictIndex].strategy = resolution.strategy;
    if (resolution.customName) {
      operation.conflicts[conflictIndex].customName = resolution.customName;
    }

    // If all conflicts resolved, start execution
    const allResolved = operation.conflicts.every(c => c.strategy !== undefined);
    if (allResolved && operation.status === OPERATION_STATUS.PENDING) {
      executeOperationAsync(operation, operation.type);
    }
  }

  /**
   * Cancel operation
   */
  function cancelOperation(operationId) {
    const operation = operations.get(operationId);
    if (operation && operation.status === OPERATION_STATUS.IN_PROGRESS) {
      operation.status = OPERATION_STATUS.CANCELLED;
    }
  }

  /**
   * Get operation status
   */
  function getOperation(operationId) {
    return operations.get(operationId);
  }

  /**
   * Get all operations
   */
  function getAllOperations() {
    return Array.from(operations.values());
  }

  /**
   * Clear completed operations
   */
  function clearCompletedOperations() {
    for (const [id, op] of operations.entries()) {
      if (op.status === OPERATION_STATUS.COMPLETED || op.status === OPERATION_STATUS.FAILED) {
        operations.delete(id);
      }
    }
  }

  return {
    startOperation,
    resolveConflictAndResume,
    cancelOperation,
    getOperation,
    getAllOperations,
    clearCompletedOperations,
    detectConflicts,
    validateOperation,
    OPERATION_STATUS,
    OPERATION_TYPE,
    CONFLICT_STRATEGY
  };
}

// Singleton instance
export const batchOperationManager = createBatchOperationManager();
