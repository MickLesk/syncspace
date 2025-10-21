<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { auth } from "../stores/auth.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast, info } from "../stores/toast";
  import Button from "../components/ui/Button.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import InputDialog from "../components/ui/InputDialog.svelte";
  import Avatar from "../components/ui/Avatar.svelte";

  let users = [];
  let loading = true;
  
  // Dialog states
  let showAddDialog = false;
  let showPasswordDialog = false;
  let showDeleteDialog = false;
  let showEditDialog = false;
  let newUsername = '';
  let newPassword = '';
  let userToDelete = null;
  let userToChangePassword = null;
  let userToEdit = null;

  onMount(() => {
    loadUsers();
  });

  async function loadUsers() {
    // Mock data for now since backend might not have user list endpoint
    users = [
      {
        id: 1,
        username: "admin",
        created: new Date().toLocaleDateString($currentLang),
        twoFactor: false,
      },
    ];
    loading = false;
  }

  function handleAddUser() {
    showAddDialog = true;
  }

  function handleAddUserConfirm() {
    if (!newUsername || !newPassword) {
      errorToast('Bitte alle Felder ausfüllen');
      return;
    }

    // TODO: Add backend API call
    users = [
      ...users,
      {
        id: users.length + 1,
        username: newUsername,
        created: new Date().toLocaleDateString($currentLang),
        twoFactor: false,
      },
    ];
    
    success(`Benutzer "${newUsername}" wurde erstellt`);
    newUsername = '';
    newPassword = '';
    showAddDialog = false;
  }

  function handleChangePassword(user) {
    userToChangePassword = user;
    showPasswordDialog = true;
  }

  function handlePasswordChange(event) {
    const newPassword = event.detail;
    if (!newPassword || !userToChangePassword) return;
    
    // TODO: Add backend API call
    success(`Passwort für "${userToChangePassword.username}" wurde geändert`);
    userToChangePassword = null;
  }

  function handleEditUser(user) {
    userToEdit = user;
    showEditDialog = true;
  }

  function handleEditConfirm() {
    if (!userToEdit) return;
    
    // TODO: Add backend API call for editing user details
    info(`Benutzerdaten für "${userToEdit.username}" bearbeiten`);
    userToEdit = null;
  }

  function handleDeleteUser(user) {
    // Protect admin user
    if (user.username === 'admin') {
      errorToast('Der Admin-Benutzer kann nicht gelöscht werden!');
      return;
    }
    
    userToDelete = user;
    showDeleteDialog = true;
  }

  function handleDeleteConfirm() {
    if (!userToDelete) return;
    
    const username = userToDelete.username;
    
    // TODO: Add backend API call
    users = users.filter((u) => u.id !== userToDelete.id);
    success(`Benutzer "${username}" wurde gelöscht`);
    userToDelete = null;
  }
</script>

