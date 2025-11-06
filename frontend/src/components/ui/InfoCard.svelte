<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    title = "",
    description = "",
    variant = "default", // default, bordered, glass, gradient
    hoverable = false,
    clickable = false,
    padding = "medium", // small, medium, large
    onClick = () => {},
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

{#if clickable}
  <button
    class="info-card {variant} {padding}"
    class:hoverable
    onclick={onClick}
    type="button"
  >
    {#if title}
      <h3 class="card-title">{title}</h3>
    {/if}

    {#if description}
      <p class="card-description">{description}</p>
    {/if}

    <div class="card-content">
      <slot />
    </div>
  </button>
{:else}
  <div class="info-card {variant} {padding}" class:hoverable>
    {#if title}
      <h3 class="card-title">{title}</h3>
    {/if}

    {#if description}
      <p class="card-description">{description}</p>
    {/if}

    <div class="card-content">
      <slot />
    </div>
  </div>
{/if}

<style>
  .info-card {
    border-radius: 16px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    width: 100%;
    text-align: left;
  }

  button.info-card {
    border: none;
    cursor: pointer;
    font-family: inherit;
  }

  .card-title {
    font-family: "Inter", sans-serif;
    font-size: 18px;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 8px 0;
    letter-spacing: -0.3px;
  }

  .card-description {
    font-family: "Inter", sans-serif;
    font-size: 13px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0 0 16px 0;
    line-height: 1.5;
  }

  .card-content {
    font-size: 14px;
    line-height: 1.6;
  }

  /* Padding */
  .small {
    padding: 12px;
  }

  .medium {
    padding: 20px;
  }

  .large {
    padding: 28px;
  }

  /* Variants */
  .default {
    background: rgba(15, 23, 42, 0.04);
    border: 1px solid rgba(15, 23, 42, 0.08);
  }

  .bordered {
    background: var(--md-sys-color-surface);
    border: 2px solid rgba(99, 102, 241, 0.2);
  }

  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
  }

  .gradient {
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.08),
      rgba(139, 92, 246, 0.08)
    );
    border: 1px solid rgba(99, 102, 241, 0.15);
    box-shadow: 0 4px 16px rgba(99, 102, 241, 0.08);
  }

  /* States */
  .hoverable:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
  }

  button.info-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }

  button.info-card:active {
    transform: translateY(0);
  }

  /* Dark Mode */
  :global(.dark) .default {
    background: rgba(255, 255, 255, 0.04);
    border-color: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .bordered {
    background: var(--md-sys-color-surface);
    border-color: rgba(129, 140, 248, 0.3);
  }

  :global(.dark) .glass {
    background: rgba(30, 41, 59, 0.85);
    border-color: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .gradient {
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.12),
      rgba(139, 92, 246, 0.12)
    );
    border-color: rgba(99, 102, 241, 0.2);
  }
</style>
