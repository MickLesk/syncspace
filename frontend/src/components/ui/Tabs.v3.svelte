<script>
  /**
   * Tabs Component v3 - SyncSpace Design System
   * 
   * Unified tab navigation with multiple variants.
   * Supports icons, badges, and responsive behavior.
   */
  import { cn } from "../../lib/design-system/utils.js";

  let {
    tabs = [],
    activeTab = $bindable(""),
    variant = "default",
    size = "md",
    fullWidth = false,
    class: className = "",
    ontabchange = null,
  } = $props();

  // Set first tab as active if none specified
  $effect(() => {
    if (!activeTab && tabs.length > 0) {
      activeTab = tabs[0].id;
    }
  });

  const containerClasses = {
    default: "bg-gray-100 dark:bg-gray-800 p-1 rounded-xl",
    pills: "gap-2",
    underline: "border-b border-gray-200 dark:border-gray-700 gap-0",
    glass: "glass-card p-1 rounded-xl",
    segment: "bg-gray-100 dark:bg-gray-800 p-1 rounded-xl inline-flex",
  };

  const baseTabClasses = `
    inline-flex items-center justify-center gap-2
    font-medium
    transition-all duration-200
    focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2
  `;

  const variantTabClasses = {
    default: {
      active: "bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm",
      inactive: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-white/50 dark:hover:bg-gray-700/50",
    },
    pills: {
      active: "bg-primary-500 text-white shadow-md",
      inactive: "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800",
    },
    underline: {
      active: "text-primary-600 dark:text-primary-400 border-b-2 border-primary-500",
      inactive: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white border-b-2 border-transparent hover:border-gray-300 dark:hover:border-gray-600",
    },
    glass: {
      active: "bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-md",
      inactive: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-white/30 dark:hover:bg-gray-700/30",
    },
    segment: {
      active: "bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm",
      inactive: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
    },
  };

  const sizeClasses = {
    sm: "px-3 py-1.5 text-sm rounded-lg",
    md: "px-4 py-2 text-sm rounded-lg",
    lg: "px-5 py-2.5 text-base rounded-xl",
  };

  const underlineSizeClasses = {
    sm: "px-3 py-2 text-sm -mb-px",
    md: "px-4 py-3 text-sm -mb-px",
    lg: "px-5 py-4 text-base -mb-px",
  };

  function handleTabClick(tabId) {
    activeTab = tabId;
    ontabchange?.(tabId);
  }

  function getTabClasses(tab) {
    const isActive = activeTab === tab.id;
    const tabVariant = variantTabClasses[variant] || variantTabClasses.default;
    const sizeClass = variant === "underline" 
      ? underlineSizeClasses[size] 
      : sizeClasses[size];
    
    return cn(
      baseTabClasses,
      sizeClass,
      isActive ? tabVariant.active : tabVariant.inactive,
      tab.disabled && "opacity-50 cursor-not-allowed pointer-events-none",
      fullWidth && "flex-1"
    );
  }
</script>

<div
  class={cn(
    "flex",
    fullWidth && "w-full",
    containerClasses[variant] || containerClasses.default,
    className
  )}
  role="tablist"
>
  {#each tabs as tab (tab.id)}
    <button
      type="button"
      role="tab"
      class={getTabClasses(tab)}
      aria-selected={activeTab === tab.id}
      aria-controls="tabpanel-{tab.id}"
      disabled={tab.disabled}
      onclick={() => handleTabClick(tab.id)}
    >
      {#if tab.icon}
        <i class="bi bi-{tab.icon}"></i>
      {/if}
      
      <span>{tab.label}</span>
      
      {#if tab.badge !== undefined}
        <span class="ml-1 px-1.5 py-0.5 text-xs font-semibold rounded-full {activeTab === tab.id ? 'bg-white/20 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400'}">
          {tab.badge}
        </span>
      {/if}
    </button>
  {/each}
</div>
