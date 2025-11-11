<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    shapeExpressive,
    shape,
    typographyEmphasized,
  } from "../shared/index.js";

  type ShapeVariant =
    | "normal"
    | "extra-rounded"
    | "squircle"
    | "ultra-rounded"
    | "pill";
  type ProgressSize = "xs" | "sm" | "md" | "lg" | "xl";
  type ProgressVariant =
    | "primary"
    | "success"
    | "danger"
    | "warning"
    | "gradient";

  interface Props {
    value?: number;
    max?: number;
    variant?: ProgressVariant;
    size?: ProgressSize;
    showLabel?: boolean;
    striped?: boolean;
    animated?: boolean;

    // 🆕 ADVANCED FEATURES
    emphasized?: boolean; // Bold label text
    shapeStyle?: ShapeVariant; // Shape variety
    gradient?: boolean; // Gradient fill
    glowing?: boolean; // Glow effect
    smooth?: boolean; // Smooth transitions with spring physics

    class?: string;
    children?: Snippet;
  }

  let {
    value = 0,
    max = 100,
    variant = "primary",
    size = "md",
    showLabel = false,
    striped = false,
    animated = false,
    emphasized = false,
    shapeStyle = "pill",
    gradient = false,
    glowing = false,
    smooth = true,
    class: customClass = "",
    children,
  }: Props = $props();

  const percentage = $derived(Math.min(Math.max((value / max) * 100, 0), 100));

  // 🆕 ADVANCED: Size variants
  const sizeClasses = {
    xs: "h-1",
    sm: "h-2",
    md: "h-3",
    lg: "h-5",
    xl: "h-7",
  };

  // 🆕 ADVANCED: Shape variants
  const shapeClasses = {
    normal: shape.md,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-sm"],
    "ultra-rounded": shape["2xl"],
    pill: "rounded-full",
  };

  // 🆝 ADVANCED: Variant classes with gradient option
  const variantClasses = gradient
    ? {
        primary: "bg-gradient-to-r from-blue-500 to-purple-600",
        success: "bg-gradient-to-r from-green-500 to-emerald-600",
        danger: "bg-gradient-to-r from-red-500 to-pink-600",
        warning: "bg-gradient-to-r from-amber-500 to-orange-500",
        gradient: "bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500",
      }
    : {
        primary: "bg-blue-500",
        success: "bg-green-500",
        danger: "bg-red-500",
        warning: "bg-amber-500",
        gradient: "bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500",
      };

  // 🆕 ADVANCED: Transition classes
  const transitionClasses = smooth
    ? "transition-all duration-500 ease-[cubic-bezier(0.34,1.56,0.64,1)]"
    : "transition-all duration-300 ease-out";

  // 🆕 ADVANCED: Glow effect
  const glowClasses = glowing
    ? {
        primary: "shadow-lg shadow-blue-500/50",
        success: "shadow-lg shadow-green-500/50",
        danger: "shadow-lg shadow-red-500/50",
        warning: "shadow-lg shadow-amber-500/50",
        gradient: "shadow-lg shadow-purple-500/50",
      }
    : {
        primary: "",
        success: "",
        danger: "",
        warning: "",
        gradient: "",
      };

  // 🆕 ADVANCED: Label emphasis
  const labelClasses = emphasized
    ? typographyEmphasized.label.medium
    : "text-sm font-medium";
</script>

<div class={`w-full ${customClass}`}>
  {#if showLabel || children}
    <div class="flex justify-between mb-2">
      {#if children}
        <span class={`${labelClasses} text-gray-700 dark:text-gray-300`}>
          {@render children()}
        </span>
      {/if}
      {#if showLabel}
        <span class={`${labelClasses} text-gray-700 dark:text-gray-300`}>
          {percentage.toFixed(0)}%
        </span>
      {/if}
    </div>
  {/if}

  <div
    class={`
      w-full bg-gray-200 dark:bg-gray-700 overflow-hidden
      ${sizeClasses[size]}
      ${shapeClasses[shapeStyle]}
    `}
  >
    <div
      class={`
        h-full
        ${transitionClasses}
        ${variantClasses[variant]}
        ${glowClasses[variant]}
        ${striped ? "bg-stripes" : ""}
        ${animated && striped ? "bg-stripes-animated" : ""}
      `}
      style={`width: ${percentage}%`}
      role="progressbar"
      aria-valuenow={value}
      aria-valuemin="0"
      aria-valuemax={max}
    ></div>
  </div>
</div>

<style>
  .bg-stripes {
    background-image: linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.15) 25%,
      transparent 25%,
      transparent 50%,
      rgba(255, 255, 255, 0.15) 50%,
      rgba(255, 255, 255, 0.15) 75%,
      transparent 75%,
      transparent
    );
    background-size: 1rem 1rem;
  }

  .bg-stripes-animated {
    animation: progress-stripes 1s linear infinite;
  }

  @keyframes progress-stripes {
    0% {
      background-position: 1rem 0;
    }
    100% {
      background-position: 0 0;
    }
  }
</style>
