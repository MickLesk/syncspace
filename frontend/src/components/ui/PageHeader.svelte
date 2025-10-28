<script>
  /**
   * Page Header Component v2 - Tailwind v4
   * Unified header for all pages with dark/light mode
   */
  let {
    title = "",
    subtitle = "",
    icon = "",
    children,
    actions,
    stats,
  } = $props();
</script>

<div class="page-header-modern mb-8 animate-slide-up">
  <div class="flex items-start justify-between gap-6 flex-wrap">
    <!-- Left: Title & Subtitle -->
    <div class="flex-1 min-w-0">
      <h1 class="page-title-modern">
        {#if icon}
          <i class="bi bi-{icon}"></i>
        {/if}
        {title}
      </h1>
      {#if subtitle}
        <p class="page-subtitle-modern mt-2">
          {subtitle}
        </p>
      {/if}
      {#if stats}
        <div class="mt-4">
          {@render stats()}
        </div>
      {/if}
    </div>

    <!-- Right: Actions -->
    {#if actions}
      <div class="flex items-center gap-2 flex-wrap">
        {@render actions()}
      </div>
    {/if}
  </div>

  <!-- Additional Content -->
  {#if children}
    <div class="mt-6">
      {@render children()}
    </div>
  {/if}
</div>
