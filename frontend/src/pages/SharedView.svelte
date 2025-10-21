<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
</script>

<div class="view-container">
  <div class="page-header">
    <h2>🔗 {t($currentLang, "shared")}</h2>
  </div>

  <div class="empty-state">
    <div class="empty-icon">🔗</div>
    <h3>{t($currentLang, "noSharedFiles")}</h3>
    <p>{t($currentLang, "shareWithOthers")}</p>
  </div>
</div>

<style>
  .view-container {
    padding: 24px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 32px;
  }

  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
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