<div class="view-container">
  <div class="page-header">
    <h2>👥 {t($currentLang, "users")}</h2>
    <Button icon="➕" onClick={handleAddUser}>
      {t($currentLang, "addUser")}
    </Button>
  </div>

  {#if loading}
    <div class="loading">Loading...</div>
  {:else}
    <div class="users-table">
      <div class="table-header">
        <div class="col-username">{t($currentLang, "username")}</div>
        <div class="col-created">{t($currentLang, "created")}</div>
        <div class="col-2fa">{t($currentLang, "twoFactorAuth")}</div>
        <div class="col-actions">{t($currentLang, "actions")}</div>
      </div>

      {#each users as user}
        <div class="table-row">
          <div class="col-username">
            <Avatar name={user.username} size="small" />
            <span>{user.username}</span>
            {#if user.username === 'admin'}
              <span class="admin-badge">Admin</span>
            {/if}
          </div>
          <div class="col-created">{user.created}</div>
          <div class="col-2fa">
            <span class="badge" class:enabled={user.twoFactor} class:disabled={!user.twoFactor}>
              {user.twoFactor ? t($currentLang, "enabled") : t($currentLang, "disabled")}
            </span>
          </div>
          <div class="col-actions">
            <button class="btn-icon" on:click={() => handleChangePassword(user)} title="Passwort ändern">
              🔑
            </button>
            <button class="btn-icon" on:click={() => handleEditUser(user)} title="Bearbeiten">
              ✏️
            </button>
            <button 
              class="btn-icon delete-btn" 
              on:click={() => handleDeleteUser(user)} 
              title="Löschen"
              disabled={user.username === 'admin'}
            >
              🗑️
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Dialogs -->
<Dialog
  bind:open={showAddDialog}
  title="Neuer Benutzer"
  confirmText="Hinzufügen"
  cancelText="Abbrechen"
  on:confirm={handleAddUserConfirm}
>
  <div style="display: flex; flex-direction: column; gap: 16px;">
    <div>
      <label for="new-username" style="display: block; margin-bottom: 8px; font-weight: 500;">Benutzername</label>
      <input
        id="new-username"
        type="text"
        bind:value={newUsername}
        placeholder="Benutzername eingeben"
        style="width: 100%; padding: 12px; border: 1px solid var(--md-sys-color-outline); border-radius: 8px; font-size: 14px;"
      />
    </div>
    <div>
      <label for="new-password" style="display: block; margin-bottom: 8px; font-weight: 500;">Passwort</label>
      <input
        id="new-password"
        type="password"
        bind:value={newPassword}
        placeholder="Passwort eingeben"
        style="width: 100%; padding: 12px; border: 1px solid var(--md-sys-color-outline); border-radius: 8px; font-size: 14px;"
      />
    </div>
  </div>
</Dialog>

<InputDialog
  bind:open={showPasswordDialog}
  title="Passwort ändern"
  label="Neues Passwort"
  placeholder="Neues Passwort eingeben"
  type="password"
  confirmText="Ändern"
  on:confirm={handlePasswordChange}
/>

<Dialog
  bind:open={showEditDialog}
  title="Benutzer bearbeiten"
  confirmText="Speichern"
  cancelText="Abbrechen"
  on:confirm={handleEditConfirm}
>
  <div style="display: flex; flex-direction: column; gap: 16px;">
    <p><strong>Benutzer:</strong> {userToEdit?.username}</p>
    <div>
      <label for="edit-email" style="display: block; margin-bottom: 8px; font-weight: 500;">E-Mail</label>
      <input
        id="edit-email"
        type="email"
        placeholder="email@beispiel.de"
        style="width: 100%; padding: 12px; border: 1px solid var(--md-sys-color-outline); border-radius: 8px; font-size: 14px;"
      />
    </div>
    <div>
      <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
        <input type="checkbox" />
        <span>2-Faktor-Authentifizierung aktivieren</span>
      </label>
    </div>
    <div>
      <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
        <input type="checkbox" />
        <span>Administrator-Rechte</span>
      </label>
    </div>
  </div>
</Dialog>

<Dialog
  bind:open={showDeleteDialog}
  title="Benutzer löschen"
  confirmText="Löschen"
  cancelText="Abbrechen"
  danger={true}
  on:confirm={handleDeleteConfirm}
>
  <p>Möchten Sie den Benutzer <strong>"{userToDelete?.username}"</strong> wirklich löschen?</p>
  <p style="color: var(--md-sys-color-error); margin-top: 12px;">Diese Aktion kann nicht rückgängig gemacht werden.</p>
</Dialog>

<style>
  .view-container {
    padding: 24px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }

  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .users-table {
    background: var(--md-sys-color-surface);
    border-radius: 16px;
    overflow: hidden;
    box-shadow: var(--md-elevation-1);
  }

  .table-header,
  .table-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr;
    gap: 16px;
    padding: 16px;
    align-items: center;
  }

  .table-header {
    background: var(--md-sys-color-surface-variant);
    font-weight: 600;
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .table-row {
    border-bottom: 1px solid var(--md-sys-color-outline);
  }

  .table-row:last-child {
    border-bottom: none;
  }

  .col-username {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .admin-badge {
    padding: 2px 8px;
    background: var(--md-sys-color-tertiary-container);
    color: var(--md-sys-color-on-tertiary-container);
    border-radius: 8px;
    font-size: 11px;
    font-weight: 600;
  }

  .badge {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
  }

  .badge.disabled {
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface-variant);
  }

  .badge.enabled {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .col-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-icon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    border: none;
    background: var(--md-sys-color-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
  }

  .btn-icon:hover:not(:disabled) {
    background: var(--md-sys-color-secondary-container);
    transform: scale(1.1);
  }

  .btn-icon:disabled {
    opacity: 0.38;
    cursor: not-allowed;
  }

  .btn-icon.delete-btn:hover:not(:disabled) {
    background: var(--md-sys-color-error);
    color: var(--md-sys-color-on-error);
  }

  .loading {
    text-align: center;
    padding: 60px;
    color: var(--md-sys-color-on-surface-variant);
  }
</style>
