<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    name: string;
    value: string;
    group?: string;
    disabled?: boolean;
    variant?: "primary" | "danger" | "success";
    size?: "sm" | "md" | "lg";
    class?: string;
    onchange?: (event: Event) => void;
    children?: Snippet;
  }

  let {
    name,
    value,
    group = $bindable(""),
    disabled = false,
    variant = "primary",
    size = "md",
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  // Derived state to check if this radio is selected
  const isChecked = $derived(group === value);

  const sizeClasses = {
    sm: { box: "w-4 h-4", inner: "w-2 h-2" },
    md: { box: "w-5 h-5", inner: "w-2.5 h-2.5" },
    lg: { box: "w-6 h-6", inner: "w-3 h-3" },
  };

  const variantClasses = {
    primary: {
      border: "border-blue-300 hover:border-blue-400",
      bg: "bg-blue-500",
      ring: "focus:ring-blue-500/20",
    },
    danger: {
      border: "border-red-300 hover:border-red-400",
      bg: "bg-red-500",
      ring: "focus:ring-red-500/20",
    },
    success: {
      border: "border-green-300 hover:border-green-400",
      bg: "bg-green-500",
      ring: "focus:ring-green-500/20",
    },
  };

  const currentVariant = variantClasses[variant] || variantClasses.primary;
  const currentSize = sizeClasses[size] || sizeClasses.md;
</script>

<label
  class={`inline-flex items-center cursor-pointer select-none group ${customClass} ${
    disabled ? "opacity-50 cursor-not-allowed" : ""
  }`}
>
  <!-- Hidden native radio for accessibility -->
  <input
    type="radio"
    {name}
    {value}
    bind:group
    {disabled}
    {onchange}
    class="sr-only peer"
  />

  <!-- Custom radio visual -->
  <div
    class={`
      relative flex items-center justify-center
      ${currentSize.box}
      border-2 rounded-full
      transition-all duration-200 ease-in-out
      ${
        isChecked
          ? `${currentVariant.bg} border-transparent`
          : `bg-white border-gray-300 ${currentVariant.border}`
      }
      ${disabled ? "" : "group-hover:shadow-md"}
      focus-within:ring-4 ${currentVariant.ring}
    `}
  >
    <!-- Inner dot with animation -->
    <div
      class={`
        ${currentSize.inner}
        bg-white rounded-full
        transition-all duration-200
        ${isChecked ? "scale-100 opacity-100" : "scale-0 opacity-0"}
      `}
    ></div>
  </div>

  <!-- Label text -->
  {#if children}
    <span class="ml-3 text-sm text-gray-700 dark:text-gray-300">
      {@render children()}
    </span>
  {/if}
</label>
