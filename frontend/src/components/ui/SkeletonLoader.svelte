<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  export let count = 6; // Number of skeleton items to show
  export let type = "grid"; // 'grid' or 'list'

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="skeleton-container" class:list={type === "list"}>
  {#each Array(count) as _, i}
    <div class="skeleton-item" class:list-item={type === "list"}>
      <div class="skeleton-icon"></div>
      <div class="skeleton-content">
        <div class="skeleton-title"></div>
        <div class="skeleton-subtitle"></div>
      </div>
    </div>
  {/each}
</div>

<style>
  .skeleton-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 16px;
    padding: 16px;
  }

  .skeleton-container.list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .skeleton-item {
    background: var(--md-sys-color-surface-container);
    border-radius: 16px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    animation: pulse 1.5s ease-in-out infinite;
  }

  .skeleton-item.list-item {
    flex-direction: row;
    padding: 12px 16px;
  }

  .skeleton-icon {
    width: 64px;
    height: 64px;
    border-radius: 12px;
    background: var(--md-sys-color-surface-variant);
    flex-shrink: 0;
  }

  .list-item .skeleton-icon {
    width: 40px;
    height: 40px;
  }

  .skeleton-content {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .skeleton-title {
    height: 16px;
    width: 80%;
    border-radius: 4px;
    background: var(--md-sys-color-surface-variant);
  }

  .skeleton-subtitle {
    height: 12px;
    width: 50%;
    border-radius: 4px;
    background: var(--md-sys-color-surface-variant);
    opacity: 0.7;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  /* Stagger animation for each item */
  .skeleton-item:nth-child(1) {
    animation-delay: 0s;
  }
  .skeleton-item:nth-child(2) {
    animation-delay: 0.1s;
  }
  .skeleton-item:nth-child(3) {
    animation-delay: 0.2s;
  }
  .skeleton-item:nth-child(4) {
    animation-delay: 0.3s;
  }
  .skeleton-item:nth-child(5) {
    animation-delay: 0.4s;
  }
  .skeleton-item:nth-child(6) {
    animation-delay: 0.5s;
  }
</style>
