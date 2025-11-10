<script lang="ts">
  interface Props {
    variant?: "text" | "rect" | "circle" | "avatar";
    width?: string;
    height?: string;
    class?: string;
  }

  let {
    variant = "text",
    width,
    height,
    class: customClass = "",
  }: Props = $props();

  const variantClasses = {
    text: "h-4 w-full rounded",
    rect: "w-full h-full rounded-lg",
    circle: "rounded-full",
    avatar: "w-12 h-12 rounded-full",
  };

  const baseClasses = "animate-pulse bg-gray-200 dark:bg-gray-700";
</script>

<div
  class={`
    ${baseClasses}
    ${variantClasses[variant]}
    ${customClass}
  `}
  style={`${width ? `width: ${width};` : ""} ${height ? `height: ${height};` : ""}`}
  aria-label="Loading..."
  role="status"
>
  <span class="sr-only">Loading...</span>
</div>

<style>
  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>
