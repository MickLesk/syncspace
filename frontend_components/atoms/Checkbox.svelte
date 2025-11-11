<script lang="ts">
  import {
    springs,
    shape,
    shapeExpressive,
    colors,
    typographyEmphasized,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  type ShapeVariant = "normal" | "extra-rounded" | "squircle";
  type CheckboxSize = "sm" | "md" | "lg";
  type CheckboxColor = "primary" | "success" | "error" | "secondary";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    color?: CheckboxColor;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold label
    size?: CheckboxSize;
    shapeStyle?: ShapeVariant; // rounded vs squircle
    bouncy?: boolean; // Spring check animation

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
    shapeStyle = "normal",
    bouncy = true,
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Size variants
  const sizeClasses = {
    sm: "w-4 h-4",
    md: "w-5 h-5",
    lg: "w-6 h-6",
  };

  // 🆕 M3 EXPRESSIVE: Shape variants
  const shapeClasses = {
    normal: shape.md,
    "extra-rounded": shape.lg,
    squircle: shapeExpressive["squircle-sm"],
  };

  // 🆕 M3 EXPRESSIVE: Color variants
  const colorClasses = {
    primary: colors.primary.default,
    success: colors.success.default,
    error: colors.error.default,
    secondary: colors.secondary.default,
  };

  const springClass = bouncy
    ? "transition-all duration-400 ease-[cubic-bezier(0.34,1.56,0.64,1)]"
    : "transition-all duration-200 ease-in-out";
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

  <div
    class={`
      ${sizeClasses[size]}
      ${shapeClasses[shapeStyle]}
      ${checked ? colorClasses[color] : "bg-gray-200 dark:bg-gray-700"}
      border-2
      ${checked ? "border-transparent" : "border-gray-300 dark:border-gray-600"}
      ${disabled ? "opacity-50 cursor-not-allowed" : ""}
      ${springClass}
      flex items-center justify-center
      hover:scale-110
      active:scale-90
    `}
  >
    {#if checked}
      <svg
        class="w-full h-full text-white p-0.5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        style="animation: checkBounce 400ms cubic-bezier(0.34, 1.56, 0.64, 1);"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="3"
          d="M5 13l4 4L19 7"
        />
      </svg>
    {/if}
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
  @keyframes checkBounce {
    0% {
      transform: scale(0) rotate(-45deg);
      opacity: 0;
    }
    50% {
      transform: scale(1.2) rotate(5deg);
      opacity: 1;
    }
    100% {
      transform: scale(1) rotate(0deg);
      opacity: 1;
    }
  }
</style>
