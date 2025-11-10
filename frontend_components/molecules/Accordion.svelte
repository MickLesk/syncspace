<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    title?: string;
    icon?: string;
    variant?: "default" | "bordered" | "filled";
    class?: string;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    title = "",
    icon,
    variant = "default",
    class: customClass = "",
    children,
  }: Props = $props();

  const variantStyles = {
    default: {
      container: "border-b border-gray-200 dark:border-gray-700",
      header: "hover:bg-gray-50 dark:hover:bg-gray-800/50",
      content: "py-4",
    },
    bordered: {
      container:
        "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden",
      header: "hover:bg-gray-50 dark:hover:bg-gray-800/50",
      content: "p-4 border-t border-gray-200 dark:border-gray-700",
    },
    filled: {
      container:
        "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden",
      header:
        "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700",
      content: "p-4 bg-white dark:bg-gray-900",
    },
  };

  const styles = variantStyles[variant];
</script>

<div class={`${styles.container} ${customClass}`}>
  <!-- Header -->
  <button
    type="button"
    onclick={() => (open = !open)}
    class={`
      w-full flex items-center justify-between px-4 py-3
      text-left font-medium transition-colors
      ${styles.header}
    `}
    aria-expanded={open}
  >
    <div class="flex items-center gap-3">
      {#if icon}
        <i class={`bi ${icon} text-gray-600 dark:text-gray-400`}></i>
      {/if}
      <span class="text-gray-900 dark:text-gray-100">{title}</span>
    </div>

    <i
      class={`
        bi bi-chevron-down text-gray-600 dark:text-gray-400
        transition-transform duration-200
        ${open ? "rotate-180" : ""}
      `}
    ></i>
  </button>

  <!-- Content -->
  {#if open && children}
    <div
      class={`
        ${styles.content}
        text-gray-700 dark:text-gray-300
        animate-in slide-in-from-top-2 fade-in duration-200
      `}
    >
      {@render children()}
    </div>
  {/if}
</div>
