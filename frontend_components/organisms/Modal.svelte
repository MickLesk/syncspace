<script lang="ts">
  import {
    radius,
    shadows,
    transitions,
    typographyEmphasized,
    shapeExpressive,
    springs,
    motionPresets,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    title?: string;
    size?: "sm" | "md" | "lg" | "xl" | "2xl";
    closeButton?: boolean;
    glass?: boolean;
    emphasized?: boolean; // M3 Expressive: Bold title
    shapeStyle?: keyof typeof shapeExpressive; // M3 Expressive: Shape variety
    heroMoment?: boolean; // M3 Expressive: Dramatic entrance
    motion?: keyof typeof springs.spatial; // M3 Expressive: Spring physics
    children?: Snippet;
    footer?: Snippet;
  }

  let {
    open = false,
    title,
    size = "md",
    closeButton = true,
    glass = false,
    emphasized = false,
    shapeStyle = "extra-large",
    heroMoment = false,
    motion = "bouncy",
    children,
    footer,
  }: Props = $props();

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-md",
    lg: "max-w-lg",
    xl: "max-w-2xl",
    "2xl": "max-w-4xl",
  };

  function close() {
    open = false;
  }

  // M3 Expressive: Enhanced glassmorphism
  const modalClasses = glass
    ? "bg-white/80 dark:bg-gray-900/80 backdrop-blur-2xl border border-white/20 dark:border-gray-700/20"
    : "bg-white dark:bg-gray-800";

  // M3 Expressive: Shape styles
  const shapeClass =
    shapeExpressive[shapeStyle] || shapeExpressive["extra-large"];

  // M3 Expressive: Title typography
  const titleClass = emphasized
    ? typographyEmphasized.headline.large
    : "text-lg font-semibold";

  // M3 Expressive: Animation timing
  const springCurve = springs.spatial[motion];
  const animationDuration = heroMoment ? "600ms" : "400ms";
</script>

{#if open}
  <!-- Backdrop with enhanced blur -->
  <div
    class={`
      fixed inset-0 z-40
      ${heroMoment ? "bg-black/70" : "bg-black/60"}
      backdrop-blur-sm
    `}
    style={`animation: fadeIn ${springs.duration.normal} ${springs.effects.fade};`}
    onclick={close}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Escape" && close()}
    aria-label="Close modal backdrop"
  ></div>

  <!-- Modal with M3 Expressive animations -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    style={`animation: modalSlideIn ${animationDuration} ${springCurve};`}
    onclick={(e) => e.stopPropagation()}
    role="presentation"
  >
    <div
      class={`
        ${modalClasses}
        ${shapeClass}
        ${heroMoment ? shadows["2xl"] : shadows.xl}
        w-full ${sizeClasses[size]}
        ${transitions.smooth}
        ${heroMoment ? "transform hover:scale-[1.01]" : ""}
      `}
      role="dialog"
      aria-modal="true"
    >
      <!-- Header with M3 Expressive typography -->
      {#if title || closeButton}
        <div
          class={`
            flex items-center justify-between px-6 py-4
            border-b border-gray-200 dark:border-gray-700
            ${emphasized ? "bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-950 dark:to-purple-950" : ""}
          `}
        >
          {#if title}
            <h2
              class={`
                ${titleClass}
                text-gray-900 dark:text-gray-100
                ${emphasized ? "bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent" : ""}
              `}
            >
              {title}
            </h2>
          {/if}
          {#if closeButton}
            <button
              onclick={close}
              class={`
                text-gray-500 dark:text-gray-400
                hover:text-gray-700 dark:hover:text-gray-200
                hover:bg-gray-100 dark:hover:bg-gray-700
                focus:outline-none focus:ring-2 focus:ring-blue-500
                ${shapeExpressive.large} p-2
                ${transitions.smooth}
                hover:scale-110 active:scale-95
              `}
              aria-label="Close modal"
            >
              <i class="bi bi-x text-xl"></i>
            </button>
          {/if}
        </div>
      {/if}

      <!-- Content with enhanced spacing -->
      <div class={`px-6 ${emphasized ? "py-6" : "py-4"}`}>
        {#if children}
          {@render children()}
        {/if}
      </div>

      <!-- Footer with M3 Expressive styling -->
      {#if footer}
        <div
          class={`
            px-6 py-4
            border-t border-gray-200 dark:border-gray-700
            flex justify-end gap-3
            ${emphasized ? "bg-gradient-to-r from-blue-50/50 to-purple-50/50 dark:from-blue-950/50 dark:to-purple-950/50" : ""}
          `}
        >
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* M3 Expressive: Enhanced fade-in for backdrop */
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* M3 Expressive: Spring slide-in with hero moment support */
  @keyframes modalSlideIn {
    0% {
      opacity: 0;
      transform: translateY(-3rem) scale(0.92);
    }
    60% {
      opacity: 1;
      transform: translateY(0.5rem) scale(1.02);
    }
    100% {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Prevent body scroll when modal is open */
  :global(body:has(div[role="dialog"])) {
    overflow: hidden;
  }
</style>
