<script lang="ts">
  import {
    colors,
    elevation,
    shape,
    shapeExpressive,
    sizes,
    stateLayers,
    surface,
    outline,
    springs,
    motionPresets,
    typographyEmphasized,
    type ButtonSize,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  type ButtonVariant = "filled" | "outlined" | "text" | "elevated" | "tonal";
  type ButtonColor = "primary" | "secondary" | "tertiary" | "error" | "success";
  type ShapeVariant = "normal" | "extra-rounded" | "squircle" | "mixed";
  type MotionVariant = "smooth" | "bouncy" | "energetic";

  interface Props {
    variant?: ButtonVariant;
    color?: ButtonColor;
    size?: ButtonSize;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold typography for impact
    shapeStyle?: ShapeVariant; // Shape variety for visual interest
    motion?: MotionVariant; // Spring physics animations
    fullWidth?: boolean;

    // Standard props
    disabled?: boolean;
    loading?: boolean;
    href?: string;
    onclick?: () => void;
    class?: string;
    children?: Snippet;
  }

  let {
    variant = "filled",
    color = "primary",
    size = "md",
    emphasized = false,
    shapeStyle = "normal",
    motion = "smooth",
    fullWidth = false,
    disabled = false,
    loading = false,
    href,
    onclick,
    class: customClass = "",
    children,
  }: Props = $props();

  // Base classes - M3 foundation
  const baseClasses = `
    inline-flex items-center justify-center gap-2
    font-medium
    focus-visible:outline-none
    disabled:opacity-38 disabled:cursor-not-allowed disabled:pointer-events-none
    ${sizes[size]}
  `;

  // 🆕 M3 EXPRESSIVE: Shape variety
  const shapeClasses = {
    normal: shape.lg,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-lg"],
    mixed: shapeExpressive["mixed-rounded"],
  };

  // 🆕 M3 EXPRESSIVE: Spring motion
  const motionClasses = {
    smooth: `transition-all duration-300 ease-[${springs.spatial.smooth}]`,
    bouncy: `transition-all duration-300 ease-[${springs.spatial.bouncy}]`,
    energetic: `transition-all duration-400 ease-[${springs.spatial.energetic}]`,
  };

  // 🆕 M3 EXPRESSIVE: Typography emphasis
  const textClasses = emphasized
    ? "font-bold tracking-tight" // 700 weight for impact!
    : "font-medium";

  // Material 3 Button Variants
  const variantStyles = {
    filled: `
      ${colors[color].default}
      ${colors[color].on}
      ${colors[color].hover}
      ${colors[color].focus}
      ${elevation[1]}
      ${stateLayers.hover}
      ${stateLayers.pressed}
      hover:scale-105
      active:scale-95
    `,
    outlined: `
      ${surface.base}
      ${colors[color].onContainer}
      ${outline.default}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
      hover:scale-102
      active:scale-98
    `,
    text: `
      bg-transparent
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
      hover:scale-105
      active:scale-95
    `,
    elevated: `
      ${surface.containerLow}
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${elevation[1]}
      hover:${elevation[2]}
      ${stateLayers.hover}
      ${stateLayers.pressed}
      hover:scale-105
      hover:-translate-y-0.5
      active:scale-100
      active:translate-y-0
    `,
    tonal: `
      ${colors[color].container}
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
      hover:scale-105
      active:scale-95
    `,
  };

  const classes = `
    ${baseClasses} 
    ${shapeClasses[shapeStyle]} 
    ${motionClasses[motion]}
    ${textClasses}
    ${variantStyles[variant]} 
    ${fullWidth ? "w-full" : ""} 
    ${customClass}
  `;
</script>

{#if href}
  <a {href} class={classes} aria-disabled={disabled}>
    {#if loading}
      <span
        class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"
        aria-hidden="true"
      ></span>
    {/if}
    {#if children}
      {@render children()}
    {/if}
  </a>
{:else}
  <button {onclick} {disabled} class={classes} type="button">
    {#if loading}
      <span
        class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"
        aria-hidden="true"
      ></span>
    {/if}
    {#if children}
      {@render children()}
    {/if}
  </button>
{/if}

<style>
  /* M3 Expressive: Spring hover effects with scale */
  button:hover:not(:disabled),
  a:hover:not([aria-disabled="true"]) {
    transform-origin: center;
  }

  /* Pressed state with spring bounce-back */
  button:active:not(:disabled),
  a:active:not([aria-disabled="true"]) {
    transition-duration: 100ms;
  }

  /* Disabled state */
  button:disabled,
  a[aria-disabled="true"] {
    opacity: 0.38;
  }
</style>
