<script>
  import { onMount } from "svelte";
  import { auth } from "../stores/auth";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

  let currentStep = $state(1);
  let totalSteps = 6;

  // Form data
  let formData = $state({
    // Step 1: Admin Account
    admin_username: "",
    admin_password: "",
    admin_password_confirm: "",
    admin_email: "",
    admin_display_name: "",

    // Step 2: Server Info
    server_name: "SyncSpace",
    server_description: "Self-hosted file synchronization",

    // Step 3: Language
    default_language: "en",

    // Step 4: Storage
    default_quota_gb: 10,

    // Step 5: Security & Registration
    allow_registration: false,
    require_email_verification: true,
    enable_2fa_requirement: false,
    password_min_length: 8,
    password_require_uppercase: true,
    password_require_lowercase: true,
    password_require_numbers: true,
    password_require_special: false,
    session_timeout_minutes: 1440, // 24 hours
  });

  let errors = $state({});
  let loading = $state(false);

  const steps = [
    {
      id: 1,
      title: "Admin Account",
      icon: "person-circle",
      description: "Create your administrator account",
    },
    {
      id: 2,
      title: "Server Info",
      icon: "server",
      description: "Configure server settings",
    },
    {
      id: 3,
      title: "Language",
      icon: "translate",
      description: "Choose default language",
    },
    {
      id: 4,
      title: "Storage",
      icon: "hdd-fill",
      description: "Configure storage settings",
    },
    {
      id: 5,
      title: "Security",
      icon: "shield-lock-fill",
      description: "Security and registration settings",
    },
    {
      id: 6,
      title: "Complete",
      icon: "check-circle-fill",
      description: "Finish setup",
    },
  ];

  function validateStep(step) {
    errors = {};

    if (step === 1) {
      if (!formData.admin_username || formData.admin_username.length < 3) {
        errors.admin_username = "Username must be at least 3 characters";
      }
      if (!formData.admin_password || formData.admin_password.length < 8) {
        errors.admin_password = "Password must be at least 8 characters";
      }
      if (formData.admin_password !== formData.admin_password_confirm) {
        errors.admin_password_confirm = "Passwords do not match";
      }
      if (!formData.admin_email || !formData.admin_email.includes("@")) {
        errors.admin_email = "Valid email required";
      }
      if (!formData.admin_display_name) {
        errors.admin_display_name = "Display name required";
      }
    }

    if (step === 2) {
      if (!formData.server_name) {
        errors.server_name = "Server name required";
      }
    }

    if (step === 4) {
      if (formData.default_quota_gb < 1 || formData.default_quota_gb > 1000) {
        errors.default_quota_gb = "Quota must be between 1 and 1000 GB";
      }
    }

    return Object.keys(errors).length === 0;
  }

  function nextStep() {
    if (validateStep(currentStep)) {
      if (currentStep < totalSteps) {
        currentStep++;
      }
    }
  }

  function prevStep() {
    if (currentStep > 1) {
      currentStep--;
    }
  }

  async function completeSetup() {
    if (!validateStep(currentStep)) return;

    loading = true;

    try {
      const response = await fetch("http://localhost:8080/api/setup/complete", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        // Auto-login with new admin credentials
        await auth.login(formData.admin_username, formData.admin_password);
        window.location.href = "/"; // Redirect to main app
      } else {
        const error = await response.text();
        errors.general = error || "Setup failed. Please try again.";
      }
    } catch (e) {
      console.error("Setup error:", e);
      errors.general = "Setup failed. Is the backend running?";
    } finally {
      loading = false;
    }
  }

  const progress = $derived(((currentStep - 1) / (totalSteps - 1)) * 100);
</script>

