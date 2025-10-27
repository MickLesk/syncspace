<script>
  import { onMount } from "svelte";
  import { showToast } from "../stores/toast.js";

  let users = $state([]);
  let loading = $state(true);
  let viewMode = $state("table");
  let selectedUsers = $state(new Set());
  let filterRole = $state("all");

  const mockUsers = [
    {
      id: 1,
      username: "admin",
      email: "admin@syncspace.com",
      role: "admin",
      status: "online",
      avatar: null,
      lastActive: new Date(),
      filesCount: 1247,
      storageUsed: 15.4 * 1024 * 1024 * 1024,
    },
    {
      id: 2,
      username: "john_doe",
      email: "john@example.com",
      role: "user",
      status: "offline",
      avatar: null,
      lastActive: new Date(Date.now() - 2 * 60 * 60 * 1000),
      filesCount: 532,
      storageUsed: 8.2 * 1024 * 1024 * 1024,
    },
    {
      id: 3,
      username: "jane_smith",
      email: "jane@example.com",
      role: "user",
      status: "online",
      avatar: null,
      lastActive: new Date(),
      filesCount: 891,
      storageUsed: 12.1 * 1024 * 1024 * 1024,
    },
  ];

  onMount(() => {
    setTimeout(() => {
      users = mockUsers;
      loading = false;
    }, 300);
  });

  function getRoleBadgeClass(role) {
    const badges = {
      admin: "bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200",
      moderator:
        "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-200",
      user: "bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200",
    };
    return (
      badges[role] ||
      "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300"
    );
  }

  function getStatusIndicator(status) {
    return status === "online"
      ? { color: "bg-success", text: "Online" }
      : { color: "bg-slate-100 dark:bg-slate-700", text: "Offline" };
  }

  function getUserInitials(username) {
    return username.slice(0, 2).toUpperCase();
  }

  function formatBytes(bytes) {
    return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
  }

  function formatLastActive(date) {
    const diff = Date.now() - date;
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    if (minutes < 1) return "Just now";
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return date.toLocaleDateString();
  }

  function toggleSelectUser(userId) {
    selectedUsers.has(userId)
      ? selectedUsers.delete(userId)
      : selectedUsers.add(userId);
    selectedUsers = selectedUsers;
  }

  function toggleSelectAll() {
    selectedUsers =
      selectedUsers.size === filteredUsers.length
        ? new Set()
        : new Set(filteredUsers.map((u) => u.id));
  }

  let filteredUsers = $derived(
    filterRole === "all" ? users : users.filter((u) => u.role === filterRole)
  );
</script>

