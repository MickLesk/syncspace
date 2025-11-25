import { writable, derived } from 'svelte/store';
import { t } from '../lib/i18n';
import api from '../lib/api';

function createBackupRestoreStore() {
  const backups = writable([]);
  const selectedBackup = writable(null);
  const restoreProgress = writable(0);
  const isRestoring = writable(false);
  const restoreError = writable(null);
  const filterType = writable('all'); // all, full, incremental, differential
  const searchQuery = writable('');
  const sortBy = writable('date'); // date, size, status
  const sortOrder = writable('desc'); // asc, desc

  // Load backups from API
  async function loadBackups() {
    try {
      const response = await api.backup?.listBackups?.() || await api.backup?.list?.();
      if (!response) {
        throw new Error('Backup API not available');
      }
      backups.set(response);
    } catch (error) {
      console.error('Failed to load backups from API:', error.message);
      restoreError.set(error.message || 'Failed to load backups');
      backups.set([]);
    }
  }

  // Filter and search backups
  const filteredBackups = derived([backups, filterType, searchQuery, sortBy, sortOrder], 
    ([$backups, $filterType, $searchQuery, $sortBy, $sortOrder]) => {
      let result = [...$backups];

      // Filter by type
      if ($filterType !== 'all') {
        result = result.filter(b => b.type === $filterType);
      }

      // Search
      if ($searchQuery) {
        const query = $searchQuery.toLowerCase();
        result = result.filter(b => 
          b.name.toLowerCase().includes(query) || 
          b.notes.toLowerCase().includes(query)
        );
      }

      // Sort
      result.sort((a, b) => {
        let comparison = 0;
        if ($sortBy === 'date') {
          comparison = a.date - b.date;
        } else if ($sortBy === 'size') {
          comparison = a.size - b.size;
        } else if ($sortBy === 'status') {
          const statusOrder = { success: 0, pending: 1, failed: 2, in_progress: 3 };
          comparison = (statusOrder[a.status] || 3) - (statusOrder[b.status] || 3);
        }
        return $sortOrder === 'desc' ? -comparison : comparison;
      });

      return result;
    }
  );

  // Backup statistics
  const stats = derived(backups, ($backups) => {
    const totalSize = $backups.reduce((sum, b) => sum + b.size, 0);
    const successCount = $backups.filter(b => b.status === 'success').length;
    const failedCount = $backups.filter(b => b.status === 'failed').length;
    const totalFileCount = $backups.reduce((sum, b) => sum + b.fileCount, 0);
    const avgDuration = $backups.length > 0 
      ? Math.round($backups.reduce((sum, b) => sum + b.duration, 0) / $backups.length) 
      : 0;

    return {
      totalBackups: $backups.length,
      totalSize,
      successCount,
      failedCount,
      totalFileCount,
      avgDuration,
      lastBackup: $backups.length > 0 ? $backups[0] : null
    };
  });

  // Format size
  function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }

  // Format duration
  function formatDuration(seconds) {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
    return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
  }

  // Get status color
  function getStatusColor(status) {
    const colors = {
      success: 'success',
      failed: 'error',
      in_progress: 'warning',
      pending: 'info',
      verified: 'success',
      corrupted: 'error'
    };
    return colors[status] || 'info';
  }

  // Restore backup
  async function restoreBackup(backupId) {
    const backup = get(backups).find(b => b.id === backupId);
    if (!backup) return;

    selectedBackup.set(backup);
    isRestoring.set(true);
    restoreProgress.set(0);
    restoreError.set(null);

    // Simulate restore progress
    return new Promise((resolve) => {
      const interval = setInterval(() => {
        restoreProgress.update(p => {
          if (p >= 100) {
            clearInterval(interval);
            isRestoring.set(false);
            resolve();
            return 100;
          }
          return p + Math.random() * 15;
        });
      }, 500);
    });
  }

  // Cancel restore
  function cancelRestore() {
    isRestoring.set(false);
    restoreProgress.set(0);
    selectedBackup.set(null);
    restoreError.set(null);
  }

  // Verify backup
  async function verifyBackup(backupId) {
    const backup = get(backups).find(b => b.id === backupId);
    if (!backup) return;

    try {
      backups.update(list => 
        list.map(b => 
          b.id === backupId 
            ? { ...b, verificationStatus: 'verifying' }
            : b
        )
      );

      // Simulate verification
      await new Promise(resolve => setTimeout(resolve, 2000));

      backups.update(list =>
        list.map(b =>
          b.id === backupId
            ? { ...b, verificationStatus: 'verified' }
            : b
        )
      );
    } catch (error) {
      restoreError.set(error.message);
    }
  }

  // Delete backup
  function deleteBackup(backupId) {
    backups.update(list => list.filter(b => b.id !== backupId));
  }

  // Export backup list
  function exportBackupList() {
    let $backups;
    backups.subscribe(v => $backups = v)();
    
    const data = {
      exported: new Date().toISOString(),
      backupCount: $backups.length,
      backups: $backups
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `backups-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  return {
    backups,
    filteredBackups,
    selectedBackup,
    stats,
    restoreProgress,
    isRestoring,
    restoreError,
    filterType,
    searchQuery,
    sortBy,
    sortOrder,
    loadBackups,
    restoreBackup,
    cancelRestore,
    verifyBackup,
    deleteBackup,
    formatSize,
    formatDuration,
    getStatusColor,
    exportBackupList
  };
}

function get(store) {
  let value;
  store.subscribe(v => value = v)();
  return value;
}

export const backupRestore = createBackupRestoreStore();
