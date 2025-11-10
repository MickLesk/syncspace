<script lang="ts">
  interface Props {
    disabled?: boolean;
    variant?: "primary" | "danger" | "success";
    size?: "sm" | "md" | "lg";
    icon?: string;
    onchange?: (event: Event) => void;
  }

  let {
    disabled = false,
    variant = "primary",
    size = "md",
    icon,
    onchange,
  }: Props = $props();

  const sizeClasses = {
    sm: "w-4 h-4",
    md: "w-6 h-6",
    lg: "w-8 h-8",
  };

  const variantClasses = {
    primary: "accent-blue-500",
    danger: "accent-red-500",
    success: "accent-green-500",
  };
</script>

<div class="flex items-center">
  <input
    type="checkbox"
    {disabled}
    class={`rounded cursor-pointer ${sizeClasses[size]} ${variantClasses[variant]}`}
    {onchange}
  />
  <label class="ml-2 text-sm text-gray-700 cursor-pointer select-none">
    <slot />
  </label>
</div>
