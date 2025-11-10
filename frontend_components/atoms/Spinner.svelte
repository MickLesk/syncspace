<script lang="ts">
  interface Props {
    size?: "sm" | "md" | "lg" | "xl";
    variant?: "primary" | "secondary" | "success" | "danger" | "warning";
    class?: string;
  }

  let {
    size = "md",
    variant = "primary",
    class: customClass = "",
  }: Props = $props();

  const sizeClasses = {
    sm: "w-4 h-4 border-2",
    md: "w-8 h-8 border-2",
    lg: "w-12 h-12 border-3",
    xl: "w-16 h-16 border-4",
  };

  const variantClasses = {
    primary: "border-blue-500 border-t-transparent",
    secondary: "border-gray-500 border-t-transparent",
    success: "border-green-500 border-t-transparent",
    danger: "border-red-500 border-t-transparent",
    warning: "border-yellow-500 border-t-transparent",
  };
</script>

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
