<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Crystal Glass Stat Card Component
   * @component
   */

  /** @type {string} */
  export let icon = "";

  /** @type {string} */
  export let label = "";

  /** @type {string|number} */
  export let value = "";

  /** @type {string} */
  export let gradient = "linear-gradient(135deg, #6366f1, #8b5cf6)";

  /** @type {number} */
  export let iconSize = 28;

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="stat-card glass-card">
  <div class="stat-icon" style="background: {gradient}">
    <i class="bi bi-{icon}" style="font-size: {iconSize}px;"></i>
  </div>
  <div class="stat-content">
    <div class="stat-value">{value}</div>
    <div class="stat-label">{label}</div>
  </div>
</div>

<style>
  .stat-card {
    padding: 20px;
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .stat-card:hover {
    transform: translateY(-6px) scale(1.02);
  }

  .stat-icon {
    width: 60px;
    height: 60px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.15),
      inset 0 1px 1px rgba(255, 255, 255, 0.2);
    flex-shrink: 0;
  }

  .stat-content {
    flex: 1;
  }

  .stat-value {
    font-size: 28px;
    font-weight: 800;
    color: var(--md-sys-color-on-surface);
    line-height: 1;
    margin-bottom: 6px;
    letter-spacing: -0.02em;
  }

  .stat-label {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  @media (max-width: 768px) {
    .stat-card {
      padding: 16px;
    }

    .stat-icon {
      width: 52px;
      height: 52px;
    }

    .stat-value {
      font-size: 24px;
    }

    .stat-label {
      font-size: 12px;
    }
  }
</style>
