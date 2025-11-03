<script>
  import { currentLang } from "../../stores/ui.js";
  import { auth } from "../../stores/auth.js";
  import { success, error as errorToast } from "../../stores/toast";
  import { onMount } from "svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";

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

<ModernCard variant="glass">
  <div class="p-6">
    <div class="flex items-center justify-between mb-4">
      <h2
        class="text-xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2"
      >
        <i class="bi bi-people-fill text-primary-600 dark:text-primary-400"></i>
        User Management
      </h2>
      <ModernButton
        variant="primary"
        size="sm"
        onclick={() => (showAddUserModal = true)}
      >
        <i class="bi bi-person-plus-fill mr-1"></i>
        Add User
      </ModernButton>
    </div>

    {#if loadingUsers}
      <div class="flex justify-center items-center h-64">
        <span class="loading loading-spinner loading-lg text-primary-600"
        ></span>
      </div>
    {:else if users.length > 0}
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="border-b-2 border-gray-200 dark:border-gray-700">
            <tr class="text-left">
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >User</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >Email</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >Role</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >Created</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >Last Login</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                >2FA</th
              >
              <th
                class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300 text-right"
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
                      class="w-8 h-8 rounded-full gradient-bg-primary text-white flex items-center justify-center"
                    >
                      <span class="text-xs font-semibold"
                        >{user.username.substring(0, 2).toUpperCase()}</span
                      >
                    </div>
                    <div class="font-semibold text-gray-900 dark:text-gray-100">
                      {user.username}
                    </div>
                  </div>
                </td>
                <td class="p-3 text-gray-700 dark:text-gray-300"
                  >{user.email}</td
                >
                <td class="p-3">
                  <span
                    class={user.role === "admin"
                      ? "badge-glass-error"
                      : "badge-glass-info"}
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
                    <span class="badge-glass-success">Enabled</span>
                  {:else}
                    <span class="badge-glass-error">Disabled</span>
                  {/if}
                </td>
                <td class="p-3">
                  <div class="flex gap-1 justify-end">
                    <ModernButton
                      variant="danger"
                      size="sm"
                      onclick={() => openDeleteUserModal(user)}
                      aria-label="Delete user"
                      disabled={user.username === $auth.username}
                    >
                      <i class="bi bi-trash"></i>
                    </ModernButton>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <div class="text-center py-12">
        <i class="bi bi-people text-7xl text-gray-300 dark:text-gray-600 mb-4"
        ></i>
        <h3 class="text-2xl font-bold mb-2 text-gray-900 dark:text-gray-100">
          No users yet
        </h3>
        <p class="text-gray-600 dark:text-gray-400 mb-4">
          Create your first user account
        </p>
        <ModernButton
          variant="gradient"
          onclick={() => (showAddUserModal = true)}
        >
          <i class="bi bi-person-plus-fill mr-2"></i>
          Add User
        </ModernButton>
      </div>
    {/if}
  </div>
</ModernCard>

<!-- Add User Modal -->
{#if showAddUserModal}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={() => (showAddUserModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showAddUserModal = false)}
    role="button"
    tabindex="-1"
  >
    <ModernCard
      variant="glass"
      class="max-w-md w-full mx-4 relative"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-6">
        <button
          class="absolute right-4 top-4 w-8 h-8 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 flex items-center justify-center transition-colors"
          onclick={() => (showAddUserModal = false)}
          aria-label="Close modal"
        >
          ✕
        </button>

        <h3
          class="font-bold text-lg mb-4 text-gray-900 dark:text-gray-100 flex items-center gap-2"
        >
          <i
            class="bi bi-person-plus-fill text-primary-600 dark:text-primary-400"
          ></i>
          Add New User
        </h3>

        <div class="space-y-4">
          <div>
            <label
              class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
              for="username"
            >
              Username
            </label>
            <input
              type="text"
              id="username"
              class="glass-input w-full"
              placeholder="Enter username..."
              bind:value={newUsername}
            />
          </div>

          <div>
            <label
              class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
              for="password"
            >
              Password
            </label>
            <input
              type="password"
              id="password"
              class="glass-input w-full"
              placeholder="Enter password..."
              bind:value={newUserPassword}
            />
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
              Password must be at least 8 characters
            </p>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <ModernButton
            variant="ghost"
            onclick={() => (showAddUserModal = false)}
          >
            Cancel
          </ModernButton>
          <ModernButton variant="primary" onclick={handleAddUser}>
            <i class="bi bi-check-lg mr-1"></i>
            Create User
          </ModernButton>
        </div>
      </div>
    </ModernCard>
  </div>
{/if}

<!-- Delete User Modal -->
{#if showDeleteUserModal}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={() => (showDeleteUserModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showDeleteUserModal = false)}
    role="button"
    tabindex="-1"
  >
    <ModernCard
      variant="glass"
      class="max-w-md w-full mx-4 text-center"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-6">
        <div class="text-6xl text-red-600 dark:text-red-400 mb-4">
          <i class="bi bi-person-x-fill"></i>
        </div>
        <h3 class="font-bold text-lg mb-2 text-gray-900 dark:text-gray-100">
          Delete User "{userToDelete?.username}"?
        </h3>
        <p class="text-gray-600 dark:text-gray-400 mb-4">
          This action cannot be undone. All user data will be permanently
          removed.
        </p>
        <div class="flex justify-center gap-2">
          <ModernButton
            variant="ghost"
            onclick={() => (showDeleteUserModal = false)}
          >
            Cancel
          </ModernButton>
          <ModernButton variant="danger" onclick={handleDeleteUser}>
            <i class="bi bi-trash mr-1"></i>
            Delete User
          </ModernButton>
        </div>
      </div>
    </ModernCard>
  </div>
{/if}
