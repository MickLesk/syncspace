// ========================
// Material Dialog System
// ========================

class MaterialDialog {
  constructor() {
    this.overlay = null;
    this.dialog = null;
    this.callback = null;
  }

  show(config) {
    return new Promise((resolve) => {
      this.callback = resolve;
      this.createDialog(config);
    });
  }

  createDialog(config) {
    // Create overlay
    this.overlay = document.createElement('div');
    this.overlay.className = 'dialog-overlay';
    this.overlay.onclick = () => this.close(false);

    // Create dialog
    this.dialog = document.createElement('div');
    this.dialog.className = 'dialog-container';
    this.dialog.onclick = (e) => e.stopPropagation();

    // Dialog content
    let content = `
      <div class="dialog-header">
        <h2 class="dialog-title">${config.title}</h2>
      </div>
      <div class="dialog-body">
        ${config.message ? `<p class="dialog-message">${config.message}</p>` : ''}
        ${config.warning ? `<p class="dialog-warning">⚠️ ${config.warning}</p>` : ''}
        ${config.input ? `
          <div class="dialog-input-group">
            <label class="dialog-label">${config.inputLabel}</label>
            <input type="text" id="dialogInput" class="dialog-input" 
                   value="${config.inputValue || ''}" 
                   placeholder="${config.inputPlaceholder || ''}">
          </div>
        ` : ''}
      </div>
      <div class="dialog-actions">
        <button class="dialog-btn dialog-btn-secondary" onclick="window.materialDialog.close(false)">
          ${config.cancelText || t('common.cancel')}
        </button>
        <button class="dialog-btn dialog-btn-primary" onclick="window.materialDialog.confirm()">
          ${config.confirmText || t('common.confirm')}
        </button>
      </div>
    `;

    this.dialog.innerHTML = content;
    this.overlay.appendChild(this.dialog);
    document.body.appendChild(this.overlay);

    // Animate in
    setTimeout(() => {
      this.overlay.classList.add('active');
      this.dialog.classList.add('active');
    }, 10);

    // Focus input if exists
    if (config.input) {
      setTimeout(() => {
        const input = document.getElementById('dialogInput');
        input.focus();
        input.select();
        
        // Submit on Enter
        input.addEventListener('keypress', (e) => {
          if (e.key === 'Enter') {
            this.confirm();
          }
        });
      }, 100);
    }
  }

  confirm() {
    const input = document.getElementById('dialogInput');
    const value = input ? input.value : true;
    this.close(value);
  }

  close(result) {
    if (this.overlay) {
      this.overlay.classList.remove('active');
      this.dialog.classList.remove('active');
      
      setTimeout(() => {
        if (this.overlay && this.overlay.parentNode) {
          this.overlay.parentNode.removeChild(this.overlay);
        }
        this.overlay = null;
        this.dialog = null;
        
        if (this.callback) {
          this.callback(result);
          this.callback = null;
        }
      }, 300);
    }
  }
}

// Global instance
window.materialDialog = new MaterialDialog();

// Helper functions
async function showRenameDialog(currentName) {
  return await window.materialDialog.show({
    title: t('dialog.rename.title'),
    input: true,
    inputLabel: t('dialog.rename.label'),
    inputValue: currentName,
    inputPlaceholder: currentName,
    confirmText: t('dialog.rename.confirm'),
    cancelText: t('dialog.rename.cancel')
  });
}

async function showDeleteDialog(itemName) {
  return await window.materialDialog.show({
    title: t('dialog.delete.title'),
    message: `${t('dialog.delete.message')}: <strong>${itemName}</strong>?`,
    warning: t('dialog.delete.warning'),
    confirmText: t('dialog.delete.confirm'),
    cancelText: t('dialog.delete.cancel')
  });
}

async function showCreateDirDialog() {
  return await window.materialDialog.show({
    title: t('dialog.mkdir.title'),
    input: true,
    inputLabel: t('dialog.mkdir.label'),
    inputPlaceholder: t('files.newdir.placeholder'),
    confirmText: t('dialog.mkdir.confirm'),
    cancelText: t('dialog.mkdir.cancel')
  });
}
