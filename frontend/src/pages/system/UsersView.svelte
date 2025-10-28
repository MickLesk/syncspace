<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";

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

<!-- Main Container -->
<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Content Container -->
  <div class="relative z-10 max-w-7xl mx-auto">
    <!-- Header -->
    <div class="glass-card mb-6 p-6">
      <div class="flex items-center justify-between flex-wrap gap-4 mb-6">
        <div>
          <h1
            class="text-3xl font-bold gradient-text mb-2 flex items-center gap-3"
          >
            <i class="bi bi-people-fill"></i>
            Users
          </h1>
          <p class="text-gray-600 dark:text-gray-400">
            {filteredUsers.length} user{filteredUsers.length !== 1 ? "s" : ""}
          </p>
        </div>

        <div class="flex gap-3 flex-wrap">
          <!-- View Mode Toggle -->
          <div
            class="flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600"
          >
            <button
              onclick={() => (viewMode = "table")}
              class="px-3 py-2 text-sm transition-colors"
              class:bg-gradient-to-r={viewMode === "table"}
              class:from-blue-500={viewMode === "table"}
              class:to-purple-600={viewMode === "table"}
              class:text-white={viewMode === "table"}
              class:bg-white={viewMode !== "table"}
              class:dark:bg-gray-800={viewMode !== "table"}
              class:hover:bg-gray-100={viewMode !== "table"}
              class:dark:hover:bg-gray-700={viewMode !== "table"}
              class:text-gray-700={viewMode !== "table"}
              class:dark:text-gray-200={viewMode !== "table"}
              aria-label="Table view"
            >
              <i class="bi bi-w-full border-collapse"></i>
            </button>
            <button
              onclick={() => (viewMode = "cards")}
              class="px-3 py-2 text-sm border-l border-gray-300 dark:border-gray-600 transition-colors"
              class:bg-gradient-to-r={viewMode === "cards"}
              class:from-blue-500={viewMode === "cards"}
              class:to-purple-600={viewMode === "cards"}
              class:text-white={viewMode === "cards"}
              class:bg-white={viewMode !== "cards"}
              class:dark:bg-gray-800={viewMode !== "cards"}
              class:hover:bg-gray-100={viewMode !== "cards"}
              class:dark:hover:bg-gray-700={viewMode !== "cards"}
              class:text-gray-700={viewMode !== "cards"}
              class:dark:text-gray-200={viewMode !== "cards"}
              aria-label="Cards view"
            >
              <i class="bi bi-grid-3x3-gap"></i>
            </button>
          </div>

          <button
            class="px-4 py-2 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2"
          >
            <i class="bi bi-plus-lg"></i>
            Add User
          </button>
        </div>
      </div>

      <!-- Filters -->
      <div class="flex items-center justify-between flex-wrap gap-4">
        <div
          class="flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600"
        >
          <button
            onclick={() => (filterRole = "all")}
            class="px-4 py-2 text-sm transition-colors"
            class:bg-gradient-to-r={filterRole === "all"}
            class:from-blue-500={filterRole === "all"}
            class:to-purple-600={filterRole === "all"}
            class:text-white={filterRole === "all"}
            class:bg-white={filterRole !== "all"}
            class:dark:bg-gray-800={filterRole !== "all"}
            class:hover:bg-gray-100={filterRole !== "all"}
            class:dark:hover:bg-gray-700={filterRole !== "all"}
            class:text-gray-700={filterRole !== "all"}
            class:dark:text-gray-200={filterRole !== "all"}
          >
            All
          </button>
          <button
            onclick={() => (filterRole = "admin")}
            class="px-4 py-2 text-sm border-l border-gray-300 dark:border-gray-600 transition-colors"
            class:bg-gradient-to-r={filterRole === "admin"}
            class:from-blue-500={filterRole === "admin"}
            class:to-purple-600={filterRole === "admin"}
            class:text-white={filterRole === "admin"}
            class:bg-white={filterRole !== "admin"}
            class:dark:bg-gray-800={filterRole !== "admin"}
            class:hover:bg-gray-100={filterRole !== "admin"}
            class:dark:hover:bg-gray-700={filterRole !== "admin"}
            class:text-gray-700={filterRole !== "admin"}
            class:dark:text-gray-200={filterRole !== "admin"}
          >
            Admins
          </button>
          <button
            onclick={() => (filterRole = "user")}
            class="px-4 py-2 text-sm border-l border-gray-300 dark:border-gray-600 transition-colors"
            class:bg-gradient-to-r={filterRole === "user"}
            class:from-blue-500={filterRole === "user"}
            class:to-purple-600={filterRole === "user"}
            class:text-white={filterRole === "user"}
            class:bg-white={filterRole !== "user"}
            class:dark:bg-gray-800={filterRole !== "user"}
            class:hover:bg-gray-100={filterRole !== "user"}
            class:dark:hover:bg-gray-700={filterRole !== "user"}
            class:text-gray-700={filterRole !== "user"}
            class:dark:text-gray-200={filterRole !== "user"}
          >
            Users
          </button>
        </div>

        {#if selectedUsers.size > 0}
          <div
            class="flex items-center gap-4 glass-card-light px-4 py-2 rounded-lg"
          >
            <span class="text-sm font-medium text-gray-900 dark:text-gray-100">
              {selectedUsers.size} selected
            </span>
            <button
              class="px-3 py-1.5 text-sm rounded-lg font-medium bg-gradient-to-r from-red-500 to-red-600 text-white hover:from-red-600 hover:to-red-700 transition-all flex items-center gap-2"
            >
              <i class="bi bi-trash"></i>
              Delete
            </button>
          </div>
        {/if}
      </div>
    </div>

    {#if loading}
      <div class="glass-card text-center py-16">
        <div
          class="w-12 h-12 border-4 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin mx-auto"
        ></div>
      </div>
    {:else if viewMode === "table"}
      <!-- Table View -->
      <div class="glass-card overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr
                class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50"
              >
                <th class="px-6 py-4 text-left">
                  <input
                    type="checkbox"
                    class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500"
                    onchange={toggleSelectAll}
                    checked={selectedUsers.size === filteredUsers.length}
                  />
                </th>
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >User</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Role</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Status</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Files</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Storage</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Last Active</th
                >
                <th
                  class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                  >Actions</th
                >
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
              {#each filteredUsers as user (user.id)}
                <tr
                  class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors {selectedUsers.has(
                    user.id
                  )
                    ? 'bg-blue-50 dark:bg-blue-900/20'
                    : ''}"
                >
                  <td class="px-6 py-4">
                    <input
                      type="checkbox"
                      class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500"
                      checked={selectedUsers.has(user.id)}
                      onchange={() => toggleSelectUser(user.id)}
                    />
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-3">
                      <div class="relative">
                        <div
                          class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-semibold text-sm"
                        >
                          {getUserInitials(user.username)}
                        </div>
                        <div
                          class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full border-2 border-white dark:border-gray-800 {getStatusIndicator(
                            user.status
                          ).color}"
                        ></div>
                      </div>
                      <div>
                        <div
                          class="font-semibold text-gray-900 dark:text-gray-100"
                        >
                          {user.username}
                        </div>
                        <div class="text-sm text-gray-500 dark:text-gray-400">
                          {user.email}
                        </div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <span
                      class="px-2 py-1 text-xs font-medium {getRoleBadgeClass(
                        user.role
                      )} rounded-full"
                    >
                      {user.role}
                    </span>
                  </td>
                  <td class="px-6 py-4">
                    <span class="text-sm text-gray-700 dark:text-gray-300">
                      {getStatusIndicator(user.status).text}
                    </span>
                  </td>
                  <td class="px-6 py-4 text-gray-700 dark:text-gray-300"
                    >{user.filesCount.toLocaleString()}</td
                  >
                  <td class="px-6 py-4 text-gray-700 dark:text-gray-300"
                    >{formatBytes(user.storageUsed)}</td
                  >
                  <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400"
                    >{formatLastActive(user.lastActive)}</td
                  >
                  <td class="px-6 py-4">
                    <div class="flex gap-2">
                      <button
                        class="p-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                        aria-label="Edit user"
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                      <button
                        class="p-2 rounded-lg text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                        aria-label="Delete user"
                      >
                        <i class="bi bi-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {:else}
      <!-- Cards View -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each filteredUsers as user (user.id)}
          <div
            class="glass-card p-6 hover:shadow-xl transition-all duration-300 relative"
            class:ring-2={selectedUsers.has(user.id)}
            class:ring-blue-500={selectedUsers.has(user.id)}
          >
            <!-- Selection Checkbox -->
            <div class="absolute top-4 right-4 flex items-center gap-2">
              <input
                type="checkbox"
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500"
                checked={selectedUsers.has(user.id)}
                onchange={() => toggleSelectUser(user.id)}
              />
              <div
                class="w-3 h-3 rounded-full {getStatusIndicator(user.status)
                  .color}"
              ></div>
            </div>

            <!-- Avatar -->
            <div class="flex flex-col items-center mb-4">
              <div
                class="w-20 h-20 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-2xl mb-3"
              >
                {getUserInitials(user.username)}
              </div>
              <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100">
                {user.username}
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">
                {user.email}
              </p>
              <span
                class="px-3 py-1 text-xs font-medium {getRoleBadgeClass(
                  user.role
                )} rounded-full"
              >
                {user.role}
              </span>
            </div>

            <!-- Stats -->
            <div
              class="grid grid-cols-2 gap-4 mb-4 pt-4 border-t border-gray-200 dark:border-gray-700"
            >
              <div class="text-center">
                <div
                  class="flex items-center justify-center gap-2 text-gray-500 dark:text-gray-400 mb-1"
                >
                  <i class="bi bi-files"></i>
                  <span class="text-xs uppercase tracking-wider">Files</span>
                </div>
                <div
                  class="text-lg font-semibold text-gray-900 dark:text-gray-100"
                >
                  {user.filesCount.toLocaleString()}
                </div>
              </div>
              <div class="text-center">
                <div
                  class="flex items-center justify-center gap-2 text-gray-500 dark:text-gray-400 mb-1"
                >
                  <i class="bi bi-hdd"></i>
                  <span class="text-xs uppercase tracking-wider">Storage</span>
                </div>
                <div
                  class="text-lg font-semibold text-gray-900 dark:text-gray-100"
                >
                  {formatBytes(user.storageUsed)}
                </div>
              </div>
            </div>

            <!-- Footer -->
            <div
              class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-700"
            >
              <span class="text-xs text-gray-500 dark:text-gray-400">
                {formatLastActive(user.lastActive)}
              </span>
              <div class="flex gap-2">
                <button
                  class="p-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                  aria-label="Edit user"
                >
                  <i class="bi bi-pencil"></i>
                </button>
                <button
                  class="p-2 rounded-lg text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                  aria-label="Delete user"
                >
                  <i class="bi bi-trash"></i>
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  /* Status indicator colors */
  :global(.bg-green-400) {
    background-color: rgb(74, 222, 128);
  }

  :global(.bg-gray-400) {
    background-color: rgb(156, 163, 175);
  }

  :global(.bg-yellow-400) {
    background-color: rgb(250, 204, 21);
  }
</style>


