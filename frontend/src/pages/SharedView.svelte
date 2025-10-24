<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import EmptyState from "../components/ui/EmptyState.svelte";
</script>

<div class="view-container">
  <div class="page-content">
    <EmptyState
      icon="🔗"
      title={t($currentLang, "noSharedFiles")}
      description={t($currentLang, "shareWithOthers")}
    />
  </div>
</div>

<style>
  .view-container {
    padding: 0;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-content {
    padding: 0 32px;
  }
</style>
