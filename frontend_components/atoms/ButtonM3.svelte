<script lang="ts">
  import {
    colors,
    elevation,
    shape,
    sizes,
    stateLayers,
    surface,
    transitions,
    outline,
    type ButtonVariant,
    type ButtonSize,
    type ButtonColor,
  } from "../shared/index.js";

  interface Props {
    variant?: ButtonVariant;
    color?: ButtonColor;
    size?: ButtonSize;
    disabled?: boolean;
    loading?: boolean;
    fullWidth?: boolean;
    href?: string;
    onclick?: () => void;
    class?: string;
  }

  let {
    variant = "filled",
    color = "primary",
    size = "md",
    disabled = false,
    loading = false,
    fullWidth = false,
    href,
    onclick,
    class: customClass = "",
  }: Props = $props();

  // Base classes - Material 3 foundation
  const baseClasses = `
    inline-flex items-center justify-center gap-2
    font-medium ${shape.lg}
    ${transitions.normal}
    focus-visible:outline-none
    disabled:opacity-38 disabled:cursor-not-allowed disabled:pointer-events-none
    ${sizes[size]}
  `;

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
    `,
    outlined: `
      ${surface.base}
      ${colors[color].onContainer}
      ${outline.default}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
    `,
    text: `
      bg-transparent
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
    `,
    elevated: `
      ${surface.containerLow}
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${elevation[1]}
      hover:${elevation[2]}
      ${stateLayers.hover}
      ${stateLayers.pressed}
    `,
    tonal: `
      ${colors[color].container}
      ${colors[color].onContainer}
      ${colors[color].focus}
      ${stateLayers.hover}
      ${stateLayers.pressed}
    `,
  };

  const classes = `${baseClasses} ${variantStyles[variant]} ${fullWidth ? "w-full" : ""} ${customClass}`;
</script>

{#if href}
  <a {href} class={classes} {disabled}>
    {#if loading}
      <span
        class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"
        aria-hidden="true"
      ></span>
    {/if}
    <slot />
  </a>
{:else}
  <button {onclick} {disabled} class={classes} type="button">
    {#if loading}
      <span
        class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"
        aria-hidden="true"
      ></span>
    {/if}
    <slot />
  </button>
{/if}

<style>
  /* Material 3: Subtle state layer transitions */
  button,
  a {
    position: relative;
    overflow: hidden;
  }

  /* Ripple effect placeholder (can be enhanced with JS) */
  button::before,
  a::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    opacity: 0;
    transition: opacity 200ms ease-in-out;
    pointer-events: none;
  }

  button:hover::before,
  a:hover::before {
    opacity: 0.08;
    background: currentColor;
  }

  button:active::before,
  a:active::before {
    opacity: 0.12;
    background: currentColor;
  }

  /* Disabled state */
  button:disabled,
  a[disabled] {
    opacity: 0.38;
  }
</style>
