<script>
  import { auth } from "../stores/auth";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n";
  import { onMount } from "svelte";
  import Icon from "../components/ui/Icon.svelte";
  import Button from "../components/ui/Button.svelte";

  let username = "";
  let password = "";
  let loading = false;
  let error = "";
  let mounted = false;
  let showPassword = false;

  onMount(() => {
    setTimeout(() => (mounted = true), 100);
  });

  async function handleLogin(e) {
    e.preventDefault();
    loading = true;
    error = "";

    const result = await auth.login(username, password);

    loading = false;

    if (!result.success) {
      error = result.error;
    }
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }
</script>

<div class="login-page" class:mounted>
  <!-- Animated Background Gradient -->
  <div class="gradient-bg">
    <div class="gradient-orb orb-1"></div>
    <div class="gradient-orb orb-2"></div>
    <div class="gradient-orb orb-3"></div>
  </div>

  <!-- Modern Login Card -->
  <div class="login-container">
    <div class="login-card">
      <!-- Brand Header -->
      <div class="brand-header">
        <div class="logo-container">
          <div class="logo">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M13 2L3 14h8l-1 8 10-12h-8l1-8z" fill="currentColor"/>
            </svg>
          </div>
          <div class="logo-glow"></div>
        </div>
        <h1 class="brand-title">SyncSpace</h1>
        <p class="brand-subtitle">Secure Cloud Storage</p>
      </div>

      <!-- Login Form -->
      <form class="login-form" onsubmit={handleLogin}>
        <!-- Username Field -->
        <div class="form-field">
          <label for="username" class="field-label">
            <Icon name="person" size={16} />
            <span>{t($currentLang, "username")}</span>
          </label>
          <div class="input-wrapper">
            <input
              id="username"
              type="text"
              class="form-input"
              bind:value={username}
              placeholder={t($currentLang, "enterUsername")}
              required
              disabled={loading}
              autocomplete="username"
            />
          </div>
        </div>

        <!-- Password Field -->
        <div class="form-field">
          <label for="password" class="field-label">
            <Icon name="lock" size={16} />
            <span>{t($currentLang, "password")}</span>
          </label>
          <div class="input-wrapper password-wrapper">
            <input
              id="password"
              type={showPassword ? "text" : "password"}
              class="form-input"
              bind:value={password}
              placeholder={t($currentLang, "enterPassword")}
              required
              disabled={loading}
              autocomplete="current-password"
            />
            <button
              type="button"
              class="toggle-password"
              onclick={togglePasswordVisibility}
              title={showPassword ? "Hide password" : "Show password"}
            >
              <Icon name={showPassword ? "eye-slash" : "eye"} size={18} />
            </button>
          </div>
        </div>

        <!-- Error Message -->
        {#if error}
          <div class="error-banner">
            <div class="error-icon">
              <Icon name="exclamation-triangle-fill" size={20} />
            </div>
            <div class="error-content">
              <div class="error-title">Login Failed</div>
              <div class="error-message">{error}</div>
            </div>
          </div>
        {/if}

        <!-- Submit Button -->
        <button type="submit" class="submit-button" disabled={loading}>
          {#if loading}
            <div class="button-spinner"></div>
            <span>Signing In...</span>
          {:else}
            <Icon name="box-arrow-in-right" size={20} />
            <span>{t($currentLang, "login")}</span>
          {/if}
        </button>
      </form>

      <!-- Demo Credentials -->
      <div class="demo-badge">
        <Icon name="info-circle-fill" size={16} />
        <div class="demo-text">
          <strong>Demo:</strong> admin / admin
        </div>
      </div>
    </div>

    <!-- Footer Info -->
    <div class="login-footer">
      <div class="footer-links">
        <Button variant="ghost" size="small">
          <Icon name="shield-check" size={16} />
          <span>Privacy Policy</span>
        </Button>
        <Button variant="ghost" size="small">
          <Icon name="file-text" size={16} />
          <span>Terms of Service</span>
        </Button>
      </div>
      <p class="copyright">© 2025 SyncSpace. All rights reserved.</p>
    </div>
  </div>
</div>

<style>
  /* Main Container */
  .login-page {
    position: fixed;
    inset: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #0f0f17;
    overflow: hidden;
  }

  /* Animated Gradient Background */
  .gradient-bg {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      135deg,
      #6366f1 0%,
      #8b5cf6 25%,
      #d946ef 50%,
      #f97316 75%,
      #6366f1 100%
    );
    background-size: 200% 200%;
    animation: gradientShift 15s ease infinite;
  }

  @keyframes gradientShift {
    0%, 100% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
  }

  .gradient-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(120px);
    opacity: 0.6;
    animation: float 20s ease-in-out infinite;
  }

  .orb-1 {
    width: 600px;
    height: 600px;
    background: radial-gradient(circle, rgba(99, 102, 241, 0.6), transparent);
    top: -200px;
    left: -200px;
    animation-delay: 0s;
  }

  .orb-2 {
    width: 800px;
    height: 800px;
    background: radial-gradient(circle, rgba(217, 70, 239, 0.5), transparent);
    bottom: -300px;
    right: -300px;
    animation-delay: 7s;
  }

  .orb-3 {
    width: 500px;
    height: 500px;
    background: radial-gradient(circle, rgba(139, 92, 246, 0.5), transparent);
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    animation-delay: 14s;
  }

  @keyframes float {
    0%, 100% {
      transform: translate(0, 0) scale(1);
    }
    33% {
      transform: translate(80px, -80px) scale(1.1);
    }
    66% {
      transform: translate(-80px, 80px) scale(0.9);
    }
  }

  /* Login Container */
  .login-container {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 480px;
    padding: 24px;
    opacity: 0;
    transform: translateY(40px) scale(0.95);
    animation: slideIn 0.8s cubic-bezier(0.34, 1.56, 0.64, 1) 0.2s forwards;
  }

  .login-page.mounted .login-container {
    animation-play-state: running;
  }

  @keyframes slideIn {
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Modern Login Card */
  .login-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(40px) saturate(180%);
    -webkit-backdrop-filter: blur(40px) saturate(180%);
    border-radius: 32px;
    padding: 48px;
    box-shadow: 
      0 32px 64px rgba(0, 0, 0, 0.3),
      0 0 0 1px rgba(255, 255, 255, 0.1) inset,
      0 0 100px rgba(99, 102, 241, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.2);
    position: relative;
    overflow: hidden;
  }

  .login-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: linear-gradient(90deg, #6366f1, #8b5cf6, #d946ef, #f97316);
    animation: shimmer 3s linear infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }

  /* Brand Header */
  .brand-header {
    text-align: center;
    margin-bottom: 40px;
  }

  .logo-container {
    position: relative;
    width: 100px;
    height: 100px;
    margin: 0 auto 24px;
  }

  .logo {
    position: relative;
    width: 100%;
    height: 100%;
    border-radius: 24px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 
      0 12px 40px rgba(99, 102, 241, 0.5),
      inset 0 2px 0 rgba(255, 255, 255, 0.3);
    z-index: 2;
    animation: logoFloat 3s ease-in-out infinite;
  }

  .logo svg {
    width: 50px;
    height: 50px;
    filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.2));
  }

  @keyframes logoFloat {
    0%, 100% {
      transform: translateY(0) rotate(0deg);
    }
    50% {
      transform: translateY(-10px) rotate(5deg);
    }
  }

  .logo-glow {
    position: absolute;
    inset: -20px;
    background: radial-gradient(circle, rgba(99, 102, 241, 0.4), transparent 70%);
    filter: blur(30px);
    animation: pulse 2s ease-in-out infinite;
    z-index: 1;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 0.5;
      transform: scale(1);
    }
    50% {
      opacity: 1;
      transform: scale(1.1);
    }
  }

  .brand-title {
    font-size: 32px;
    font-weight: 700;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 8px;
    letter-spacing: -0.02em;
  }

  .brand-subtitle {
    font-size: 14px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.5);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  /* Login Form */
  .login-form {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .field-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.7);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .input-wrapper {
    position: relative;
  }

  .form-input {
    width: 100%;
    height: 56px;
    padding: 16px 20px;
    font-size: 16px;
    font-family: inherit;
    color: rgba(0, 0, 0, 0.9);
    background: rgba(255, 255, 255, 0.8);
    border: 2px solid rgba(0, 0, 0, 0.08);
    border-radius: 16px;
    outline: none;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .form-input:focus {
    background: white;
    border-color: #6366f1;
    box-shadow: 
      0 0 0 4px rgba(99, 102, 241, 0.1),
      0 8px 24px rgba(99, 102, 241, 0.15);
    transform: translateY(-2px);
  }

  .form-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .password-wrapper {
    position: relative;
  }

  .password-wrapper .form-input {
    padding-right: 56px;
  }

  .toggle-password {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 40px;
    height: 40px;
    border: none;
    background: transparent;
    color: rgba(0, 0, 0, 0.4);
    cursor: pointer;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .toggle-password:hover {
    background: rgba(0, 0, 0, 0.05);
    color: rgba(0, 0, 0, 0.7);
  }

  /* Error Banner */
  .error-banner {
    display: flex;
    gap: 14px;
    padding: 16px 18px;
    background: linear-gradient(135deg, rgba(239, 68, 68, 0.1), rgba(220, 38, 38, 0.08));
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-left: 4px solid #ef4444;
    border-radius: 14px;
    animation: slideInError 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes slideInError {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .error-icon {
    color: #ef4444;
    flex-shrink: 0;
  }

  .error-content {
    flex: 1;
  }

  .error-title {
    font-size: 14px;
    font-weight: 600;
    color: #dc2626;
    margin-bottom: 4px;
  }

  .error-message {
    font-size: 13px;
    color: rgba(220, 38, 38, 0.9);
    line-height: 1.5;
  }

  /* Submit Button */
  .submit-button {
    width: 100%;
    height: 56px;
    padding: 0 24px;
    font-size: 16px;
    font-weight: 600;
    font-family: inherit;
    letter-spacing: 0.02em;
    color: white;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    border: none;
    border-radius: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 
      0 8px 24px rgba(99, 102, 241, 0.4),
      inset 0 1px 0 rgba(255, 255, 255, 0.2);
    position: relative;
    overflow: hidden;
  }

  .submit-button::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.2),
      transparent
    );
    transform: translateX(-100%);
    transition: transform 0.6s;
  }

  .submit-button:hover:not(:disabled)::before {
    transform: translateX(100%);
  }

  .submit-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 
      0 12px 32px rgba(99, 102, 241, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
  }

  .submit-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 16px rgba(99, 102, 241, 0.4);
  }

  .submit-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .button-spinner {
    width: 20px;
    height: 20px;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Demo Badge */
  .demo-badge {
    margin-top: 32px;
    padding: 16px 20px;
    background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(139, 92, 246, 0.08));
    border: 1px solid rgba(99, 102, 241, 0.2);
    border-radius: 14px;
    display: flex;
    align-items: center;
    gap: 12px;
    color: #6366f1;
  }

  .demo-text {
    font-size: 14px;
    color: rgba(0, 0, 0, 0.7);
  }

  .demo-text strong {
    font-weight: 600;
    color: #6366f1;
  }

  /* Footer */
  .login-footer {
    margin-top: 32px;
    text-align: center;
  }

  .footer-links {
    display: flex;
    justify-content: center;
    gap: 24px;
    margin-bottom: 16px;
  }

  .copyright {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 500;
  }

  /* Responsive */
  @media (max-width: 640px) {
    .login-container {
      padding: 16px;
    }

    .login-card {
      padding: 32px 24px;
    }

    .brand-title {
      font-size: 28px;
    }

    .footer-links {
      flex-direction: column;
      gap: 8px;
    }
  }

  .login-page {
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
    padding: 24px;
    position: fixed;
    top: 0;
    left: 0;
    overflow: hidden;
  }

  .login-page::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: radial-gradient(
        circle at 20% 50%,
        rgba(102, 126, 234, 0.4),
        transparent 50%
      ),
      radial-gradient(
        circle at 80% 50%,
        rgba(118, 75, 162, 0.4),
        transparent 50%
      ),
      radial-gradient(
        circle at 50% 80%,
        rgba(240, 147, 251, 0.4),
        transparent 50%
      );
    animation: pulse 10s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }

  .gradient-orb {
    position: absolute;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.4), transparent);
    filter: blur(60px);
    animation: float 20s ease-in-out infinite;
  }

  .orb-1 {
    width: 300px;
    height: 300px;
    top: -100px;
    left: -100px;
    animation-delay: 0s;
  }

  .orb-2 {
    width: 400px;
    height: 400px;
    bottom: -150px;
    right: -150px;
    animation-delay: 5s;
  }

  .orb-3 {
    width: 250px;
    height: 250px;
    top: 50%;
    right: 10%;
    animation-delay: 10s;
  }

  @keyframes float {
    0%,
    100% {
      transform: translate(0, 0) scale(1);
    }
    33% {
      transform: translate(50px, -50px) scale(1.1);
    }
    66% {
      transform: translate(-50px, 50px) scale(0.9);
    }
  }

  .login-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 32px;
    padding: 56px 48px;
    width: 100%;
    max-width: 440px;
    box-shadow:
      0 20px 60px rgba(0, 0, 0, 0.3),
      0 0 1px rgba(255, 255, 255, 0.5) inset;
    border: 1px solid rgba(255, 255, 255, 0.3);
    position: relative;
    z-index: 10;
    opacity: 0;
    transform: translateY(30px) scale(0.95);
    animation: slideIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
  }

  .login-page.mounted .login-card {
    animation-play-state: running;
  }

  @keyframes slideIn {
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .brand-header {
    text-align: center;
    margin-bottom: 40px;
  }

  .logo-container {
    width: 96px;
    height: 96px;
    margin: 0 auto 24px;
    border-radius: 28px;
    background: linear-gradient(135deg, #667eea, #764ba2);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 8px 32px rgba(102, 126, 234, 0.4),
      0 0 0 8px rgba(255, 255, 255, 0.8) inset;
    position: relative;
    animation: logoGlow 3s ease-in-out infinite;
  }

  @keyframes logoGlow {
    0%,
    100% {
      box-shadow:
        0 8px 32px rgba(102, 126, 234, 0.4),
        0 0 0 8px rgba(255, 255, 255, 0.8) inset;
    }
    50% {
      box-shadow:
        0 8px 48px rgba(118, 75, 162, 0.6),
        0 0 0 8px rgba(255, 255, 255, 1) inset;
    }
  }

  .logo {
    width: 100%;
    height: 100%;
    color: white;
  }

  .brand-title {
    font-size: 36px;
    font-weight: 600;
    background: linear-gradient(135deg, #667eea, #764ba2);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 8px;
  }

  .brand-subtitle {
    font-size: 15px;
    color: rgba(0, 0, 0, 0.6);
    font-weight: 500;
  }

</style>
