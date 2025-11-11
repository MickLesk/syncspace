<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    typographyEmphasized,
    shapeExpressive,
    springs,
    shadows,
    transitions,
  } from "../shared/index.js";

  interface Item {
    id: string;
    label: string;
    icon?: string;
    disabled?: boolean;
    divider?: boolean;
    danger?: boolean;
  }

  interface Props {
    items: Item[];
    open?: boolean;
    position?: "bottom-left" | "bottom-right" | "top-left" | "top-right";
    emphasized?: boolean; // M3 Expressive: Bold labels
    shapeStyle?: keyof typeof shapeExpressive; // M3 Expressive: Shape variety
    motion?: keyof typeof springs.spatial; // M3 Expressive: Spring animation
    size?: "sm" | "md" | "lg"; // M3 Expressive: Size variants
    gradient?: boolean; // M3 Expressive: Gradient hover
    class?: string;
    onselect?: (itemId: string) => void;
    children?: Snippet;
  }

  let {
    items,
    open = $bindable(false),
    position = "bottom-left",
    emphasized = false,
    shapeStyle = "large",
    motion = "bouncy",
    size = "md",
    gradient = false,
    class: customClass = "",
    onselect,
    children,
  }: Props = $props();

  const positionClasses = {
    "bottom-left": "top-full left-0 mt-2",
    "bottom-right": "top-full right-0 mt-2",
    "top-left": "bottom-full left-0 mb-2",
    "top-right": "bottom-full right-0 mb-2",
  };

  // M3 Expressive: Size variants
  const sizeClasses = {
    sm: {
      trigger: "px-3 py-1.5 text-xs",
      item: "px-3 py-1.5 text-xs",
      icon: "text-sm",
      minWidth: "min-w-[160px]",
    },
    md: {
      trigger: "px-4 py-2 text-sm",
      item: "px-4 py-2 text-sm",
      icon: "text-base",
      minWidth: "min-w-[200px]",
    },
    lg: {
      trigger: "px-5 py-3 text-base",
      item: "px-5 py-3 text-base",
      icon: "text-lg",
      minWidth: "min-w-[240px]",
    },
  };

  const currentSize = sizeClasses[size];

  // M3 Expressive: Typography
  const labelClass = emphasized
    ? typographyEmphasized.label.medium
    : "font-medium";

  // M3 Expressive: Spring animation
  const springCurve = springs.spatial[motion];
  const springTransition = `transition-all duration-300 ease-out`;

  // M3 Expressive: Shape
  const shapeClass = shapeExpressive[shapeStyle] || shapeExpressive.large;

  function handleSelect(item: Item) {
    if (item.disabled) return;
    onselect?.(item.id);
    open = false;
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest("[data-dropdown]")) {
      open = false;
    }
  }

  $effect(() => {
    if (open) {
      document.addEventListener("click", handleClickOutside);
      return () => document.removeEventListener("click", handleClickOutside);
    }
  });
</script>

<div class={`relative inline-block ${customClass}`} data-dropdown>
  <!-- Trigger -->
  <button
    type="button"
    onclick={() => (open = !open)}
    class={`
      inline-flex items-center gap-2
      ${currentSize.trigger}
      ${shapeClass}
      ${
        emphasized
          ? gradient
            ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white border-transparent shadow-md shadow-blue-500/50"
            : "bg-blue-500 text-white border-transparent shadow-md"
          : "bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600"
      }
      ${emphasized ? "hover:scale-105" : "hover:bg-gray-50 dark:hover:bg-gray-700"}
      ${springTransition}
      focus:outline-none focus:ring-2 focus:ring-blue-500
    `}
  >
    {#if children}
      {@render children()}
    {/if}
    <i
      class={`
        bi bi-chevron-down 
        ${currentSize.icon}
        ${emphasized ? "text-white" : "text-gray-600 dark:text-gray-400"}
        ${springTransition}
        ${open ? "rotate-180" : ""}
      `}
    ></i>
  </button>

  <!-- Dropdown Menu with M3 Expressive animations -->
  {#if open}
    <div
      class={`
        absolute z-50
        ${currentSize.minWidth}
        ${shapeClass}
        ${shadows.xl}
        bg-white dark:bg-gray-800 
        border border-gray-200 dark:border-gray-700
        py-1
        ${positionClasses[position]}
      `}
      style={`animation: dropdownSlideIn 300ms ${springCurve};`}
    >
      {#each items as item (item.id)}
        {#if item.divider}
          <div class="my-1 border-t border-gray-200 dark:border-gray-700"></div>
        {:else}
          <button
            type="button"
            onclick={() => handleSelect(item)}
            disabled={item.disabled}
            class={`
              w-full flex items-center gap-3
              ${currentSize.item}
              ${emphasized ? labelClass : "font-medium"}
              text-left
              ${springTransition}
              ${
                item.disabled
                  ? "opacity-50 cursor-not-allowed"
                  : item.danger
                    ? `text-red-600 dark:text-red-400 
                       ${
                         gradient
                           ? "hover:bg-gradient-to-r hover:from-red-500 hover:to-pink-500 hover:text-white"
                           : "hover:bg-red-50 dark:hover:bg-red-900/20"
                       }
                       active:scale-95`
                    : `text-gray-700 dark:text-gray-300 
                       ${
                         gradient
                           ? "hover:bg-gradient-to-r hover:from-blue-500 hover:to-purple-500 hover:text-white"
                           : "hover:bg-gray-100 dark:hover:bg-gray-700"
                       }
                       active:scale-95`
              }
            `}
          >
            {#if item.icon}
              <i class={`bi ${item.icon} ${currentSize.icon}`}></i>
            {/if}
            <span>{item.label}</span>
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  /* M3 Expressive: Spring slide-in animation */
  @keyframes dropdownSlideIn {
    0% {
      opacity: 0;
      transform: translateY(-0.5rem) scale(0.95);
    }
    60% {
      opacity: 1;
      transform: translateY(0.25rem) scale(1.02);
    }
    100% {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
