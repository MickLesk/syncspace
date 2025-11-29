<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import Breadcrumbs from "./Breadcrumbs.svelte";
  import LoadingState from "./ui/LoadingState.svelte";
  import EmptyState from "./ui/EmptyState.svelte";

  /**
   * PageLayout - Standard page wrapper for consistent layout
   * @prop {string} title - Page title
   * @prop {string} icon - Bootstrap icon name (without bi- prefix)
   * @prop {Array} breadcrumbs - Breadcrumb items [{label, path}]
   * @prop {boolean} loading - Show loading state
   * @prop {boolean} empty - Show empty state
   * @prop {string} emptyIcon - Icon for empty state
   * @prop {string} emptyTitle - Title for empty state
   * @prop {string} emptyMessage - Message for empty state
   * @prop {string} maxWidth - Max width class (default: max-w-7xl)
   * @prop {boolean} noPadding - Remove default padding
   * @prop {boolean} showHeader - Show page header (default: true)
   */

  let {
    title = "",
    icon = "",
    breadcrumbs = [],
    loading = false,
    empty = false,
    emptyIcon = "inbox",
    emptyTitle = "No data",
    emptyMessage = "There's nothing here yet",
    maxWidth = "max-w-7xl",
    noPadding = false,
    showHeader = true,
    class: className = "",
    children,
    headerActions,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="page-layout {className}">
  <!-- Breadcrumbs -->
  {#if breadcrumbs.length > 0}
    <div class="page-breadcrumbs">
      <div class="{maxWidth} mx-auto px-4 sm:px-6 lg:px-8">
        <Breadcrumbs items={breadcrumbs} />
      </div>
    </div>
  {/if}

  <!-- Page Header -->
  {#if showHeader && (title || headerActions)}
    <div class="page-header">
      <div class="{maxWidth} mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-4">
            {#if icon}
              <div class="page-icon">
                <i class="bi bi-{icon}"></i>
              </div>
            {/if}
            <h1 class="page-title">{title}</h1>
          </div>
          
          {#if headerActions}
            <div class="page-header-actions">
              {@render headerActions?.()}
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Page Content -->
  <div class="page-content {noPadding ? '' : 'px-4 sm:px-6 lg:px-8'}">
    <div class="{maxWidth} mx-auto">
      {#if loading}
        <LoadingState />
      {:else if empty}
        <EmptyState 
          icon={emptyIcon} 
          title={emptyTitle} 
          message={emptyMessage} 
        />
      {:else}
        {@render children?.()}
      {/if}
    </div>
  </div>
</div>

<style>
  .page-layout {
    min-height: 100vh;
    background: rgb(249 250 251);
    transition: background-color 200ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global(.dark) .page-layout {
    background: rgb(17 24 39);
  }

  .page-breadcrumbs {
    background: white;
    border-bottom: 1px solid rgb(229 231 235);
    padding: 0.75rem 0;
  }

  :global(.dark) .page-breadcrumbs {
    background: rgb(31 41 55);
    border-bottom-color: rgb(55 65 81);
  }

  .page-header {
    background: white;
    border-bottom: 1px solid rgb(229 231 235);
    padding: 1.5rem 0;
    position: sticky;
    top: 0;
    z-index: 10;
    backdrop-filter: blur(8px);
    background-color: rgba(255, 255, 255, 0.95);
  }

  :global(.dark) .page-header {
    background-color: rgba(31, 41, 55, 0.95);
    border-bottom-color: rgb(55 65 81);
  }

  .page-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 1.5rem;
    box-shadow: 0 4px 6px -1px rgba(102, 126, 234, 0.3);
  }

  .page-title {
    font-size: 1.875rem;
    font-weight: 700;
    color: rgb(17 24 39);
    margin: 0;
  }

  :global(.dark) .page-title {
    color: rgb(243 244 246);
  }

  .page-header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .page-content {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .page-header {
      padding: 1rem 0;
    }

    .page-icon {
      width: 40px;
      height: 40px;
      font-size: 1.25rem;
    }

    .page-title {
      font-size: 1.5rem;
    }

    .page-content {
      padding-top: 1.5rem;
      padding-bottom: 1.5rem;
    }

    .page-header-actions {
      gap: 0.5rem;
    }
  }

  @media (max-width: 640px) {
    .page-header .flex {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .page-header-actions {
      width: 100%;
      justify-content: stretch;
    }

    .page-header-actions :global(button) {
      flex: 1;
    }
  }
</style>
