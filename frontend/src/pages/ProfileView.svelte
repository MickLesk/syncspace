<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../stores/ui";
  import { t } from "../i18n";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import Avatar from "../components/ui/Avatar.svelte";
  import Button from "../components/ui/Button.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import Input from "../components/ui/Input.svelte";
  import Icon from "../components/ui/Icon.svelte";
  import Spinner from "../components/ui/Spinner.svelte";
  import TabBar from "../components/ui/TabBar.svelte";

  let user = {
    id: "",
    username: "",
    email: "",
    display_name: "",
    avatar_base64: "",
    created_at: "",
  };

  let settings = {
    theme: "light",
    language: "de",
    default_view: "grid",
  };

  let loading = true;
  let saving = false;
  let showPasswordDialog = false;
  let showImageUploadDialog = false;
  let passwordData = { current: "", new: "", confirm: "" };
  let passwordError = "";
  let imageFile = null;
  let imagePreview = "";
  let activeTab = "profile"; // profile, settings, security

  onMount(() => {
    loadUserProfile();
    loadUserSettings();
  });

  async function loadUserProfile() {
    try {
      loading = true;
      const profile = await api.users.getProfile();
      user = profile;
      imagePreview = profile.avatar_base64 || "";
      console.log("[ProfileView] Loaded user:", user);
    } catch (err) {
      console.error("Failed to load user profile:", err);
      errorToast("Fehler beim Laden des Profils");
    } finally {
      loading = false;
    }
  }

  async function loadUserSettings() {
    try {
      const userSettings = await api.users.getSettings();
      settings = userSettings;
      // Sync with UI stores
      currentTheme.set(userSettings.theme);
      currentLang.set(userSettings.language);
      console.log("[ProfileView] Loaded settings:", userSettings);
    } catch (err) {
      console.error("Failed to load settings:", err);
    }
  }

  async function handleSaveProfile() {
    if (!user.email) {
      errorToast("E-Mail ist erforderlich");
      return;
    }

    try {
      saving = true;
      const updated = await api.users.updateProfile({
        email: user.email,
        display_name: user.display_name,
        avatar_base64: imagePreview,
      });
      user = updated;
      success("Profil erfolgreich gespeichert!");
    } catch (err) {
      console.error("Failed to save profile:", err);
      errorToast("Fehler beim Speichern des Profils");
    } finally {
      saving = false;
    }
  }

  async function handleSaveSettings() {
    try {
      saving = true;
      const updated = await api.users.updateSettings({
        language: settings.language,
        theme: settings.theme,
        default_view: settings.default_view,
      });
      settings = updated;
      currentTheme.set(updated.theme);
      currentLang.set(updated.language);
      success("Einstellungen gespeichert!");
    } catch (err) {
      console.error("Failed to save settings:", err);
      errorToast("Fehler beim Speichern der Einstellungen");
    } finally {
      saving = false;
    }
  }

  async function handlePasswordChange() {
    passwordError = "";

    if (!passwordData.current || !passwordData.new || !passwordData.confirm) {
      passwordError = "Bitte fülle alle Felder aus";
      return;
    }

    if (passwordData.new !== passwordData.confirm) {
      passwordError = "Passwörter stimmen nicht überein";
      return;
    }

    if (passwordData.new.length < 8) {
      passwordError = "Passwort muss mindestens 8 Zeichen lang sein";
      return;
    }

    try {
      saving = true;
      await api.auth.changePassword(passwordData.current, passwordData.new);
      showPasswordDialog = false;
      passwordData = { current: "", new: "", confirm: "" };
      success("Passwort erfolgreich geändert!");
    } catch (err) {
      console.error("Password change failed:", err);
      passwordError =
        "Fehler beim Ändern des Passworts. Überprüfe dein aktuelles Passwort.";
    } finally {
      saving = false;
    }
  }

  async function handleImageSelect(event) {
    const file = event.target.files?.[0];
    if (file) {
      imageFile = file;
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target && typeof e.target.result === "string") {
          imagePreview = e.target.result;
        }
      };
      reader.readAsDataURL(file);
    }
  }

  async function handleImageUpload() {
    if (!imagePreview) return;

    try {
      saving = true;
      // Update profile with new avatar
      user.avatar_base64 = imagePreview;
      const updated = await api.users.updateProfile({
        avatar_base64: imagePreview,
        email: user.email,
        display_name: user.display_name,
      });
      user = updated;
      showImageUploadDialog = false;
      imageFile = null;
      success("Profilbild hochgeladen!");
    } catch (err) {
      console.error("Failed to upload profile image:", err);
      errorToast("Fehler beim Hochladen des Profilbilds");
      imagePreview = user.avatar_base64 || "";
    } finally {
      saving = false;
    }
  }
