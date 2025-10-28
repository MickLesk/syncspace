<script>
  /**
   * Modern Button Component with Gradient Support
   * Wiederverwendbarer Button für alle Pages
   */
  let {
    children,
    variant = "primary", // primary, secondary, ghost, danger, success, gradient
    size = "md", // sm, md, lg
    icon = "",
    iconPosition = "left", // left, right
    loading = false,
    disabled = false,
    fullWidth = false,
    onclick = null,
    type = "button",
  } = $props();
</script>

<button
  {type}
  class="modern-btn variant-{variant} size-{size}"
  class:full-width={fullWidth}
  class:loading
  disabled={disabled || loading}
  {onclick}
>
  {#if loading}
    <svg
      class="loading-spinner"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
  {:else}
    {#if icon && iconPosition === "left"}
      <i class="bi bi-{icon} btn-icon-left"></i>
    {/if}
    <span class="btn-text">
      {@render children()}
    </span>
    {#if icon && iconPosition === "right"}
      <i class="bi bi-{icon} btn-icon-right"></i>
    {/if}
  {/if}
</button>

<style>
  .modern-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-weight: 600;
    border-radius: 12px;
    border: none;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    position: relative;
    overflow: hidden;
  }

  .modern-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .modern-btn:not(:disabled):hover {
    transform: translateY(-1px);
  }

  .modern-btn:not(:disabled):active {
    transform: translateY(0);
  }

  .full-width {
    width: 100%;
  }

  /* Sizes */
  .size-sm {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }

  .size-md {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
  }

  .size-lg {
    padding: 1rem 2rem;
    font-size: 1.125rem;
  }

  /* Variants */
  .variant-primary {
    background: rgb(59 130 246);
    color: white;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .variant-primary:not(:disabled):hover {
    background: rgb(37 99 235);
    box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
  }

  .variant-secondary {
    background: white;
    color: rgb(55 65 81);
    border: 2px solid rgb(229 231 235);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    .variant-secondary {
      background: rgb(31 41 55);
      color: rgb(209 213 219);
      border-color: rgb(55 65 81);
    }
  }

  .variant-secondary:not(:disabled):hover {
    background: rgb(249 250 251);
    border-color: rgb(209 213 219);
  }

  @media (prefers-color-scheme: dark) {
    .variant-secondary:not(:disabled):hover {
      background: rgb(55 65 81);
      border-color: rgb(75 85 99);
    }
  }

  .variant-ghost {
    background: transparent;
    color: rgb(107 114 128);
  }

  .variant-ghost:not(:disabled):hover {
    background: rgb(243 244 246);
    color: rgb(17 24 39);
  }

  @media (prefers-color-scheme: dark) {
    .variant-ghost {
      color: rgb(156 163 175);
    }

    .variant-ghost:not(:disabled):hover {
      background: rgb(31 41 55);
      color: white;
    }
  }

  .variant-danger {
    background: rgb(239 68 68);
    color: white;
    box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
  }

  .variant-danger:not(:disabled):hover {
    background: rgb(220 38 38);
    box-shadow: 0 6px 16px rgba(239, 68, 68, 0.4);
  }

  .variant-success {
    background: rgb(34 197 94);
    color: white;
    box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
  }

  .variant-success:not(:disabled):hover {
    background: rgb(22 163 74);
    box-shadow: 0 6px 16px rgba(34, 197, 94, 0.4);
  }

  .variant-gradient {
    background: linear-gradient(135deg, rgb(59 130 246), rgb(147 51 234));
    color: white;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .variant-gradient:not(:disabled):hover {
    background: linear-gradient(135deg, rgb(37 99 235), rgb(126 34 206));
    box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
  }

  /* Loading State */
  .loading-spinner {
    width: 1.25em;
    height: 1.25em;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Icon Animations */
  .btn-icon-left {
    transition: transform 0.2s;
  }

  .modern-btn:not(:disabled):hover .btn-icon-left {
    transform: translateX(-2px);
  }

  .btn-icon-right {
    transition: transform 0.2s;
  }

  .modern-btn:not(:disabled):hover .btn-icon-right {
    transform: translateX(2px);
  }
</style>

