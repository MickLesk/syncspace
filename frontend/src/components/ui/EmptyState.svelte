<script>
  /**
   * EmptyState Component
   * Wiederverwendbarer Component für leere Views
   *
   * @component
   * @example
   * <EmptyState
   *   icon="star-fill"
   *   title="No Favorites"
   *   description="Mark files as favorite to see them here"
   *   actionText="Browse Files"
   *   onAction={() => navigateToFiles()}
   * />
   */

  import Icon from "./Icon.svelte";
  import Button from "./Button.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  /** @type {string} - Bootstrap Icon name oder Emoji */
  export let icon = "";

  /** @type {boolean} - Ob Icon ein Bootstrap-Icon ist (true) oder Emoji (false) */
  export let isBootstrapIcon = false;

  /** @type {string} - Hauptüberschrift */
  export let title = tr("noItemsFound");

  /** @type {string} - Beschreibungstext */
  export let description = "";

  /** @type {string} - Text für Action-Button (optional) */
  export let actionText = "";

  /** @type {Function} - Callback für Action-Button */
  export let onAction = null;

  /** @type {"small" | "medium" | "large"} - Größe des EmptyState */
  export let size = "medium";
</script>

<div
  class="empty-state"
  class:size-small={size === "small"}
  class:size-large={size === "large"}
>
  {#if icon}
    <div class="empty-icon">
      {#if isBootstrapIcon}
        <Icon
          name={icon}
          size={size === "small" ? 48 : size === "large" ? 96 : 72}
        />
      {:else}
        <span class="emoji-icon">{icon}</span>
      {/if}
    </div>
  {/if}

  <h3 class="empty-title">{title}</h3>

  {#if description}
    <p class="empty-description">{description}</p>
  {/if}

  <slot name="content" />

  {#if actionText && onAction}
    <div class="empty-action">
      <Button onClick={onAction} variant="outlined" size="medium">
        {actionText}
      </Button>
    </div>
  {/if}

  <slot name="actions" />
</div>

<style>
  .empty-state {
    text-align: center;
    padding: 80px 32px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .empty-state.size-small {
    padding: 40px 24px;
    gap: 12px;
  }

  .empty-state.size-large {
    padding: 120px 40px;
    gap: 24px;
  }

  .empty-icon {
    margin-bottom: 8px;
    opacity: 0.5;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .empty-state:hover .empty-icon {
    opacity: 0.7;
    transform: scale(1.05);
  }

  .emoji-icon {
    font-size: 72px;
    display: inline-block;
  }

  .size-small .emoji-icon {
    font-size: 48px;
  }

  .size-large .emoji-icon {
    font-size: 96px;
  }

  .empty-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin: 0;
    font-family:
      "Inter",
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;
  }

  .size-small .empty-title {
    font-size: 20px;
  }

  .size-large .empty-title {
    font-size: 28px;
  }

  .empty-description {
    font-size: 16px;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
    max-width: 500px;
    line-height: 1.5;
  }

  .size-small .empty-description {
    font-size: 14px;
    max-width: 400px;
  }

  .size-large .empty-description {
    font-size: 18px;
    max-width: 600px;
  }

  .empty-action {
    margin-top: 8px;
  }

  /* Dark Mode */
  :global([data-theme="dark"]) .empty-icon {
    opacity: 0.4;
  }

  :global([data-theme="dark"]) .empty-state:hover .empty-icon {
    opacity: 0.6;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .empty-state {
      padding: 60px 20px;
    }

    .empty-title {
      font-size: 20px;
    }

    .empty-description {
      font-size: 14px;
    }

    .emoji-icon {
      font-size: 56px;
    }
  }
</style>
