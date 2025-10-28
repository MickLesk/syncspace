<script>
  import { currentLang } from "../../stores/ui.js";
  import { auth } from "../../stores/auth.js";
  import { success, error as errorToast } from "../../stores/toast";
  import { onMount } from "svelte";

  let users = [];
  let loadingUsers = false;
  let showAddUserModal = false;
  let showDeleteUserModal = false;
  let newUsername = "";
  let newUserPassword = "";
  let userToDelete = null;

  onMount(() => {
    loadUsers();
  });

  async function loadUsers() {
    loadingUsers = true;
    try {
      users = [
        {
          id: 1,
          username: "admin",
          email: "admin@syncspace.local",
          role: "admin",
          created: new Date().toLocaleDateString($currentLang),
          lastLogin: new Date().toLocaleDateString($currentLang),
          twoFactor: true,
        },
        {
          id: 2,
          username: "demo",
          email: "demo@syncspace.local",
          role: "user",
          created: new Date(
            Date.now() - 7 * 24 * 60 * 60 * 1000
          ).toLocaleDateString($currentLang),
          lastLogin: new Date().toLocaleDateString($currentLang),
          twoFactor: false,
        },
      ];
    } catch (err) {
      errorToast(err.message || "Failed to load users");
    } finally {
      loadingUsers = false;
    }
  }

  function handleAddUser() {
    if (!newUsername || !newUserPassword) {
      errorToast("Please fill all fields");
      return;
    }

    users = [
      ...users,
      {
        id: users.length + 1,
        username: newUsername,
        email: `${newUsername}@syncspace.local`,
        role: "user",
        created: new Date().toLocaleDateString($currentLang),
        lastLogin: "—",
        twoFactor: false,
      },
    ];

    success(`User "${newUsername}" created`);
    newUsername = "";
    newUserPassword = "";
    showAddUserModal = false;
  }

  function handleDeleteUser() {
    if (!userToDelete) return;

    users = users.filter((u) => u.id !== userToDelete.id);
    success(`User "${userToDelete.username}" deleted`);
    userToDelete = null;
    showDeleteUserModal = false;
  }

  function openDeleteUserModal(user) {
    userToDelete = user;
    showDeleteUserModal = true;
  }
</script>

<div
  class="bg-white dark:bg-gray-900 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700"