</script>

<div class="modern-profile-view">
  <!-- Modern PageHeader -->
  <PageHeader
    title={user.display_name || user.username}
    subtitle="@{user.username}"
    icon="person-circle"
    gradient="purple"
  >
    <div slot="stats" class="profile-stats">
      <div class="stat">
        <Icon name="calendar-check" size={16} />
        <span
          >Joined {new Date(user.created_at).toLocaleDateString("en-US", {
            month: "short",
            year: "numeric",
          })}</span
        >
      </div>
      <div class="stat">
        <Icon name="envelope" size={16} />
        <span>{user.email || "No email set"}</span>
      </div>
    </div>
  </PageHeader>

  <!-- Avatar Section (below header) -->
  <div class="profile-avatar-section">
    <div class="avatar-container">
      <Avatar
        name={user.username}
        imageUrl={user.avatar_base64}
        size="xlarge"
      />
      <button
        class="avatar-edit-btn glass-button"
        onclick={() => (showImageUploadDialog = true)}
        title="Change profile picture"
      >
        <Icon name="camera-fill" size={20} />
      </button>
    </div>
  </div>

  <!-- Modern Tab Navigation -->
  <TabBar
    tabs={[
      { id: "profile", label: "Profile", icon: "person" },
      { id: "settings", label: "Settings", icon: "gear" },
      { id: "security", label: "Security", icon: "shield-check" },
    ]}
    {activeTab}
    onChange={(tabId) => (activeTab = tabId)}
  />

  {#if loading}
    <div class="loading-state">
      <Spinner size="large" />
      <p>Loading profile...</p>
    </div>
  {:else}
    <div class="tab-content">
      <!-- Profile Tab -->
      {#if activeTab === "profile"}
        <div class="content-grid">
          <div class="modern-card">
            <div class="card-header">
              <Icon name="person-badge" size={24} />
              <h2>Personal Information</h2>
            </div>
            <div class="card-body">
              <div class="form-group">
                <label for="display-name">
                  <Icon name="tag" size={16} />
                  <span>Display Name</span>
                </label>
                <Input
                  id="display-name"
                  bind:value={user.display_name}
                  type="text"
                  placeholder="Your display name"
                />
              </div>

              <div class="form-group">
                <label for="email">
                  <Icon name="envelope" size={16} />
                  <span>Email Address</span>
                </label>
                <Input
                  id="email"
                  bind:value={user.email}
                  type="email"
                  placeholder="your@email.com"
                />
              </div>

              <div class="form-group">
                <label>
                  <Icon name="person-circle" size={16} />
                  <span>Username</span>
                </label>
                <div class="readonly-field">
                  <Icon name="lock" size={14} />
                  <span>{user.username}</span>
                  <span class="readonly-badge">Read-only</span>
                </div>
              </div>

              <div class="card-actions">
                <button
                  class="modern-button modern-button-primary"
                  onclick={handleSaveProfile}
                  disabled={saving}
                >
                  {#if saving}
                    <div class="button-spinner"></div>
                  {:else}
                    <Icon name="check-circle" size={18} />
                  {/if}
                  <span>Save Changes</span>
                </button>
              </div>
            </div>
          </div>

          <div class="modern-card">
            <div class="card-header">
              <Icon name="info-circle" size={24} />
              <h2>Account Details</h2>
            </div>
            <div class="card-body">
              <div class="detail-row">
                <div class="detail-label">
                  <Icon name="hash" size={16} />
                  <span>User ID</span>
                </div>
                <div class="detail-value">{user.id}</div>
              </div>
              <div class="detail-row">
                <div class="detail-label">
                  <Icon name="calendar3" size={16} />
                  <span>Account Created</span>
                </div>
                <div class="detail-value">
                  {new Date(user.created_at).toLocaleDateString("en-US", {
                    year: "numeric",
                    month: "long",
                    day: "numeric",
                  })}
                </div>
              </div>
              <div class="detail-row">
                <div class="detail-label">
                  <Icon name="clock-history" size={16} />
                  <span>Last Updated</span>
                </div>
                <div class="detail-value">Just now</div>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Settings Tab -->
      {#if activeTab === "settings"}
        <div class="content-grid">
          <div class="modern-card">
            <div class="card-header">
              <Icon name="palette" size={24} />
              <h2>Appearance</h2>
            </div>
            <div class="card-body">
              <div class="setting-row">
                <div class="setting-info">
                  <div class="setting-label">
                    <Icon
                      name={settings.theme === "dark"
                        ? "moon-stars-fill"
                        : "sun-fill"}
                      size={20}
                    />
                    <span>Theme</span>
                  </div>
                  <p class="setting-description">
                    Choose your preferred color scheme
                  </p>
                </div>
                <div class="theme-selector">
                  {#each ["light", "dark", "system"] as themeOption}
                    <button
                      class="theme-option"
                      class:active={settings.theme === themeOption}
                      onclick={() => (settings.theme = themeOption)}
                    >
                      <Icon
                        name={themeOption === "light"
                          ? "sun-fill"
                          : themeOption === "dark"
                            ? "moon-stars-fill"
                            : "laptop"}
                        size={18}
                      />
                      <span
                        >{themeOption.charAt(0).toUpperCase() +
                          themeOption.slice(1)}</span
                      >
                    </button>
                  {/each}
                </div>
              </div>

              <div class="setting-row">
                <div class="setting-info">
                  <div class="setting-label">
                    <Icon name="globe" size={20} />
                    <span>Language</span>
                  </div>
                  <p class="setting-description">
                    Select your preferred language
                  </p>
                </div>
                <select bind:value={settings.language} class="modern-select">
                  <option value="de">🇩🇪 Deutsch</option>
                  <option value="en">🇬🇧 English</option>
                </select>
              </div>

              <div class="setting-row">
                <div class="setting-info">
                  <div class="setting-label">
                    <Icon name="grid-3x3" size={20} />
                    <span>Default View</span>
                  </div>
                  <p class="setting-description">
                    Choose how files are displayed by default
                  </p>
                </div>
                <select
                  bind:value={settings.default_view}
                  class="modern-select"
                >
                  <option value="grid">Grid View</option>
                  <option value="list">List View</option>
                </select>
              </div>

              <div class="card-actions">
                <button
                  class="modern-button modern-button-primary"
                  onclick={handleSaveSettings}
                  disabled={saving}
                >
                  {#if saving}
                    <div class="button-spinner"></div>
                  {:else}
                    <Icon name="check-circle" size={18} />
                  {/if}
                  <span>Save Settings</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Security Tab -->
      {#if activeTab === "security"}
        <div class="content-grid">
          <div class="modern-card">
            <div class="card-header">
              <Icon name="key" size={24} />
              <h2>Password & Security</h2>
            </div>
            <div class="card-body">
              <div class="security-item">
                <div class="security-icon">
                  <Icon name="lock-fill" size={24} />
                </div>
                <div class="security-info">
                  <h3>Change Password</h3>
                  <p>Update your password to keep your account secure</p>
                </div>
                <button
                  class="modern-button"
                  onclick={() => (showPasswordDialog = true)}
                >
                  <Icon name="key" size={18} />
                  <span>Change</span>
                </button>
              </div>

              <div class="security-item">
                <div class="security-icon success">
                  <Icon name="shield-check" size={24} />
                </div>
                <div class="security-info">
                  <h3>Two-Factor Authentication</h3>
                  <p>Add an extra layer of security to your account</p>
                </div>
                <Button variant="outlined" disabled>
                  <Icon name="shield-plus" size={18} />
                  <span>Coming Soon</span>
                </Button>
              </div>

              <div class="security-item">
                <div class="security-icon">
                  <Icon name="clock-history" size={24} />
                </div>
                <div class="security-info">
                  <h3>Active Sessions</h3>
                  <p>Manage devices where you're currently logged in</p>
                </div>
                <Button variant="outlined" disabled>
                  <Icon name="list-ul" size={18} />
                  <span>View</span>
                </Button>
              </div>
            </div>
          </div>

          <div class="modern-card danger-zone">
            <div class="card-header">
              <Icon name="exclamation-triangle-fill" size={24} />
              <h2>Danger Zone</h2>
            </div>
            <div class="card-body">
              <div class="danger-item">
                <div class="danger-info">
                  <h3>Delete Account</h3>
                  <p>Permanently delete your account and all associated data</p>
                </div>
                <Button variant="danger" disabled>
                  <Icon name="trash" size={18} />
                  <span>Delete Account</span>
                </Button>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Dialogs -->
<Dialog
  bind:open={showPasswordDialog}
  title="Passwort ändern"
  confirmText="Ändern"
  cancelText="Abbrechen"
  on:confirm={handlePasswordChange}
>
  <div class="password-form">
    <Input
      bind:value={passwordData.current}
      label="Aktuelles Passwort"
      type="password"
      required
      icon="🔒"
    />
    <Input
      bind:value={passwordData.new}
      label="Neues Passwort"
      type="password"
      required
      icon="🔑"
    />
    <Input
      bind:value={passwordData.confirm}
      label="Passwort bestätigen"
      type="password"
      required
      icon="✅"
    />
    {#if passwordError}
      <div class="error-message">{passwordError}</div>
    {/if}
  </div>
</Dialog>

<!-- Image Upload Dialog -->
<Dialog
  bind:open={showImageUploadDialog}
  title="Profilbild hochladen"
  confirmText="Hochladen"
  cancelText="Abbrechen"
  on:confirm={handleImageUpload}
>
  <div class="image-upload">
    {#if imagePreview}
      <img src={imagePreview} alt="Preview" class="image-preview" />
    {:else}
      <div class="upload-placeholder">
        <span class="upload-icon">📷</span>
        <p>Bild auswählen</p>
      </div>
    {/if}
    <input
      type="file"
      accept="image/*"
      onchange={handleImageSelect}
      class="file-input"
    />
  </div>
</Dialog>

<style>
  .modern-profile-view {
    min-height: 100vh;
    background: var(--md-sys-color-background);
    padding: 0;
  }

  /* Profile Stats in Header */
  .profile-stats {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
  }

  .profile-stats .stat {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    font-size: 13px;
    color: white;
  }

  /* Avatar Section (below header) */
  .profile-avatar-section {
    max-width: 1200px;
    margin: -60px auto 32px auto;
    padding: 0 32px;
    position: relative;
    z-index: 10;
    display: flex;
    justify-content: center;
  }

  .avatar-container {
    position: relative;
    display: inline-block;
  }

  .avatar-edit-btn {
    position: absolute;
    bottom: 8px;
    right: 8px;
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    border: 3px solid white;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  :global([data-theme="dark"]) .avatar-edit-btn {
    border-color: #1e1e26;
  }

  .avatar-edit-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 6px 16px rgba(99, 102, 241, 0.4);
  }

  /* Tab Content */
  .tab-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 32px;
  }

  .content-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
    gap: 24px;
  }

  /* Modern Card */
  .modern-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 24px;
    padding: 32px;
    border: 1px solid rgba(0, 0, 0, 0.06);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.04);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global([data-theme="dark"]) .modern-card {
    background: rgba(30, 30, 38, 0.95);
    border-color: rgba(255, 255, 255, 0.08);
  }

  .modern-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.08);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 28px;
    padding-bottom: 20px;
    border-bottom: 2px solid rgba(0, 0, 0, 0.06);
  }

  :global([data-theme="dark"]) .card-header {
    border-bottom-color: rgba(255, 255, 255, 0.08);
  }

  .card-header h2 {
    font-size: 22px;
    font-weight: 600;
    margin: 0;
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  /* Form Group */
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .form-group label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.7);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global([data-theme="dark"]) .form-group label {
    color: rgba(255, 255, 255, 0.7);
  }

  .readonly-field {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    background: rgba(0, 0, 0, 0.03);
    border: 2px solid rgba(0, 0, 0, 0.06);
    border-radius: 14px;
    color: rgba(0, 0, 0, 0.6);
  }

  :global([data-theme="dark"]) .readonly-field {
    background: rgba(255, 255, 255, 0.03);
    border-color: rgba(255, 255, 255, 0.06);
    color: rgba(255, 255, 255, 0.6);
  }

  .readonly-badge {
    margin-left: auto;
    padding: 4px 12px;
    background: rgba(99, 102, 241, 0.1);
    color: #6366f1;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Detail Row */
  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  }

  :global([data-theme="dark"]) .detail-row {
    border-bottom-color: rgba(255, 255, 255, 0.06);
  }

  .detail-row:last-child {
    border-bottom: none;
  }

  .detail-label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 14px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.6);
  }

  :global([data-theme="dark"]) .detail-label {
    color: rgba(255, 255, 255, 0.6);
  }

  .detail-value {
    font-size: 14px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.9);
  }

  :global([data-theme="dark"]) .detail-value {
    color: rgba(255, 255, 255, 0.9);
  }

  /* Setting Row */
  .setting-row {
    padding: 24px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 24px;
  }

  :global([data-theme="dark"]) .setting-row {
    background: rgba(255, 255, 255, 0.02);
  }

  .setting-info {
    flex: 1;
  }

  .setting-label {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 6px;
  }

  .setting-description {
    font-size: 13px;
    color: rgba(0, 0, 0, 0.5);
    margin: 0;
  }

  :global([data-theme="dark"]) .setting-description {
    color: rgba(255, 255, 255, 0.5);
  }

  /* Theme Selector */
  .theme-selector {
    display: flex;
    gap: 8px;
  }

  .theme-option {
    padding: 12px 20px;
    border-radius: 12px;
    border: 2px solid rgba(0, 0, 0, 0.1);
    background: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.7);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .theme-option:hover {
    border-color: #6366f1;
    transform: translateY(-2px);
  }

  .theme-option.active {
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    border-color: #6366f1;
    color: white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .modern-select {
    padding: 12px 18px;
    border-radius: 12px;
    border: 2px solid rgba(0, 0, 0, 0.08);
    background: white;
    font-size: 14px;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .modern-select:focus {
    outline: none;
    border-color: #6366f1;
    box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.1);
  }

  /* Security Item */
  .security-item {
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 24px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 16px;
  }

  :global([data-theme="dark"]) .security-item {
    background: rgba(255, 255, 255, 0.02);
  }

  .security-icon {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    background: rgba(99, 102, 241, 0.1);
    color: #6366f1;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .security-icon.success {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  .security-info {
    flex: 1;
  }

  .security-info h3 {
    font-size: 16px;
    font-weight: 600;
    margin: 0 0 6px 0;
  }

  .security-info p {
    font-size: 13px;
    color: rgba(0, 0, 0, 0.6);
    margin: 0;
  }

  :global([data-theme="dark"]) .security-info p {
    color: rgba(255, 255, 255, 0.6);
  }

  /* Danger Zone */
  .danger-zone {
    border: 2px solid rgba(239, 68, 68, 0.2);
  }

  .danger-zone .card-header {
    color: #ef4444;
    border-bottom-color: rgba(239, 68, 68, 0.2);
  }

  .danger-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 24px;
    padding: 20px;
    background: rgba(239, 68, 68, 0.05);
    border-radius: 14px;
  }

  .danger-info h3 {
    font-size: 16px;
    font-weight: 600;
    color: #dc2626;
    margin: 0 0 6px 0;
  }

  .danger-info p {
    font-size: 13px;
    color: rgba(220, 38, 38, 0.8);
    margin: 0;
  }

  /* Card Actions */
  .card-actions {
    margin-top: 8px;
    padding-top: 24px;
    border-top: 2px solid rgba(0, 0, 0, 0.06);
  }

  :global([data-theme="dark"]) .card-actions {
    border-top-color: rgba(255, 255, 255, 0.06);
  }

  .button-spinner {
    width: 18px;
    height: 18px;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  /* Loading State */
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 32px;
    gap: 20px;
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .content-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 768px) {
    .tab-content {
      padding: 16px;
    }

    .modern-card {
      padding: 20px;
    }

    .theme-selector {
      flex-direction: column;
    }
  }
</style>
