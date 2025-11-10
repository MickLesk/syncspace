<script lang="ts">
  interface Step {
    label: string;
    description?: string;
    icon?: string;
  }

  interface Props {
    steps: Step[];
    currentStep?: number;
    variant?: "default" | "primary" | "glass";
    orientation?: "horizontal" | "vertical";
    class?: string;
  }

  let {
    steps,
    currentStep = $bindable(0),
    variant = "primary",
    orientation = "horizontal",
    class: customClass = "",
  }: Props = $props();

  const variantClasses = {
    default: {
      active: "bg-gray-600 text-white border-gray-600",
      completed: "bg-green-500 text-white border-green-500",
      upcoming:
        "bg-gray-200 text-gray-600 border-gray-300 dark:bg-gray-700 dark:text-gray-400 dark:border-gray-600",
      line: "bg-gray-300 dark:bg-gray-600",
      lineCompleted: "bg-green-500",
    },
    primary: {
      active:
        "bg-blue-500 text-white border-blue-500 shadow-lg shadow-blue-500/50",
      completed: "bg-green-500 text-white border-green-500",
      upcoming:
        "bg-gray-200 text-gray-600 border-gray-300 dark:bg-gray-700 dark:text-gray-400 dark:border-gray-600",
      line: "bg-gray-300 dark:bg-gray-600",
      lineCompleted: "bg-blue-500",
    },
    glass: {
      active:
        "bg-white/20 backdrop-blur-md text-white border-white/40 shadow-lg",
      completed: "bg-green-500/30 backdrop-blur-md text-white border-green-400",
      upcoming: "bg-white/5 backdrop-blur-md text-white/60 border-white/20",
      line: "bg-white/20",
      lineCompleted: "bg-white/40",
    },
  };

  const classes = $derived(variantClasses[variant]);
</script>

<div
  class={`${orientation === "horizontal" ? "flex items-start" : "flex flex-col"} ${customClass}`}
>
  {#each steps as step, index}
    {@const isCompleted = index < currentStep}
    {@const isActive = index === currentStep}
    {@const isUpcoming = index > currentStep}

    <div
      class={`flex ${orientation === "horizontal" ? "flex-col items-center" : "flex-row"} flex-1`}
    >
      <div class="flex items-center">
        <!-- Step circle -->
        <button
          onclick={() => (currentStep = index)}
          class={`
            w-10 h-10 rounded-full border-2 flex items-center justify-center
            font-semibold transition-all duration-200
            hover:scale-110 active:scale-95
            ${isCompleted ? classes.completed : isActive ? classes.active : classes.upcoming}
          `}
        >
          {#if isCompleted}
            <i class="bi bi-check-lg text-lg"></i>
          {:else if step.icon}
            <i class="bi {step.icon}"></i>
          {:else}
            {index + 1}
          {/if}
        </button>

        <!-- Connector line -->
        {#if index < steps.length - 1}
          <div
            class={`
            ${orientation === "horizontal" ? "w-full h-0.5" : "w-0.5 h-12 ml-5"}
            relative overflow-hidden
          `}
          >
            <div class={`absolute inset-0 ${classes.line}`}></div>
            {#if index < currentStep}
              <div
                class={`absolute inset-0 ${classes.lineCompleted} transition-all duration-500`}
              ></div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Step label -->
      <div
        class={`${orientation === "horizontal" ? "mt-2 text-center" : "ml-4"}`}
      >
        <p
          class={`
          text-sm font-medium
          ${isActive ? "text-gray-900 dark:text-white" : "text-gray-600 dark:text-gray-400"}
        `}
        >
          {step.label}
        </p>
        {#if step.description}
          <p class="text-xs text-gray-500 dark:text-gray-500 mt-0.5">
            {step.description}
          </p>
        {/if}
      </div>
    </div>
  {/each}
</div>
