<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import { onMount } from "svelte";
  import { users as usersApi } from "../../lib/api.js";
  import { success as toastSuccess, error as toastError } from "../../stores/toast.js";

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
      toastError(err.message || tr("settings.users.force_password_change_error"));
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

<div class="users-settings">
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-circle"></i>
      <span>{error}</span>
      <button
        class="alert-close"
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
  <div class="page-header">
    <div>
      <h2>{tr("settings.users.title")}</h2>
      <p>{tr("settings.users.description")}</p>
    </div>
    <button class="btn btn-primary" onclick={() => (showAddModal = true)}>
      <i class="bi bi-plus-lg"></i>
      {tr("settings.users.add_user")}
    </button>
  </div>

  <!-- Users Table -->
  <div class="card">
    <div class="card-header">
      <div class="card-icon blue">
        <i class="bi bi-people"></i>
      </div>
      <div>
        <h3>{tr("settings.users.user_list")}</h3>
        <p class="card-subtitle">
          {users.length}
          {tr("settings.users.users_total")}
        </p>
      </div>
    </div>

    <div class="card-body">
      {#if loading}
        <div class="loading-container">
          <div class="loading-spinner"></div>
          <p>{tr("common.loading")}</p>
        </div>
      {:else if users.length === 0}
        <div class="empty-state">
          <i class="bi bi-people"></i>
          <p>{tr("settings.users.no_users")}</p>
        </div>
      {:else}
        <div class="table-container">
          <table class="data-table">
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
                <tr>
                  <td>
                    <div class="user-info">
                      <div class="user-avatar">
                        {user.username?.charAt(0).toUpperCase() || "U"}
                      </div>
                      <span class="user-name">{user.username}</span>
                    </div>
                  </td>
                  <td>{user.email || "-"}</td>
                  <td>
                    <span
                      class="badge {user.role === 'admin'
                        ? 'badge-amber'
                        : 'badge-blue'}"
                    >
                      {user.role === "admin"
                        ? tr("settings.users.admin")
                        : tr("settings.users.user")}
                    </span>
                  </td>
                  <td>{formatDate(user.created_at)}</td>
                  <td>
                    <span
                      class="status-dot {user.is_active !== false
                        ? 'active'
                        : 'inactive'}"
                    ></span>
                    {user.is_active !== false
                      ? tr("settings.users.active")
                      : tr("settings.users.inactive")}
                  </td>
                  <td>
                    <div class="action-buttons">
                      <button
                        class="btn-icon"
                        onclick={() => openEditModal(user)}
                        title={tr("common.edit")}
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                      <button
                        class="btn-icon btn-danger"
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
{#if showAddModal}
  <div
    class="modal-overlay"
    onclick={() => (showAddModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showAddModal = false)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="document"
    >
      <div class="modal-header">
        <h3>{tr("settings.users.add_user")}</h3>
        <button
          class="modal-close"
          onclick={() => (showAddModal = false)}
          aria-label="Close"
        >
          <i class="bi bi-x"></i>
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label for="add-username">{tr("settings.users.username")} *</label>
          <input
            type="text"
            id="add-username"
            class="form-input"
            bind:value={newUser.username}
            placeholder={tr("settings.users.username_placeholder")}
          />
        </div>

        <div class="form-group">
          <label for="add-password">{tr("settings.users.password")} *</label>
          <input
            type="password"
            id="add-password"
            class="form-input"
            bind:value={newUser.password}
            placeholder={tr("settings.users.password_placeholder")}
          />
        </div>

        <div class="form-group">
          <label for="add-email">{tr("settings.users.email")}</label>
          <input
            type="email"
            id="add-email"
            class="form-input"
            bind:value={newUser.email}
            placeholder={tr("settings.users.email_placeholder")}
          />
        </div>

        <div class="form-group">
          <label for="add-role">{tr("settings.users.role")}</label>
          <select id="add-role" class="form-input" bind:value={newUser.role}>
            <option value="user">{tr("settings.users.user")}</option>
            <option value="admin">{tr("settings.users.admin")}</option>
          </select>
        </div>
      </div>

      <div class="modal-footer">
        <button
          class="btn btn-secondary"
          onclick={() => (showAddModal = false)}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="btn btn-primary"
          onclick={handleAddUser}
          disabled={addingUser}
        >
          {#if addingUser}
            <span class="btn-spinner"></span>
          {/if}
          {tr("common.create")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit User Modal -->
{#if showEditModal && editUser}
  <div
    class="modal-overlay"
    onclick={() => (showEditModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showEditModal = false)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="document"
    >
      <div class="modal-header">
        <h3>{tr("settings.users.edit_user")}</h3>
        <button
          class="modal-close"
          onclick={() => (showEditModal = false)}
          aria-label="Close"
        >
          <i class="bi bi-x"></i>
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label for="edit-username">{tr("settings.users.username")}</label>
          <input
            type="text"
            id="edit-username"
            class="form-input"
            bind:value={editUser.username}
            disabled
          />
        </div>

        <div class="form-group">
          <label for="edit-email">{tr("settings.users.email")}</label>
          <input
            type="email"
            id="edit-email"
            class="form-input"
            bind:value={editUser.email}
            placeholder={tr("settings.users.email_placeholder")}
          />
        </div>

        <div class="form-group">
          <label for="edit-role">{tr("settings.users.role")}</label>
          <select id="edit-role" class="form-input" bind:value={editUser.role}>
            <option value="user">{tr("settings.users.user")}</option>
            <option value="admin">{tr("settings.users.admin")}</option>
          </select>
        </div>
      </div>

      <div class="modal-footer">
        <button
          class="btn btn-secondary"
          onclick={() => (showEditModal = false)}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="btn btn-primary"
          onclick={handleSaveUser}
          disabled={savingUser}
        >
          {#if savingUser}
            <span class="btn-spinner"></span>
          {/if}
          {tr("common.save")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && deleteTarget}
  <div
    class="modal-overlay"
    onclick={() => (showDeleteModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showDeleteModal = false)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal modal-sm"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="document"
    >
      <div class="modal-header">
        <h3>{tr("settings.users.delete_user")}</h3>
        <button
          class="modal-close"
          onclick={() => (showDeleteModal = false)}
          aria-label="Close"
        >
          <i class="bi bi-x"></i>
        </button>
      </div>

      <div class="modal-body">
        <p>
          {tr("settings.users.delete_confirm", {
            username: deleteTarget.username,
          })}
        </p>
      </div>

      <div class="modal-footer">
        <button
          class="btn btn-secondary"
          onclick={() => (showDeleteModal = false)}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="btn btn-danger"
          onclick={handleDeleteUser}
          disabled={deletingUser}
        >
          {#if deletingUser}
            <span class="btn-spinner"></span>
          {/if}
          {tr("common.delete")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .users-settings {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Alerts */
  .alert {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
  }

  .alert-error {
    background: #fef2f2;
    color: #dc2626;
    border: 1px solid #fecaca;
  }

  .alert-success {
    background: #f0fdf4;
    color: #16a34a;
    border: 1px solid #bbf7d0;
  }

  :global([data-theme="dark"]) .alert-error {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  :global([data-theme="dark"]) .alert-success {
    background: rgba(22, 163, 74, 0.1);
    border-color: rgba(22, 163, 74, 0.3);
  }

  .alert-close {
    margin-left: auto;
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
    opacity: 0.6;
  }

  .alert-close:hover {
    opacity: 1;
  }

  /* Page Header */
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .page-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .page-header h2 {
    color: #f9fafb;
  }

  .page-header p {
    font-size: 0.875rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .card-header {
    border-bottom-color: #374151;
  }

  .card-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .card-header h3 {
    color: #f9fafb;
  }

  .card-subtitle {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  :global([data-theme="dark"]) .card-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  .card-body {
    padding: 1.25rem;
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.5rem;
    border: none;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #22c55e;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #16a34a;
  }

  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #e5e7eb;
  }

  :global([data-theme="dark"]) .btn-secondary {
    background: #374151;
    color: #e5e7eb;
  }

  :global([data-theme="dark"]) .btn-secondary:hover:not(:disabled) {
    background: #4b5563;
  }

  .btn-danger {
    background: #ef4444;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }

  .btn-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Loading & Empty */
  .loading-container,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    gap: 1rem;
    color: #6b7280;
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .empty-state i {
    font-size: 2rem;
    opacity: 0.5;
  }

  /* Table */
  .table-container {
    overflow-x: auto;
    margin: -1.25rem;
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }

  .data-table th {
    text-align: left;
    padding: 0.75rem 1.25rem;
    background: #f9fafb;
    font-weight: 500;
    color: #6b7280;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .data-table th {
    background: #374151;
    color: #9ca3af;
    border-bottom-color: #4b5563;
  }

  .data-table td {
    padding: 0.75rem 1.25rem;
    border-top: 1px solid #f3f4f6;
    color: #374151;
  }

  :global([data-theme="dark"]) .data-table td {
    border-top-color: #374151;
    color: #e5e7eb;
  }

  .data-table tbody tr:hover {
    background: #f9fafb;
  }

  :global([data-theme="dark"]) .data-table tbody tr:hover {
    background: #374151;
  }

  /* User Info */
  .user-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: #dbeafe;
    color: #2563eb;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 0.875rem;
  }

  :global([data-theme="dark"]) .user-avatar {
    background: rgba(37, 99, 235, 0.2);
  }

  .user-name {
    font-weight: 500;
  }

  /* Badges */
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.625rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
  }

  .badge-blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .badge-amber {
    background: #fef3c7;
    color: #d97706;
  }

  :global([data-theme="dark"]) .badge-blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .badge-amber {
    background: rgba(217, 119, 6, 0.2);
  }

  /* Status Dot */
  .status-dot {
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    margin-right: 0.5rem;
  }

  .status-dot.active {
    background: #22c55e;
  }

  .status-dot.inactive {
    background: #9ca3af;
  }

  /* Action Buttons */
  .action-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    border: 1px solid #e5e7eb;
    background: white;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-icon:hover:not(:disabled) {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-icon.btn-danger:hover:not(:disabled) {
    background: #fef2f2;
    color: #dc2626;
    border-color: #fecaca;
  }

  .btn-icon:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  :global([data-theme="dark"]) .btn-icon {
    background: #374151;
    border-color: #4b5563;
    color: #9ca3af;
  }

  :global([data-theme="dark"]) .btn-icon:hover:not(:disabled) {
    background: #4b5563;
    color: #e5e7eb;
  }

  :global([data-theme="dark"]) .btn-icon.btn-danger:hover:not(:disabled) {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }

  .modal {
    background: white;
    border-radius: 0.75rem;
    width: 100%;
    max-width: 450px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.2);
  }

  .modal-sm {
    max-width: 360px;
  }

  :global([data-theme="dark"]) .modal {
    background: #1f2937;
    border: 1px solid #374151;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .modal-header {
    border-bottom-color: #374151;
  }

  .modal-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .modal-header h3 {
    color: #f9fafb;
  }

  .modal-close {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: #6b7280;
    cursor: pointer;
    border-radius: 0.375rem;
  }

  .modal-close:hover {
    background: #f3f4f6;
    color: #374151;
  }

  :global([data-theme="dark"]) .modal-close:hover {
    background: #374151;
    color: #e5e7eb;
  }

  .modal-body {
    padding: 1.25rem;
  }

  .modal-body p {
    color: #6b7280;
    margin: 0;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.25rem;
    border-top: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .modal-footer {
    border-top-color: #374151;
  }

  /* Form */
  .form-group {
    margin-bottom: 1rem;
  }

  .form-group:last-child {
    margin-bottom: 0;
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.5rem;
  }

  :global([data-theme="dark"]) .form-group label {
    color: #e5e7eb;
  }

  .form-input {
    width: 100%;
    padding: 0.625rem 0.875rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    background: white;
    color: #374151;
  }

  .form-input:focus {
    outline: none;
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
  }

  .form-input:disabled {
    background: #f9fafb;
    cursor: not-allowed;
  }

  :global([data-theme="dark"]) .form-input {
    background: #374151;
    border-color: #4b5563;
    color: #e5e7eb;
  }

  :global([data-theme="dark"]) .form-input:disabled {
    background: #1f2937;
  }
</style>
