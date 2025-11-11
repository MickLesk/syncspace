<script lang="ts">
  interface Props {
    variant?: "text" | "rect" | "circle" | "avatar" | "card" | "button";
    width?: string;
    height?: string;
    animated?: boolean;
    shimmer?: boolean;
    class?: string;
  }

  let {
    variant = "text",
    width,
    height,
    animated = true,
    shimmer = true,
    class: customClass = "",
  }: Props = $props();

  const variantClasses = {
    text: "h-4 w-full rounded",
    rect: "w-full h-full rounded-lg",
    circle: "rounded-full",
    avatar: "w-12 h-12 rounded-full",
    card: "w-full h-48 rounded-xl",
    button: "w-32 h-10 rounded-lg",
  };
</script>

<div
  class={`
    relative overflow-hidden
    bg-gray-200 dark:bg-gray-700
    ${animated ? "animate-pulse" : ""}
    ${variantClasses[variant]}
    ${customClass}
  `}
  style={`${width ? `width: ${width};` : ""} ${height ? `height: ${height};` : ""}`}
  aria-label="Loading..."
  role="status"
>
  {#if shimmer}
    <div
      class="absolute inset-0 -translate-x-full animate-shimmer bg-gradient-to-r from-transparent via-white/20 to-transparent"
    ></div>
  {/if}
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

  @keyframes shimmer {
    100% {
      transform: translateX(100%);
    }
  }

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  .animate-shimmer {
    animation: shimmer 2s infinite;
  }
</style>
