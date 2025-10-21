<script>
  import { auth } from '../stores/auth';
  
  let username = '';
  let password = '';
  let loading = false;
  let error = '';

  async function handleLogin(e) {
    e.preventDefault();
    loading = true;
    error = '';
    
    const result = await auth.login(username, password);
    
    loading = false;
    
    if (!result.success) {
      error = result.error;
    }
  }
</script>

<div class="login-page">
  <div class="login-card">
    <div class="brand">
      <div class="logo">S</div>
      <h1 class="title">SyncSpace</h1>
      <p class="subtitle">Modern File Synchronization</p>
    </div>
    
    <form class="login-form" on:submit={handleLogin}>
      <div class="text-field">
        <label for="username">Username</label>
        <input 
          id="username" 
          type="text" 
          bind:value={username}
          placeholder="Enter username" 
          required 
          disabled={loading}
          autocomplete="username"
        />
      </div>
      
      <div class="text-field">
        <label for="password">Password</label>
        <input 
          id="password" 
          type="password" 
          bind:value={password}
          placeholder="Enter password" 
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
          Logging in...
        {:else}
          Login
        {/if}
      </button>
    </form>
    
    <div class="demo-hint">
      <strong>Demo credentials:</strong> admin / admin
    </div>
  </div>
</div>

<style>
  .login-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, 
      var(--md-sys-color-primary) 0%, 
      var(--md-sys-color-secondary) 100%
    );
    padding: 24px;
  }
  
  .login-card {
    background: var(--md-sys-color-surface);
    border-radius: 28px;
    padding: 48px 40px;
    width: 100%;
    max-width: 400px;
    box-shadow: var(--md-elevation-3);
  }
  
  .brand {
    text-align: center;
    margin-bottom: 40px;
  }
  
  .logo {
    width: 80px;
    height: 80px;
    margin: 0 auto 20px;
    border-radius: 50%;
    background: linear-gradient(135deg, 
      var(--md-sys-color-primary), 
      var(--md-sys-color-secondary)
    );
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 40px;
    font-weight: 700;
    color: var(--md-sys-color-on-primary);
    box-shadow: var(--md-elevation-2);
  }
  
  .title {
    font-size: 32px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 8px;
  }
  
  .subtitle {
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
  }
  
  .login-form {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }
  
  /* Material 3 Text Field */
  .text-field {
    position: relative;
  }
  
  .text-field label {
    display: block;
    font-size: 12px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    margin-bottom: 8px;
    letter-spacing: 0.5px;
  }
  
  .text-field input {
    width: 100%;
    padding: 16px;
    font-size: 16px;
    font-family: inherit;
    color: var(--md-sys-color-on-surface);
    background: var(--md-sys-color-surface-variant);
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 12px;
    transition: all 0.2s ease;
    outline: none;
  }
  
  .text-field input:focus {
    border-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-surface);
    box-shadow: 0 0 0 3px rgba(103, 80, 164, 0.12);
  }
  
  .text-field input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  /* Material 3 Filled Button */
  .filled-button {
    width: 100%;
    padding: 16px 24px;
    font-size: 14px;
    font-weight: 500;
    font-family: inherit;
    letter-spacing: 0.1px;
    color: var(--md-sys-color-on-primary);
    background: var(--md-sys-color-primary);
    border: none;
    border-radius: 100px;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: var(--md-elevation-1);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }
  
  .filled-button:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
    box-shadow: var(--md-elevation-2);
  }
  
  .filled-button:active:not(:disabled) {
    box-shadow: var(--md-elevation-1);
  }
  
  .filled-button:disabled {
    opacity: 0.38;
    cursor: not-allowed;
    box-shadow: none;
  }
  
  .loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--md-sys-color-on-primary);
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .error-message {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: rgba(179, 38, 30, 0.1);
    border-left: 3px solid var(--md-sys-color-error);
    border-radius: 8px;
    font-size: 14px;
    color: var(--md-sys-color-error);
  }
  
  .error-icon {
    font-size: 20px;
  }
  
  .demo-hint {
    margin-top: 24px;
    padding: 16px;
    background: var(--md-sys-color-primary-container);
    border-radius: 12px;
    font-size: 13px;
    text-align: center;
    color: var(--md-sys-color-on-primary-container);
  }
  
  .demo-hint strong {
    display: block;
    margin-bottom: 4px;
  }
</style>
