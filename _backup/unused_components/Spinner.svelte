<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    size = "medium", // small, medium, large
    variant = "circular", // circular, dots, bars
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="spinner-wrapper {size}">
  {#if variant === "circular"}
    <div class="spinner-circular"></div>
  {:else if variant === "dots"}
    <div class="spinner-dots">
      <span></span>
      <span></span>
      <span></span>
    </div>
  {:else if variant === "bars"}
    <div class="spinner-bars">
      <span></span>
      <span></span>
      <span></span>
      <span></span>
    </div>
  {/if}
</div>

<style>
  .spinner-wrapper {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  /* Circular Spinner */
  .spinner-circular {
    border: 3px solid rgba(99, 102, 241, 0.15);
    border-top-color: #6366f1;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Dots Spinner */
  .spinner-dots {
    display: flex;
    gap: 6px;
  }

  .spinner-dots span {
    border-radius: 50%;
    background: #6366f1;
    animation: bounce 1.4s infinite ease-in-out both;
  }

  .spinner-dots span:nth-child(1) {
    animation-delay: -0.32s;
  }

  .spinner-dots span:nth-child(2) {
    animation-delay: -0.16s;
  }

  @keyframes bounce {
    0%,
    80%,
    100% {
      transform: scale(0);
      opacity: 0.5;
    }
    40% {
      transform: scale(1);
      opacity: 1;
    }
  }

  /* Bars Spinner */
  .spinner-bars {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .spinner-bars span {
    background: #6366f1;
    animation: stretch 1.2s infinite ease-in-out;
  }

  .spinner-bars span:nth-child(1) {
    animation-delay: -0.45s;
  }

  .spinner-bars span:nth-child(2) {
    animation-delay: -0.3s;
  }

  .spinner-bars span:nth-child(3) {
    animation-delay: -0.15s;
  }

  @keyframes stretch {
    0%,
    40%,
    100% {
      transform: scaleY(0.4);
    }
    20% {
      transform: scaleY(1);
    }
  }

  /* Sizes */
  .small .spinner-circular {
    width: 20px;
    height: 20px;
    border-width: 2px;
  }

  .small .spinner-dots span {
    width: 6px;
    height: 6px;
  }

  .small .spinner-bars span {
    width: 3px;
    height: 16px;
    border-radius: 2px;
  }

  .medium .spinner-circular {
    width: 32px;
    height: 32px;
  }

  .medium .spinner-dots span {
    width: 10px;
    height: 10px;
  }

  .medium .spinner-bars span {
    width: 4px;
    height: 24px;
    border-radius: 2px;
  }

  .large .spinner-circular {
    width: 48px;
    height: 48px;
    border-width: 4px;
  }

  .large .spinner-dots span {
    width: 14px;
    height: 14px;
  }

  .large .spinner-bars span {
    width: 6px;
    height: 36px;
    border-radius: 3px;
  }

  /* Dark Mode */
  :global(.dark) .spinner-circular {
    border-color: rgba(129, 140, 248, 0.2);
    border-top-color: #818cf8;
  }

  :global(.dark) .spinner-dots span,
  :global(.dark) .spinner-bars span {
    background: #818cf8;
  }
</style>
