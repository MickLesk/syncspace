<script>
  import { auth } from "../stores/auth";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n";
  import { onMount } from "svelte";

  let username = "";
  let password = "";
  let loading = false;
  let error = "";
  let mounted = false;

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
</script>

<div class="login-page" class:mounted>
  <div class="login-card">
    <div class="brand">
      <div class="logo">
        <span class="logo-text">S</span>
      </div>
      <h1 class="title">{t($currentLang, "loginTitle")}</h1>
      <p class="subtitle">{t($currentLang, "loginSubtitle")}</p>
    </div>

    <form class="login-form" on:submit={handleLogin}>
      <div class="text-field">
        <label for="username">{t($currentLang, "username")}</label>
        <input
          id="username"
          type="text"
          bind:value={username}
          placeholder={t($currentLang, "enterUsername")}
          required
          disabled={loading}
          autocomplete="username"
        />
      </div>

      <div class="text-field">
        <label for="password">{t($currentLang, "password")}</label>
        <input
          id="password"
          type="password"
          bind:value={password}
          placeholder={t($currentLang, "enterPassword")}
          required
          disabled={loading}
          autocomplete="current-password"
        />
      </div>

      {#if error}
        <div class="error-message">
          <span class="error-icon">⚠️</span>
          <span>{error}</span>
        </div>
      {/if}

      <button type="submit" class="filled-button" disabled={loading}>
        {#if loading}
          <span class="loading-spinner"></span>
          {t($currentLang, "loggingIn")}
        {:else}
          {t($currentLang, "login")}
        {/if}
      </button>
    </form>

    <div class="demo-hint">
      <strong>{t($currentLang, "demoCredentials")}:</strong> admin / admin
    </div>
  </div>

  <!-- Floating orbs animation -->
  <div class="orb orb-1"></div>
  <div class="orb orb-2"></div>
  <div class="orb orb-3"></div>
</div>

<style>
  .login-page {
    min-height: 100vh;
    width: 100vw;
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
    content: "";
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

  .orb {
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

  .brand {
    text-align: center;
    margin-bottom: 40px;
  }

  .logo {
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

  .logo-text {
    font-size: 48px;
    font-weight: 800;
    color: white;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }

  .title {
    font-size: 36px;
    font-weight: 600;
    background: linear-gradient(135deg, #667eea, #764ba2);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 8px;
  }

  .subtitle {
    font-size: 15px;
    color: rgba(0, 0, 0, 0.6);
    font-weight: 500;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .text-field {
    position: relative;
  }

  .text-field label {
    display: block;
    font-size: 13px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.7);
    margin-bottom: 10px;
    letter-spacing: 0.3px;
  }

  .text-field input {
    width: 100%;
    padding: 16px 20px;
    font-size: 16px;
    font-family: inherit;
    color: rgba(0, 0, 0, 0.9);
    background: rgba(255, 255, 255, 0.9);
    border: 2px solid rgba(0, 0, 0, 0.1);
    border-radius: 16px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    outline: none;
  }

  .text-field input:focus {
    border-color: #667eea;
    background: white;
    box-shadow: 0 0 0 4px rgba(102, 126, 234, 0.15);
    transform: translateY(-1px);
  }

  .text-field input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .filled-button {
    width: 100%;
    padding: 18px 24px;
    font-size: 16px;
    font-weight: 600;
    font-family: inherit;
    letter-spacing: 0.5px;
    color: white;
    background: linear-gradient(135deg, #667eea, #764ba2);
    border: none;
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 8px 24px rgba(102, 126, 234, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    position: relative;
    overflow: hidden;
  }

  .filled-button::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.3),
      transparent
    );
    transition: left 0.5s;
  }

  .filled-button:hover:not(:disabled)::before {
    left: 100%;
  }

  .filled-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 12px 32px rgba(102, 126, 234, 0.5);
  }

  .filled-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 16px rgba(102, 126, 234, 0.4);
  }

  .filled-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .loading-spinner {
    width: 18px;
    height: 18px;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 18px;
    background: rgba(220, 53, 69, 0.1);
    border-left: 4px solid #dc3545;
    border-radius: 12px;
    font-size: 14px;
    color: #dc3545;
    font-weight: 500;
    animation: shake 0.4s ease-in-out;
  }

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    25% {
      transform: translateX(-10px);
    }
    75% {
      transform: translateX(10px);
    }
  }

  .error-icon {
    font-size: 20px;
  }

  .demo-hint {
    text-align: center;
    margin-top: 32px;
    padding: 16px;
    background: rgba(102, 126, 234, 0.1);
    border-radius: 12px;
    font-size: 14px;
    color: rgba(0, 0, 0, 0.7);
  }

  .demo-hint strong {
    color: #667eea;
  }
</style>
