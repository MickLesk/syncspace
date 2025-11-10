<script lang="ts">
  interface Props {
    size?: "sm" | "md" | "lg" | "xl";
    variant?: "primary" | "secondary" | "success" | "danger" | "warning";
    type?: "spin" | "dots" | "pulse" | "bounce";
    class?: string;
  }

  let {
    size = "md",
    variant = "primary",
    type = "spin",
    class: customClass = "",
  }: Props = $props();

  const sizeClasses = {
    sm: "w-4 h-4 border-2",
    md: "w-8 h-8 border-2",
    lg: "w-12 h-12 border-3",
    xl: "w-16 h-16 border-4",
  };

  const dotSizeClasses = {
    sm: "w-1.5 h-1.5",
    md: "w-2.5 h-2.5",
    lg: "w-4 h-4",
    xl: "w-6 h-6",
  };

  const variantClasses = {
    primary: "border-blue-500 border-t-transparent",
    secondary: "border-gray-500 border-t-transparent",
    success: "border-green-500 border-t-transparent",
    danger: "border-red-500 border-t-transparent",
    warning: "border-yellow-500 border-t-transparent",
  };

  const variantBgClasses = {
    primary: "bg-blue-500",
    secondary: "bg-gray-500",
    success: "bg-green-500",
    danger: "bg-red-500",
    warning: "bg-yellow-500",
  };
</script>

{#if type === "spin"}
  <!-- Classic spinning circle -->
  <div
    class={`
      inline-block rounded-full animate-spin
      ${sizeClasses[size]}
      ${variantClasses[variant]}
      ${customClass}
    `}
    role="status"
    aria-label="Loading"
  >
    <span class="sr-only">Loading...</span>
  </div>
{:else if type === "dots"}
  <!-- Three bouncing dots -->
  <div
    class={`inline-flex gap-1.5 ${customClass}`}
    role="status"
    aria-label="Loading"
  >
    <div
      class={`${dotSizeClasses[size]} ${variantBgClasses[variant]} rounded-full`}
      style="animation: bounce 1.4s infinite ease-in-out both; animation-delay: -0.32s;"
    ></div>
    <div
      class={`${dotSizeClasses[size]} ${variantBgClasses[variant]} rounded-full`}
      style="animation: bounce 1.4s infinite ease-in-out both; animation-delay: -0.16s;"
    ></div>
    <div
      class={`${dotSizeClasses[size]} ${variantBgClasses[variant]} rounded-full`}
      style="animation: bounce 1.4s infinite ease-in-out both;"
    ></div>
    <span class="sr-only">Loading...</span>
  </div>
{:else if type === "pulse"}
  <!-- Pulsing circle -->
  <div
    class={`
      inline-block rounded-full
      ${sizeClasses[size]}
      ${variantBgClasses[variant]}
      ${customClass}
    `}
    style="animation: pulse-scale 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;"
    role="status"
    aria-label="Loading"
  >
    <span class="sr-only">Loading...</span>
  </div>
{:else if type === "bounce"}
  <!-- Bouncing bars -->
  <div
    class={`inline-flex gap-1 items-end ${customClass}`}
    role="status"
    aria-label="Loading"
  >
    {#each [0, 1, 2, 3] as i}
      <div
        class={`${dotSizeClasses[size].split(" ")[0]} ${variantBgClasses[variant]} rounded-sm`}
        style="height: {size === 'sm'
          ? '8px'
          : size === 'md'
            ? '16px'
            : size === 'lg'
              ? '24px'
              : '32px'}; animation: bar-bounce 1.2s infinite ease-in-out; animation-delay: {i *
          0.1}s;"
      ></div>
    {/each}
    <span class="sr-only">Loading...</span>
  </div>
{/if}

<style>
  @keyframes bounce {
    0%,
    80%,
    100% {
      transform: scale(0);
      opacity: 0.5;
    }
    40% {
      transform: scale(1);
      opacity: 1;
    }
  }

  @keyframes pulse-scale {
    0%,
    100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.5);
      opacity: 0.3;
    }
  }

  @keyframes bar-bounce {
    0%,
    40%,
    100% {
      transform: scaleY(0.4);
    }
    20% {
      transform: scaleY(1);
    }
  }
</style>