>
  <div class="p-6">
    <div class="flex items-center justify-between mb-4">
      <h2
        class="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2"
      >
        <i class="bi bi-people-fill text-blue-600 dark:text-blue-400"></i>
        User Management
      </h2>
      <button
        class="px-3 py-1.5 text-sm bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
        onclick={() => (showAddUserModal = true)}
      >
        <i class="bi bi-person-plus-fill"></i>
        Add User
      </button>
    </div>

    {#if loadingUsers}
      <div class="flex justify-center items-center h-64">
        <div
          class="w-12 h-12 border-4 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin"
        ></div>
      </div>
    {:else if users.length > 0}
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="border-b-2 border-gray-200 dark:border-gray-700">
            <tr class="text-left">
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >User</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >Email</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >Role</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >Created</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >Last Login</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                >2FA</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200 text-right"
                >Actions</th
              >
            </tr>
          </thead>
          <tbody>
            {#each users as user, i}
              <tr
                class="{i % 2 === 0
                  ? 'bg-white dark:bg-gray-900'
                  : 'bg-gray-50 dark:bg-gray-800'} border-b border-gray-100 dark:border-gray-800 last:border-0"
              >
                <td class="p-3">
                  <div class="flex items-center gap-3">
                    <div
                      class="w-8 h-8 rounded-full bg-blue-600 dark:bg-blue-500 text-white flex items-center justify-center"
                    >
                      <span class="text-xs font-semibold"
                        >{user.username.substring(0, 2).toUpperCase()}</span
                      >
                    </div>
                    <div class="font-semibold text-gray-900 dark:text-white">
                      {user.username}
                    </div>
                  </div>
                </td>
                <td class="p-3 text-gray-700 dark:text-gray-300"
                  >{user.email}</td
                >
                <td class="p-3">
                  <span
                    class="px-2 py-0.5 text-xs rounded-full {user.role ===
                    'admin'
                      ? 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200'
                      : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300'}"
                  >
                    {user.role}
                  </span>
                </td>
                <td class="p-3 text-sm text-gray-500 dark:text-gray-400"
                  >{user.created}</td
                >
                <td class="p-3 text-sm text-gray-500 dark:text-gray-400"
                  >{user.lastLogin}</td
                >
                <td class="p-3">
                  {#if user.twoFactor}
                    <span
                      class="px-2 py-0.5 text-xs bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded-full"
                      >Enabled</span
                    >
                  {:else}
                    <span
                      class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-full"
                      >Disabled</span
                    >
                  {/if}
                </td>
                <td class="p-3">
                  <div class="flex gap-1 justify-end">
                    <button
                      class="w-8 h-8 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 text-red-600 dark:text-red-400 flex items-center justify-center transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                      onclick={() => openDeleteUserModal(user)}
                      aria-label="Delete user"
                      disabled={user.username === $auth.username}
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
    {:else}
      <div class="text-center py-12">
        <i class="bi bi-people text-7xl text-base-300 mb-4"></i>
        <h3 class="text-2xl font-bold mb-2">No users yet</h3>
        <p class="opacity-70 mb-4">Create your first user account</p>
        <button
          class="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-xl hover:from-blue-700 hover:to-purple-700 transition-all flex items-center gap-2 shadow-lg shadow-blue-500/25"
          onclick={() => (showAddUserModal = true)}
        >
          <i class="bi bi-person-plus-fill"></i>
          Add User
        </button>
      </div>
    {/if}
  </div>
</div>

<!-- Add User Modal -->
{#if showAddUserModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={() => (showAddUserModal = false)}
    role="button"
    tabindex="-1"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 max-w-md w-full mx-4 relative"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
    >
      <button
        class="absolute right-2 top-2 w-8 h-8 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 flex items-center justify-center transition-colors"
        onclick={() => (showAddUserModal = false)}
        aria-label="Close modal"
      >
        ✕
      </button>
      <div class="p-6">
        <h3
          class="font-bold text-lg mb-4 text-gray-900 dark:text-white flex items-center gap-2"
        >
          <i class="bi bi-person-plus-fill text-blue-600 dark:text-blue-400"
          ></i>
          Add New User
        </h3>

        <div class="space-y-4">
          <div class="mb-4">
            <label
              class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
              for="username"
            >
              Username
            </label>
            <div
              class="flex items-center gap-2 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 focus-within:ring-2 focus-within:ring-blue-500"
            >
              <i class="bi bi-person-fill text-gray-400 dark:text-gray-500"></i>
              <input
                type="text"
                id="username"
                class="flex-1 bg-transparent border-none focus:outline-none text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500"
                placeholder="Enter username..."
                bind:value={newUsername}
              />
            </div>
          </div>

          <div class="mb-4">
            <label
              class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
              for="password"
            >
              Password
            </label>
            <div
              class="flex items-center gap-2 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 focus-within:ring-2 focus-within:ring-blue-500"
            >
              <i class="bi bi-lock-fill text-gray-400 dark:text-gray-500"></i>
              <input
                type="password"
                id="password"
                class="flex-1 bg-transparent border-none focus:outline-none text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500"
                placeholder="Enter password..."
                bind:value={newUserPassword}
              />
            </div>
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
              Password must be at least 8 characters
            </p>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <button
            class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors"
            onclick={() => (showAddUserModal = false)}
          >
            Cancel
          </button>
          <button
            class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
            onclick={handleAddUser}
          >
            <i class="bi bi-check-lg"></i>
            Create User
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Delete User Modal -->
{#if showDeleteUserModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={() => (showDeleteUserModal = false)}
    role="button"
    tabindex="-1"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 max-w-md w-full mx-4 text-center p-6"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
    >
      <div class="text-6xl text-red-600 dark:text-red-400 mb-4">
        <i class="bi bi-person-x-fill"></i>
      </div>
      <h3 class="font-bold text-lg mb-2 text-gray-900 dark:text-white">
        Delete User "{userToDelete?.username}"?
      </h3>
      <p class="text-gray-600 dark:text-gray-400 mb-4">
        This action cannot be undone. All user data will be permanently removed.
      </p>
      <div class="flex justify-center gap-2">
        <button
          class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors"
          onclick={() => (showDeleteUserModal = false)}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors flex items-center gap-2"
          onclick={handleDeleteUser}
        >
          <i class="bi bi-trash"></i>
          Delete User
        </button>
      </div>
    </div>
  </div>
{/if}
