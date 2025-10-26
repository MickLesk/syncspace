<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { auth } from "../stores/auth.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast, info } from "../stores/toast";
  import Icon from "../components/ui/Icon.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import InputDialog from "../components/ui/InputDialog.svelte";
  import Avatar from "../components/ui/Avatar.svelte";
  import StatCard from "../components/ui/StatCard.svelte";
  import Button from "../components/ui/Button.svelte";
  import Spinner from "../components/ui/Spinner.svelte";
  import Badge from "../components/ui/Badge.svelte";

  let users = [];
  let loading = true;

  // Dialog states
  let showAddDialog = false;
  let showPasswordDialog = false;
  let showDeleteDialog = false;
  let showEditDialog = false;
  let newUsername = "";
  let newPassword = "";
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
      errorToast("Bitte alle Felder ausfüllen");
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
    newUsername = "";
    newPassword = "";
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
    if (user.username === "admin") {
      errorToast("Der Admin-Benutzer kann nicht gelöscht werden!");
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

<div class="users-view">
  <PageHeader
    title={t($currentLang, "users")}
    subtitle=""
    icon="people-fill"
    gradient="purple"
  >
    <div slot="actions" class="header-actions">
      <Button onClick={handleAddUser} variant="outlined" size="medium">
        <Icon name="person-plus-fill" size={16} />
        {t($currentLang, "addUser")}
      </Button>
    </div>
  </PageHeader>

  <div class="page-content">
    <!-- Stats -->
    <div class="stats-grid">
      <StatCard
        icon="bi-people-fill"
        label="Total Users"
        value={users.length}
        gradient="linear-gradient(135deg, #6366f1, #8b5cf6)"
      />

      <StatCard
        icon="bi-shield-check"
        label="2FA Enabled"
        value={users.filter((u) => u.twoFactor).length}
        gradient="linear-gradient(135deg, #10b981, #34d399)"
      />
    </div>

    {#if loading}
      <div class="loading">
        <Spinner size="large" />
        <p>Loading users...</p>
      </div>
    {:else}
      <!-- Users Table -->
      <div class="glass-card table-container">
        <div class="table-head">
          <div class="col-user">{t($currentLang, "username")}</div>
          <div class="col-date">{t($currentLang, "created")}</div>
          <div class="col-2fa">{t($currentLang, "twoFactorAuth")}</div>
          <div class="col-actions">{t($currentLang, "actions")}</div>
        </div>

        {#each users as user}
          <div class="row">
            <div class="col-user">
              <Avatar name={user.username} size="small" />
              <span class="name">{user.username}</span>
              {#if user.username === "admin"}
                <Badge variant="success">Admin</Badge>
              {/if}
            </div>
            <div class="col-date">{user.created}</div>
            <div class="col-2fa">
              <Badge variant={user.twoFactor ? "success" : "error"}>
                {user.twoFactor
                  ? t($currentLang, "enabled")
                  : t($currentLang, "disabled")}
              </Badge>
            </div>
            <div class="col-actions">
              <Button
                onClick={() => handleChangePassword(user)}
                variant="ghost"
                size="small"
              >
                <Icon name="key-fill" size={14} />
              </Button>
              <Button
                onClick={() => handleEditUser(user)}
                variant="ghost"
                size="small"
              >
                <Icon name="pencil-fill" size={14} />
              </Button>
              <Button
                onClick={() => handleDeleteUser(user)}
                variant="danger-ghost"
                size="small"
                disabled={user.username === "admin"}
              >
                <Icon name="trash-fill" size={14} />
              </Button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
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
      <label
        for="new-username"
        style="display: block; margin-bottom: 8px; font-weight: 500;"
        >Benutzername</label
      >
      <input
        id="new-username"
        type="text"
        bind:value={newUsername}
        placeholder="Benutzername eingeben"
        style="width: 100%; padding: 12px; border: 1px solid var(--md-sys-color-outline); border-radius: 8px; font-size: 14px;"
      />
    </div>
    <div>
      <label
        for="new-password"
        style="display: block; margin-bottom: 8px; font-weight: 500;"
        >Passwort</label
      >
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
      <label
        for="edit-email"
        style="display: block; margin-bottom: 8px; font-weight: 500;"
        >E-Mail</label
      >
      <input
        id="edit-email"
        type="email"
        placeholder="email@beispiel.de"
        style="width: 100%; padding: 12px; border: 1px solid var(--md-sys-color-outline); border-radius: 8px; font-size: 14px;"
      />
    </div>
    <div>
      <label
        style="display: flex; align-items: center; gap: 8px; cursor: pointer;"
      >
        <input type="checkbox" />
        <span>2-Faktor-Authentifizierung aktivieren</span>
      </label>
    </div>
    <div>
      <label
        style="display: flex; align-items: center; gap: 8px; cursor: pointer;"
      >
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
  <p>
    Möchten Sie den Benutzer <strong>"{userToDelete?.username}"</strong> wirklich
    löschen?
  </p>
  <p style="color: var(--md-sys-color-error); margin-top: 12px;">
    Diese Aktion kann nicht rückgängig gemacht werden.
  </p>
</Dialog>

<style>
  .users-view {
    min-height: 100vh;
    background: var(--md-sys-color-surface);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
  }

  /* Actions Bar */
  .actions {
    margin-bottom: 24px;
    display: flex;
    justify-content: flex-end;
  }

  /* Loading */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 64px 32px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .loading p {
    font-size: 14px;
    font-weight: 500;
    margin-top: 16px;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  /* Table Container */
  .table-container {
    overflow: hidden;
  }

  .table-head,
  .row {
    display: grid;
    grid-template-columns: 2fr 1.2fr 1.2fr 140px;
    gap: 16px;
    padding: 16px 20px;
    align-items: center;
  }

  .table-head {
    background: rgba(99, 102, 241, 0.06);
    font-weight: 600;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .row {
    border-bottom: 1px solid rgba(15, 23, 42, 0.06);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .row:last-child {
    border-bottom: none;
  }

  .row:hover {
    background: rgba(99, 102, 241, 0.04);
  }

  /* Columns */
  .col-user {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .name {
    font-weight: 600;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    letter-spacing: -0.2px;
  }

  .col-date {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  /* Action Buttons */
  .col-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  /* Dark Mode */
  @media (prefers-color-scheme: dark) {
    .table-head {
      background: rgba(99, 102, 241, 0.1);
    }

    .row:hover {
      background: rgba(99, 102, 241, 0.08);
    }

    .row {
      border-color: rgba(255, 255, 255, 0.06);
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: 1fr;
      gap: 16px;
    }

    .table-head,
    .row {
      grid-template-columns: 1.5fr 1fr 1fr 100px;
      gap: 8px;
      padding: 12px 14px;
      font-size: 12px;
    }

    .name {
      font-size: 13px;
    }

    .col-date,
    .col-2fa {
      font-size: 12px;
    }

    .col-actions {
      gap: 4px;
    }
  }
</style>
