<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../stores/ui.js";
  import { auth } from "../stores/auth.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast, info } from "../stores/toast";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import ButtonV2 from "../components/ui/ButtonV2.svelte";
  import CardV2 from "../components/ui/CardV2.svelte";
  import InputV2 from "../components/ui/InputV2.svelte";
  import SelectV2 from "../components/ui/SelectV2.svelte";
  import CheckboxV2 from "../components/ui/CheckboxV2.svelte";
  import Avatar from "../components/ui/Avatar.svelte";
  import Badge from "../components/ui/Badge.svelte";

  let activeTab = "general"; // general, users, storage, security, advanced, about

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
      // Mock data - replace with actual API call
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

  // Language options
  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch" },
    { value: "en", label: "🇬🇧 English" },
    { value: "fr", label: "🇫🇷 Français" },
    { value: "es", label: "🇪🇸 Español" },
  ];

  // Settings form
  let storageLocation = "./data";
  let autoBackup = true;
  let enableNotifications = true;
  let twoFactorAuth = false;
  let maxFileSize = "100";

  function handleClearCache() {
    alert("Cache cleared successfully!");
  }

  function handleExportSettings() {
    alert("Settings exported!");
  }

  function handleSaveSettings() {
    alert("Settings saved successfully!");
  }
</script>

<div class="settings-view">
  <!-- Tabs -->
  <div class="tabs tabs-boxed mb-6 bg-base-200">
    <button
      class="tab"
      class:tab-active={activeTab === "general"}
      on:click={() => handleTabChange("general")}
    >
      <i class="bi bi-sliders mr-2"></i>
      General
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === "users"}
      on:click={() => handleTabChange("users")}
    >
      <i class="bi bi-people-fill mr-2"></i>
      Users
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === "storage"}
      on:click={() => handleTabChange("storage")}
    >
      <i class="bi bi-hdd-fill mr-2"></i>
      Storage
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === "security"}
      on:click={() => handleTabChange("security")}
    >
      <i class="bi bi-shield-fill-check mr-2"></i>
      Security
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === "advanced"}
      on:click={() => handleTabChange("advanced")}
    >
      <i class="bi bi-tools mr-2"></i>
      Advanced
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === "about"}
      on:click={() => handleTabChange("about")}
    >
      <i class="bi bi-info-circle mr-2"></i>
      About
    </button>
  </div>

  <!-- Content -->
  <div class="settings-content">
    {#if activeTab === "general"}
      <div class="settings-grid fade-in">
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
      <div class="fade-in">
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
          {:else if users.length === 0}
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
          {:else}
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
          {/if}
        </CardV2>
      </div>
    {:else if activeTab === "storage"}
      <div class="settings-grid fade-in">
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
              class="flex justify-between items-center p-3 bg-base-200 rounded-lg"
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
      <div class="settings-grid fade-in">
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
      <div class="settings-grid fade-in">
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
      <div class="max-w-2xl mx-auto fade-in">
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

  <!-- Save Button (Fixed Bottom) -->
  <div class="settings-actions">
    <ButtonV2 variant="ghost" size="lg" iconLeft="arrow-clockwise">
      Reset to Defaults
    </ButtonV2>
    <ButtonV2
      variant="primary"
      size="lg"
      iconLeft="check-lg"
      on:click={handleSaveSettings}
    >
      Save All Changes
    </ButtonV2>
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
    padding: var(--spacing-6);
    max-width: 1400px;
    margin: 0 auto;
  }

  .settings-content {
    margin-bottom: var(--spacing-20);
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: var(--spacing-6);
  }

  .space-y-4 > * + * {
    margin-top: var(--spacing-4);
  }

  .space-y-6 > * + * {
    margin-top: var(--spacing-6);
  }

  .fade-in {
    animation: fadeIn var(--duration-300) var(--ease-standard);
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

  .settings-actions {
    position: fixed;
    bottom: 0;
    left: 280px;
    right: 0;
    padding: var(--spacing-4) var(--spacing-6);
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    border-top: 1px solid var(--color-outline);
    display: flex;
    justify-content: flex-end;
    gap: var(--spacing-3);
    z-index: var(--z-sticky);
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }

    .settings-actions {
      left: 0;
    }
  }

  @media (max-width: 768px) {
    .settings-view {
      padding: var(--spacing-4);
    }

    .settings-actions {
      flex-direction: column;
    }
  }
</style>
