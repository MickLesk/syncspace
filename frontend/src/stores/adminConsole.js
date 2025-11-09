/**
 * Admin Console Store
 * Mock data for user management, roles, audit log, quota management
 * 
 * Usage:
 * import { adminConsole } from './stores/adminConsole.js';
 * const users = $adminConsole.users();
 */

import { writable, derived } from 'svelte/store';

// Mock admin data
const mockUsers = [
  {
    id: 'user-1',
    username: 'admin',
    email: 'admin@example.com',
    displayName: 'Administrator',
    role: 'admin',
    status: 'active',
    createdAt: '2024-01-01',
    lastLogin: '2025-11-09T10:30:00Z',
    quota: { used: 50000000, limit: 100000000 },
    avatar: '👨‍💼',
  },
  {
    id: 'user-2',
    username: 'john',
    email: 'john@example.com',
    displayName: 'John Doe',
    role: 'user',
    status: 'active',
    createdAt: '2024-02-15',
    lastLogin: '2025-11-09T09:45:00Z',
    quota: { used: 2500000, limit: 50000000 },
    avatar: '👨‍💻',
  },
  {
    id: 'user-3',
    username: 'jane',
    email: 'jane@example.com',
    displayName: 'Jane Smith',
    role: 'moderator',
    status: 'active',
    createdAt: '2024-03-10',
    lastLogin: '2025-11-08T14:20:00Z',
    quota: { used: 15000000, limit: 75000000 },
    avatar: '👩‍💼',
  },
  {
    id: 'user-4',
    username: 'bob',
    email: 'bob@example.com',
    displayName: 'Bob Wilson',
    role: 'user',
    status: 'inactive',
    createdAt: '2024-04-05',
    lastLogin: '2025-11-01T16:00:00Z',
    quota: { used: 500000, limit: 50000000 },
    avatar: '👨',
  },
];

const mockAuditLog = [
  {
    id: 'audit-1',
    timestamp: '2025-11-09T10:30:00Z',
    user: 'john',
    action: 'file_upload',
    resource: '/documents/report.pdf',
    ipAddress: '192.168.1.100',
    status: 'success',
    metadata: { size: 2500000 },
  },
  {
    id: 'audit-2',
    timestamp: '2025-11-09T10:15:00Z',
    user: 'jane',
    action: 'user_login',
    resource: 'user_session',
    ipAddress: '192.168.1.101',
    status: 'success',
    metadata: { device: 'Chrome/Windows' },
  },
  {
    id: 'audit-3',
    timestamp: '2025-11-09T09:45:00Z',
    user: 'admin',
    action: 'user_delete',
    resource: 'user-old-123',
    ipAddress: '192.168.1.50',
    status: 'success',
    metadata: { reason: 'Account inactive' },
  },
  {
    id: 'audit-4',
    timestamp: '2025-11-09T09:30:00Z',
    user: 'bob',
    action: 'failed_login',
    resource: 'user_session',
    ipAddress: '203.0.113.50',
    status: 'failed',
    metadata: { attempts: 3, locked: true },
  },
  {
    id: 'audit-5',
    timestamp: '2025-11-09T08:00:00Z',
    user: 'admin',
    action: 'system_config_change',
    resource: 'system_settings',
    ipAddress: '192.168.1.50',
    status: 'success',
    metadata: { setting: 'max_upload_size', oldValue: '1GB', newValue: '2GB' },
  },
];

const mockSystemStats = {
  totalUsers: 4,
  activeUsers: 3,
  totalStorage: 68000000,
  maxStorage: 275000000,
  uploadCount24h: 42,
  loginCount24h: 156,
  errorCount24h: 8,
};

const mockRoles = [
  {
    id: 'role-admin',
    name: 'Administrator',
    description: 'Full system access',
    permissions: [
      'users.read',
      'users.create',
      'users.edit',
      'users.delete',
      'system.read',
      'system.edit',
      'audit.read',
      'storage.manage',
    ],
    userCount: 1,
  },
  {
    id: 'role-moderator',
    name: 'Moderator',
    description: 'Content moderation',
    permissions: [
      'users.read',
      'files.moderate',
      'audit.read',
      'reports.read',
    ],
    userCount: 1,
  },
  {
    id: 'role-user',
    name: 'User',
    description: 'Standard user',
    permissions: [
      'files.read',
      'files.create',
      'files.edit',
      'sharing.create',
    ],
    userCount: 2,
  },
];

// Create stores
const users = writable(mockUsers);
const auditLog = writable(mockAuditLog);
const systemStats = writable(mockSystemStats);
const roles = writable(mockRoles);

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
    const allUsers = mockUsers.find((u) => u.id === id);
    return allUsers;
  },

  async createUser(userData) {
    const newUser = {
      id: `user-${Date.now()}`,
      status: 'active',
      createdAt: new Date().toISOString(),
      lastLogin: null,
      quota: { used: 0, limit: 50000000 },
      avatar: '👤',
      ...userData,
    };

    users.update((u) => [...u, newUser]);
    return newUser;
  },

  async updateUser(id, updates) {
    users.update((u) =>
      u.map((user) => (user.id === id ? { ...user, ...updates } : user))
    );
  },

  async deleteUser(id) {
    users.update((u) => u.filter((user) => user.id !== id));
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
    return mockAuditLog.find((e) => e.id === id);
  },

  async exportAuditLog(format = 'json') {
    const data = mockAuditLog;
    if (format === 'csv') {
      const headers = Object.keys(mockAuditLog[0]);
      const rows = mockAuditLog.map((e) =>
        headers.map((h) => JSON.stringify(e[h])).join(',')
      );
      return [headers.join(','), ...rows].join('\n');
    }
    return JSON.stringify(data, null, 2);
  },

  // Roles management
  roles: { subscribe: roles.subscribe },

  async getRoles() {
    return mockRoles;
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
    return mockSystemStats;
  },

  async getStorageStats() {
    const totalUsed = mockUsers.reduce((sum, u) => sum + u.quota.used, 0);
    const totalLimit = mockUsers.reduce((sum, u) => sum + u.quota.limit, 0);

    return {
      used: totalUsed,
      limit: totalLimit,
      percentage: Math.round((totalUsed / totalLimit) * 100),
      topUsers: mockUsers
        .sort((a, b) => b.quota.used - a.quota.used)
        .slice(0, 5)
        .map((u) => ({
          username: u.username,
          used: u.quota.used,
          percentage: Math.round((u.quota.used / u.quota.limit) * 100),
        })),
    };
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
    const data = mockUsers;
    if (format === 'csv') {
      const headers = ['username', 'email', 'displayName', 'role', 'status'];
      const rows = mockUsers.map((u) =>
        headers.map((h) => JSON.stringify(u[h])).join(',')
      );
      return [headers.join(','), ...rows].join('\n');
    }
    return JSON.stringify(data, null, 2);
  },

  async importUsers(data) {
    // Parse and validate CSV/JSON
    const users = Array.isArray(data) ? data : JSON.parse(data);
    
    const newUsers = users.map((u) => ({
      id: `user-${Date.now()}-${Math.random()}`,
      status: 'active',
      createdAt: new Date().toISOString(),
      lastLogin: null,
      quota: { used: 0, limit: 50000000 },
      avatar: '👤',
      ...u,
    }));

    users.update((u) => [...u, ...newUsers]);
    return newUsers;
  },
};

export default adminConsole;
