<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    variant = "filled",
    size = "medium",
    disabled = false,
    icon = "",
    fullWidth = false,
    onClick = () => {},
    onclick = undefined,
    children,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<button
  class="crystal-button {variant} {size}"
  class:full-width={fullWidth}
  {disabled}
  type="button"
  onclick={onclick || onClick}
>
  {#if icon}
    <span class="icon">{icon}</span>
  {/if}
  <span class="label">
    {@render children?.()}
  </span>
</button>

<style>
  .crystal-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: none;
    border-radius: 12px;
    font-family: "Inter", sans-serif;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    white-space: nowrap;
    letter-spacing: -0.2px;
  }

  .crystal-button:disabled {
    cursor: not-allowed;
    opacity: 0.4;
  }

  /* Sizes */
  .small {
    height: 32px;
    padding: 0 14px;
    font-size: 13px;
  }

  .medium {
    height: 40px;
    padding: 0 20px;
    font-size: 14px;
  }

  .large {
    height: 48px;
    padding: 0 28px;
    font-size: 15px;
  }

  .full-width {
    width: 100%;
  }

  /* Variants */
  .filled {
    background: var(--md-sys-color-primary);
    color: white;
    box-shadow: 0 2px 8px rgba(99, 102, 241, 0.2);
  }

  .filled:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(99, 102, 241, 0.3);
  }

  .filled:active:not(:disabled) {
    transform: translateY(0);
  }

  .outlined {
    background: transparent;
    border: 1.5px solid var(--md-sys-color-outline);
    color: var(--md-sys-color-primary);
  }

  .outlined:hover:not(:disabled) {
    background: rgba(99, 102, 241, 0.06);
    border-color: var(--md-sys-color-primary);
  }

  .text {
    background: transparent;
    color: var(--md-sys-color-primary);
  }

  .text:hover:not(:disabled) {
    background: rgba(99, 102, 241, 0.08);
  }

  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    color: var(--md-sys-color-on-surface);
    box-shadow: var(--glass-shadow);
  }

  .glass:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.9);
    transform: translateY(-1px);
  }

  .glass-primary {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    color: white;
    box-shadow: 0 4px 16px rgba(99, 102, 241, 0.25);
    border: none;
  }

  .glass-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
  }

  .glass-primary::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.2), transparent);
    opacity: 0;
    transition: opacity 0.2s;
  }

  .glass-primary:hover:not(:disabled)::after {
    opacity: 1;
  }

  .danger {
    background: linear-gradient(135deg, #ef4444, #dc2626);
    color: white;
    box-shadow: 0 4px 16px rgba(239, 68, 68, 0.25);
    border: none;
  }

  .danger:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(239, 68, 68, 0.35);
  }

  /* Dark Mode */
  :global(.dark) .glass {
    background: rgba(30, 41, 59, 0.85);
    border-color: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .glass:hover:not(:disabled) {
    background: rgba(30, 41, 59, 0.95);
  }

  .outlined {
    background: transparent;
    color: var(--md-sys-color-primary);
    border: 1px solid var(--md-sys-color-outline);
  }

  .text {
    background: transparent;
    color: var(--md-sys-color-primary);
    padding: 0 12px;
  }

  .tonal {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
  }

  .full-width {
    width: 100%;
  }

  .icon {
    font-size: 18px;
    line-height: 1;
  }

  .label {
    position: relative;
    z-index: 1;
  }
</style>
