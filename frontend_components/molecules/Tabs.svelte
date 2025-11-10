<script lang="ts">
  import type { Snippet } from "svelte";

  interface Tab {
    id: string;
    label: string;
    icon?: string;
    disabled?: boolean;
    badge?: string | number;
  }

  interface Props {
    tabs: Tab[];
    active?: string;
    variant?: "default" | "pills" | "underline";
    class?: string;
    onchange?: (tabId: string) => void;
    children?: Snippet<[string]>;
  }

  let {
    tabs,
    active = $bindable(tabs[0]?.id || ""),
    variant = "default",
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  function handleTabClick(tabId: string, disabled?: boolean) {
    if (disabled) return;
    active = tabId;
    onchange?.(tabId);
  }

  const variantStyles = {
    default: {
      container: "border-b border-gray-200 dark:border-gray-700",
      tab: "px-4 py-2 border-b-2 transition-colors",
      active: "border-blue-500 text-blue-600 dark:text-blue-400 font-medium",
      inactive:
        "border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 hover:border-gray-300 dark:hover:border-gray-600",
    },
    pills: {
      container: "gap-2",
      tab: "px-4 py-2 rounded-lg transition-colors",
      active: "bg-blue-500 text-white font-medium",
      inactive:
        "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800",
    },
    underline: {
      container: "gap-6 border-b border-gray-200 dark:border-gray-700",
      tab: "px-1 py-2 border-b-2 transition-colors",
      active: "border-blue-500 text-blue-600 dark:text-blue-400 font-medium",
      inactive:
        "border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200",
    },
  };

  const styles = variantStyles[variant];
</script>

<div class={`w-full ${customClass}`}>
  <!-- Tab Navigation -->
  <div class={`flex ${styles.container}`} role="tablist">
    {#each tabs as tab (tab.id)}
      <button
        type="button"
        role="tab"
        aria-selected={active === tab.id}
        aria-controls={`tabpanel-${tab.id}`}
        disabled={tab.disabled}
        onclick={() => handleTabClick(tab.id, tab.disabled)}
        class={`
          flex items-center gap-2
          ${styles.tab}
          ${active === tab.id ? styles.active : styles.inactive}
          ${tab.disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}
        `}
      >
        {#if tab.icon}
          <i class={`bi ${tab.icon}`}></i>
        {/if}
        <span>{tab.label}</span>
        {#if tab.badge}
          <span
            class="px-2 py-0.5 text-xs rounded-full bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300"
          >
            {tab.badge}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Tab Content -->
  {#if children}
    <div class="mt-4">
      {@render children(active)}
    </div>
  {/if}
</div>
