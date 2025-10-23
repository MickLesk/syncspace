<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import PageHeader from "../components/ui/PageHeader.svelte";
</script>

<div class="view-container">
  <PageHeader 
    title={t($currentLang, "shared")}
    subtitle=""
    icon="share"
    gradient="blue"
  />

  <div class="empty-state">
    <div class="empty-icon">🔗</div>
    <h3>{t($currentLang, "noSharedFiles")}</h3>
    <p>{t($currentLang, "shareWithOthers")}</p>
  </div>
</div>

<style>
  .view-container {
    padding: 0;
    max-width: 1400px;
    margin: 0 auto;
  }

  .empty-state {
    text-align: center;
    padding: 80px 20px;
    margin: 0 32px;
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
