<script>
  /**
   * Admin Console View
   * User management, roles, audit log, system statistics
   * 
   * Tabs:
   * - Users: User list with filters, actions (edit, delete, role change)
   * - Roles: Role management with permissions
   * - Audit Log: System activity and security logs
   * - Statistics: System stats, storage usage, user analytics
   */

  import { onMount } from 'svelte';
  import { t } from '../lib/i18n.js';
  import { adminConsole, filteredUsers, userStats, storageStats, filteredAuditLog } from '../stores/adminConsole.js';

  let activeTab = $state('users');
  let selectedUsers = $state(new Set());
  let showDeleteConfirm = $state(false);
  let showUserModal = $state(false);
  let editingUser = $state(null);

  let filter = $state({
    search: '',
    role: 'all',
    status: 'all',
  });

  let auditFilter = $state({
    search: '',
    action: 'all',
    status: 'all',
  });

  let users = $state([]);
  let userStatistics = $state({});
  let storageStatistics = $state({});
  let auditLog = $state([]);

  onMount(() => {
    // Subscribe to stores
    const unsubscribeUsers = filteredUsers.subscribe((u) => (users = u));
    const unsubscribeStats = userStats.subscribe((s) => (userStatistics = s));
    const unsubscribeStorage = storageStats.subscribe((s) => (storageStatistics = s));
    const unsubscribeAudit = filteredAuditLog.subscribe((a) => (auditLog = a));

    return () => {
      unsubscribeUsers();
      unsubscribeStats();
      unsubscribeStorage();
      unsubscribeAudit();
    };
  });

  function updateFilter(updates) {
    filter = { ...filter, ...updates };
    adminConsole.setUserFilter(filter);
  }

  function updateAuditFilter(updates) {
    auditFilter = { ...auditFilter, ...updates };
    adminConsole.setAuditFilter(auditFilter);
  }

  function toggleUserSelection(userId) {
    const newSelection = new Set(selectedUsers);
    if (newSelection.has(userId)) {
      newSelection.delete(userId);
    } else {
      newSelection.add(userId);
    }
    selectedUsers = newSelection;
  }

  function selectAllUsers() {
    if (selectedUsers.size === users.length) {
      selectedUsers = new Set();
    } else {
      selectedUsers = new Set(users.map((u) => u.id));
    }
  }

  async function deleteSelectedUsers() {
    if (selectedUsers.size > 0) {
      await adminConsole.bulkDeleteUsers(Array.from(selectedUsers));
      selectedUsers = new Set();
      showDeleteConfirm = false;
    }
  }

  async function updateUserRole(userId, newRole) {
    await adminConsole.updateUser(userId, { role: newRole });
  }

  async function updateUserStatus(userId, newStatus) {
    await adminConsole.updateUser(userId, { status: newStatus });
  }

  async function exportData(type, format) {
    let data;
    if (type === 'users') {
      data = await adminConsole.exportUsers(format);
    } else if (type === 'audit') {
      data = await adminConsole.exportAuditLog(format);
    }

    // Download file
    const blob = new Blob([data], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${type}-export.${format === 'json' ? 'json' : 'csv'}`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  // Calculate percentages for progress bars
  function getQuotaPercentage(used, limit) {
    return Math.round((used / limit) * 100);
  }
</script>

<div class="admin-console h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
  <!-- Header -->
  <div class="border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-sm">
    <div class="px-6 py-4">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
        <i class="bi bi-sliders text-primary-500"></i>
        Admin Console
      </h1>
      <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
        Manage users, roles, audit logs, and system settings
      </p>
    </div>

    <!-- Tabs -->
    <div class="px-6 border-t border-gray-200 dark:border-gray-700 flex gap-8">
      <button
        onclick={() => (activeTab = 'users')}
        class="px-2 py-3 border-b-2 font-medium text-sm transition-colors"
        class:border-primary-500={activeTab === 'users'}
        class:text-primary-600={activeTab === 'users'}
        class:border-transparent={activeTab !== 'users'}
        class:text-gray-600={activeTab !== 'users'}
      >
        <i class="bi bi-people"></i> Users ({userStatistics.total || 0})
      </button>

      <button
        onclick={() => (activeTab = 'roles')}
        class="px-2 py-3 border-b-2 font-medium text-sm transition-colors"
        class:border-primary-500={activeTab === 'roles'}
        class:text-primary-600={activeTab === 'roles'}
        class:border-transparent={activeTab !== 'roles'}
        class:text-gray-600={activeTab !== 'roles'}
      >
        <i class="bi bi-shield-check"></i> Roles
      </button>

      <button
        onclick={() => (activeTab = 'audit')}
        class="px-2 py-3 border-b-2 font-medium text-sm transition-colors"
        class:border-primary-500={activeTab === 'audit'}
        class:text-primary-600={activeTab === 'audit'}
        class:border-transparent={activeTab !== 'audit'}
        class:text-gray-600={activeTab !== 'audit'}
      >
        <i class="bi bi-clock-history"></i> Audit Log
      </button>

      <button
        onclick={() => (activeTab = 'stats')}
        class="px-2 py-3 border-b-2 font-medium text-sm transition-colors"
        class:border-primary-500={activeTab === 'stats'}
        class:text-primary-600={activeTab === 'stats'}
        class:border-transparent={activeTab !== 'stats'}
        class:text-gray-600={activeTab !== 'stats'}
      >
        <i class="bi bi-bar-chart"></i> Statistics
      </button>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto">
    {#if activeTab === 'users'}
      <!-- Users Tab -->
      <div class="p-6">
        <!-- Filters -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
          <input
            type="text"
            placeholder="Search users..."
            value={filter.search}
            onchange={(e) => updateFilter({ search: e.target.value })}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          />

          <select
            value={filter.role}
            onchange={(e) => updateFilter({ role: e.target.value })}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Roles</option>
            <option value="admin">Administrator</option>
            <option value="moderator">Moderator</option>
            <option value="user">User</option>
          </select>

          <select
            value={filter.status}
            onchange={(e) => updateFilter({ status: e.target.value })}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Status</option>
            <option value="active">Active</option>
            <option value="inactive">Inactive</option>
          </select>

          <button
            onclick={() => adminConsole.resetUserFilter()}
            class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
          >
            <i class="bi bi-arrow-clockwise"></i> Reset
          </button>
        </div>

        <!-- Bulk Actions -->
        {#if selectedUsers.size > 0}
          <div class="mb-4 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg flex justify-between items-center">
            <span class="text-sm font-medium text-blue-900 dark:text-blue-200">
              {selectedUsers.size} user(s) selected
            </span>
            <div class="flex gap-2">
              <button
                onclick={() => (showDeleteConfirm = true)}
                class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 text-sm"
              >
                <i class="bi bi-trash"></i> Delete
              </button>
            </div>
          </div>
        {/if}

        <!-- Users Table -->
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
          <table class="w-full">
            <thead class="bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600">
              <tr>
                <th class="px-6 py-3 text-left">
                  <input
                    type="checkbox"
                    checked={selectedUsers.size === users.length && users.length > 0}
                    onchange={selectAllUsers}
                    class="rounded"
                  />
                </th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">User</th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">Email</th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">Role</th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">Status</th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">Storage</th>
                <th class="px-6 py-3 text-left text-sm font-semibold text-gray-700 dark:text-gray-300">Last Login</th>
                <th class="px-6 py-3 text-right text-sm font-semibold text-gray-700 dark:text-gray-300">Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each users as user (user.id)}
                <tr class="border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                  <td class="px-6 py-4">
                    <input
                      type="checkbox"
                      checked={selectedUsers.has(user.id)}
                      onchange={() => toggleUserSelection(user.id)}
                      class="rounded"
                    />
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-2">
                      <span class="text-xl">{user.avatar}</span>
                      <div>
                        <p class="font-medium text-gray-900 dark:text-white">{user.displayName}</p>
                        <p class="text-sm text-gray-500 dark:text-gray-400">@{user.username}</p>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400">{user.email}</td>
                  <td class="px-6 py-4">
                    <select
                      value={user.role}
                      onchange={(e) => updateUserRole(user.id, e.target.value)}
                      class="px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded dark:bg-gray-700 dark:text-white"
                    >
                      <option value="admin">Admin</option>
                      <option value="moderator">Moderator</option>
                      <option value="user">User</option>
                    </select>
                  </td>
                  <td class="px-6 py-4">
                    <select
                      value={user.status}
                      onchange={(e) => updateUserStatus(user.id, e.target.value)}
                      class="px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded dark:bg-gray-700 dark:text-white"
                    >
                      <option value="active">Active</option>
                      <option value="inactive">Inactive</option>
                    </select>
                  </td>
                  <td class="px-6 py-4">
                    <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        class="bg-primary-500 h-2 rounded-full"
                        style="width: {getQuotaPercentage(user.quota.used, user.quota.limit)}%"
                      />
                    </div>
                    <p class="text-xs text-gray-600 dark:text-gray-400 mt-1">
                      {Math.round(user.quota.used / 1000000)} / {Math.round(user.quota.limit / 1000000)} MB
                    </p>
                  </td>
                  <td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400">
                    {new Date(user.lastLogin).toLocaleDateString()}
                  </td>
                  <td class="px-6 py-4 text-right">
                    <button class="text-primary-600 hover:text-primary-700 text-sm font-medium">
                      <i class="bi bi-pencil"></i>
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>

          {#if users.length === 0}
            <div class="p-8 text-center text-gray-500 dark:text-gray-400">
              <i class="bi bi-inbox text-4xl mb-2 block"></i>
              <p>No users found</p>
            </div>
          {/if}
        </div>
      </div>
    {:else if activeTab === 'roles'}
      <!-- Roles Tab -->
      <div class="p-6">
        <p class="text-gray-600 dark:text-gray-400">Roles management UI - coming soon</p>
      </div>
    {:else if activeTab === 'audit'}
      <!-- Audit Log Tab -->
      <div class="p-6">
        <!-- Filters -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
          <input
            type="text"
            placeholder="Search audit logs..."
            value={auditFilter.search}
            onchange={(e) => updateAuditFilter({ search: e.target.value })}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          />

          <select
            value={auditFilter.status}
            onchange={(e) => updateAuditFilter({ status: e.target.value })}
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          >
            <option value="all">All Status</option>
            <option value="success">Success</option>
            <option value="failed">Failed</option>
          </select>

          <button
            onclick={() => exportData('audit', 'json')}
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          >
            <i class="bi bi-download"></i> Export JSON
          </button>

          <button
            onclick={() => exportData('audit', 'csv')}
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          >
            <i class="bi bi-download"></i> Export CSV
          </button>
        </div>

        <!-- Audit Log Table -->
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
          <table class="w-full text-sm">
            <thead class="bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600">
              <tr>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">Time</th>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">User</th>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">Action</th>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">Resource</th>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">IP Address</th>
                <th class="px-6 py-3 text-left font-semibold text-gray-700 dark:text-gray-300">Status</th>
              </tr>
            </thead>
            <tbody>
              {#each auditLog as entry (entry.id)}
                <tr class="border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/50">
                  <td class="px-6 py-4">{new Date(entry.timestamp).toLocaleString()}</td>
                  <td class="px-6 py-4">{entry.user}</td>
                  <td class="px-6 py-4">{entry.action}</td>
                  <td class="px-6 py-4 text-gray-600 dark:text-gray-400">{entry.resource}</td>
                  <td class="px-6 py-4 font-mono text-xs">{entry.ipAddress}</td>
                  <td class="px-6 py-4">
                    <span
                      class="px-2 py-1 rounded text-xs font-medium"
                      class:bg-green-100={entry.status === 'success'}
                      class:text-green-800={entry.status === 'success'}
                      class:bg-red-100={entry.status === 'failed'}
                      class:text-red-800={entry.status === 'failed'}
                    >
                      {entry.status}
                    </span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>

          {#if auditLog.length === 0}
            <div class="p-8 text-center text-gray-500 dark:text-gray-400">
              <i class="bi bi-inbox text-4xl mb-2 block"></i>
              <p>No audit logs found</p>
            </div>
          {/if}
        </div>
      </div>
    {:else if activeTab === 'stats'}
      <!-- Statistics Tab -->
      <div class="p-6">
        <!-- Stats Cards -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">Total Users</p>
            <p class="text-3xl font-bold text-gray-900 dark:text-white">{userStatistics.total || 0}</p>
          </div>

          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">Active Users</p>
            <p class="text-3xl font-bold text-green-600">{userStatistics.active || 0}</p>
          </div>

          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">Inactive Users</p>
            <p class="text-3xl font-bold text-gray-600">{userStatistics.inactive || 0}</p>
          </div>

          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">Storage Used</p>
            <p class="text-3xl font-bold text-primary-600">
              {Math.round((storageStatistics.used || 0) / 1000000)} MB
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              {storageStatistics.percentage || 0}% of {Math.round((storageStatistics.limit || 0) / 1000000)} MB
            </p>
          </div>
        </div>

        <!-- Storage by User -->
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Storage by User</h3>
          <div class="space-y-4">
            {#each storageStatistics.byUser || [] as userStorage}
              <div>
                <div class="flex justify-between mb-1">
                  <span class="text-sm font-medium text-gray-900 dark:text-white">{userStorage.username}</span>
                  <span class="text-sm text-gray-600 dark:text-gray-400">
                    {Math.round(userStorage.used / 1000000)} / {Math.round(userStorage.limit / 1000000)} MB
                  </span>
                </div>
                <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                  <div
                    class="bg-primary-500 h-2 rounded-full"
                    style="width: {userStorage.percentage}%"
                  />
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

{#if showDeleteConfirm}
  <!-- Delete Confirmation Modal -->
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 max-w-md">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        Delete {selectedUsers.size} user(s)?
      </h3>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        This action cannot be undone. All data will be permanently deleted.
      </p>
      <div class="flex gap-3">
        <button
          onclick={() => (showDeleteConfirm = false)}
          class="flex-1 px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
        >
          Cancel
        </button>
        <button
          onclick={deleteSelectedUsers}
          class="flex-1 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Delete
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .admin-console {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
      'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  }
</style>
