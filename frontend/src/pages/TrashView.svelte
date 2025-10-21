<script>
  // Trash view
</script>

<div class="view-container">
  <div class="page-header">
    <h2>🗑️ Trash</h2>
    <button class="empty-trash-btn" disabled>Empty Trash</button>
  </div>
  
  <div class="empty-state">
    <div class="empty-icon">🗑️</div>
    <h3>Trash is empty</h3>
    <p>Deleted files will be stored here for 30 days</p>
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
  
  .empty-trash-btn {
    padding: 10px 20px;
    background: var(--md-sys-color-error);
    color: var(--md-sys-color-on-primary);
    border: none;
    border-radius: 20px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .empty-trash-btn:disabled {
    opacity: 0.38;
    cursor: not-allowed;
  }
  
  .empty-state {
    text-align: center;
    padding: 80px 20px;
  }
  
  .empty-icon {
    font-size: 80px;
    margin-bottom: 24px;
    opacity: 0.5;
  }
  
  h3 {
    font-size: 24px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 8px;
  }
  
  p {
    font-size: 16px;
    color: var(--md-sys-color-on-surface-variant);
  }
</style>
