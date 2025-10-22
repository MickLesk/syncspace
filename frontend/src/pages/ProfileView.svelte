<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../stores/ui";
  import { t } from "../i18n";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import Avatar from "../components/ui/Avatar.svelte";
  import Button from "../components/ui/Button.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import Input from "../components/ui/Input.svelte";

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

<div class="profile-view">
  <div class="profile-header">
    <h1>👤 {t($currentLang, "profile")}</h1>
  </div>

  {#if loading}
    <div class="loading-state">
      <p>Lädt Profil...</p>
    </div>
  {:else}
    <div class="profile-content">
      <!-- Avatar Section -->
      <div class="card avatar-section">
        <h2>Profilbild</h2>
        <div class="avatar-controls">
          <Avatar
            name={user.username}
            imageUrl={user.avatar_base64}
            size="xlarge"
          />
          <div class="avatar-actions">
            <Button onClick={() => (showImageUploadDialog = true)} icon="📷">
              Bild hochladen
            </Button>
            {#if user.avatar_base64}
              <Button
                variant="outlined"
                onClick={() => {
                  imagePreview = "";
                  user.avatar_base64 = "";
                }}
                icon="🗑️"
              >
                Bild entfernen
              </Button>
            {/if}
          </div>
        </div>
      </div>

      <!-- Basic Info -->
      <div class="card info-section">
        <h2>Benutzerdaten</h2>
        <div class="info-grid">
          <div class="info-item">
            <span class="item-label">Benutzername</span>
            <div class="info-value">{user.username}</div>
          </div>
          <div class="info-item">
            <span class="item-label">E-Mail</span>
            <Input
              bind:value={user.email}
              type="email"
              placeholder="deine@email.com"
            />
          </div>
          <div class="info-item">
            <span class="item-label">Anzeigename</span>
            <Input
              bind:value={user.display_name}
              type="text"
              placeholder="Dein Name"
            />
          </div>
          <div class="info-item">
            <span class="item-label">Mitglied seit</span>
            <div class="info-value">
              {new Date(user.created_at).toLocaleDateString("de-DE")}
            </div>
          </div>
          <div class="info-item">
            <Button
              variant="outlined"
              onClick={() => (showPasswordDialog = true)}
              icon="🔒"
              fullWidth
            >
              Passwort ändern
            </Button>
          </div>
        </div>
      </div>

      <!-- Appearance Settings -->
      <div class="card appearance-section">
        <h2>🎨 Darstellung</h2>
        <div class="settings-grid">
          <div class="setting-item">
            <label for="theme">Theme</label>
            <select id="theme" bind:value={settings.theme} class="md-select">
              <option value="light">☀️ Hell</option>
              <option value="dark">🌙 Dunkel</option>
              <option value="system">💻 System</option>
            </select>
          </div>

          <div class="setting-item">
            <label for="language">Sprache</label>
            <select
              id="language"
              bind:value={settings.language}
              class="md-select"
            >
              <option value="de">🇩🇪 Deutsch</option>
              <option value="en">🇬🇧 English</option>
            </select>
          </div>

          <div class="setting-item">
            <label for="default_view">Standard-Ansicht</label>
            <select
              id="default_view"
              bind:value={settings.default_view}
              class="md-select"
            >
              <option value="grid">🔲 Kacheln</option>
              <option value="list">📋 Liste</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Save Buttons -->
      <div class="save-section">
        <Button
          onClick={handleSaveProfile}
          icon="💾"
          size="large"
          fullWidth
          disabled={saving}
        >
          {saving ? "Speichern..." : "Profil speichern"}
        </Button>
        <Button
          onClick={handleSaveSettings}
          variant="outlined"
          icon="⚙️"
          size="large"
          fullWidth
          disabled={saving}
        >
          {saving ? "Speichern..." : "Einstellungen speichern"}
        </Button>
      </div>
    </div>
  {/if}
</div>

<!-- Password Change Dialog -->
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
      on:change={handleImageSelect}
      class="file-input"
    />
  </div>
</Dialog>

<style>
  .profile-view {
    height: 100%;
    overflow: auto;
    background: var(--md-sys-color-surface);
  }

  .profile-header {
    padding: 32px;
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary) 0%,
      var(--md-sys-color-tertiary) 100%
    );
    color: white;
  }

  .profile-header h1 {
    font-size: 32px;
    font-weight: 600;
    margin: 0;
  }

  .profile-content {
    max-width: 800px;
    margin: 0 auto;
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .card {
    background: var(--md-sys-color-surface-variant);
    border-radius: 24px;
    padding: 24px;
    box-shadow: var(--md-elevation-1);
  }

  .card h2 {
    font-size: 20px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 20px 0;
  }

  .avatar-section .avatar-controls {
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .avatar-actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .info-item .item-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
  }

  .info-value {
    padding: 16px;
    background: var(--md-sys-color-surface);
    border-radius: 12px;
    color: var(--md-sys-color-on-surface);
  }

  .settings-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-item label {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
  }

  .md-select {
    height: 56px;
    padding: 0 16px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 12px;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: "Roboto", sans-serif;
    font-size: 16px;
    cursor: pointer;
    outline: none;
  }

  .md-select:focus {
    border-color: var(--md-sys-color-primary);
    border-width: 2px;
  }

  .password-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .error-message {
    padding: 12px;
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
    border-radius: 12px;
    font-size: 14px;
  }

  .image-upload {
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: center;
  }

  .image-preview {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    object-fit: cover;
    border: 4px solid var(--md-sys-color-primary);
  }

  .upload-placeholder {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    background: var(--md-sys-color-surface-variant);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    border: 2px dashed var(--md-sys-color-outline);
  }

  .upload-icon {
    font-size: 48px;
  }

  .file-input {
    width: 100%;
    padding: 12px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 12px;
    background: var(--md-sys-color-surface);
    cursor: pointer;
  }

  @media (max-width: 768px) {
    .info-grid,
    .settings-grid {
      grid-template-columns: 1fr;
    }

    .avatar-section .avatar-controls {
      flex-direction: column;
      align-items: center;
    }
  }
</style>
