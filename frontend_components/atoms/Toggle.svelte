<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    shape,
    shapeExpressive,
    springs,
    elevation,
    colors,
    typographyEmphasized,
  } from "../shared/index.js";

  type ToggleSize = "sm" | "md" | "lg";
  type ToggleColor = "primary" | "secondary" | "tertiary" | "success" | "error";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    color?: ToggleColor;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold label
    size?: ToggleSize; // Size variety
    bouncy?: boolean; // Extra bouncy spring

    class?: string;
    onchange?: (event: Event) => void;
    children?: Snippet;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    color = "primary",
    emphasized = false,
    size = "md",
    bouncy = true,
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Size variants
  const sizeClasses = {
    sm: { track: "w-9 h-5", thumb: "w-4 h-4", translate: "translate-x-4" },
    md: { track: "w-11 h-6", thumb: "w-5 h-5", translate: "translate-x-5" },
    lg: { track: "w-14 h-7", thumb: "w-6 h-6", translate: "translate-x-7" },
  };

  // 🆕 M3 EXPRESSIVE: Color variants
  const colorClasses = {
    primary: colors.primary.default,
    secondary: colors.secondary.default,
    tertiary: colors.tertiary.default,
    success: colors.success.default,
    error: colors.error.default,
  };

  // 🆕 M3 EXPRESSIVE: Spring animation
  const springStyle = bouncy
    ? `transition: transform 400ms cubic-bezier(0.34, 1.56, 0.64, 1), background-color 300ms ease;`
    : `transition: transform 300ms cubic-bezier(0.4, 0, 0.2, 1), background-color 300ms ease;`;
</script>

<label
  class={`inline-flex items-center cursor-pointer select-none ${customClass}`}
>
  <input
    type="checkbox"
    bind:checked
    {disabled}
    class="sr-only peer"
    {onchange}
  />

  <!-- Track -->
  <div
    class={`
      relative
      ${sizeClasses[size].track}
      ${shape.full}
      ${checked ? colorClasses[color] : "bg-gray-300 dark:bg-gray-600"}
      ${disabled ? "opacity-50 cursor-not-allowed" : ""}
      ${elevation[1]}
      transition-all duration-300
      hover:scale-105
      active:scale-95
    `}
  >
    <!-- Thumb -->
    <span
      class={`
        absolute top-0.5 left-0.5
        bg-white dark:bg-gray-100
        ${sizeClasses[size].thumb}
        ${shape.full}
        ${elevation[2]}
        ${checked ? sizeClasses[size].translate : "translate-x-0"}
        transform-gpu
      `}
      style={springStyle}
    ></span>
  </div>

  {#if children}
    <span
      class={`
      ml-3 text-sm
      ${emphasized ? `${typographyEmphasized.label.large} text-gray-900 dark:text-white` : "font-medium text-gray-700 dark:text-gray-300"}
    `}
    >
      {@render children()}
    </span>
  {/if}
</label>

<style>
  /* M3 Expressive: Bounce on toggle */
  label:has(.peer:checked) span {
    animation: toggleBounce 500ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes toggleBounce {
    0% {
      transform: translateX(0) scale(1);
    }
    50% {
      transform: translateX(var(--translate)) scale(1.15);
    }
    100% {
      transform: translateX(var(--translate)) scale(1);
    }
  }

  /* Track scale on hover */
  label:hover:not(:has(.peer:disabled)) div {
    transform: scale(1.05);
  }
</style>
