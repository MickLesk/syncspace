<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Page Header Component v2 - Tailwind v4
   * Unified header for all pages with dark/light mode
   */
  let {
    title = "",
    subtitle = "",
    icon = "",
    children = null,
    actions = null,
    stats = null,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="mb-8 animate-slide-up">
  <div class="flex items-start justify-between gap-6 flex-wrap">
    <!-- Left: Title & Subtitle -->
    <div class="flex-1 min-w-0">
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-white flex items-center gap-3"
      >
        {#if icon}
          <i class="bi bi-{icon} text-primary-500 dark:text-primary-400"></i>
        {/if}
        {title}
      </h1>
      {#if subtitle}
        <p class="mt-2 text-base text-gray-600 dark:text-gray-400">
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
