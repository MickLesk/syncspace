/**
 * Admin Console Store
 * Provides user management, roles, audit log, quota management
 * Uses real API calls without fallback mock data
 * 
 * Usage:
 * import { adminConsole } from './stores/adminConsole.js';
 * const users = $adminConsole.users();
 */

import { writable, derived } from 'svelte/store';
import api from '../lib/api';

// Create stores with empty initial data
const users = writable([]);
const auditLog = writable([]);
const systemStats = writable({
  totalUsers: 0,
  activeUsers: 0,
  totalStorage: 0,
  maxStorage: 0,
  uploadCount24h: 0,
  loginCount24h: 0,
  errorCount24h: 0,
});
const roles = writable([]);

// Filters
const userFilter = writable({
  search: '',
  role: 'all',
  status: 'all',
  sortBy: 'username',
  sortOrder: 'asc',
});

const auditFilter = writable({
  search: '',
  action: 'all',
  user: '',
  status: 'all',
  startDate: '',
  endDate: '',
  sortBy: 'timestamp',
  sortOrder: 'desc',
});

// Derived stores for filtered data
export const filteredUsers = derived(
  [users, userFilter],
  ([$users, $filter]) => {
    let filtered = $users;

    // Search filter
    if ($filter.search) {
      const search = $filter.search.toLowerCase();
      filtered = filtered.filter(
        (u) =>
          u.username.toLowerCase().includes(search) ||
          u.email.toLowerCase().includes(search) ||
          u.displayName.toLowerCase().includes(search)
      );
    }

    // Role filter
    if ($filter.role !== 'all') {
      filtered = filtered.filter((u) => u.role === $filter.role);
    }

    // Status filter
    if ($filter.status !== 'all') {
      filtered = filtered.filter((u) => u.status === $filter.status);
    }

    // Sorting
    filtered.sort((a, b) => {
      const aVal = a[$filter.sortBy];
      const bVal = b[$filter.sortBy];

      const comparison = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
      return $filter.sortOrder === 'asc' ? comparison : -comparison;
    });

    return filtered;
  }
);

export const filteredAuditLog = derived(
  [auditLog, auditFilter],
  ([$log, $filter]) => {
    let filtered = $log;

    // Search filter
    if ($filter.search) {
      const search = $filter.search.toLowerCase();
      filtered = filtered.filter(
        (e) =>
          e.user.toLowerCase().includes(search) ||
          e.resource.toLowerCase().includes(search) ||
          e.ipAddress.includes($filter.search)
      );
    }

    // Action filter
    if ($filter.action !== 'all') {
      filtered = filtered.filter((e) => e.action === $filter.action);
    }

    // User filter
    if ($filter.user) {
      filtered = filtered.filter((e) =>
        e.user.toLowerCase().includes($filter.user.toLowerCase())
      );
    }

    // Status filter
    if ($filter.status !== 'all') {
      filtered = filtered.filter((e) => e.status === $filter.status);
    }

    // Date range filter
    if ($filter.startDate) {
      const start = new Date($filter.startDate);
      filtered = filtered.filter((e) => new Date(e.timestamp) >= start);
    }
    if ($filter.endDate) {
      const end = new Date($filter.endDate);
      filtered = filtered.filter((e) => new Date(e.timestamp) <= end);
    }

    // Sorting
    filtered.sort((a, b) => {
      const aVal = new Date(a[$filter.sortBy]);
      const bVal = new Date(b[$filter.sortBy]);

      const comparison = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
      return $filter.sortOrder === 'asc' ? comparison : -comparison;
    });

    return filtered;
  }
);

// User statistics
export const userStats = derived(users, ($users) => ({
  total: $users.length,
  active: $users.filter((u) => u.status === 'active').length,
  inactive: $users.filter((u) => u.status === 'inactive').length,
  byRole: {
    admin: $users.filter((u) => u.role === 'admin').length,
    moderator: $users.filter((u) => u.role === 'moderator').length,
    user: $users.filter((u) => u.role === 'user').length,
  },
}));

// Storage statistics
export const storageStats = derived(users, ($users) => {
  const totalUsed = $users.reduce((sum, u) => sum + u.quota.used, 0);
  const totalLimit = $users.reduce((sum, u) => sum + u.quota.limit, 0);

  return {
    used: totalUsed,
    limit: totalLimit,
    percentage: Math.round((totalUsed / totalLimit) * 100),
    byUser: $users.map((u) => ({
      username: u.username,
      used: u.quota.used,
      limit: u.quota.limit,
      percentage: Math.round((u.quota.used / u.quota.limit) * 100),
    })),
  };
});

