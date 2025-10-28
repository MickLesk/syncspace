<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../stores/ui.js";
  import { auth } from "../stores/auth.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast, info } from "../stores/toast";
  import ButtonV2 from "../components/ui/ButtonV2.svelte";
  import CardV2 from "../components/ui/CardV2.svelte";
  import InputV2 from "../components/ui/InputV2.svelte";
  import SelectV2 from "../components/ui/SelectV2.svelte";
  import CheckboxV2 from "../components/ui/CheckboxV2.svelte";
  import Avatar from "../components/ui/Avatar.svelte";
  import Badge from "../components/ui/Badge.svelte";

  let activeTab = "general";

  const tabs = [
    { id: "general", label: "General", icon: "sliders" },
    { id: "users", label: "Users", icon: "people-fill" },
    { id: "storage", label: "Storage", icon: "hdd-fill" },
    { id: "security", label: "Security", icon: "shield-fill-check" },
    { id: "advanced", label: "Advanced", icon: "tools" },
    { id: "about", label: "About", icon: "info-circle" },
  ];

  // Users tab data
  let users = [];
  let loadingUsers = false;
  let showAddUserModal = false;
  let showEditUserModal = false;
  let showDeleteUserModal = false;
  let newUsername = "";
  let newUserPassword = "";
  let userToEdit = null;
  let userToDelete = null;

  onMount(() => {
    if (activeTab === "users") {
      loadUsers();
    }
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

  function openEditUserModal(user) {
    userToEdit = user;
    showEditUserModal = true;
  }

  function openDeleteUserModal(user) {
    userToDelete = user;
    showDeleteUserModal = true;
  }

  function handleTabChange(tab) {
    activeTab = tab;
    if (tab === "users" && users.length === 0) {
      loadUsers();
    }
  }

  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch" },
    { value: "en", label: "🇬🇧 English" },
    { value: "fr", label: "🇫🇷 Français" },
    { value: "es", label: "🇪🇸 Español" },
  ];

  let storageLocation = "./data";
  let autoBackup = true;
  let enableNotifications = true;
  let twoFactorAuth = false;
  let maxFileSize = "100";

  function handleClearCache() {
    success("Cache cleared successfully!");
  }

  function handleExportSettings() {
    success("Settings exported!");
  }

  function handleSaveSettings() {
    success("Settings saved successfully!");
  }
</script>

<div class="settings-view" style="background: red; padding: 50px;">
  <p style="color: white; font-size: 30px;">SETTINGS VIEW IS RENDERING</p>

  <!-- Modern Tab Navigation -->
  <div class="tab-navigation" style="background: yellow; padding: 20px;">
    {#each tabs as tab}
      <button
        class="tab-button"
        class:active={activeTab === tab.id}
        on:click={() => handleTabChange(tab.id)}
      >
        <i class="bi bi-{tab.icon}"></i>
        <span>{tab.label}</span>
      </button>
    {/each}
  </div>
</div>

<!-- Tab Content OUTSIDE for debugging -->
<div style="background: blue; padding: 40px; color: white; margin: 20px;">
  <h1 style="font-size: 40px; color: yellow;">TAB CONTENT TEST - OUTSIDE SETTINGS VIEW</h1>
  <p style="font-size: 24px;">Active tab: {activeTab}</p>
</div>

<div class="settings-view-content" style="background: green; padding: 40px;">
  <h2 style="color: white; font-size: 30px;">ANOTHER TEST DIV</h2>

  <!-- Tab Content -->
  <div class="tab-content" style="background: blue; padding: 20px; color: white; min-height: 200px;">
    <p style="color: yellow; font-size: 24px; margin: 20px 0;">DEBUG: Tab Content DIV is here! Active tab = "{activeTab}"</p>
    <p style="color: lime; font-size: 20px;">This should ALWAYS be visible regardless of activeTab</p>
    
    {#if activeTab === "general"}
      <p style="color: lime; font-size: 20px;">GENERAL TAB ACTIVE</p>
      <div class="settings-grid">
        <!-- Theme Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-palette-fill text-primary"></i>
            Theme
          </svelte:fragment>
          <p class="text-sm opacity-70 mb-4">
            Choose your preferred color scheme
          </p>
          <div class="flex gap-3">
            <ButtonV2
              variant={$currentTheme === "light" ? "primary" : "outline"}
              size="md"
              iconLeft="sun-fill"
              on:click={() => {
                currentTheme.set("light");
                document.documentElement.setAttribute(
                  "data-theme",
                  "syncspace"
                );
              }}
              class="flex-1"
            >
              Light
            </ButtonV2>
            <ButtonV2
              variant={$currentTheme === "dark" ? "primary" : "outline"}
              size="md"
              iconLeft="moon-fill"
              on:click={() => {
                currentTheme.set("dark");
                document.documentElement.setAttribute(
                  "data-theme",
                  "syncspace-dark"
                );
              }}
              class="flex-1"
            >
              Dark
            </ButtonV2>
          </div>
        </CardV2>

        <!-- Language Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-translate text-secondary"></i>
            Language
          </svelte:fragment>
          <p class="text-sm opacity-70 mb-4">Select your preferred language</p>
          <SelectV2
            bind:value={$currentLang}
            options={languageOptions}
            label="Interface Language"
            size="md"
          />
        </CardV2>

        <!-- Notification Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-bell-fill text-accent"></i>
            Notifications
          </svelte:fragment>
          <div class="space-y-4">
            <CheckboxV2
              bind:checked={enableNotifications}
              label="Enable Notifications"
              description="Receive desktop notifications for important events"
              color="primary"
            />
            <CheckboxV2
              bind:checked={autoBackup}
              label="Auto Backup Notifications"
              description="Get notified when automatic backups complete"
              color="secondary"
            />
          </div>
        </CardV2>

        <!-- Security Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-shield-fill-check text-success"></i>
            Security
          </svelte:fragment>
          <div class="space-y-4">
            <CheckboxV2
              bind:checked={twoFactorAuth}
              label="Two-Factor Authentication"
              description="Add an extra layer of security to your account"
              color="success"
            />
            <InputV2
              type="password"
              label="Change Password"
              placeholder="Enter new password"
              iconLeft="lock-fill"
              helpText="Password must be at least 8 characters"
            />
          </div>
          <svelte:fragment slot="actions">
            <ButtonV2 variant="ghost" size="sm">Cancel</ButtonV2>
            <ButtonV2 variant="success" size="sm" iconLeft="check-lg">
              Update Password
            </ButtonV2>
          </svelte:fragment>
        </CardV2>
      </div>
    {:else if activeTab === "users"}
      <div>
        <!-- Users Table -->
        <CardV2>
          <svelte:fragment slot="title">
            <i class="bi bi-people-fill text-primary"></i>
            User Management
          </svelte:fragment>
          <svelte:fragment slot="actions">
            <ButtonV2
              variant="primary"
              size="sm"
              iconLeft="person-plus-fill"
              on:click={() => (showAddUserModal = true)}
            >
              Add User
            </ButtonV2>
          </svelte:fragment>

          {#if loadingUsers}
            <div class="flex justify-center items-center h-64">
              <span class="loading loading-spinner loading-lg text-primary"
              ></span>
            </div>
          {:else if users.length > 0}
            <div class="overflow-x-auto">
              <table class="table w-full border-collapse-zebra">
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
                          <Avatar name={user.username} size="sm" />
                          <div class="font-semibold">{user.username}</div>
                        </div>
                      </td>
                      <td>{user.email}</td>
                      <td>
                        <Badge
                          variant={user.role === "admin" ? "error" : "ghost"}
                        >
                          {user.role}
                        </Badge>
                      </td>
                      <td class="text-sm opacity-70">{user.created}</td>
                      <td class="text-sm opacity-70">{user.lastLogin}</td>
                      <td>
                        {#if user.twoFactor}
                          <Badge variant="success">Enabled</Badge>
                        {:else}
                          <Badge variant="ghost">Disabled</Badge>
                        {/if}
                      </td>
                      <td>
                        <div class="flex gap-1 justify-end">
                          <button
                            class="btn btn-ghost btn-sm btn-circle"
                            on:click={() => openEditUserModal(user)}
                            aria-label="Edit user"
                          >
                            <i class="bi bi-pencil"></i>
                          </button>
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
              <ButtonV2
                variant="primary"
                iconLeft="person-plus-fill"
                on:click={() => (showAddUserModal = true)}
              >
                Add User
              </ButtonV2>
            </div>
          {/if}
        </CardV2>
      </div>
    {:else if activeTab === "storage"}
      <div class="settings-grid">
        <!-- Storage Settings Card -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-hdd-fill text-primary"></i>
            Storage Configuration
          </svelte:fragment>
          <div class="space-y-4">
            <InputV2
              bind:value={storageLocation}
              label="Storage Location"
              iconLeft="folder-fill"
              helpText="Path where files are stored"
              disabled
            />
            <InputV2
              bind:value={maxFileSize}
              label="Max File Size (MB)"
              type="number"
              iconLeft="file-earmark-arrow-up"
              helpText="Maximum size for uploaded files"
            />
            <div
              class="flex justify-between items-center p-3 bg-slate-50 dark:bg-slate-800 rounded-lg"
            >
              <div>
                <div class="font-semibold">Cache Size</div>
                <div class="text-sm opacity-70">Currently using 245 MB</div>
              </div>
              <ButtonV2
                variant="error"
                size="sm"
                iconLeft="trash-fill"
                on:click={handleClearCache}
              >
                Clear Cache
              </ButtonV2>
            </div>
          </div>
        </CardV2>
      </div>
    {:else if activeTab === "security"}
      <div class="settings-grid">
        <!-- Security Settings Card -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-shield-fill-check text-success"></i>
            Security Settings
          </svelte:fragment>
          <div class="space-y-4">
            <CheckboxV2
              bind:checked={twoFactorAuth}
              label="Two-Factor Authentication"
              description="Add an extra layer of security to your account"
              color="success"
            />
            <InputV2
              type="password"
              label="Change Password"
              placeholder="Enter new password"
              iconLeft="lock-fill"
              helpText="Password must be at least 8 characters"
            />
          </div>
          <svelte:fragment slot="actions">
            <ButtonV2 variant="ghost" size="sm">Cancel</ButtonV2>
            <ButtonV2 variant="success" size="sm" iconLeft="check-lg">
              Update Password
            </ButtonV2>
          </svelte:fragment>
        </CardV2>
      </div>
    {:else if activeTab === "advanced"}
      <div class="settings-grid">
        <!-- Backup Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-cloud-arrow-up-fill text-info"></i>
            Backup
          </svelte:fragment>
          <div class="space-y-4">
            <CheckboxV2
              bind:checked={autoBackup}
              label="Automatic Backups"
              description="Automatically backup your data daily at 2:00 AM"
              color="info"
            />
            <SelectV2
              value="daily"
              options={[
                { value: "hourly", label: "Every Hour" },
                { value: "daily", label: "Daily" },
                { value: "weekly", label: "Weekly" },
                { value: "monthly", label: "Monthly" },
              ]}
              label="Backup Frequency"
            />
            <div class="stats shadow w-full">
              <div class="stat">
                <div class="stat-figure text-primary">
                  <i class="bi bi-clock-history text-3xl"></i>
                </div>
                <div class="stat-title">Last Backup</div>
                <div class="stat-value text-primary text-lg">2 hours ago</div>
                <div class="stat-desc">Next backup in 22 hours</div>
              </div>
            </div>
          </div>
          <svelte:fragment slot="actions">
            <ButtonV2 variant="primary" size="sm" iconLeft="cloud-arrow-up">
              Backup Now
            </ButtonV2>
          </svelte:fragment>
        </CardV2>

        <!-- Developer Settings -->
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-code-slash text-secondary"></i>
            Developer
          </svelte:fragment>
          <div class="space-y-4">
            <div class="alert alert-info">
              <i class="bi bi-info-circle-fill"></i>
              <span>These settings are for advanced users only</span>
            </div>
            <CheckboxV2
              checked={false}
              label="Debug Mode"
              description="Enable verbose logging"
              color="warning"
            />
            <CheckboxV2
              checked={false}
              label="API Access"
              description="Enable REST API endpoints"
              color="secondary"
            />
          </div>
          <svelte:fragment slot="actions">
            <ButtonV2
              variant="outline"
              size="sm"
              iconLeft="download"
              on:click={handleExportSettings}
            >
              Export Settings
            </ButtonV2>
          </svelte:fragment>
        </CardV2>
      </div>
    {:else if activeTab === "about"}
      <div class="max-w-2xl mx-auto">
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-app-indicator text-primary"></i>
            SyncSpace
          </svelte:fragment>
          <div class="text-center space-y-6 py-4">
            <div class="logo-large">
              <svg
                class="w-24 h-24 mx-auto"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M13 2L3 14h8l-1 8 10-12h-8l1-8z"
                  fill="url(#logo-gradient-about)"
                />
                <defs>
                  <linearGradient
                    id="logo-gradient-about"
                    x1="3"
                    y1="2"
                    x2="21"
                    y2="22"
                  >
                    <stop offset="0%" stop-color="#667eea" />
                    <stop offset="100%" stop-color="#764ba2" />
                  </linearGradient>
                </defs>
              </svg>
            </div>

            <div>
              <h2 class="text-3xl font-bold mb-2">SyncSpace</h2>
              <p class="text-lg opacity-70">Cloud Storage Solution</p>
            </div>

            <div class="divider"></div>

            <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
              <div class="stat">
                <div class="stat-title">Version</div>
                <div class="stat-value text-primary text-2xl">1.0.0</div>
                <div class="stat-desc">Latest Release</div>
              </div>

              <div class="stat">
                <div class="stat-title">License</div>
                <div class="stat-value text-secondary text-2xl">MIT</div>
                <div class="stat-desc">Open Source</div>
              </div>

              <div class="stat">
                <div class="stat-title">Build</div>
                <div class="stat-value text-accent text-2xl">2025.10</div>
                <div class="stat-desc">October 2025</div>
              </div>
            </div>

            <div class="text-sm opacity-70 space-y-2">
              <p>Built with Rust, Svelte, and DaisyUI</p>
              <p>© 2025 SyncSpace. All rights reserved.</p>
            </div>
          </div>
          <svelte:fragment slot="actions">
            <ButtonV2 variant="ghost" size="sm" iconLeft="github" fullWidth>
              View on GitHub
            </ButtonV2>
            <ButtonV2
              variant="primary"
              size="sm"
              iconLeft="info-circle"
              fullWidth
            >
              Documentation
            </ButtonV2>
          </svelte:fragment>
        </CardV2>
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
        <InputV2
          bind:value={newUsername}
          label="Username"
          placeholder="Enter username..."
          iconLeft="person-fill"
        />
        <InputV2
          bind:value={newUserPassword}
          type="password"
          label="Password"
          placeholder="Enter password..."
          iconLeft="lock-fill"
          helpText="Password must be at least 8 characters"
        />
      </div>

      <div class="flex gap-2 justify-end mt-6">
        <ButtonV2 variant="ghost" on:click={() => (showAddUserModal = false)}>
          Cancel
        </ButtonV2>
        <ButtonV2
          variant="primary"
          iconLeft="check-lg"
          on:click={handleAddUser}
        >
          Create User
        </ButtonV2>
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
        <ButtonV2
          variant="ghost"
          on:click={() => (showDeleteUserModal = false)}
        >
          Cancel
        </ButtonV2>
        <ButtonV2 variant="error" iconLeft="trash" on:click={handleDeleteUser}>
          Delete User
        </ButtonV2>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={() => (showDeleteUserModal = false)}>close</button>
    </form>
  </dialog>
{/if}

<style>
  .settings-view {
    padding: var(--spacing-8);
    max-width: 1400px;
    margin: 0 auto;
    min-height: 100vh;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--spacing-8);
  }

  .settings-title {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 var(--spacing-2) 0;
    display: flex;
    align-items: center;
    gap: var(--spacing-3);
  }

  .settings-subtitle {
    font-size: 1rem;
    opacity: 0.7;
    margin: 0;
  }

  /* Modern Tab Navigation */
  .tab-navigation {
    display: flex;
    gap: var(--spacing-2);
    padding: var(--spacing-1);
    background: var(--md-sys-color-surface-variant);
    border-radius: 12px;
    margin-bottom: var(--spacing-8);
    overflow-x: auto;
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    padding: var(--spacing-3) var(--spacing-5);
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    border-radius: 8px;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    position: relative;
  }

  .tab-button i {
    font-size: 1.125rem;
  }

  .tab-button:hover {
    background: var(--md-sys-color-surface-container-high);
  }

  .tab-button.active {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  /* Tab Content */
  .tab-content {
    animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
    gap: var(--spacing-6);
  }

  /* Utility Classes */
  .space-y-4 > * + * {
    margin-top: var(--spacing-4);
  }

  .space-y-6 > * + * {
    margin-top: var(--spacing-6);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 768px) {
    .settings-view {
      padding: var(--spacing-4);
    }

    .settings-title {
      font-size: 1.5rem;
    }

    .tab-navigation {
      gap: var(--spacing-1);
    }

    .tab-button {
      padding: var(--spacing-2) var(--spacing-3);
      font-size: 0.875rem;
    }

    .tab-button span {
      display: none;
    }

    .tab-button i {
      font-size: 1.25rem;
    }
  }
</style>

