<script lang="ts">
  import {
    shape,
    shapeExpressive,
    springs,
    outline,
    colors,
    stateLayers,
    typographyEmphasized,
  } from "../shared/index.js";

  type ShapeVariant = "normal" | "extra-rounded" | "squircle";
  type InputSize = "sm" | "md" | "lg";

  interface Props {
    value?: string;
    placeholder?: string;
    label?: string;
    id?: string;
    helperText?: string;
    error?: boolean;
    disabled?: boolean;
    type?: string;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold label typography
    shapeStyle?: ShapeVariant; // Shape variety
    size?: InputSize;

    class?: string;
    oninput?: (event: Event) => void;
  }

  let {
    value = $bindable(""),
    placeholder = "",
    label = "",
    id = "",
    helperText = "",
    error = false,
    disabled = false,
    type = "text",
    emphasized = false,
    shapeStyle = "normal",
    size = "md",
    class: customClass = "",
    oninput,
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Shape variety
  const shapeClasses = {
    normal: shape.lg,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-md"],
  };

  // 🆕 M3 EXPRESSIVE: Size variants
  const sizeClasses = {
    sm: "h-9 px-3 text-sm",
    md: "h-11 px-4 text-base",
    lg: "h-14 px-5 text-lg",
  };

  // 🆕 M3 EXPRESSIVE: Label emphasis
  const labelClasses = emphasized
    ? `${typographyEmphasized.label.large} mb-2 text-gray-900 dark:text-white`
    : "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2";
</script>

<div class={customClass}>
  {#if label}
    <label for={id} class={`block ${labelClasses}`}>
      {label}
    </label>
  {/if}

  <input
    {id}
    {type}
    bind:value
    {placeholder}
    {disabled}
    {oninput}
    class={`
      w-full
      ${sizeClasses[size]}
      ${shapeClasses[shapeStyle]}
      ${outline.default}
      ${error ? outline.error : colors.primary.focus}
      ${stateLayers.hover}
      bg-white dark:bg-gray-800
      text-gray-900 dark:text-white
      placeholder:text-gray-400 dark:placeholder:text-gray-500
      disabled:opacity-50 disabled:cursor-not-allowed
      transition-all duration-300 ease-[${springs.effects.color}]
      focus:scale-[1.01]
      focus:shadow-lg
    `}
  />

  {#if helperText}
    <p
      class={`
      text-xs mt-2
      ${error ? "text-red-600 dark:text-red-400" : "text-gray-500 dark:text-gray-400"}
      ${emphasized ? "font-semibold" : ""}
    `}
    >
      {helperText}
    </p>
  {/if}
</div>

<style>
  /* M3 Expressive: Spring focus animation */
  input:focus {
    transition: all 300ms cubic-bezier(0.34, 1.56, 0.64, 1);
  }
</style>
