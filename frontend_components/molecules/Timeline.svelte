<script lang="ts">
  interface TimelineItem {
    id: string;
    title: string;
    description?: string;
    timestamp: string;
    icon?: string;
    variant?: "default" | "primary" | "success" | "danger" | "warning";
  }

  interface Props {
    items: TimelineItem[];
    class?: string;
  }

  let { items, class: customClass = "" }: Props = $props();

  const variantClasses = {
    default: {
      dot: "bg-gray-400 dark:bg-gray-600",
      line: "bg-gray-200 dark:bg-gray-700",
      icon: "text-gray-600 dark:text-gray-400",
    },
    primary: {
      dot: "bg-blue-500 shadow-lg shadow-blue-500/50",
      line: "bg-blue-200 dark:bg-blue-900",
      icon: "text-blue-500",
    },
    success: {
      dot: "bg-green-500 shadow-lg shadow-green-500/50",
      line: "bg-green-200 dark:bg-green-900",
      icon: "text-green-500",
    },
    danger: {
      dot: "bg-red-500 shadow-lg shadow-red-500/50",
      line: "bg-red-200 dark:bg-red-900",
      icon: "text-red-500",
    },
    warning: {
      dot: "bg-amber-500 shadow-lg shadow-amber-500/50",
      line: "bg-amber-200 dark:bg-amber-900",
      icon: "text-amber-500",
    },
  };
</script>

<div class={`relative ${customClass}`}>
  {#each items as item, index}
    {@const variant = item.variant || "default"}
    {@const classes = variantClasses[variant]}

    <div class="relative flex gap-4 pb-8">
      <!-- Timeline line -->
      {#if index < items.length - 1}
        <div
          class={`absolute left-4 top-10 bottom-0 w-0.5 ${classes.line}`}
        ></div>
      {/if}

      <!-- Icon/Dot -->
      <div class="relative flex-shrink-0">
        <div
          class={`
          w-8 h-8 rounded-full flex items-center justify-center
          ${classes.dot}
          transition-all duration-200 hover:scale-110
        `}
        >
          {#if item.icon}
            <i class="bi {item.icon} text-white text-sm"></i>
          {:else}
            <div class="w-2 h-2 bg-white rounded-full"></div>
          {/if}
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        <div
          class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700
                    transition-all duration-200 hover:shadow-lg hover:scale-[1.02]"
        >
          <div class="flex items-start justify-between mb-2">
            <h4 class="font-semibold text-gray-900 dark:text-white">
              {item.title}
            </h4>
            <span
              class="text-xs text-gray-500 dark:text-gray-400 flex-shrink-0 ml-4"
            >
              {item.timestamp}
            </span>
          </div>

          {#if item.description}
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {item.description}
            </p>
          {/if}
        </div>
      </div>
    </div>
  {/each}
</div>
