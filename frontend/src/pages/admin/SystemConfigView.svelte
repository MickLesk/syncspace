<script>
  import { onMount } from "svelte";
  import UIInput from "../../components/ui/UIInput.svelte";
  import UITextarea from "../../components/ui/UITextarea.svelte";
  import UISelect from "../../components/ui/UISelect.svelte";
  import UIToggle from "../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../components/ui/UICheckbox.svelte";
  import UIButton from "../../components/ui/UIButton.svelte";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";

  // State
  let config = $state({
    max_upload_size: 104857600,
    allowed_file_types: ["*"],
    enable_sharing: true,
    enable_versioning: true,
    enable_trash: true,
    trash_retention_days: 30,
    version_retention_count: 10,
  });

  let loading = $state(true);
  let saving = $state(false);
  let error = $state("");
  let successMessage = $state("");
  let activeTab = $state("general");

  // Additional settings
  let smtpSettings = $state({
    host: "",
    port: 587,
    username: "",
    password: "",
    from_email: "",
    from_name: "SyncSpace",
    encryption: "tls",
    enabled: false,
  });

  let securitySettings = $state({
    password_min_length: 8,
    require_uppercase: true,
    require_lowercase: true,
    require_number: true,
    require_special: false,
    session_timeout_minutes: 60,
    max_login_attempts: 5,
    lockout_duration_minutes: 15,
    require_2fa_admin: false,
    require_2fa_all: false,
  });

  // Format bytes to human readable
  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  // Convert MB to bytes
  function mbToBytes(mb) {
    return mb * 1024 * 1024;
  }

  // Convert bytes to MB
  function bytesToMb(bytes) {
    return Math.round(bytes / (1024 * 1024));
  }

  // Load configuration
  async function loadConfig() {
    loading = true;
    error = "";
    try {
      const data = await api.config.get();
      config = { ...config, ...data };
    } catch (e) {
      error = e.message || $t("systemConfig.loadError");
    } finally {
      loading = false;
    }
  }

  // Save configuration
  async function saveConfig() {
    saving = true;
    error = "";
    try {
      await api.config.update(config);
      successMessage = $t("systemConfig.saveSuccess");
      setTimeout(() => (successMessage = ""), 3000);
    } catch (e) {
      error = e.message || $t("systemConfig.saveError");
    } finally {
      saving = false;
    }
  }

  // Reset to defaults
  function resetDefaults() {
    config = {
      max_upload_size: 104857600, // 100MB
      allowed_file_types: ["*"],
      enable_sharing: true,
      enable_versioning: true,
      enable_trash: true,
      trash_retention_days: 30,
      version_retention_count: 10,
    };
  }

  // File type presets
  const fileTypePresets = [
    { label: $t("systemConfig.allFiles"), value: ["*"] },
    {
      label: $t("systemConfig.imagesOnly"),
      value: ["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp"],
    },
    {
      label: $t("systemConfig.documentsOnly"),
      value: ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md"],
    },
    {
      label: $t("systemConfig.mediaOnly"),
      value: [
        "jpg",
        "jpeg",
        "png",
        "gif",
        "mp4",
        "mp3",
        "wav",
        "avi",
        "mkv",
        "webm",
      ],
    },
  ];

  // Upload size presets
  const uploadSizePresets = [
    { label: "10 MB", value: 10 * 1024 * 1024 },
    { label: "50 MB", value: 50 * 1024 * 1024 },
    { label: "100 MB", value: 100 * 1024 * 1024 },
    { label: "500 MB", value: 500 * 1024 * 1024 },
    { label: "1 GB", value: 1024 * 1024 * 1024 },
    { label: "5 GB", value: 5 * 1024 * 1024 * 1024 },
  ];

  // Tabs
  const tabs = [
    { id: "general", label: $t("systemConfig.tabGeneral"), icon: "bi-gear" },
    { id: "storage", label: $t("systemConfig.tabStorage"), icon: "bi-hdd" },
    {
      id: "security",
      label: $t("systemConfig.tabSecurity"),
      icon: "bi-shield-lock",
    },
    { id: "email", label: $t("systemConfig.tabEmail"), icon: "bi-envelope" },
  ];

  onMount(() => {
    loadConfig();
  });
