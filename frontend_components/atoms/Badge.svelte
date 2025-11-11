<script lang="ts">
  import {
    colorMap,
    colorText,
    colorBorder,
    shape,
    shapeExpressive,
    springs,
    typographyEmphasized,
    type BadgeVariant,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  type ShapeVariant = "normal" | "extra-rounded" | "squircle";
  type BadgeSize = "xs" | "sm" | "md" | "lg" | "xl";

  interface Props {
    variant?: BadgeVariant;
    outline?: boolean;

    // 🆕 M3 EXPRESSIVE FEATURES
    size?: BadgeSize;
    emphasized?: boolean; // Bold text
    shapeStyle?: ShapeVariant; // Shape variety
    gradient?: boolean; // Gradient background

    icon?: boolean;
    children?: Snippet;
  }

  let {
    variant = "primary",
    outline = false,
    size = "md",
    emphasized = false,
    shapeStyle = "normal",
    gradient = false,
    icon = false,
    children,
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Expanded size scale
  const sizeClasses = {
    xs: "px-1.5 py-0.5 text-[10px]",
    sm: "px-2 py-1 text-xs",
    md: "px-3 py-1.5 text-sm",
    lg: "px-4 py-2 text-base",
    xl: "px-5 py-2.5 text-lg",
  };

  // 🆕 M3 EXPRESSIVE: Shape variety
  const shapeClasses = {
    normal: "rounded-full",
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-sm"],
  };

  // Gradient variants
  const gradientClasses = gradient
    ? {
        primary: "bg-gradient-to-br from-blue-500 to-blue-700",
        secondary: "bg-gradient-to-br from-gray-500 to-gray-700",
        success: "bg-gradient-to-br from-green-500 to-green-700",
        danger: "bg-gradient-to-br from-red-500 to-red-700",
        warning: "bg-gradient-to-br from-yellow-500 to-yellow-700",
        info: "bg-gradient-to-br from-cyan-500 to-cyan-700",
      }
    : null;

  const variantClasses = {
    primary: outline
      ? `border-2 ${colorBorder.primary} ${colorText.primary} bg-transparent`
      : gradient
        ? gradientClasses!.primary
        : `bg-gradient-to-r ${colorMap.primary} text-white`,
    secondary: outline
      ? `border-2 ${colorBorder.secondary} ${colorText.secondary} bg-transparent`
      : gradient
        ? gradientClasses!.secondary
        : `bg-gradient-to-r ${colorMap.secondary} text-white`,
    success: outline
      ? `border-2 ${colorBorder.success} ${colorText.success} bg-transparent`
      : gradient
        ? gradientClasses!.success
        : `bg-gradient-to-r ${colorMap.success} text-white`,
    danger: outline
      ? `border-2 ${colorBorder.danger} ${colorText.danger} bg-transparent`
      : gradient
        ? gradientClasses!.danger
        : `bg-gradient-to-r ${colorMap.danger} text-white`,
    warning: outline
      ? `border-2 ${colorBorder.warning} ${colorText.warning} bg-transparent`
      : gradient
        ? gradientClasses!.warning
        : `bg-gradient-to-r ${colorMap.warning} text-white`,
    info: outline
      ? `border-2 ${colorBorder.info} ${colorText.info} bg-transparent`
      : gradient
        ? gradientClasses!.info
        : `bg-gradient-to-r ${colorMap.info} text-white`,
  };

  const textClass = emphasized ? "font-bold" : "font-semibold";

  const classes = `
    ${shapeClasses[shapeStyle]} 
    ${textClass}
    inline-flex items-center gap-2 
    ${sizeClasses[size]} 
    ${variantClasses[variant]} 
    ${icon ? "p-1.5" : ""}
    transition-all duration-300 ease-[${springs.spatial.bouncy}]
    hover:scale-110
    active:scale-95
  `;
</script>

<span class={classes}>
  {#if children}
    {@render children()}
  {/if}
</span>
