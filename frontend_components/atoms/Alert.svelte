<script lang="ts">
  import {
    shape,
    shapeExpressive,
    springs,
    colors,
    typographyEmphasized,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  type AlertType = "info" | "success" | "warning" | "error";
  type ShapeVariant = "normal" | "extra-rounded" | "squircle";

  interface Props {
    type?: AlertType;
    title?: string;
    message?: string;
    dismissible?: boolean;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold title/message
    shapeStyle?: ShapeVariant;
    iconSize?: "sm" | "md" | "lg";

    class?: string;
    onclose?: () => void;
    children?: Snippet;
  }

  let {
    type = "info",
    title = "",
    message = "",
    dismissible = false,
    emphasized = false,
    shapeStyle = "normal",
    iconSize = "md",
    class: customClass = "",
    onclose,
    children,
  }: Props = $props();

  const typeConfig = {
    info: {
      bg: "bg-blue-50 dark:bg-blue-950/20",
      border: "border-blue-500",
      text: "text-blue-900 dark:text-blue-100",
      icon: "bi-info-circle-fill text-blue-600",
    },
    success: {
      bg: "bg-green-50 dark:bg-green-950/20",
      border: "border-green-500",
      text: "text-green-900 dark:text-green-100",
      icon: "bi-check-circle-fill text-green-600",
    },
    warning: {
      bg: "bg-yellow-50 dark:bg-yellow-950/20",
      border: "border-yellow-500",
      text: "text-yellow-900 dark:text-yellow-100",
      icon: "bi-exclamation-triangle-fill text-yellow-600",
    },
    error: {
      bg: "bg-red-50 dark:bg-red-950/20",
      border: "border-red-500",
      text: "text-red-900 dark:text-red-100",
      icon: "bi-x-circle-fill text-red-600",
    },
  };

  const shapeClasses = {
    normal: shape.lg,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-lg"],
  };

  const iconSizeClasses = {
    sm: "text-lg",
    md: "text-xl",
    lg: "text-2xl",
  };

  const config = $derived(typeConfig[type] || typeConfig.info);
</script>

<div
  class={`
    p-4
    ${shapeClasses[shapeStyle]}
    ${config.bg}
    border-l-4 ${config.border}
    ${config.text}
    ${customClass}
    transition-all duration-400 ease-[${springs.spatial.bouncy}]
    animate-in
  `}
  style="animation: slideIn 400ms cubic-bezier(0.34, 1.56, 0.64, 1);"
>
  <div class="flex items-start gap-3">
    <i class={`${config.icon} ${iconSizeClasses[iconSize]} flex-shrink-0`}></i>

    <div class="flex-1">
      {#if title}
        <div
          class={emphasized
            ? typographyEmphasized.title.medium
            : "font-semibold mb-1"}
        >
          {title}
        </div>
      {/if}

      {#if message}
        <div class={`text-sm ${emphasized ? "font-semibold" : ""}`}>
          {message}
        </div>
      {/if}

      {#if children}
        <div class="mt-2">
          {@render children()}
        </div>
      {/if}
    </div>

    {#if dismissible}
      <button
        onclick={onclose}
        aria-label="Close alert"
        class="flex-shrink-0 text-current opacity-70 hover:opacity-100 transition-opacity"
      >
        <i class="bi bi-x text-xl"></i>
      </button>
    {/if}
  </div>
</div>

<style>
  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