</script>

<div class="p-6 max-w-5xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-3">
      <i class="bi bi-sliders text-2xl text-primary"></i>
      <div>
        <h1 class="text-2xl font-bold">{$t("systemConfig.title")}</h1>
        <p class="text-sm text-base-content/60">
          {$t("systemConfig.subtitle")}
        </p>
      </div>
    </div>
    <div class="flex gap-2">
      <button
        class="btn btn-ghost btn-sm"
        onclick={loadConfig}
        disabled={loading}
      >
        <i class="bi bi-arrow-clockwise" class:animate-spin={loading}></i>
        {$t("common.refresh")}
      </button>
    </div>
  </div>

  <!-- Messages -->
  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Close"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if successMessage}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle"></i>
      <span>{successMessage}</span>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="tabs tabs-boxed mb-6">
    {#each tabs as tab}
      <button
        class="tab gap-2"
        class:tab-active={activeTab === tab.id}
        onclick={() => (activeTab = tab.id)}
      >
        <i class="bi {tab.icon}"></i>
        {tab.label}
      </button>
    {/each}
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else}
    <!-- General Settings -->
    {#if activeTab === "general"}
      <div class="card bg-base-200 shadow-lg">
        <div class="card-body">
          <h2 class="card-title mb-4">
            <i class="bi bi-gear text-primary"></i>
            {$t("systemConfig.generalSettings")}
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Enable Features -->
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={config.enable_sharing}
                />
                <div>
                  <span class="label-text font-medium"
                    >{$t("systemConfig.enableSharing")}</span
                  >
                  <p class="text-xs text-base-content/60">
                    {$t("systemConfig.enableSharingDesc")}
                  </p>
                </div>
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={config.enable_versioning}
                />
                <div>
                  <span class="label-text font-medium"
                    >{$t("systemConfig.enableVersioning")}</span
                  >
                  <p class="text-xs text-base-content/60">
                    {$t("systemConfig.enableVersioningDesc")}
                  </p>
                </div>
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={config.enable_trash}
                />
                <div>
                  <span class="label-text font-medium"
                    >{$t("systemConfig.enableTrash")}</span
                  >
                  <p class="text-xs text-base-content/60">
                    {$t("systemConfig.enableTrashDesc")}
                  </p>
                </div>
              </label>
            </div>
          </div>

          <!-- Retention Settings -->
          <div class="divider"></div>
          <h3 class="font-medium mb-4">
            {$t("systemConfig.retentionSettings")}
          </h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.trashRetention")}</span
                >
              </div>
              <div class="flex gap-2">
                <input
                  type="number"
                  class="input input-bordered flex-1"
                  bind:value={config.trash_retention_days}
                  min="1"
                  max="365"
                  aria-label={$t("systemConfig.trashRetention")}
                />
                <span class="btn btn-ghost pointer-events-none"
                  >{$t("systemConfig.days")}</span
                >
              </div>
              <div class="label">
                <span class="label-text-alt text-base-content/60"
                  >{$t("systemConfig.trashRetentionHint")}</span
                >
              </div>
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.versionRetention")}</span
                >
              </div>
              <div class="flex gap-2">
                <input
                  type="number"
                  class="input input-bordered flex-1"
                  bind:value={config.version_retention_count}
                  min="1"
                  max="100"
                  aria-label={$t("systemConfig.versionRetention")}
                />
                <span class="btn btn-ghost pointer-events-none"
                  >{$t("systemConfig.versions")}</span
                >
              </div>
              <div class="label">
                <span class="label-text-alt text-base-content/60"
                  >{$t("systemConfig.versionRetentionHint")}</span
                >
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Storage Settings -->
    {#if activeTab === "storage"}
      <div class="card bg-base-200 shadow-lg">
        <div class="card-body">
          <h2 class="card-title mb-4">
            <i class="bi bi-hdd text-primary"></i>
            {$t("systemConfig.storageSettings")}
          </h2>

          <!-- Upload Size Limit -->
          <div class="form-control mb-6">
            <div class="label">
              <span class="label-text font-medium"
                >{$t("systemConfig.maxUploadSize")}</span
              >
              <span class="label-text-alt"
                >{formatBytes(config.max_upload_size)}</span
              >
            </div>
            <input
              type="range"
              aria-label={$t("systemConfig.maxUploadSize")}
              class="range range-primary"
              min={1048576}
              max={5368709120}
              step={10485760}
              bind:value={config.max_upload_size}
            />
            <div class="flex flex-wrap gap-2 mt-2">
              {#each uploadSizePresets as preset}
                <button
                  class="btn btn-xs"
                  class:btn-primary={config.max_upload_size === preset.value}
                  onclick={() => (config.max_upload_size = preset.value)}
                >
                  {preset.label}
                </button>
              {/each}
            </div>
          </div>

          <!-- Allowed File Types -->
          <div class="form-control">
            <div class="label">
              <span class="label-text font-medium"
                >{$t("systemConfig.allowedFileTypes")}</span
              >
            </div>
            <textarea
              class="textarea textarea-bordered h-24"
              placeholder="jpg, png, pdf, docx..."
              bind:value={config.allowed_file_types}
              aria-label={$t("systemConfig.allowedFileTypes")}
            ></textarea>
            <div class="label">
              <span class="label-text-alt text-base-content/60"
                >{$t("systemConfig.allowedFileTypesHint")}</span
              >
            </div>
            <div class="flex flex-wrap gap-2 mt-2">
              {#each fileTypePresets as preset}
                <button
                  class="btn btn-xs btn-outline"
                  onclick={() => (config.allowed_file_types = preset.value)}
                >
                  {preset.label}
                </button>
              {/each}
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Security Settings -->
    {#if activeTab === "security"}
      <div class="card bg-base-200 shadow-lg">
        <div class="card-body">
          <h2 class="card-title mb-4">
            <i class="bi bi-shield-lock text-primary"></i>
            {$t("systemConfig.securitySettings")}
          </h2>

          <!-- Password Policy -->
          <h3 class="font-medium mb-4">{$t("systemConfig.passwordPolicy")}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.minPasswordLength")}</span
                >
              </div>
              <input
                type="number"
                class="input input-bordered"
                bind:value={securitySettings.password_min_length}
                min="6"
                max="32"
                aria-label={$t("systemConfig.minPasswordLength")}
              />
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={securitySettings.require_uppercase}
                />
                <span class="label-text"
                  >{$t("systemConfig.requireUppercase")}</span
                >
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={securitySettings.require_lowercase}
                />
                <span class="label-text"
                  >{$t("systemConfig.requireLowercase")}</span
                >
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={securitySettings.require_number}
                />
                <span class="label-text"
                  >{$t("systemConfig.requireNumber")}</span
                >
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  bind:checked={securitySettings.require_special}
                />
                <span class="label-text"
                  >{$t("systemConfig.requireSpecial")}</span
                >
              </label>
            </div>
          </div>

          <!-- Session & Login -->
          <div class="divider"></div>
          <h3 class="font-medium mb-4">{$t("systemConfig.sessionSettings")}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.sessionTimeout")}</span
                >
              </div>
              <div class="flex gap-2">
                <input
                  type="number"
                  class="input input-bordered flex-1"
                  bind:value={securitySettings.session_timeout_minutes}
                  min="5"
                  max="1440"
                  aria-label={$t("systemConfig.sessionTimeout")}
                />
                <span class="btn btn-ghost pointer-events-none"
                  >{$t("systemConfig.minutes")}</span
                >
              </div>
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.maxLoginAttempts")}</span
                >
              </div>
              <input
                type="number"
                class="input input-bordered"
                bind:value={securitySettings.max_login_attempts}
                min="3"
                max="20"
                aria-label={$t("systemConfig.maxLoginAttempts")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text"
                  >{$t("systemConfig.lockoutDuration")}</span
                >
              </div>
              <div class="flex gap-2">
                <input
                  type="number"
                  class="input input-bordered flex-1"
                  bind:value={securitySettings.lockout_duration_minutes}
                  min="1"
                  max="60"
                  aria-label={$t("systemConfig.lockoutDuration")}
                />
                <span class="btn btn-ghost pointer-events-none"
                  >{$t("systemConfig.minutes")}</span
                >
              </div>
            </div>
          </div>

          <!-- 2FA Settings -->
          <div class="divider"></div>
          <h3 class="font-medium mb-4">{$t("systemConfig.twoFactorAuth")}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={securitySettings.require_2fa_admin}
                />
                <div>
                  <span class="label-text font-medium"
                    >{$t("systemConfig.require2faAdmin")}</span
                  >
                  <p class="text-xs text-base-content/60">
                    {$t("systemConfig.require2faAdminDesc")}
                  </p>
                </div>
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-4">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={securitySettings.require_2fa_all}
                />
                <div>
                  <span class="label-text font-medium"
                    >{$t("systemConfig.require2faAll")}</span
                  >
                  <p class="text-xs text-base-content/60">
                    {$t("systemConfig.require2faAllDesc")}
                  </p>
                </div>
              </label>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Email Settings -->
    {#if activeTab === "email"}
      <div class="card bg-base-200 shadow-lg">
        <div class="card-body">
          <h2 class="card-title mb-4">
            <i class="bi bi-envelope text-primary"></i>
            {$t("systemConfig.emailSettings")}
          </h2>

          <div class="form-control mb-6">
            <label class="label cursor-pointer justify-start gap-4">
              <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={smtpSettings.enabled}
              />
              <div>
                <span class="label-text font-medium"
                  >{$t("systemConfig.enableEmail")}</span
                >
                <p class="text-xs text-base-content/60">
                  {$t("systemConfig.enableEmailDesc")}
                </p>
              </div>
            </label>
          </div>

          <div
            class="grid grid-cols-1 md:grid-cols-2 gap-4"
            class:opacity-50={!smtpSettings.enabled}
          >
            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.smtpHost")}</span>
              </div>
              <input
                type="text"
                class="input input-bordered"
                placeholder="smtp.example.com"
                bind:value={smtpSettings.host}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.smtpHost")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.smtpPort")}</span>
              </div>
              <input
                type="number"
                class="input input-bordered"
                bind:value={smtpSettings.port}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.smtpPort")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.smtpUsername")}</span
                >
              </div>
              <input
                type="text"
                class="input input-bordered"
                bind:value={smtpSettings.username}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.smtpUsername")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.smtpPassword")}</span
                >
              </div>
              <input
                type="password"
                class="input input-bordered"
                placeholder="••••••••"
                bind:value={smtpSettings.password}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.smtpPassword")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.fromEmail")}</span>
              </div>
              <input
                type="email"
                class="input input-bordered"
                placeholder="noreply@example.com"
                bind:value={smtpSettings.from_email}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.fromEmail")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.fromName")}</span>
              </div>
              <input
                type="text"
                class="input input-bordered"
                placeholder="SyncSpace"
                bind:value={smtpSettings.from_name}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.fromName")}
              />
            </div>

            <div class="form-control">
              <div class="label">
                <span class="label-text">{$t("systemConfig.encryption")}</span>
              </div>
              <select
                class="select select-bordered"
                bind:value={smtpSettings.encryption}
                disabled={!smtpSettings.enabled}
                aria-label={$t("systemConfig.encryption")}
              >
                <option value="none">{$t("systemConfig.encryptionNone")}</option
                >
                <option value="tls">TLS</option>
                <option value="ssl">SSL</option>
              </select>
            </div>
          </div>

          {#if smtpSettings.enabled}
            <div class="mt-4">
              <button class="btn btn-outline btn-sm">
                <i class="bi bi-send"></i>
                {$t("systemConfig.testEmail")}
              </button>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Action Buttons -->
    <div class="flex justify-between mt-6">
      <button class="btn btn-ghost" onclick={resetDefaults}>
        <i class="bi bi-arrow-counterclockwise"></i>
        {$t("systemConfig.resetDefaults")}
      </button>
      <button class="btn btn-primary" onclick={saveConfig} disabled={saving}>
        {#if saving}
          <span class="loading loading-spinner loading-sm"></span>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
        {$t("common.save")}
      </button>
    </div>
  {/if}
</div>
