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
  type ChipSize = "xs" | "sm" | "md" | "lg" | "xl";
  type ChipVariant =
    | "default"
    | "primary"
    | "success"
    | "danger"
    | "warning"
    | "info"
    | "glass";

  interface Props {
    variant?: ChipVariant;
    size?: ChipSize;
    removable?: boolean;
    icon?: string;

    // 🆕 ADVANCED FEATURES
    emphasized?: boolean; // Bold text
    shapeStyle?: ShapeVariant; // Shape variety
    gradient?: boolean; // Gradient backgrounds
    animated?: boolean; // Hover animations
    outlined?: boolean; // Outlined style
    elevationLevel?: 0 | 1 | 2 | 3; // Shadow depth

    class?: string;
    onremove?: () => void;
    children?: Snippet;
  }

  let {
    variant = "default",
    size = "md",
    removable = false,
    icon,
    emphasized = false,
    shapeStyle = "pill",
    gradient = false,
    animated = true,
    outlined = false,
    elevationLevel = 0,
    class: customClass = "",
    onremove,
    children,
  }: Props = $props();

  // 🆕 ADVANCED: Size variants with more options
  const sizeClasses = {
    xs: "text-xs px-2 py-0.5 gap-1",
    sm: "text-xs px-2 py-1 gap-1",
    md: "text-sm px-3 py-1.5 gap-1.5",
    lg: "text-base px-4 py-2 gap-2",
    xl: "text-lg px-5 py-2.5 gap-2.5",
  };

  // 🆕 ADVANCED: Shape variants
  const shapeClasses = {
    normal: shape.md,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-md"],
    "ultra-rounded": shape["2xl"],
    pill: "rounded-full",
  };

  // 🆕 ADVANCED: Elevation levels
  const elevationClasses = {
    0: "",
    1: "shadow-sm",
    2: "shadow-md",
    3: "shadow-lg",
  };

  // 🆕 ADVANCED: Variant classes with gradient option
  const variantClasses = gradient
    ? {
        default:
          "bg-gradient-to-r from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 text-gray-700 dark:text-gray-300",
        primary:
          "bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-blue-500/30",
        success:
          "bg-gradient-to-r from-green-500 to-emerald-600 text-white shadow-green-500/30",
        danger:
          "bg-gradient-to-r from-red-500 to-pink-600 text-white shadow-red-500/30",
        warning:
          "bg-gradient-to-r from-amber-500 to-orange-500 text-white shadow-amber-500/30",
        info: "bg-gradient-to-r from-cyan-500 to-blue-500 text-white shadow-cyan-500/30",
        glass:
          "bg-gradient-to-r from-white/10 to-white/5 backdrop-blur-md text-gray-800 dark:text-white border border-white/20",
      }
    : outlined
      ? {
          default:
            "bg-transparent border-2 border-gray-300 text-gray-700 dark:border-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800",
          primary:
            "bg-transparent border-2 border-blue-500 text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-950",
          success:
            "bg-transparent border-2 border-green-500 text-green-600 dark:text-green-400 hover:bg-green-50 dark:hover:bg-green-950",
          danger:
            "bg-transparent border-2 border-red-500 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950",
          warning:
            "bg-transparent border-2 border-amber-500 text-amber-600 dark:text-amber-400 hover:bg-amber-50 dark:hover:bg-amber-950",
          info: "bg-transparent border-2 border-cyan-500 text-cyan-600 dark:text-cyan-400 hover:bg-cyan-50 dark:hover:bg-cyan-950",
          glass:
            "bg-transparent backdrop-blur-md border-2 border-white/20 text-gray-800 dark:text-white hover:bg-white/10",
        }
      : {
          default:
            "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700",
          primary:
            "bg-blue-500 text-white hover:bg-blue-600 shadow-blue-500/20",
          success:
            "bg-green-500 text-white hover:bg-green-600 shadow-green-500/20",
          danger: "bg-red-500 text-white hover:bg-red-600 shadow-red-500/20",
          warning:
            "bg-amber-500 text-white hover:bg-amber-600 shadow-amber-500/20",
          info: "bg-cyan-500 text-white hover:bg-cyan-600 shadow-cyan-500/20",
          glass:
            "bg-white/10 backdrop-blur-md text-gray-800 dark:text-white border border-white/20 hover:bg-white/20",
        };

  // 🆕 ADVANCED: Animation classes
  const animationClasses = animated
    ? "transition-all duration-300 hover:scale-110 active:scale-95"
    : "transition-colors duration-200";

  // 🆕 ADVANCED: Text emphasis
  const textClasses = emphasized
    ? typographyEmphasized.label.medium
    : "font-medium";
</script>

<div
  class={`
    inline-flex items-center
    ${textClasses}
    ${sizeClasses[size]}
    ${shapeClasses[shapeStyle]}
    ${variantClasses[variant]}
    ${elevationClasses[elevationLevel]}
    ${animationClasses}
    ${customClass}
  `}
>
  {#if icon}
    <i class="bi {icon}"></i>
  {/if}

  {#if children}
    <span>{@render children()}</span>
  {/if}

  {#if removable}
    <button
      onclick={onremove}
      class="ml-1 hover:bg-black/10 dark:hover:bg-white/10 rounded-full p-0.5 transition-all duration-200 hover:scale-110 active:scale-90"
      aria-label="Remove"
    >
      <i class="bi bi-x text-sm"></i>
    </button>
  {/if}
</div>
