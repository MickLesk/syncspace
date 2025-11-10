<script lang="ts">
  interface Props {
    title: string;
    value: string | number;
    change?: number;
    icon?: string;
    variant?:
      | "default"
      | "primary"
      | "success"
      | "danger"
      | "warning"
      | "glass";
    trend?: "up" | "down" | "neutral";
    class?: string;
  }

  let {
    title,
    value,
    change,
    icon,
    variant = "default",
    trend = "neutral",
    class: customClass = "",
  }: Props = $props();

  const variantClasses = {
    default:
      "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700",
    primary:
      "bg-gradient-to-br from-blue-500 to-blue-600 text-white border-0 shadow-xl shadow-blue-500/30",
    success:
      "bg-gradient-to-br from-green-500 to-emerald-600 text-white border-0 shadow-xl shadow-green-500/30",
    danger:
      "bg-gradient-to-br from-red-500 to-red-600 text-white border-0 shadow-xl shadow-red-500/30",
    warning:
      "bg-gradient-to-br from-amber-500 to-orange-500 text-white border-0 shadow-xl shadow-amber-500/30",
    glass: "bg-white/10 backdrop-blur-md border border-white/20 text-white",
  };

  const iconVariantClasses = {
    default: "bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400",
    primary: "bg-white/20 text-white",
    success: "bg-white/20 text-white",
    danger: "bg-white/20 text-white",
    warning: "bg-white/20 text-white",
    glass: "bg-white/20 text-white",
  };

  const trendIcon = $derived(
    trend === "up"
      ? "bi-arrow-up"
      : trend === "down"
        ? "bi-arrow-down"
        : "bi-dash"
  );
  const trendColor = $derived(
    trend === "up"
      ? "text-green-500"
      : trend === "down"
        ? "text-red-500"
        : "text-gray-500"
  );
</script>

<div
  class={`
    rounded-xl p-6 transition-all duration-200
    hover:scale-105 hover:shadow-2xl
    ${variantClasses[variant]}
    ${customClass}
  `}
>
  <div class="flex items-start justify-between mb-4">
    <div>
      <p
        class={`text-sm font-medium ${variant === "default" ? "text-gray-600 dark:text-gray-400" : "text-white/80"}`}
      >
        {title}
      </p>
      <p
        class={`text-3xl font-bold mt-2 ${variant === "default" ? "text-gray-900 dark:text-white" : "text-white"}`}
      >
        {value}
      </p>
    </div>

    {#if icon}
      <div
        class={`
        w-12 h-12 rounded-xl flex items-center justify-center
        ${iconVariantClasses[variant]}
      `}
      >
        <i class="bi {icon} text-xl"></i>
      </div>
    {/if}
  </div>

  {#if change !== undefined}
    <div class="flex items-center gap-1 text-sm">
      <i class="bi {trendIcon} {trendColor}"></i>
      <span class={variant === "default" ? trendColor : "text-white/80"}>
        {Math.abs(change)}%
      </span>
      <span
        class={`ml-1 ${variant === "default" ? "text-gray-500 dark:text-gray-400" : "text-white/60"}`}
      >
        vs last period
      </span>
    </div>
  {/if}
</div>
