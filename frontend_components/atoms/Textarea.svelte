<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    placeholder?: string;
    rows?: number;
    disabled?: boolean;
    error?: boolean;
    class?: string;
    value?: string;
    oninput?: (event: Event) => void;
    onchange?: (event: Event) => void;
    onblur?: (event: Event) => void;
    children?: Snippet;
  }

  let {
    placeholder = "",
    rows = 4,
    disabled = false,
    error = false,
    class: customClass = "",
    value = $bindable(""),
    oninput,
    onchange,
    onblur,
    children,
  }: Props = $props();
</script>

<div class={`w-full ${customClass}`}>
  {#if children}
    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
      {@render children()}
    </label>
  {/if}

  <textarea
    bind:value
    {placeholder}
    {rows}
    {disabled}
    {oninput}
    {onchange}
    {onblur}
    class={`
      w-full px-4 py-3 border rounded-lg resize-y
      focus:outline-none focus:ring-2 transition-all
      disabled:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50
      ${
        error
          ? "border-red-500 focus:ring-red-500/20 focus:border-red-500"
          : "border-gray-300 focus:ring-blue-500/20 focus:border-blue-500"
      }
      text-gray-900 dark:text-gray-100 dark:bg-gray-800 dark:border-gray-600
      placeholder-gray-400 dark:placeholder-gray-500
    `}
  />
</div>
