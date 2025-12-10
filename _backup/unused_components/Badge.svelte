<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    variant = "default", // default, success, warning, error, info, glass
    size = "medium", // small, medium, large
    icon = "",
    children,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<span class="crystal-badge {variant} {size}">
  {#if icon}
    <i class="bi {icon}"></i>
  {/if}
  {@render children?.()}
</span>

<style>
  .crystal-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    border-radius: 8px;
    font-family: "Inter", sans-serif;
    font-weight: 600;
    white-space: nowrap;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  /* Sizes */
  .small {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 6px;
  }

  .small i {
    font-size: 10px;
  }

  .medium {
    font-size: 11px;
  }

  .medium i {
    font-size: 12px;
  }

  .large {
    font-size: 13px;
    padding: 6px 12px;
    border-radius: 10px;
  }

  .large i {
    font-size: 14px;
  }

  /* Variants */
  .default {
    background: rgba(15, 23, 42, 0.08);
    color: var(--md-sys-color-on-surface);
  }

  .success {
    background: linear-gradient(135deg, #10b981, #14b8a6);
    color: white;
    box-shadow: 0 2px 8px rgba(16, 185, 129, 0.2);
  }

  .warning {
    background: linear-gradient(135deg, #f59e0b, #f97316);
    color: white;
    box-shadow: 0 2px 8px rgba(245, 158, 11, 0.2);
  }

  .error {
    background: linear-gradient(135deg, #ef4444, #dc2626);
    color: white;
    box-shadow: 0 2px 8px rgba(239, 68, 68, 0.2);
  }

  .info {
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
  }

  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    color: var(--md-sys-color-on-surface);
    box-shadow: var(--glass-shadow);
  }

  /* Dark Mode */
  :global(.dark) .default {
    background: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .glass {
    background: rgba(30, 41, 59, 0.85);
    border-color: rgba(255, 255, 255, 0.08);
  }
</style>
