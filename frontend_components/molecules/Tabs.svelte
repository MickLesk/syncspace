<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    typographyEmphasized,
    shapeExpressive,
    springs,
    transitions,
  } from "../shared/index.js";

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
    emphasized?: boolean; // M3 Expressive: Bold active tab
    shapeStyle?: keyof typeof shapeExpressive; // M3 Expressive: Shape variety for pills
    motion?: keyof typeof springs.spatial; // M3 Expressive: Spring indicator animation
    size?: "sm" | "md" | "lg"; // M3 Expressive: Size variants
    gradient?: boolean; // M3 Expressive: Gradient active tab
    class?: string;
    onchange?: (tabId: string) => void;
    children?: Snippet<[string]>;
  }

  let {
    tabs,
    active = $bindable(tabs[0]?.id || ""),
    variant = "default",
    emphasized = false,
    shapeStyle = "large",
    motion = "bouncy",
    size = "md",
    gradient = false,
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  function handleTabClick(tabId: string, disabled?: boolean) {
    if (disabled) return;
    active = tabId;
    onchange?.(tabId);
  }

  // M3 Expressive: Typography based on emphasized prop
  const labelClass = emphasized
    ? typographyEmphasized.label.large
    : "text-sm font-medium";

  // M3 Expressive: Size variants
  const sizeClasses = {
    sm: {
      tab: "px-3 py-1.5 text-xs",
      icon: "text-sm",
      badge: "px-1.5 py-0.5 text-[10px]",
    },
    md: {
      tab: "px-4 py-2 text-sm",
      icon: "text-base",
      badge: "px-2 py-0.5 text-xs",
    },
    lg: {
      tab: "px-5 py-3 text-base",
      icon: "text-lg",
      badge: "px-2.5 py-1 text-sm",
    },
  };

  const currentSize = sizeClasses[size];

  // M3 Expressive: Spring animation curve
  const springCurve = springs.spatial[motion];
  const springTransition = `transition-all duration-300 ${transitions.smooth}`;

  // M3 Expressive: Shape for pills variant
  const pillShape = shapeExpressive[shapeStyle] || shapeExpressive.large;

  // M3 Expressive: Enhanced variant styles
  const variantStyles = {
    default: {
      container: "border-b border-gray-200 dark:border-gray-700 relative",
      tab: `${currentSize.tab} border-b-2 ${springTransition}`,
      active: emphasized
        ? gradient
          ? `border-transparent bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent ${labelClass}`
          : `border-blue-500 text-blue-600 dark:text-blue-400 ${labelClass}`
        : "border-blue-500 text-blue-600 dark:text-blue-400 font-medium",
      inactive: `border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 hover:border-gray-300 dark:hover:border-gray-600 ${springTransition}`,
    },
    pills: {
      container: "gap-2",
      tab: `${currentSize.tab} ${pillShape} ${springTransition}`,
      active: emphasized
        ? gradient
          ? `bg-gradient-to-r from-blue-600 to-purple-600 text-white ${labelClass} shadow-lg shadow-blue-500/50`
          : `bg-blue-500 text-white ${labelClass} shadow-md`
        : "bg-blue-500 text-white font-medium",
      inactive: `text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 hover:scale-105 ${springTransition}`,
    },
    underline: {
      container: "gap-6 border-b border-gray-200 dark:border-gray-700 relative",
      tab: `px-1 ${currentSize.tab} border-b-2 ${springTransition}`,
      active: emphasized
        ? gradient
          ? `border-transparent bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent ${labelClass}`
          : `border-blue-500 text-blue-600 dark:text-blue-400 ${labelClass}`
        : "border-blue-500 text-blue-600 dark:text-blue-400 font-medium",
      inactive: `border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 ${springTransition}`,
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
          ${active === tab.id && emphasized ? "transform scale-105" : ""}
        `}
        style={active === tab.id && variant !== "pills"
          ? `transition: all 300ms ${springCurve};`
          : ""}
      >
        {#if tab.icon}
          <i class={`bi ${tab.icon} ${currentSize.icon}`}></i>
        {/if}
        <span>{tab.label}</span>
        {#if tab.badge}
          <span
            class={`
              ${currentSize.badge}
              rounded-full
              ${
                active === tab.id && (emphasized || gradient)
                  ? "bg-white/20 text-white"
                  : "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300"
              }
              ${springTransition}
            `}
          >
            {tab.badge}
          </span>
        {/if}
      </button>
    {/each}

    <!-- M3 Expressive: Spring indicator for underline/default variants -->
    {#if (variant === "default" || variant === "underline") && emphasized}
      <div
        class="absolute bottom-0 h-0.5 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full"
        style={`
          transition: all 300ms ${springCurve};
          width: calc(100% / ${tabs.length});
          left: calc(100% / ${tabs.length} * ${tabs.findIndex((t) => t.id === active)});
        `}
      ></div>
    {/if}
  </div>

  <!-- Tab Content -->
  {#if children}
    <div class="mt-4" role="tabpanel" id={`tabpanel-${active}`}>
      {@render children(active)}
    </div>
  {/if}
</div>
