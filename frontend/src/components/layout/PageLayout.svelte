<script>
  import Breadcrumbs from "../Breadcrumbs.svelte";
  import LoadingState from "../ui/LoadingState.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import { colors, transitions, effects } from "../../lib/DesignSystem.js";

  let {
    title = "",
    subtitle = "",
    icon = "",
    breadcrumbs = [],
    loading = false,
    error = null,
    empty = false,
    emptyProps = {},
    showHeader = true,
    showFooter = false,
    showSidebar = false,
    stickyHeader = true,
    maxWidth = "max-w-7xl",
    class: className = "",
  } = $props();

  const gradient = `linear-gradient(135deg, ${colors.gradient.start}, ${colors.gradient.mid}, ${colors.gradient.end})`;
  const hasSidebar = $derived(() => Boolean(showSidebar));
</script>

<div
  class="min-h-screen bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 flex flex-col {className}"
  style={`--ss-blur:${effects.blur};--ss-radius:${effects.radius};--ss-shadow:${effects.shadow};--ss-transition:${transitions.easing};`}
>
  {#if breadcrumbs?.length}
    <div
      class="w-full border-b border-white/10 bg-white/60 dark:bg-slate-900/60 backdrop-blur"
      aria-label="Breadcrumbs"
    >
      <div class="{maxWidth} mx-auto px-4 sm:px-6 lg:px-8 py-3">
        <Breadcrumbs items={breadcrumbs} />
      </div>
    </div>
  {/if}

  {#if showHeader}
    <header
      class="w-full border-b border-slate-200 dark:border-slate-800/80 bg-white/90 dark:bg-slate-900/80 {stickyHeader
        ? 'sticky top-0 z-30 backdrop-blur'
        : ''}"
    >
      <div class="{maxWidth} mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div class="flex flex-wrap items-center gap-6 justify-between">
          <div class="flex items-center gap-4">
            {#if icon}
              <div
                class="h-14 w-14 rounded-2xl text-white shadow-lg flex items-center justify-center shrink-0"
                style={`background-image:${gradient};`}
              >
                <i class={`bi bi-${icon} text-2xl`}></i>
              </div>
            {/if}
            <div>
              <h1 class="text-2xl sm:text-3xl font-semibold tracking-tight">
                {title}
              </h1>
              {#if subtitle}
                <p class="text-sm text-slate-500 dark:text-slate-400 mt-1">
                  {subtitle}
                </p>
              {/if}
            </div>
          </div>
          <div
            class="flex flex-wrap gap-3 items-center justify-end w-full sm:w-auto"
          >
            <slot name="header-actions" />
          </div>
        </div>
      </div>
    </header>
  {/if}

  <main class="flex-1 w-full px-4 sm:px-6 lg:px-8 py-8">
    <div class="{maxWidth} mx-auto">
      {#if loading}
        <LoadingState />
      {:else if error}
        <section
          class="rounded-2xl border border-red-200 dark:border-red-900/40 bg-red-50/80 dark:bg-red-950/30 p-6 flex gap-4 items-start shadow-sm"
        >
          <div
            class="h-10 w-10 rounded-2xl bg-red-500/20 text-red-600 dark:text-red-300 flex items-center justify-center"
          >
            <i class="bi bi-exclamation-triangle-fill text-lg"></i>
          </div>
          <div>
            <h2 class="text-lg font-semibold mb-1">Something went wrong</h2>
            <p class="text-sm text-red-700 dark:text-red-300">{error}</p>
          </div>
        </section>
      {:else if empty}
        <EmptyState {...emptyProps} />
      {:else}
        <div
          class="grid gap-8 {hasSidebar
            ? 'lg:grid-cols-[minmax(0,1fr),320px]'
            : ''}"
        >
          <div class="space-y-6">
            <slot />
          </div>

          {#if hasSidebar}
            <aside
              class="bg-white/80 dark:bg-slate-900/70 backdrop-blur-[var(--ss-blur)] rounded-3xl border border-white/40 dark:border-slate-800 shadow-[var(--ss-shadow)] p-6 min-h-[200px]"
            >
              <slot name="sidebar" />
            </aside>
          {/if}
        </div>
      {/if}
    </div>
  </main>

  {#if showFooter}
    <footer
      class="w-full border-t border-white/10 bg-white/60 dark:bg-slate-900/60 backdrop-blur py-6 mt-auto"
    >
      <div class="{maxWidth} mx-auto px-4 sm:px-6 lg:px-8">
        <slot name="footer" />
      </div>
    </footer>
  {/if}
</div>
