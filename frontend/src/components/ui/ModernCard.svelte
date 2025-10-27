<script>
  /**
   * Modern Glassmorphism Card Component
   * Wiederverwendbare Card mit verschiedenen Styles
   */
  let {
    children,
    title = "",
    subtitle = "",
    icon = "",
    variant = "default", // default, glass, gradient, elevated
    padding = "normal", // none, small, normal, large
    hoverable = false,
    clickable = false,
    onclick = null,
  } = $props();

  const paddingClasses = {
    none: "",
    small: "p-4",
    normal: "p-6",
    large: "p-8",
  };
</script>

<div
  class="modern-card variant-{variant} {paddingClasses[padding]}"
  class:hoverable
  class:clickable
  role={clickable ? "button" : undefined}
  tabindex={clickable ? 0 : undefined}
  {onclick}
  onkeydown={(e) => {
    if (clickable && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      onclick?.(e);
    }
  }}
>
  {#if title || subtitle || icon}
    <div class="card-header">
      {#if icon}
        <div class="card-icon">
          <i class="bi bi-{icon}"></i>
        </div>
      {/if}
      <div class="card-header-text">
        {#if title}
          <h3 class="card-title">{title}</h3>
        {/if}
        {#if subtitle}
          <p class="card-subtitle">{subtitle}</p>
        {/if}
      </div>
    </div>
  {/if}

  <div class="card-content">
    {@render children()}
  </div>
</div>

<style>
  .modern-card {
    position: relative;
    background: white;
    border-radius: 16px;
    border: 1px solid rgb(229 231 235);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @media (prefers-color-scheme: dark) {
    .modern-card {
      background: rgb(17 24 39);
      border-color: rgb(55 65 81);
    }
  }

  /* Variants */
  .variant-glass {
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(12px) saturate(150%);
    border: 1px solid rgba(255, 255, 255, 0.3);
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.05),
      0 0 0 1px rgba(255, 255, 255, 0.1) inset;
  }

  @media (prefers-color-scheme: dark) {
    .variant-glass {
      background: rgba(17, 24, 39, 0.7);
      border-color: rgba(55, 65, 81, 0.5);
    }
  }

  .variant-gradient {
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.05),
      rgba(147, 51, 234, 0.05)
    );
    border: 2px solid transparent;
    background-clip: padding-box;
  }

  .variant-elevated {
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  @media (prefers-color-scheme: dark) {
    .variant-elevated {
      box-shadow:
        0 4px 6px -1px rgba(0, 0, 0, 0.3),
        0 2px 4px -1px rgba(0, 0, 0, 0.2);
    }
  }

  /* States */
  .hoverable:hover {
    transform: translateY(-2px);
    box-shadow:
      0 10px 20px -5px rgba(0, 0, 0, 0.1),
      0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  @media (prefers-color-scheme: dark) {
    .hoverable:hover {
      box-shadow:
        0 10px 20px -5px rgba(0, 0, 0, 0.4),
        0 4px 6px -2px rgba(0, 0, 0, 0.3);
    }
  }

  .clickable {
    cursor: pointer;
  }

  .clickable:active {
    transform: scale(0.98);
  }

  /* Header */
  .card-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .card-icon {
    width: 48px;
    height: 48px;
    background: linear-gradient(135deg, rgb(59 130 246), rgb(147 51 234));
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    color: white;
    flex-shrink: 0;
  }

  .card-header-text {
    flex: 1;
    min-width: 0;
  }

  .card-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: rgb(17 24 39);
    margin: 0;
    line-height: 1.4;
  }

  @media (prefers-color-scheme: dark) {
    .card-title {
      color: white;
    }
  }

  .card-subtitle {
    font-size: 0.875rem;
    color: rgb(107 114 128);
    margin: 0.25rem 0 0 0;
  }

  @media (prefers-color-scheme: dark) {
    .card-subtitle {
      color: rgb(156 163 175);
    }
  }

  .card-content {
    color: rgb(55 65 81);
  }

  @media (prefers-color-scheme: dark) {
    .card-content {
      color: rgb(209 213 219);
    }
  }

  /* Responsive */
  @media (max-width: 640px) {
    .modern-card {
      border-radius: 12px;
    }

    .card-icon {
      width: 40px;
      height: 40px;
      font-size: 1.25rem;
    }

    .card-title {
      font-size: 1.125rem;
    }
  }
</style>
