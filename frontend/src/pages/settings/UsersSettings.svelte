<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import { onMount } from "svelte";
  import { users as usersApi } from "../../lib/api.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../stores/toast.js";
  import Modal from "../../components/ui/Modal.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let users = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let success = $state(null);

  // Add User Modal
  let showAddModal = $state(false);
  let newUser = $state({ username: "", password: "", email: "", role: "user" });
  let addingUser = $state(false);

  // Edit User Modal
  let showEditModal = $state(false);
  let editUser = $state(null);
  let savingUser = $state(false);

  // Delete Confirmation
  let showDeleteModal = $state(false);
  let deleteTarget = $state(null);
  let deletingUser = $state(false);

  async function loadUsers() {
    loading = true;
    error = null;
    try {
      const response = await usersApi.getAll();
      // Handle different response formats
      if (Array.isArray(response)) {
        users = response;
      } else if (response?.data) {
        users = response.data;
      } else if (response?.users) {
        users = response.users;
      } else {
        users = [];
      }
    } catch (err) {
      console.error("Failed to load users:", err);
      // Fallback: Try listAll if getAll doesn't exist yet
      try {
        const fallbackResponse = await usersApi.listAll();
        if (Array.isArray(fallbackResponse)) {
          users = fallbackResponse;
        } else if (fallbackResponse?.data) {
          users = fallbackResponse.data;
        } else {
          users = [];
        }
      } catch (fallbackErr) {
        error = tr("settings.users.load_error") || "Failed to load users";
        users = [];
      }
    } finally {
      loading = false;
    }
  }

  async function handleAddUser() {
    if (!newUser.username || !newUser.password) {
      error = tr("settings.users.fill_required");
      return;
    }

    addingUser = true;
    error = null;
    try {
      await usersApi.create(newUser);
      toastSuccess(tr("settings.users.user_created"));
      showAddModal = false;
      newUser = { username: "", password: "", email: "", role: "user" };
      await loadUsers();
    } catch (err) {
      toastError(err.message || tr("settings.users.create_error"));
    } finally {
      addingUser = false;
    }
  }

  function openEditModal(user) {
    editUser = { ...user };
    showEditModal = true;
  }

  async function handleSaveUser() {
    savingUser = true;
    error = null;
    try {
      await usersApi.update(editUser.id, editUser);
      toastSuccess(tr("settings.users.user_updated"));
      showEditModal = false;
      editUser = null;
      await loadUsers();
    } catch (err) {
      toastError(err.message || tr("settings.users.update_error"));
    } finally {
      savingUser = false;
    }
  }

  function openDeleteModal(user) {
    deleteTarget = user;
    showDeleteModal = true;
  }

  async function handleDeleteUser() {
    deletingUser = true;
    error = null;
    try {
      await usersApi.delete(deleteTarget.id);
      toastSuccess(tr("settings.users.user_deleted"));
      showDeleteModal = false;
      deleteTarget = null;
      await loadUsers();
    } catch (err) {
      toastError(err.message || tr("settings.users.delete_error"));
    } finally {
      deletingUser = false;
    }
  }

  async function handleResetPassword(user) {
    const newPassword = prompt(tr("settings.users.enter_new_password"));
    if (!newPassword) return;

    try {
      await usersApi.resetPassword(user.id, newPassword);
      toastSuccess(tr("settings.users.password_reset_success"));
    } catch (err) {
      toastError(err.message || tr("settings.users.password_reset_error"));
    }
  }

  async function handleForcePasswordChange(user) {
    try {
      await usersApi.forcePasswordChange(user.id);
      toastSuccess(tr("settings.users.force_password_change_success"));
      await loadUsers();
    } catch (err) {
      toastError(
        err.message || tr("settings.users.force_password_change_error")
      );
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    return new Date(dateStr).toLocaleDateString();
  }

  onMount(() => {
    loadUsers();
  });
</script>

<div class="flex flex-col gap-6">
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-circle"></i>
      <span>{error}</span>
      <button
        class="ml-auto bg-transparent border-none cursor-pointer opacity-60 hover:opacity-100"
        onclick={() => (error = null)}
        aria-label="Close"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if success}
    <div class="alert alert-success">
      <i class="bi bi-check-circle"></i>
      <span>{success}</span>
    </div>
  {/if}

  <!-- Header -->
  <div class="flex justify-between items-center gap-4">
    <div>
      <h2 class="text-xl font-semibold text-base-content m-0">{tr("settings.users.title")}</h2>
      <p class="text-sm text-base-content/60 mt-1 mb-0">{tr("settings.users.description")}</p>
    </div>
    <button class="btn btn-primary" onclick={() => (showAddModal = true)}>
      <i class="bi bi-plus-lg"></i>
      {tr("settings.users.add_user")}
    </button>
  </div>

  <!-- Users Table -->
  <div class="bg-base-100 border border-base-300 rounded-xl overflow-hidden">
    <div class="flex items-center gap-4 p-5 border-b border-base-300">
      <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-info/20 text-info">
        <i class="bi bi-people"></i>
      </div>
      <div>
        <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.users.user_list")}</h3>
        <p class="text-xs text-base-content/60 mt-1 mb-0">
          {users.length}
          {tr("settings.users.users_total")}
        </p>
      </div>
    </div>

    <div class="p-5">
      {#if loading}
        <div class="flex flex-col items-center justify-center py-12 gap-4 text-base-content/60">
          <span class="loading loading-spinner loading-lg text-primary"></span>
          <p>{tr("common.loading")}</p>
        </div>
      {:else if users.length === 0}
        <div class="flex flex-col items-center justify-center py-12 gap-4 text-base-content/60">
          <i class="bi bi-people text-3xl opacity-50"></i>
          <p>{tr("settings.users.no_users")}</p>
        </div>
      {:else}
        <div class="overflow-x-auto -m-5">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>{tr("settings.users.username")}</th>
                <th>{tr("settings.users.email")}</th>
                <th>{tr("settings.users.role")}</th>
                <th>{tr("settings.users.created")}</th>
                <th>{tr("settings.users.status")}</th>
                <th>{tr("common.actions")}</th>
              </tr>
            </thead>
            <tbody>
              {#each users as user}
                <tr class="hover">
                  <td>
                    <div class="flex items-center gap-3">
                      <div class="w-8 h-8 rounded-full bg-info/20 text-info flex items-center justify-center font-semibold text-sm">
                        {user.username?.charAt(0).toUpperCase() || "U"}
                      </div>
                      <span class="font-medium">{user.username}</span>
                    </div>
                  </td>
                  <td>{user.email || "-"}</td>
                  <td>
                    <span class="badge {user.role === 'admin' ? 'badge-warning' : 'badge-info'}">
                      {user.role === "admin"
                        ? tr("settings.users.admin")
                        : tr("settings.users.user")}
                    </span>
                  </td>
                  <td>{formatDate(user.created_at)}</td>
                  <td>
                    <span class="inline-block w-2 h-2 rounded-full mr-2 {user.is_active !== false ? 'bg-success' : 'bg-base-content/40'}"></span>
                    {user.is_active !== false
                      ? tr("settings.users.active")
                      : tr("settings.users.inactive")}
                  </td>
                  <td>
                    <div class="flex gap-2">
                      <button
                        class="btn btn-ghost btn-sm btn-square"
                        onclick={() => openEditModal(user)}
                        title={tr("common.edit")}
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                      <button
                        class="btn btn-ghost btn-sm btn-square hover:btn-error"
                        onclick={() => openDeleteModal(user)}
                        title={tr("common.delete")}
                        disabled={user.username === "admin"}
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
      {/if}
    </div>
  </div>
</div>

<!-- Add User Modal -->
<Modal
  bind:visible={showAddModal}
  title={tr("settings.users.add_user")}
  icon="person-plus"
  size="md"
  variant="primary"
  onclose={() => (showAddModal = false)}
>
  {#snippet children()}
    <div class="space-y-4">
      <div class="form-group">
        <label
          for="add-username"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >{tr("settings.users.username")} *</label
        >
        <input
          type="text"
          id="add-username"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          bind:value={newUser.username}
          placeholder={tr("settings.users.username_placeholder")}
        />
      </div>

      <div class="form-group">
        <label
          for="add-password"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >{tr("settings.users.password")} *</label
        >
        <input
          type="password"
          id="add-password"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          bind:value={newUser.password}
          placeholder={tr("settings.users.password_placeholder")}
        />
      </div>

      <div class="form-group">
        <label
          for="add-email"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >{tr("settings.users.email")}</label
        >
        <input
          type="email"
          id="add-email"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          bind:value={newUser.email}
          placeholder={tr("settings.users.email_placeholder")}
        />
      </div>

      <div class="form-group">
        <label
          for="add-role"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >{tr("settings.users.role")}</label
        >
        <select
          id="add-role"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          bind:value={newUser.role}
        >
          <option value="user">{tr("settings.users.user")}</option>
          <option value="admin">{tr("settings.users.admin")}</option>
        </select>
      </div>
    </div>
  {/snippet}

  {#snippet actions()}
    <button class="btn btn-ghost" onclick={() => (showAddModal = false)}>
      {tr("common.cancel")}
    </button>
    <button
      class="btn btn-primary"
      onclick={handleAddUser}
      disabled={addingUser}
    >
      {#if addingUser}
        <span class="loading loading-spinner loading-sm"></span>
      {/if}
      {tr("common.create")}
    </button>
  {/snippet}
</Modal>

<!-- Edit User Modal -->
<Modal
  visible={showEditModal && !!editUser}
  title={tr("settings.users.edit_user")}
  icon="pencil"
  size="md"
  variant="primary"
  onclose={() => (showEditModal = false)}
>
  {#snippet children()}
    {#if editUser}
      <div class="space-y-4">
        <div class="form-group">
          <label
            for="edit-username"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >{tr("settings.users.username")}</label
          >
          <input
            type="text"
            id="edit-username"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 cursor-not-allowed"
            bind:value={editUser.username}
            disabled
          />
        </div>

        <div class="form-group">
          <label
            for="edit-email"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >{tr("settings.users.email")}</label
          >
          <input
            type="email"
            id="edit-email"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            bind:value={editUser.email}
            placeholder={tr("settings.users.email_placeholder")}
          />
        </div>

        <div class="form-group">
          <label
            for="edit-role"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >{tr("settings.users.role")}</label
          >
          <select
            id="edit-role"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            bind:value={editUser.role}
          >
            <option value="user">{tr("settings.users.user")}</option>
            <option value="admin">{tr("settings.users.admin")}</option>
          </select>
        </div>
      </div>
    {/if}
  {/snippet}

  {#snippet actions()}
    <button class="btn btn-ghost" onclick={() => (showEditModal = false)}>
      {tr("common.cancel")}
    </button>
    <button
      class="btn btn-primary"
      onclick={handleSaveUser}
      disabled={savingUser}
    >
      {#if savingUser}
        <span class="loading loading-spinner loading-sm"></span>
      {/if}
      {tr("common.save")}
    </button>
  {/snippet}
</Modal>

<!-- Delete Confirmation Modal -->
<Modal
  visible={showDeleteModal && !!deleteTarget}
  title={tr("settings.users.delete_user")}
  icon="trash"
  size="sm"
  variant="danger"
  onclose={() => (showDeleteModal = false)}
>
  {#snippet children()}
    {#if deleteTarget}
      <div class="text-center py-4">
        <div
          class="w-16 h-16 mx-auto mb-4 rounded-full bg-error/10 flex items-center justify-center"
        >
          <i class="bi bi-exclamation-triangle text-3xl text-error"></i>
        </div>
        <p class="text-gray-700 dark:text-gray-300">
          {tr("settings.users.delete_confirm", {
            username: deleteTarget.username,
          })}
        </p>
      </div>
    {/if}
  {/snippet}

  {#snippet actions()}
    <button class="btn btn-ghost" onclick={() => (showDeleteModal = false)}>
      {tr("common.cancel")}
    </button>
    <button
      class="btn btn-error"
      onclick={handleDeleteUser}
      disabled={deletingUser}
    >
      {#if deletingUser}
        <span class="loading loading-spinner loading-sm"></span>
      {/if}
      {tr("common.delete")}
    </button>
  {/snippet}
</Modal>

<style>
  /* Minimal CSS - most styling via Tailwind utilities */
</style>