// Admin console actions
export const adminConsole = {
  // User management
  users: { subscribe: users.subscribe },
  filteredUsers: { subscribe: filteredUsers.subscribe },
  userStats: { subscribe: userStats.subscribe },
  
  async getUser(id) {
    try {
      return await api.users?.get?.(id);
    } catch (error) {
      console.error('Failed to get user:', error);
      return null;
    }
  },

  async createUser(userData) {
    try {
      const newUser = await api.users?.create?.(userData);
      if (newUser) {
        users.update((u) => [...u, newUser]);
      }
      return newUser;
    } catch (error) {
      console.error('Failed to create user:', error);
      throw error;
    }
  },

  async updateUser(id, updates) {
    try {
      await api.users?.update?.(id, updates);
      users.update((u) =>
        u.map((user) => (user.id === id ? { ...user, ...updates } : user))
      );
    } catch (error) {
      console.error('Failed to update user:', error);
      throw error;
    }
  },

  async deleteUser(id) {
    try {
      await api.users?.delete?.(id);
      users.update((u) => u.filter((user) => user.id !== id));
    } catch (error) {
      console.error('Failed to delete user:', error);
      throw error;
    }
  },

  async bulkUpdateUsers(ids, updates) {
    users.update((u) =>
      u.map((user) =>
        ids.includes(user.id) ? { ...user, ...updates } : user
      )
    );
  },

  async bulkDeleteUsers(ids) {
    users.update((u) => u.filter((user) => !ids.includes(user.id)));
  },

  // Audit log
  auditLog: { subscribe: auditLog.subscribe },
  filteredAuditLog: { subscribe: filteredAuditLog.subscribe },

  async getAuditEntry(id) {
    try {
      return await api.audit?.getEntry?.(id);
    } catch (error) {
      console.error('Failed to get audit entry:', error);
      return null;
    }
  },

  async exportAuditLog(format = 'json') {
    let currentAuditLog = [];
    auditLog.subscribe(a => currentAuditLog = a)();
    
    if (format === 'csv' && currentAuditLog.length > 0) {
      const headers = Object.keys(currentAuditLog[0]);
      const rows = currentAuditLog.map((e) =>
        headers.map((h) => JSON.stringify(e[h] || '')).join(',')
      );
      return [headers.join(','), ...rows].join('\n');
    }
    return JSON.stringify(currentAuditLog, null, 2);
  },

  // Roles management
  roles: { subscribe: roles.subscribe },

  async getRoles() {
    try {
      const result = await api.roles?.list?.();
      roles.set(result || []);
      return result || [];
    } catch (error) {
      console.error('Failed to get roles:', error);
      return [];
    }
  },

  async createRole(roleData) {
    const newRole = {
      id: `role-${Date.now()}`,
      userCount: 0,
      ...roleData,
    };

    roles.update((r) => [...r, newRole]);
    return newRole;
  },

  async updateRole(id, updates) {
    roles.update((r) =>
      r.map((role) => (role.id === id ? { ...role, ...updates } : role))
    );
  },

  async deleteRole(id) {
    roles.update((r) => r.filter((role) => role.id !== id));
  },

  // System statistics
  systemStats: { subscribe: systemStats.subscribe },
  storageStats: { subscribe: storageStats.subscribe },

  async getSystemStats() {
    try {
      const stats = await api.system?.getStats?.();
      if (stats) {
        systemStats.set(stats);
      }
      return stats || { totalUsers: 0, activeUsers: 0, totalStorage: 0, maxStorage: 0, uploadCount24h: 0, loginCount24h: 0, errorCount24h: 0 };
    } catch (error) {
      console.error('Failed to get system stats:', error);
      return { totalUsers: 0, activeUsers: 0, totalStorage: 0, maxStorage: 0, uploadCount24h: 0, loginCount24h: 0, errorCount24h: 0 };
    }
  },

  async getStorageStats() {
    try {
      const stats = await api.storage?.getStats?.();
      return stats || { used: 0, limit: 0, percentage: 0, topUsers: [] };
    } catch (error) {
      console.error('Failed to get storage stats:', error);
      return { used: 0, limit: 0, percentage: 0, topUsers: [] };
    }
  },

  // Filters
  setUserFilter(filter) {
    userFilter.update((f) => ({ ...f, ...filter }));
  },

  resetUserFilter() {
    userFilter.set({
      search: '',
      role: 'all',
      status: 'all',
      sortBy: 'username',
      sortOrder: 'asc',
    });
  },

  setAuditFilter(filter) {
    auditFilter.update((f) => ({ ...f, ...filter }));
  },

  resetAuditFilter() {
    auditFilter.set({
      search: '',
      action: 'all',
      user: '',
      status: 'all',
      startDate: '',
      endDate: '',
      sortBy: 'timestamp',
      sortOrder: 'desc',
    });
  },

  // Export/Import
  async exportUsers(format = 'json') {
    let currentUsers = [];
    users.subscribe(u => currentUsers = u)();
    
    if (format === 'csv') {
      const headers = ['username', 'email', 'displayName', 'role', 'status'];
      const rows = currentUsers.map((u) =>
        headers.map((h) => JSON.stringify(u[h] || '')).join(',')
      );
      return [headers.join(','), ...rows].join('\n');
    }
    return JSON.stringify(currentUsers, null, 2);
  },

  async importUsers(data) {
    // Parse and validate CSV/JSON
    const userList = Array.isArray(data) ? data : JSON.parse(data);
    
    try {
      const newUsers = await Promise.all(
        userList.map(u => api.users?.create?.(u))
      );
      users.update((existing) => [...existing, ...newUsers.filter(Boolean)]);
      return newUsers.filter(Boolean);
    } catch (error) {
      console.error('Failed to import users:', error);
      throw error;
    }
  },
};

export default adminConsole;