<div class="users-view">
  <div class="users-header">
    <div class="header-top">
      <div>
        <h1 class="text-3xl font-bold">
          <i class="bi bi-people-fill"></i> Users
        </h1>
        <p class="text-base-content/60 mt-1">{filteredUsers.length} users</p>
      </div>
      <div class="header-actions">
        <div
          class="inline-flex rounded-lg border border-gray-300 dark:border-gray-600"
        >
          <button
            class="px-3 py-1.5 text-sm rounded-l-lg border-r border-gray-300 dark:border-gray-600 transition-colors {viewMode ===
            'table'
              ? 'bg-blue-600 text-white'
              : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200'}"
            onclick={() => (viewMode = "table")}
            aria-label="Table view"><i class="bi bi-table"></i></button
          >
          <button
            class="px-3 py-1.5 text-sm rounded-r-lg transition-colors {viewMode ===
            'cards'
              ? 'bg-blue-600 text-white'
              : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200'}"
            onclick={() => (viewMode = "cards")}
            aria-label="Cards view"><i class="bi bi-grid-3x3-gap"></i></button
          >
        </div>
        <button
          class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
          ><i class="bi bi-plus-lg"></i> Add User</button
        >
      </div>
    </div>
    <div class="filter-row">
      <div
        class="inline-flex rounded-lg border border-gray-300 dark:border-gray-600"
      >
        <button
          class="px-3 py-1.5 text-sm rounded-l-lg border-r border-gray-300 dark:border-gray-600 transition-colors {filterRole ===
          'all'
            ? 'bg-blue-600 text-white'
            : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200'}"
          onclick={() => (filterRole = "all")}>All</button
        >
        <button
          class="px-3 py-1.5 text-sm border-r border-gray-300 dark:border-gray-600 transition-colors {filterRole ===
          'admin'
            ? 'bg-blue-600 text-white'
            : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200'}"
          onclick={() => (filterRole = "admin")}>Admins</button
        >
        <button
          class="px-3 py-1.5 text-sm rounded-r-lg transition-colors {filterRole ===
          'user'
            ? 'bg-blue-600 text-white'
            : 'bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200'}"
          onclick={() => (filterRole = "user")}>Users</button
        >
      </div>
      {#if selectedUsers.size > 0}
        <div class="bulk-actions">
          <span class="text-sm">{selectedUsers.size} selected</span>
          <button
            class="px-3 py-1.5 text-sm bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors flex items-center gap-2"
            ><i class="bi bi-trash"></i> Delete</button
          >
        </div>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="loading-container">
      <div
        class="w-12 h-12 border-4 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin"
      ></div>
    </div>
  {:else if viewMode === "table"}
    <div class="table-container">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th
              ><input
                type="checkbox"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
                onchange={toggleSelectAll}
                checked={selectedUsers.size === filteredUsers.length}
              /></th
            >
            <th>User</th>
            <th>Role</th>
            <th>Status</th>
            <th>Files</th>
            <th>Storage</th>
            <th>Last Active</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredUsers as user (user.id)}
            <tr class:selected={selectedUsers.has(user.id)}>
              <td
                ><input
                  type="checkbox"
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
                  checked={selectedUsers.has(user.id)}
                  onchange={() => toggleSelectUser(user.id)}
                /></td
              >
              <td>
                <div class="flex items-center gap-3">
                  <div class="avatar-cell">
                    <div class="avatar-placeholder">
                      {getUserInitials(user.username)}
                    </div>
                    <div
                      class="status-dot {getStatusIndicator(user.status).color}"
                    ></div>
                  </div>
                  <div>
                    <div class="font-semibold">{user.username}</div>
                    <div class="text-sm text-base-content/60">{user.email}</div>
                  </div>
                </div>
              </td>
              <td
                ><span
                  class="px-2 py-0.5 text-xs {getRoleBadgeClass(
                    user.role
                  )} rounded-full">{user.role}</span
                ></td
              >
              <td
                ><span class="text-sm"
                  >{getStatusIndicator(user.status).text}</span
                ></td
              >
              <td>{user.filesCount.toLocaleString()}</td>
              <td>{formatBytes(user.storageUsed)}</td>
              <td class="text-sm text-base-content/60"
                >{formatLastActive(user.lastActive)}</td
              >
              <td>
                <div class="flex gap-1">
                  <button
                    class="px-2 py-1 text-xs hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded transition-colors"
                    aria-label="Edit user"><i class="bi bi-pencil"></i></button
                  >
                  <button
                    class="px-2 py-1 text-xs hover:bg-gray-100 dark:hover:bg-gray-800 text-red-600 dark:text-red-400 rounded transition-colors"
                    aria-label="Delete user"><i class="bi bi-trash"></i></button
                  >
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="cards-grid">
      {#each filteredUsers as user (user.id)}
        <div class="user-card" class:selected={selectedUsers.has(user.id)}>
          <div class="card-header">
            <input
              type="checkbox"
              class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
              checked={selectedUsers.has(user.id)}
              onchange={() => toggleSelectUser(user.id)}
            />
            <div
              class="status-dot {getStatusIndicator(user.status).color}"
            ></div>
          </div>
          <div class="card-avatar">
            <div class="avatar-placeholder-large">
              {getUserInitials(user.username)}
            </div>
          </div>
          <div class="card-info">
            <h3 class="user-name">{user.username}</h3>
            <p class="user-email">{user.email}</p>
            <span
              class="px-2 py-0.5 text-xs {getRoleBadgeClass(
                user.role
              )} rounded-full mt-2 inline-block">{user.role}</span
            >
          </div>
          <div class="card-stats">
            <div class="stat-item">
              <i class="bi bi-files"></i> <span>{user.filesCount}</span>
            </div>
            <div class="stat-item">
              <i class="bi bi-hdd"></i>
              <span>{formatBytes(user.storageUsed)}</span>
            </div>
          </div>
          <div class="card-footer">
            <span class="text-xs text-base-content/50"
              >{formatLastActive(user.lastActive)}</span
            >
            <div class="flex gap-1">
              <button
                class="px-2 py-1 text-xs hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded transition-colors"
                aria-label="Edit user"><i class="bi bi-pencil"></i></button
              >
              <button
                class="px-2 py-1 text-xs hover:bg-gray-100 dark:hover:bg-gray-800 text-red-600 dark:text-red-400 rounded transition-colors"
                aria-label="Delete user"><i class="bi bi-trash"></i></button
              >
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .users-view {
    padding: 0;
  }
  .users-header {
    background: hsl(var(--b1));
    border-bottom: 1px solid hsl(var(--bc) / 0.1);
    padding: 2rem;
    margin: -2rem -2rem 2rem -2rem;
  }
  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1.5rem;
  }
  .header-actions {
    display: flex;
    gap: 0.75rem;
  }
  .filter-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }
  .bulk-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background: hsl(var(--b2));
    border-radius: var(--rounded-btn);
  }
  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
  .table-container {
    margin: 0 2rem 2rem 2rem;
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    overflow: hidden;
  }
  .table tr.selected {
    background: hsl(var(--p) / 0.1);
  }
  .avatar-cell {
    position: relative;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
  }
  .avatar-placeholder {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: hsl(var(--p) / 0.2);
    color: hsl(var(--p));
    font-weight: 600;
    font-size: 0.875rem;
  }
  .status-dot {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 0.75rem;
    height: 0.75rem;
    border-radius: 50%;
    border: 2px solid hsl(var(--b1));
  }
  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
    padding: 2rem;
  }
  .user-card {
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    padding: 1.5rem;
    text-align: center;
    transition: all 0.3s;
  }
  .user-card:hover {
    border-color: hsl(var(--p) / 0.3);
    transform: translateY(-4px);
  }
  .user-card.selected {
    border-color: hsl(var(--p));
    background: hsl(var(--p) / 0.05);
  }
  .card-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  .card-avatar {
    margin: 0 auto 1rem auto;
    width: 5rem;
    height: 5rem;
    border-radius: 50%;
    position: relative;
  }
  .avatar-placeholder-large {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: hsl(var(--p) / 0.2);
    color: hsl(var(--p));
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: 700;
  }
  .card-info {
    margin-bottom: 1rem;
  }
  .user-name {
    font-weight: 700;
    font-size: 1.125rem;
  }
  .user-email {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.6);
  }
  .card-stats {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    padding: 1rem 0;
    border-top: 1px solid hsl(var(--bc) / 0.1);
    border-bottom: 1px solid hsl(var(--bc) / 0.1);
    margin-bottom: 1rem;
  }
  .stat-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
  }
  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  @media (max-width: 768px) {
    .users-header {
      padding: 1.5rem;
      margin: -1.5rem -1.5rem 1.5rem -1.5rem;
    }
    .header-top {
      flex-direction: column;
      gap: 1rem;
    }
    .table-container {
      margin: 0 1rem 1rem 1rem;
    }
    .cards-grid {
      grid-template-columns: 1fr;
      padding: 1rem;
    }
  }
</style>
