<script lang="ts">
  import { shapeExpressive, shape } from "../shared/index.js";

  type ShapeVariant =
    | "normal"
    | "extra-rounded"
    | "squircle"
    | "circle"
    | "ultra-rounded";
  type IconButtonSize = "xs" | "sm" | "md" | "lg" | "xl" | "2xl";
  type IconButtonVariant =
    | "default"
    | "primary"
    | "danger"
    | "success"
    | "ghost"
    | "glass";

  interface Props {
    icon: string;
    variant?: IconButtonVariant;
    size?: IconButtonSize;
    disabled?: boolean;
    loading?: boolean;

    // 🆕 ADVANCED FEATURES
    shapeStyle?: ShapeVariant; // Shape variety
    gradient?: boolean; // Gradient background
    animated?: boolean; // Hover animations
    glowing?: boolean; // Glow effect
    elevationLevel?: 0 | 1 | 2 | 3; // Shadow depth

    class?: string;
    onclick?: (event: MouseEvent) => void;
    ariaLabel?: string;
  }

  let {
    icon,
    variant = "default",
    size = "md",
    disabled = false,
    loading = false,
    shapeStyle = "circle",
    gradient = false,
    animated = true,
    glowing = false,
    elevationLevel = 0,
    class: customClass = "",
    onclick,
    ariaLabel,
  }: Props = $props();

  // 🆕 ADVANCED: Size variants
  const sizeClasses = {
    xs: "w-7 h-7 text-xs",
    sm: "w-8 h-8 text-sm",
    md: "w-10 h-10 text-base",
    lg: "w-12 h-12 text-lg",
    xl: "w-14 h-14 text-xl",
    "2xl": "w-16 h-16 text-2xl",
  };

  // 🆕 ADVANCED: Shape variants
  const shapeClasses = {
    normal: shape.md,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-md"],
    circle: "rounded-full",
    "ultra-rounded": shape["2xl"],
  };

  // 🆕 ADVANCED: Variant classes with gradient option
  const variantClasses = gradient
    ? {
        default:
          "bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 text-gray-700 dark:text-gray-300",
        primary:
          "bg-gradient-to-br from-blue-500 to-purple-600 text-white shadow-blue-500/30",
        danger:
          "bg-gradient-to-br from-red-500 to-pink-600 text-white shadow-red-500/30",
        success:
          "bg-gradient-to-br from-green-500 to-emerald-600 text-white shadow-green-500/30",
        ghost:
          "bg-transparent text-gray-700 hover:bg-gradient-to-br hover:from-gray-100 hover:to-gray-200 dark:text-gray-300 dark:hover:from-gray-800 dark:hover:to-gray-700",
        glass:
          "bg-gradient-to-br from-white/10 to-white/5 backdrop-blur-md text-gray-800 dark:text-white border border-white/20",
      }
    : {
        default:
          "bg-gray-100 text-gray-700 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700",
        primary: "bg-blue-500 text-white hover:bg-blue-600 shadow-blue-500/20",
        danger: "bg-red-500 text-white hover:bg-red-600 shadow-red-500/20",
        success:
          "bg-green-500 text-white hover:bg-green-600 shadow-green-500/20",
        ghost:
          "bg-transparent text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800",
        glass:
          "bg-white/10 backdrop-blur-md text-gray-800 dark:text-white border border-white/20 hover:bg-white/20",
      };

  // 🆕 ADVANCED: Elevation levels
  const elevationClasses = {
    0: "",
    1: "shadow-sm",
    2: "shadow-md",
    3: "shadow-lg shadow-xl",
  };

  // 🆕 ADVANCED: Animation classes
  const animationClasses = animated
    ? "transition-all duration-300 hover:scale-110 active:scale-90"
    : "transition-all duration-200 active:scale-95";

  // 🆕 ADVANCED: Glow effect
  const glowClasses = glowing
    ? {
        default: "",
        primary: "shadow-lg shadow-blue-500/50",
        danger: "shadow-lg shadow-red-500/50",
        success: "shadow-lg shadow-green-500/50",
        ghost: "",
        glass: "shadow-lg shadow-white/20",
      }
    : {
        default: "",
        primary: "",
        danger: "",
        success: "",
        ghost: "",
        glass: "",
      };
</script>

<button
  {onclick}
  {disabled}
  aria-label={ariaLabel}
  class={`
    inline-flex items-center justify-center
    font-medium
    ${sizeClasses[size]}
    ${shapeClasses[shapeStyle]}
    ${variantClasses[variant]}
    ${elevationClasses[elevationLevel]}
    ${glowClasses[variant]}
    ${animationClasses}
    ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}
    ${customClass}
  `}
>
  {#if loading}
    <i class="bi bi-arrow-repeat animate-spin"></i>
  {:else}
    <i class="bi {icon}"></i>
  {/if}
</button>
