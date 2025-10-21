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

<div class="login-container">
  <div class="login-card">
    <div class="login-logo">S</div>
    <h1>SyncSpace</h1>
    <p class="subtitle">Modern File Synchronization</p>
    
    <form on:submit={handleLogin}>
      <div class="form-group">
        <label for="username">Username</label>
        <input 
          id="username" 
          type="text" 
          bind:value={username}
          placeholder="admin" 
          required 
          disabled={loading}
        />
      </div>
      
      <div class="form-group">
        <label for="password">Password</label>
        <input 
          id="password" 
          type="password" 
          bind:value={password}
          placeholder="••••••" 
          required 
          disabled={loading}
        />
      </div>
      
      {#if error}
        <div class="error-box">❌ {error}</div>
      {/if}
      
      <button type="submit" class="login-btn" disabled={loading}>
        {loading ? '⏳ Logging in...' : '🚀 Login'}
      </button>
    </form>
    
    <div class="demo-box">
      <strong>Demo:</strong> admin / admin
    </div>
  </div>
</div>

<style>
  .login-container {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: linear-gradient(135deg, #6750a4 0%, #625b71 100%);
  }
  
  .login-card {
    background: white;
    border-radius: 24px;
    padding: 48px;
    width: 100%;
    max-width: 420px;
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
  }
  
  .login-logo {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6750a4, #625b71);
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto 24px;
    font-size: 32px;
    font-weight: bold;
    color: white;
  }
  
  h1 {
    text-align: center;
    font-size: 28px;
    margin-bottom: 8px;
    color: #202124;
  }
  
  .subtitle {
    text-align: center;
    color: #5f6368;
    margin-bottom: 32px;
    font-size: 14px;
  }
  
  .form-group {
    margin-bottom: 20px;
  }
  
  .form-group label {
    display: block;
    font-weight: 500;
    margin-bottom: 8px;
    color: #202124;
    font-size: 14px;
  }
  
  .form-group input {
    width: 100%;
    padding: 12px;
    border: 1px solid #dadce0;
    border-radius: 8px;
    font-size: 14px;
    transition: all 0.2s;
  }
  
  .form-group input:focus {
    outline: none;
    border-color: #6750a4;
    box-shadow: 0 0 0 3px rgba(103, 80, 164, 0.1);
  }
  
  .form-group input:disabled {
    background: #f1f3f4;
    cursor: not-allowed;
  }
  
  .error-box {
    background: #fce8e6;
    border: 1px solid #b3261e;
    border-radius: 8px;
    padding: 12px;
    margin-bottom: 20px;
    color: #b3261e;
    font-size: 14px;
  }
  
  .login-btn {
    width: 100%;
    padding: 12px;
    background: linear-gradient(135deg, #6750a4, #625b71);
    color: white;
    border: none;
    border-radius: 24px;
    font-weight: 600;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.3s;
  }
  
  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(103, 80, 164, 0.3);
  }
  
  .login-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .demo-box {
    margin-top: 24px;
    padding: 12px;
    background: rgba(103, 80, 164, 0.1);
    border-left: 4px solid #6750a4;
    border-radius: 8px;
    font-size: 13px;
    color: #202124;
  }
</style>
