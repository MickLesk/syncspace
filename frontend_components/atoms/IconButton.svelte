<script lang="ts">
  interface Props {
    icon: string;
    variant?: "default" | "primary" | "danger" | "success" | "ghost" | "glass";
    size?: "sm" | "md" | "lg" | "xl";
    rounded?: boolean;
    disabled?: boolean;
    loading?: boolean;
    class?: string;
    onclick?: (event: MouseEvent) => void;
    ariaLabel?: string;
  }

  let {
    icon,
    variant = "default",
    size = "md",
    rounded = false,
    disabled = false,
    loading = false,
    class: customClass = "",
    onclick,
    ariaLabel,
  }: Props = $props();

  const sizeClasses = {
    sm: "w-8 h-8 text-sm",
    md: "w-10 h-10 text-base",
    lg: "w-12 h-12 text-lg",
    xl: "w-14 h-14 text-xl",
  };

  const variantClasses = {
    default:
      "bg-gray-100 text-gray-700 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700",
    primary:
      "bg-blue-500 text-white hover:bg-blue-600 shadow-lg shadow-blue-500/30 hover:shadow-xl",
    danger:
      "bg-red-500 text-white hover:bg-red-600 shadow-lg shadow-red-500/30 hover:shadow-xl",
    success:
      "bg-green-500 text-white hover:bg-green-600 shadow-lg shadow-green-500/30 hover:shadow-xl",
    ghost:
      "bg-transparent text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800",
    glass:
      "bg-white/10 backdrop-blur-md text-gray-800 dark:text-white border border-white/20 hover:bg-white/20",
  };
</script>

<button
  {onclick}
  {disabled}
  aria-label={ariaLabel}
  class={`
    inline-flex items-center justify-center
    font-medium transition-all duration-200
    active:scale-95
    ${rounded ? "rounded-full" : "rounded-lg"}
    ${sizeClasses[size]}
    ${variantClasses[variant]}
    ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}
    ${customClass}
  `}
>
  {#if loading}
    <i class="bi bi-arrow-repeat animate-spin"></i>
  {:else}
    <i class="bi {icon}"></i>
  {/if}
</button>
