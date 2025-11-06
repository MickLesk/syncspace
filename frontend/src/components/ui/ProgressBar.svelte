<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    value = 0,
    max = 100,
    variant = "primary", // primary, success, warning, error, glass
    size = "medium", // small, medium, large
    showLabel = true,
    animated = true,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  const percentage = $derived(Math.min((value / max) * 100, 100));
</script>

<div class="progress-wrapper {size}">
  {#if showLabel}
    <div class="progress-labels">
      <span class="label">{value} / {max}</span>
      <span class="percentage">{Math.round(percentage)}%</span>
    </div>
  {/if}

  <div class="progress-track">
    <div
      class="progress-fill {variant}"
      class:animated
      style="width: {percentage}%"
    ></div>
  </div>
</div>

<style>
  .progress-wrapper {
    width: 100%;
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .label {
    font-family: "Inter", sans-serif;
    font-size: 13px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    letter-spacing: -0.2px;
  }

  .percentage {
    font-family: "Inter", sans-serif;
    font-size: 12px;
    font-weight: 700;
    color: var(--md-sys-color-on-surface-variant);
  }

  .progress-track {
    width: 100%;
    background: rgba(15, 23, 42, 0.08);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .progress-fill {
    height: 100%;
    border-radius: 8px;
    transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }

  .progress-fill.animated::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.3),
      transparent
    );
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(100%);
    }
  }

  /* Sizes */
  .small .progress-track {
    height: 6px;
  }

  .small .label {
    font-size: 12px;
  }

  .small .percentage {
    font-size: 11px;
  }

  .medium .progress-track {
    height: 8px;
  }

  .large .progress-track {
    height: 12px;
  }

  .large .label {
    font-size: 14px;
  }

  .large .percentage {
    font-size: 13px;
  }

  /* Variants */
  .primary {
    background: linear-gradient(90deg, #6366f1, #8b5cf6);
    box-shadow: 0 0 12px rgba(99, 102, 241, 0.4);
  }

  .success {
    background: linear-gradient(90deg, #10b981, #14b8a6);
    box-shadow: 0 0 12px rgba(16, 185, 129, 0.4);
  }

  .warning {
    background: linear-gradient(90deg, #f59e0b, #f97316);
    box-shadow: 0 0 12px rgba(245, 158, 11, 0.4);
  }

  .error {
    background: linear-gradient(90deg, #ef4444, #dc2626);
    box-shadow: 0 0 12px rgba(239, 68, 68, 0.4);
  }

  .glass {
    background: rgba(99, 102, 241, 0.5);
    backdrop-filter: blur(8px);
    box-shadow: 0 0 12px rgba(99, 102, 241, 0.3);
  }

  /* Dark Mode */
  :global(.dark) .progress-track {
    background: rgba(255, 255, 255, 0.08);
  }
</style>
