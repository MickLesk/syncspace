import { writable, derived } from 'svelte/store';
import { t } from '../lib/i18n';

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

  // Generate mock backup data
  function generateMockBackups() {
    const now = Date.now();
    const types = ['full', 'incremental', 'differential'];
    const statuses = ['success', 'in_progress', 'failed'];
    
    const mockData = [];
    for (let i = 0; i < 12; i++) {
      const backupDate = new Date(now - i * 24 * 60 * 60 * 1000);
      const type = types[i % 3];
      const sizeGB = Math.random() * 50 + 5;
      
      mockData.push({
        id: `backup-${i}`,
        name: `Backup ${backupDate.toLocaleDateString('de-DE')} ${type === 'full' ? '(Full)' : '(Inc)'}`,
        date: backupDate.getTime(),
        timestamp: backupDate.toISOString(),
        type,
        size: sizeGB * 1024 * 1024 * 1024,
        status: i === 0 ? 'success' : statuses[Math.floor(Math.random() * statuses.length)],
        fileCount: Math.floor(Math.random() * 5000 + 1000),
        duration: Math.floor(Math.random() * 3600 + 300), // seconds
        notes: i % 3 === 0 ? 'Manual backup before system update' : '',
        retentionDays: [7, 14, 30, 90][Math.floor(Math.random() * 4)],
        compressionRatio: (Math.random() * 0.4 + 0.4).toFixed(2), // 0.4-0.8
        encryptionEnabled: Math.random() > 0.3,
        verificationStatus: i === 0 ? 'verified' : (Math.random() > 0.2 ? 'verified' : 'pending')
      });
    }
    return mockData;
  }

  // Load backups (mock)
  async function loadBackups() {
    try {
      backups.set(generateMockBackups());
    } catch (error) {
      restoreError.set(error.message);
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
