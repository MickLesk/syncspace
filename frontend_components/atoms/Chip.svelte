<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?:
      | "default"
      | "primary"
      | "success"
      | "danger"
      | "warning"
      | "info"
      | "glass";
    size?: "sm" | "md" | "lg";
    removable?: boolean;
    icon?: string;
    class?: string;
    onremove?: () => void;
    children?: Snippet;
  }

  let {
    variant = "default",
    size = "md",
    removable = false,
    icon,
    class: customClass = "",
    onremove,
    children,
  }: Props = $props();

  const sizeClasses = {
    sm: "text-xs px-2 py-1 gap-1",
    md: "text-sm px-3 py-1.5 gap-1.5",
    lg: "text-base px-4 py-2 gap-2",
  };

  const variantClasses = {
    default:
      "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700",
    primary:
      "bg-gradient-to-br from-blue-500 to-blue-600 text-white shadow-lg shadow-blue-500/30 hover:shadow-xl hover:shadow-blue-500/40",
    success:
      "bg-gradient-to-br from-green-500 to-emerald-600 text-white shadow-lg shadow-green-500/30 hover:shadow-xl hover:shadow-green-500/40",
    danger:
      "bg-gradient-to-br from-red-500 to-red-600 text-white shadow-lg shadow-red-500/30 hover:shadow-xl hover:shadow-red-500/40",
    warning:
      "bg-gradient-to-br from-amber-500 to-orange-500 text-white shadow-lg shadow-amber-500/30 hover:shadow-xl hover:shadow-amber-500/40",
    info: "bg-gradient-to-br from-cyan-500 to-blue-500 text-white shadow-lg shadow-cyan-500/30 hover:shadow-xl hover:shadow-cyan-500/40",
    glass:
      "bg-white/10 backdrop-blur-md text-gray-800 dark:text-white border border-white/20 shadow-lg hover:bg-white/20",
  };
</script>

<div
  class={`
    inline-flex items-center font-medium rounded-full
    transition-all duration-200
    ${sizeClasses[size]}
    ${variantClasses[variant]}
    ${customClass}
  `}
>
  {#if icon}
    <i class="bi {icon}"></i>
  {/if}

  {#if children}
    <span>{@render children()}</span>
  {/if}

  {#if removable}
    <button
      onclick={onremove}
      class="ml-1 hover:bg-black/10 dark:hover:bg-white/10 rounded-full p-0.5 transition-colors"
      aria-label="Remove"
    >
      <i class="bi bi-x text-sm"></i>
    </button>
  {/if}
</div>
