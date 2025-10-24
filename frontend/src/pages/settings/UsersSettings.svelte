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

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <div class="flex items-center justify-between mb-4">
      <h2 class="card-title">
        <i class="bi bi-people-fill text-primary"></i>
        User Management
      </h2>
      <button
        class="btn btn-primary btn-sm"
        on:click={() => (showAddUserModal = true)}
      >
        <i class="bi bi-person-plus-fill"></i>
        Add User
      </button>
    </div>

    {#if loadingUsers}
      <div class="flex justify-center items-center h-64">
        <span class="loading loading-spinner loading-lg text-primary"></span>
      </div>
    {:else if users.length > 0}
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>User</th>
              <th>Email</th>
              <th>Role</th>
              <th>Created</th>
              <th>Last Login</th>
              <th>2FA</th>
              <th class="text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each users as user}
              <tr>
                <td>
                  <div class="flex items-center gap-3">
                    <div class="avatar placeholder">
                      <div class="bg-primary text-primary-content rounded-full w-8">
                        <span class="text-xs">{user.username.substring(0, 2).toUpperCase()}</span>
                      </div>
                    </div>
                    <div class="font-semibold">{user.username}</div>
                  </div>
                </td>
                <td>{user.email}</td>
                <td>
                  <span class="badge {user.role === 'admin' ? 'badge-error' : 'badge-ghost'}">
                    {user.role}
                  </span>
                </td>
                <td class="text-sm opacity-70">{user.created}</td>
                <td class="text-sm opacity-70">{user.lastLogin}</td>
                <td>
                  {#if user.twoFactor}
                    <span class="badge badge-success">Enabled</span>
                  {:else}
                    <span class="badge badge-ghost">Disabled</span>
                  {/if}
                </td>
                <td>
                  <div class="flex gap-1 justify-end">
                    <button
                      class="btn btn-ghost btn-sm btn-circle text-error"
                      on:click={() => openDeleteUserModal(user)}
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
          class="btn btn-primary"
          on:click={() => (showAddUserModal = true)}
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
  <dialog class="modal modal-open">
    <div class="modal-box">
      <button
        class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        on:click={() => (showAddUserModal = false)}
      >
        ✕
      </button>
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-person-plus-fill text-primary mr-2"></i>
        Add New User
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="username">
            <span class="label-text">Username</span>
          </label>
          <label class="input input-bordered flex items-center gap-2">
            <i class="bi bi-person-fill opacity-70"></i>
            <input
              type="text"
              id="username"
              class="grow"
              placeholder="Enter username..."
              bind:value={newUsername}
            />
          </label>
        </div>

        <div class="form-control">
          <label class="label" for="password">
            <span class="label-text">Password</span>
          </label>
          <label class="input input-bordered flex items-center gap-2">
            <i class="bi bi-lock-fill opacity-70"></i>
            <input
              type="password"
              id="password"
              class="grow"
              placeholder="Enter password..."
              bind:value={newUserPassword}
            />
          </label>
          <label class="label">
            <span class="label-text-alt">Password must be at least 8 characters</span>
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" on:click={() => (showAddUserModal = false)}>
          Cancel
        </button>
        <button class="btn btn-primary" on:click={handleAddUser}>
          <i class="bi bi-check-lg"></i>
          Create User
        </button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={() => (showAddUserModal = false)}>close</button>
    </form>
  </dialog>
{/if}

<!-- Delete User Modal -->
{#if showDeleteUserModal}
  <dialog class="modal modal-open">
    <div class="modal-box text-center">
      <div class="text-6xl text-error mb-4">
        <i class="bi bi-person-x-fill"></i>
      </div>
      <h3 class="font-bold text-lg mb-2">
        Delete User "{userToDelete?.username}"?
      </h3>
      <p class="text-base-content/70 mb-4">
        This action cannot be undone. All user data will be permanently removed.
      </p>
      <div class="modal-action justify-center">
        <button class="btn btn-ghost" on:click={() => (showDeleteUserModal = false)}>
          Cancel
        </button>
        <button class="btn btn-error" on:click={handleDeleteUser}>
          <i class="bi bi-trash"></i>
          Delete User
        </button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={() => (showDeleteUserModal = false)}>close</button>
    </form>
  </dialog>
{/if}
