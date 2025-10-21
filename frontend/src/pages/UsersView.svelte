<script>
  // Users management view
</script>

<div class="view-container">
  <div class="page-header">
    <h2>👥 Users</h2>
    <button class="add-user-btn">➕ Add User</button>
  </div>
  
  <div class="users-table">
    <div class="table-header">
      <div class="col-username">Username</div>
      <div class="col-created">Created</div>
      <div class="col-2fa">2FA</div>
      <div class="col-actions">Actions</div>
    </div>
    
    <div class="table-row">
      <div class="col-username">
        <div class="user-avatar">A</div>
        <span>admin</span>
      </div>
      <div class="col-created">Today</div>
      <div class="col-2fa">
        <span class="badge disabled">Disabled</span>
      </div>
      <div class="col-actions">
        <button class="icon-btn">🔑</button>
        <button class="icon-btn">✏️</button>
      </div>
    </div>
  </div>
</div>

<style>
  .view-container {
    padding: 24px;
    max-width: 1400px;
    margin: 0 auto;
  }
  
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }
  
  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }
  
  .add-user-btn {
    padding: 10px 20px;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    border: none;
    border-radius: 20px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .add-user-btn:hover {
    box-shadow: var(--md-elevation-2);
  }
  
  .users-table {
    background: var(--md-sys-color-surface);
    border-radius: 16px;
    overflow: hidden;
    box-shadow: var(--md-elevation-1);
  }
  
  .table-header, .table-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr;
    gap: 16px;
    padding: 16px;
    align-items: center;
  }
  
  .table-header {
    background: var(--md-sys-color-surface-variant);
    font-weight: 600;
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }
  
  .table-row {
    border-bottom: 1px solid var(--md-sys-color-outline);
  }
  
  .table-row:last-child {
    border-bottom: none;
  }
  
  .col-username {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .user-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
  }
  
  .badge {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
  }
  
  .badge.disabled {
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface-variant);
  }
  
  .col-actions {
    display: flex;
    gap: 8px;
  }
  
  .icon-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: none;
    background: var(--md-sys-color-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }
  
  .icon-btn:hover {
    background: var(--md-sys-color-secondary-container);
  }
</style>
