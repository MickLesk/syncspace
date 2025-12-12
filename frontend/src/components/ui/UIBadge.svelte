<!-- UIBadge.svelte - Status badge component -->
<script>
  let {
    variant = "default",
    size = "md",
    children,
    class: className = "",
  } = $props();

  const variantClasses = {
    default: "bg-gray-500/20 text-gray-300 border-gray-400/30",
    primary: "bg-blue-500/20 text-blue-300 border-blue-400/30",
    success: "bg-green-500/20 text-green-300 border-green-400/30",
    warning: "bg-yellow-500/20 text-yellow-300 border-yellow-400/30",
    error: "bg-red-500/20 text-red-300 border-red-400/30",
    info: "bg-cyan-500/20 text-cyan-300 border-cyan-400/30",
  };

  const sizeClasses = {
    sm: "px-2 py-0.5 text-xs",
    md: "px-2.5 py-1 text-sm",
    lg: "px-3 py-1.5 text-base",
  };
</script>

<span
  class="inline-flex items-center rounded-full border font-medium backdrop-blur-sm {variantClasses[
    variant
  ]} {sizeClasses[size]} {className}"
>
  {@render children?.()}
</span>
