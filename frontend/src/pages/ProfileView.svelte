<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../stores/ui";
  import { t } from "../i18n";
  import Avatar from "../components/ui/Avatar.svelte";
  import Button from "../components/ui/Button.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import Input from "../components/ui/Input.svelte";

  let user = {
    username: "admin",
    email: "admin@syncspace.local",
    created: "2025-01-15",
    profileImage: "",
    theme: "system",
    language: "de",
  };

  let showPasswordDialog = false;
  let showImageUploadDialog = false;
  let passwordData = { current: "", new: "", confirm: "" };
  let passwordError = "";
  let imageFile = null;
  let imagePreview = "";

  onMount(() => {
    loadUserProfile();

    // Sync theme and language from stores
    user.theme = $currentTheme || "system";
    user.language = $currentLang;
  });

  async function loadUserProfile() {
    // TODO: Load from backend API
    // const response = await fetch('/api/user/profile');
    // user = await response.json();
  }

  async function handleSaveProfile() {
    // Save theme and language to stores
    currentTheme.set(user.theme);
    currentLang.set(user.language);

    // TODO: Save to backend
    // await fetch('/api/user/profile', {
    //   method: 'PUT',
    //   body: JSON.stringify(user)
    // });

    alert("Profil gespeichert!");
  }

  function handlePasswordChange() {
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

    // TODO: Send to backend
    showPasswordDialog = false;
    passwordData = { current: "", new: "", confirm: "" };
    alert("Passwort erfolgreich geändert!");
  }

  function handleImageSelect(event) {
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
    if (!imageFile) return;

    // TODO: Upload to backend
    // const formData = new FormData();
    // formData.append('image', imageFile);
    // const response = await fetch('/api/user/profile/image', {
    //   method: 'POST',
    //   body: formData
    // });

    user.profileImage = imagePreview;
    showImageUploadDialog = false;
    imageFile = null;
    imagePreview = "";
    alert("Profilbild hochgeladen!");
  }

  function handleRemoveImage() {
    user.profileImage = "";
    // TODO: Delete from backend
  }
</script>

<div class="profile-view">
  <div class="profile-header">
    <h1>👤 {t($currentLang, "profile")}</h1>
  </div>

  <div class="profile-content">
    <!-- Avatar Section -->
    <div class="card avatar-section">
      <h2>Profilbild</h2>
      <div class="avatar-controls">
        <Avatar
          name={user.username}
          imageUrl={user.profileImage}
          size="xlarge"
        />
        <div class="avatar-actions">
          <Button onClick={() => (showImageUploadDialog = true)} icon="📷">
            Bild hochladen
          </Button>
          {#if user.profileImage}
            <Button variant="outlined" onClick={handleRemoveImage} icon="🗑️">
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
          <span class="item-label">Mitglied seit</span>
          <div class="info-value">
            {new Date(user.created).toLocaleDateString("de-DE")}
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
          <select id="theme" bind:value={user.theme} class="md-select">
            <option value="light">☀️ Hell</option>
            <option value="dark">🌙 Dunkel</option>
            <option value="system">💻 System</option>
          </select>
        </div>

        <div class="setting-item">
          <label for="language">Sprache</label>
          <select id="language" bind:value={user.language} class="md-select">
            <option value="de">🇩🇪 Deutsch</option>
            <option value="en">🇬🇧 English</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Save Button -->
    <div class="save-section">
      <Button onClick={handleSaveProfile} icon="💾" size="large" fullWidth>
        Profil speichern
      </Button>
    </div>
  </div>
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
