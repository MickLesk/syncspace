<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    shapeExpressive,
    shape,
    typographyEmphasized,
  } from "../shared/index.js";

  type TooltipPosition = "top" | "bottom" | "left" | "right";
  type TooltipSize = "sm" | "md" | "lg";
  type TooltipVariant = "dark" | "light" | "gradient" | "glass";
  type ShapeVariant = "normal" | "extra-rounded" | "squircle" | "pill";

  interface Props {
    open?: boolean;
    position?: TooltipPosition;

    // 🆕 ADVANCED FEATURES
    variant?: TooltipVariant; // Color schemes
    size?: TooltipSize; // Size variants
    shapeStyle?: ShapeVariant; // Shape variety
    emphasized?: boolean; // Bold text
    animated?: boolean; // Smooth entrance animation
    showArrow?: boolean; // Toggle arrow
    delay?: number; // Hover delay in ms

    class?: string;
    children?: Snippet;
    content?: Snippet;
  }

  let {
    open = $bindable(false),
    position = "top",
    variant = "dark",
    size = "md",
    shapeStyle = "normal",
    emphasized = false,
    animated = true,
    showArrow = true,
    delay = 200,
    class: customClass = "",
    children,
    content,
  }: Props = $props();

  let hoverTimeout: ReturnType<typeof setTimeout> | null = null;

  function handleMouseEnter() {
    if (delay > 0) {
      hoverTimeout = setTimeout(() => {
        open = true;
      }, delay);
    } else {
      open = true;
    }
  }

  function handleMouseLeave() {
    if (hoverTimeout) {
      clearTimeout(hoverTimeout);
      hoverTimeout = null;
    }
    open = false;
  }

  // 🆕 ADVANCED: Position classes
  const positionClasses = {
    top: "bottom-full left-1/2 -translate-x-1/2 mb-2",
    bottom: "top-full left-1/2 -translate-x-1/2 mt-2",
    left: "right-full top-1/2 -translate-y-1/2 mr-2",
    right: "left-full top-1/2 -translate-y-1/2 ml-2",
  };

  // 🆕 ADVANCED: Size variants
  const sizeClasses = {
    sm: "px-2 py-1 text-xs",
    md: "px-3 py-2 text-sm",
    lg: "px-4 py-3 text-base",
  };

  // 🆕 ADVANCED: Shape variants
  const shapeClasses = {
    normal: shape.md,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-md"],
    pill: "rounded-full",
  };

  // 🆕 ADVANCED: Variant styles
  const variantClasses = {
    dark: "text-white bg-gray-900 dark:bg-gray-800 shadow-xl",
    light:
      "text-gray-900 bg-white dark:bg-gray-100 dark:text-gray-900 shadow-xl border border-gray-200",
    gradient:
      "text-white bg-gradient-to-r from-blue-600 to-purple-600 shadow-xl shadow-blue-500/30",
    glass:
      "text-gray-900 dark:text-white bg-white/10 backdrop-blur-md border border-white/20 shadow-xl",
  };

  // 🆕 ADVANCED: Arrow colors
  const arrowColorClasses = {
    dark: {
      top: "border-t-gray-900 dark:border-t-gray-800",
      bottom: "border-b-gray-900 dark:border-b-gray-800",
      left: "border-l-gray-900 dark:border-l-gray-800",
      right: "border-r-gray-900 dark:border-r-gray-800",
    },
    light: {
      top: "border-t-white dark:border-t-gray-100",
      bottom: "border-b-white dark:border-b-gray-100",
      left: "border-l-white dark:border-l-gray-100",
      right: "border-r-white dark:border-r-gray-100",
    },
    gradient: {
      top: "border-t-purple-600",
      bottom: "border-b-purple-600",
      left: "border-l-purple-600",
      right: "border-r-purple-600",
    },
    glass: {
      top: "border-t-white/10",
      bottom: "border-b-white/10",
      left: "border-l-white/10",
      right: "border-r-white/10",
    },
  };

  const arrowPositionClasses = {
    top: "top-full left-1/2 -translate-x-1/2 border-x-transparent border-b-transparent",
    bottom:
      "bottom-full left-1/2 -translate-x-1/2 border-x-transparent border-t-transparent",
    left: "left-full top-1/2 -translate-y-1/2 border-y-transparent border-r-transparent",
    right:
      "right-full top-1/2 -translate-y-1/2 border-y-transparent border-l-transparent",
  };

  // 🆕 ADVANCED: Text emphasis
  const textClasses = emphasized
    ? typographyEmphasized.label.medium
    : "font-medium";

  // 🆕 ADVANCED: Animation
  const animationClasses = animated
    ? "animate-in fade-in zoom-in-95 duration-200"
    : "";
</script>

<div class={`relative inline-block ${customClass}`}>
  <!-- Trigger -->
  <div
    role="button"
    tabindex="0"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        open = !open;
      }
    }}
  >
    {#if children}
      {@render children()}
    {/if}
  </div>

  <!-- Tooltip -->
  {#if open && content}
    <div
      class={`
        absolute z-50 whitespace-nowrap pointer-events-none
        ${sizeClasses[size]}
        ${shapeClasses[shapeStyle]}
        ${variantClasses[variant]}
        ${positionClasses[position]}
        ${animationClasses}
        ${textClasses}
      `}
      role="tooltip"
    >
      {@render content()}

      <!-- Arrow -->
      {#if showArrow}
        <div
          class={`
            absolute w-0 h-0 border-4
            ${arrowPositionClasses[position]}
            ${arrowColorClasses[variant][position]}
          `}
        ></div>
      {/if}
    </div>
  {/if}
</div>
