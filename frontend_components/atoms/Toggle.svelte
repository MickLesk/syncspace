<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    variant?: "primary" | "danger" | "success" | "warning";
    class?: string;
    onchange?: (event: Event) => void;
    children?: Snippet;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    variant = "primary",
    class: customClass = "",
    onchange,
    children,
  }: Props = $props();

  const variantClasses = {
    primary: "bg-blue-500",
    danger: "bg-red-500",
    success: "bg-green-500",
    warning: "bg-yellow-500",
  };
</script>

<label
  class={`inline-flex items-center cursor-pointer select-none ${customClass}`}
>
  <input type="checkbox" bind:checked {disabled} class="sr-only" {onchange} />
  <div
    class={`
      relative w-11 h-6 rounded-full transition-colors duration-200
      ${checked ? variantClasses[variant] : "bg-gray-300"}
      ${disabled ? "opacity-50 cursor-not-allowed" : ""}
    `}
  >
    <span
      class={`
        absolute top-1 left-1 bg-white w-4 h-4 rounded-full shadow-md
        transition-transform duration-200 ease-in-out
        ${checked ? "translate-x-5" : "translate-x-0"}
      `}
    />
  </div>
  {#if children}
    <span class="ml-3 text-sm text-gray-700 dark:text-gray-300">
      {@render children()}
    </span>
  {/if}
</label>
