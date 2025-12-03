<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Crystal Glass Chart Container Component
   * @component
   */

  let { title = "", icon = "", children } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="chart-container glass-card">
  {#if title}
    <div class="chart-header">
      {#if icon}
        <div class="chart-icon">
          <i class="bi bi-{icon}" aria-hidden="true"></i>
        </div>
      {/if}
      <h3>{title}</h3>
    </div>
  {/if}
  <div class="chart-content">
    {@render children?.()}
  </div>
</div>

<style>
  .chart-container {
    padding: 24px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .chart-container:hover {
    transform: translateY(-4px);
  }

  .chart-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
  }

  .chart-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
  }

  .chart-header h3 {
    font-size: 18px;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .chart-content {
    position: relative;
    min-height: 200px;
  }
</style>
