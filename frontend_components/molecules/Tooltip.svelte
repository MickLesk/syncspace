<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    position?: "top" | "bottom" | "left" | "right";
    class?: string;
    children?: Snippet;
    content?: Snippet;
  }

  let {
    open = $bindable(false),
    position = "top",
    class: customClass = "",
    children,
    content,
  }: Props = $props();

  const positionClasses = {
    top: "bottom-full left-1/2 -translate-x-1/2 mb-2",
    bottom: "top-full left-1/2 -translate-x-1/2 mt-2",
    left: "right-full top-1/2 -translate-y-1/2 mr-2",
    right: "left-full top-1/2 -translate-y-1/2 ml-2",
  };

  const arrowClasses = {
    top: "top-full left-1/2 -translate-x-1/2 border-t-gray-900 border-x-transparent border-b-transparent",
    bottom:
      "bottom-full left-1/2 -translate-x-1/2 border-b-gray-900 border-x-transparent border-t-transparent",
    left: "left-full top-1/2 -translate-y-1/2 border-l-gray-900 border-y-transparent border-r-transparent",
    right:
      "right-full top-1/2 -translate-y-1/2 border-r-gray-900 border-y-transparent border-l-transparent",
  };
</script>

<div class={`relative inline-block ${customClass}`}>
  <!-- Trigger -->
  <div
    onmouseenter={() => (open = true)}
    onmouseleave={() => (open = false)}
  >
    {#if children}
      {@render children()}
    {/if}
  </div>

  <!-- Tooltip -->
  {#if open && content}
    <div
      class={`
        absolute z-50 px-3 py-2 text-sm text-white bg-gray-900 rounded-lg shadow-lg
        whitespace-nowrap pointer-events-none
        ${positionClasses[position]}
      `}
      role="tooltip"
    >
      {@render content()}

      <!-- Arrow -->
      <div
        class={`
          absolute w-0 h-0 border-4
          ${arrowClasses[position]}
        `}
      />
    </div>
  {/if}
</div>