<div class="setup-wizard">
  <div class="setup-container">
    <!-- Header -->
    <div class="setup-header">
      <div class="logo">
        <i class="bi bi-lightning-charge-fill text-6xl text-primary-600"></i>
      </div>
      <h1
        class="text-4xl font-bold bg-gradient-to-r from-primary-600 to-purple-600 bg-clip-text text-transparent"
      >
        Welcome to SyncSpace
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mt-2">
        Let's get your cloud storage set up in a few easy steps
      </p>
    </div>

    <!-- Progress Bar -->
    <div class="progress-container">
      <div class="progress-bar">
        <div class="progress-fill" style="width: {progress}%"></div>
      </div>
      <div class="progress-steps">
        {#each steps as step}
          <div
            class="progress-step"
            class:active={currentStep === step.id}
            class:completed={currentStep > step.id}
          >
            <div class="step-circle">
              <i class="bi bi-{step.icon}"></i>
            </div>
            <div class="step-label">{step.title}</div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Step Content -->
    <ModernCard variant="glass" class="step-content">
      {#snippet children()}
        <div class="step-inner">
          <!-- Step 1: Admin Account -->
          {#if currentStep === 1}
            <h2 class="step-title">
              <i class="bi bi-person-circle"></i>
              Create Admin Account
            </h2>
            <p class="step-description">
              This will be your primary administrator account
            </p>

            <div class="form-grid">
              <div class="form-group">
                <label for="admin_username">Username *</label>
                <input
                  id="admin_username"
                  type="text"
                  bind:value={formData.admin_username}
                  class="glass-input"
                  placeholder="admin"
                />
                {#if errors.admin_username}
                  <span class="error">{errors.admin_username}</span>
                {/if}
              </div>

              <div class="form-group">
                <label for="admin_display_name">Display Name *</label>
                <input
                  id="admin_display_name"
                  type="text"
                  bind:value={formData.admin_display_name}
                  class="glass-input"
                  placeholder="Administrator"
                />
                {#if errors.admin_display_name}
                  <span class="error">{errors.admin_display_name}</span>
                {/if}
              </div>

              <div class="form-group col-span-2">
                <label for="admin_email">Email *</label>
                <input
                  id="admin_email"
                  type="email"
                  bind:value={formData.admin_email}
                  class="glass-input"
                  placeholder="admin@example.com"
                />
                {#if errors.admin_email}
                  <span class="error">{errors.admin_email}</span>
                {/if}
              </div>

              <div class="form-group">
                <label for="admin_password">Password *</label>
                <input
                  id="admin_password"
                  type="password"
                  bind:value={formData.admin_password}
                  class="glass-input"
                  placeholder="••••••••"
                />
                {#if errors.admin_password}
                  <span class="error">{errors.admin_password}</span>
                {/if}
              </div>

              <div class="form-group">
                <label for="admin_password_confirm">Confirm Password *</label>
                <input
                  id="admin_password_confirm"
                  type="password"
                  bind:value={formData.admin_password_confirm}
                  class="glass-input"
                  placeholder="••••••••"
                />
                {#if errors.admin_password_confirm}
                  <span class="error">{errors.admin_password_confirm}</span>
                {/if}
              </div>
            </div>
          {/if}

          <!-- Step 2: Server Info -->
          {#if currentStep === 2}
            <h2 class="step-title">
              <i class="bi bi-server"></i>
              Server Information
            </h2>
            <p class="step-description">
              Configure your server name and description
            </p>

            <div class="form-grid">
              <div class="form-group col-span-2">
                <label for="server_name">Server Name *</label>
                <input
                  id="server_name"
                  type="text"
                  bind:value={formData.server_name}
                  class="glass-input"
                  placeholder="SyncSpace"
                />
                {#if errors.server_name}
                  <span class="error">{errors.server_name}</span>
                {/if}
              </div>

              <div class="form-group col-span-2">
                <label for="server_description">Description</label>
                <textarea
                  id="server_description"
                  bind:value={formData.server_description}
                  class="glass-input"
                  rows="3"
                  placeholder="Self-hosted file synchronization"
                ></textarea>
              </div>
            </div>
          {/if}

          <!-- Step 3: Language -->
          {#if currentStep === 3}
            <h2 class="step-title">
              <i class="bi bi-translate"></i>
              Default Language
            </h2>
            <p class="step-description">
              Choose the default language for new users
            </p>

            <div class="language-selector">
              <button
                class="language-option"
                class:selected={formData.default_language === "en"}
                onclick={() => (formData.default_language = "en")}
              >
                <span class="flag">🇬🇧</span>
                <span>English</span>
              </button>
              <button
                class="language-option"
                class:selected={formData.default_language === "de"}
                onclick={() => (formData.default_language = "de")}
              >
                <span class="flag">🇩🇪</span>
                <span>Deutsch</span>
              </button>
            </div>
          {/if}

          <!-- Step 4: Storage -->
          {#if currentStep === 4}
            <h2 class="step-title">
              <i class="bi bi-hdd-fill"></i>
              Storage Configuration
            </h2>
            <p class="step-description">
              Set default storage quota for new users
            </p>

            <div class="form-grid">
              <div class="form-group col-span-2">
                <label for="default_quota">Default Quota (GB) *</label>
                <input
                  id="default_quota"
                  type="number"
                  bind:value={formData.default_quota_gb}
                  class="glass-input"
                  min="1"
                  max="1000"
                />
                <small class="text-gray-500">
                  Each new user will receive {formData.default_quota_gb} GB of storage
                </small>
                {#if errors.default_quota_gb}
                  <span class="error">{errors.default_quota_gb}</span>
                {/if}
              </div>
            </div>
          {/if}

          <!-- Step 5: Security & Registration -->
          {#if currentStep === 5}
            <h2 class="step-title">
              <i class="bi bi-shield-lock-fill"></i>
              Security & Registration
            </h2>
            <p class="step-description">
              Configure security policies and user registration
            </p>

            <div class="form-grid">
              <div class="form-group col-span-2">
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    bind:checked={formData.allow_registration}
                  />
                  <span>Allow user self-registration</span>
                </label>
                <small class="text-gray-500">
                  Users can create accounts without admin approval
                </small>
              </div>

              {#if formData.allow_registration}
                <div class="form-group col-span-2">
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      bind:checked={formData.require_email_verification}
                    />
                    <span>Require email verification</span>
                  </label>
                </div>
              {/if}

              <div class="form-group col-span-2">
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    bind:checked={formData.enable_2fa_requirement}
                  />
                  <span>Require 2FA for all users</span>
                </label>
              </div>

              <div class="form-group">
                <label for="password_min_length">Min. Password Length</label>
                <input
                  id="password_min_length"
                  type="number"
                  bind:value={formData.password_min_length}
                  class="glass-input"
                  min="6"
                  max="32"
                />
              </div>

              <div class="form-group">
                <label for="session_timeout">Session Timeout (minutes)</label>
                <input
                  id="session_timeout"
                  type="number"
                  bind:value={formData.session_timeout_minutes}
                  class="glass-input"
                  min="30"
                  max="10080"
                />
              </div>

              <div class="form-group col-span-2">
                <label>Password Requirements</label>
                <div class="checkbox-grid">
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      bind:checked={formData.password_require_uppercase}
                    />
                    <span>Uppercase letters</span>
                  </label>
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      bind:checked={formData.password_require_lowercase}
                    />
                    <span>Lowercase letters</span>
                  </label>
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      bind:checked={formData.password_require_numbers}
                    />
                    <span>Numbers</span>
                  </label>
                  <label class="checkbox-label">
                    <input
                      type="checkbox"
                      bind:checked={formData.password_require_special}
                    />
                    <span>Special characters</span>
                  </label>
                </div>
              </div>
            </div>
          {/if}

          <!-- Step 6: Complete -->
          {#if currentStep === 6}
            <div class="complete-step">
              <div class="success-icon">
                <i class="bi bi-check-circle-fill text-8xl text-green-500"></i>
              </div>
              <h2 class="text-3xl font-bold mb-4">Ready to Go!</h2>
              <p class="text-gray-600 dark:text-gray-400 mb-6">
                Your SyncSpace instance is configured and ready to use.
              </p>
              <div class="summary-card">
                <h3 class="font-bold mb-4">Setup Summary:</h3>
                <ul class="summary-list">
                  <li>
                    <i class="bi bi-person-check"></i>
                    Admin: {formData.admin_username}
                  </li>
                  <li>
                    <i class="bi bi-server"></i>
                    Server: {formData.server_name}
                  </li>
                  <li>
                    <i class="bi bi-translate"></i>
                    Language: {formData.default_language === "en"
                      ? "English"
                      : "Deutsch"}
                  </li>
                  <li>
                    <i class="bi bi-hdd"></i>
                    Default Quota: {formData.default_quota_gb} GB
                  </li>
                  <li>
                    <i class="bi bi-shield"></i>
                    Registration: {formData.allow_registration
                      ? "Enabled"
                      : "Disabled"}
                  </li>
                </ul>
              </div>

              {#if errors.general}
                <div class="error-banner">{errors.general}</div>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Navigation Buttons -->
        <div class="button-group">
          {#if currentStep > 1 && currentStep < totalSteps}
            <ModernButton
              variant="secondary"
              icon="arrow-left"
              onclick={prevStep}
            >
              Back
            </ModernButton>
          {/if}

          <div class="flex-spacer"></div>

          {#if currentStep < totalSteps}
            <ModernButton
              variant="primary"
              icon="arrow-right"
              onclick={nextStep}
            >
              Next
            </ModernButton>
          {:else}
            <ModernButton
              variant="success"
              icon="check-circle"
              onclick={completeSetup}
              disabled={loading}
            >
              {loading ? "Setting up..." : "Complete Setup"}
            </ModernButton>
          {/if}
        </div>
      {/snippet}
    </ModernCard>
  </div>
</div>

<style>
  .setup-wizard {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .setup-container {
    max-width: 900px;
    width: 100%;
  }

  .setup-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .logo {
    margin-bottom: 1rem;
  }

  .progress-container {
    margin-bottom: 2rem;
  }

  .progress-bar {
    height: 8px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 2rem;
  }

  .progress-fill {
    height: 100%;
    background: white;
    transition: width 0.3s ease;
  }

  .progress-steps {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 1rem;
  }

  .progress-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    opacity: 0.5;
    transition: all 0.3s ease;
  }

  .progress-step.active,
  .progress-step.completed {
    opacity: 1;
  }

  .step-circle {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    color: white;
    transition: all 0.3s ease;
  }

  .progress-step.active .step-circle {
    background: white;
    color: #667eea;
    transform: scale(1.1);
  }

  .progress-step.completed .step-circle {
    background: #10b981;
    color: white;
  }

  .step-label {
    font-size: 0.75rem;
    color: white;
    font-weight: 500;
  }

  .step-content {
    min-height: 500px;
  }

  .step-inner {
    padding: 2rem;
  }

  .step-title {
    font-size: 1.75rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .step-description {
    color: #6b7280;
    margin-bottom: 2rem;
  }

  :global(.dark) .step-description {
    color: #9ca3af;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group.col-span-2 {
    grid-column: span 2;
  }

  label {
    font-weight: 600;
    font-size: 0.875rem;
  }

  .glass-input {
    padding: 0.75rem 1rem;
    border: 2px solid rgba(156, 163, 175, 0.3);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    transition: all 0.2s ease;
  }

  .glass-input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .checkbox-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }

  .language-selector {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .language-option {
    padding: 2rem;
    border: 2px solid rgba(156, 163, 175, 0.3);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.05);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .language-option:hover {
    transform: translateY(-2px);
    border-color: #667eea;
  }

  .language-option.selected {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }

  .flag {
    font-size: 3rem;
  }

  .complete-step {
    text-align: center;
  }

  .success-icon {
    margin-bottom: 2rem;
  }

  .summary-card {
    background: rgba(102, 126, 234, 0.05);
    border-radius: 12px;
    padding: 1.5rem;
    text-align: left;
  }

  .summary-list {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .summary-list li {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .error-banner {
    background: #fee2e2;
    color: #991b1b;
    padding: 1rem;
    border-radius: 8px;
    margin-top: 1rem;
  }

  .button-group {
    display: flex;
    gap: 1rem;
    padding: 1.5rem 2rem;
    border-top: 1px solid rgba(156, 163, 175, 0.2);
  }

  .flex-spacer {
    flex: 1;
  }
</style>
