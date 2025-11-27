/**
 * Global Modal Management Store (Svelte 5 Runes)
 * Modern portal-based modal system with centralized state
 * Includes event bus for modal-to-page communication
 */

import { writable } from 'svelte/store';

const initialState = {
  upload: { visible: false, data: null },
  newFolder: { visible: false, data: null },
  rename: { visible: false, data: null },
  delete: { visible: false, data: null },
  move: { visible: false, data: null },
  copy: { visible: false, data: null },
  share: { visible: false, data: null },
  preview: { visible: false, data: null },
  versionHistory: { visible: false, data: null },
  advancedSearch: { visible: false, data: null },
  changeFolderColor: { visible: false, data: null },
  // Admin modals
  roleEditor: { visible: false, data: null },
  permissionMatrix: { visible: false, data: null },
  workflowEditor: { visible: false, data: null },
  cronJobEditor: { visible: false, data: null },
};

/**
 * Event Bus for Modal Actions
 * Allows ModalPortal to communicate with pages without tight coupling
 */
function createEventBus() {
  const listeners = {};

  return {
    on: (event, callback) => {
      if (!listeners[event]) {
        listeners[event] = [];
      }
      listeners[event].push(callback);
      
      // Return unsubscribe function
      return () => {
        listeners[event] = listeners[event].filter(cb => cb !== callback);
      };
    },
    
    emit: (event, data) => {
      if (listeners[event]) {
        listeners[event].forEach(callback => callback(data));
      }
    },
    
    off: (event) => {
      delete listeners[event];
    }
  };
}

export const modalEvents = createEventBus();

function createModalStore() {
  const { subscribe, set, update } = writable(initialState);

  return {
    subscribe,
    
    // Generic methods
    open: (modalName, data = null) => {
      update(state => ({
        ...state,
        [modalName]: { visible: true, data }
      }));
    },
    
    close: (modalName) => {
      update(state => ({
        ...state,
        [modalName]: { visible: false, data: null }
      }));
    },
    
    closeAll: () => set(initialState),
    
    // Shorthand methods for common modals
    openUpload: (data = null) => {
      update(state => ({
        ...state,
        upload: { visible: true, data }
      }));
    },
    
    openNewFolder: (data = null) => {
      update(state => ({
        ...state,
        newFolder: { visible: true, data }
      }));
    },
    
    openRename: (file) => {
      update(state => ({
        ...state,
        rename: { visible: true, data: file }
      }));
    },
    
    openDelete: (file) => {
      update(state => ({
        ...state,
        delete: { visible: true, data: file }
      }));
    },
    
    openPreview: (file, allFiles = [], currentIndex = 0) => {
      update(state => ({
        ...state,
        preview: { visible: true, data: file, allFiles, currentIndex }
      }));
    },
    
    openAdvancedSearch: () => {
      update(state => ({
        ...state,
        advancedSearch: { visible: true, data: null }
      }));
    },
    
    openMove: (file) => {
      update(state => ({
        ...state,
        move: { visible: true, data: file }
      }));
    },
    
    openCopy: (file) => {
      update(state => ({
        ...state,
        copy: { visible: true, data: file }
      }));
    },
    
    openShare: (file) => {
      update(state => ({
        ...state,
        share: { visible: true, data: file }
      }));
    },
    
    openVersionHistory: (file) => {
      update(state => ({
        ...state,
        versionHistory: { visible: true, data: file }
      }));
    },
    
    openChangeFolderColor: (file) => {
      update(state => ({
        ...state,
        changeFolderColor: { visible: true, data: file }
      }));
    },
    
    // Admin modals
    openRoleEditor: (role = null, permissions = []) => {
      update(state => ({
        ...state,
        roleEditor: { visible: true, data: { role, permissions } }
      }));
    },
    
    openPermissionMatrix: (role, permissions = []) => {
      update(state => ({
        ...state,
        permissionMatrix: { visible: true, data: { role, permissions } }
      }));
    },
    
    openWorkflowEditor: (rule = null, triggerTypes = [], actionTypes = []) => {
      update(state => ({
        ...state,
        workflowEditor: { visible: true, data: { rule, triggerTypes, actionTypes } }
      }));
    },
    
    openCronJobEditor: (cronJob = null) => {
      update(state => ({
        ...state,
        cronJobEditor: { visible: true, data: cronJob }
      }));
    },
  };
}

export const modals = createModalStore();
