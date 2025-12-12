<!-- UITabs.svelte - Consistent tab interface -->
<script>
  export let tabs = []; // Array of {id, label, icon?, badge?, disabled?}
  export let activeTab = "";
  export let variant = "default"; // 'default', 'pills', 'underline'
  export let size = "md"; // 'sm', 'md', 'lg'

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function selectTab(tabId) {
    if (tabId !== activeTab) {
      activeTab = tabId;
      dispatch("change", { tabId });
    }
  }

  // Size classes
  const sizeClasses = {
    sm: "px-3 py-1.5 text-sm",
    md: "px-4 py-2 text-sm",
    lg: "px-6 py-3 text-base",
  };

  // Variant classes
  const variantClasses = {
    default: {
      container: "bg-gray-100 dark:bg-slate-700/50 rounded-lg p-1",
      tab: "rounded-md transition-all duration-200",
      active:
        "bg-white dark:bg-slate-600 shadow-sm text-gray-900 dark:text-white",
      inactive:
        "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-white/50 dark:hover:bg-slate-600/50",
    },
    pills: {
      container: "space-x-2",
      tab: "rounded-full transition-all duration-200 border",
      active: "bg-blue-600 text-white border-blue-600",
      inactive:
        "text-gray-600 dark:text-gray-400 border-gray-300 dark:border-slate-600 hover:border-blue-300 dark:hover:border-blue-500 hover:text-blue-600 dark:hover:text-blue-400",
    },
    underline: {
      container: "border-b border-gray-200 dark:border-slate-600",
      tab: "border-b-2 transition-all duration-200 -mb-px",
      active: "border-blue-600 text-blue-600 dark:text-blue-400",
      inactive:
        "border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:border-gray-300 dark:hover:border-slate-500",
    },
  };
</script>

<div
  class="flex {variant === 'underline' ? 'space-x-8' : ''} {variantClasses[
    variant
  ].container}"
>
  {#each tabs as tab}
    <button
      onclick={() => selectTab(tab.id)}
      disabled={tab.disabled}
      class="flex items-center {sizeClasses[size]} {variantClasses[variant]
        .tab} {activeTab === tab.id
        ? variantClasses[variant].active
        : variantClasses[variant].inactive} {tab.disabled
        ? 'opacity-50 cursor-not-allowed'
        : 'cursor-pointer'}"
      aria-selected={activeTab === tab.id}
      role="tab"
    >
      {#if tab.icon}
        <i class="bi bi-{tab.icon} mr-2" aria-hidden="true"></i>
      {/if}

      <span class="font-medium">{tab.label}</span>

      {#if tab.badge}
        <span
          class="ml-2 px-2 py-0.5 text-xs font-medium rounded-full {activeTab ===
          tab.id
            ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
            : 'bg-gray-100 text-gray-600 dark:bg-slate-600 dark:text-gray-300'}"
        >
          {tab.badge}
        </span>
      {/if}
    </button>
  {/each}
</div>

<!-- Tab Content -->
<div class="mt-6">
  <slot />
</div>
