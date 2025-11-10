<script lang="ts">
  interface Props {
    checked?: boolean;
    disabled?: boolean;
    variant?: "primary" | "danger" | "success" | "warning";
    size?: "sm" | "md" | "lg";
    class?: string;
    onchange?: (event: Event) => void;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    variant = "primary",
    size = "md",
    class: customClass = "",
    onchange,
  }: Props = $props();

  const sizeClasses = {
    sm: { box: "w-4 h-4", check: "w-2.5 h-2.5" },
    md: { box: "w-5 h-5", check: "w-3 h-3" },
    lg: { box: "w-6 h-6", check: "w-3.5 h-3.5" },
  };

  const variantClasses = {
    primary: {
      border: "border-blue-300 hover:border-blue-400",
      bg: "bg-blue-500 hover:bg-blue-600",
      ring: "focus:ring-blue-500/20",
    },
    danger: {
      border: "border-red-300 hover:border-red-400",
      bg: "bg-red-500 hover:bg-red-600",
      ring: "focus:ring-red-500/20",
    },
    success: {
      border: "border-green-300 hover:border-green-400",
      bg: "bg-green-500 hover:bg-green-600",
      ring: "focus:ring-green-500/20",
    },
    warning: {
      border: "border-yellow-300 hover:border-yellow-400",
      bg: "bg-yellow-500 hover:bg-yellow-600",
      ring: "focus:ring-yellow-500/20",
    },
  };
</script>

<label
  class={`inline-flex items-center cursor-pointer select-none group ${customClass} ${
    disabled ? "opacity-50 cursor-not-allowed" : ""
  }`}
>
  <!-- Hidden native checkbox for accessibility -->
  <input
    type="checkbox"
    bind:checked
    {disabled}
    {onchange}
    class="sr-only peer"
  />

  <!-- Custom checkbox visual -->
  <div
    class={`
      relative flex items-center justify-center
      ${sizeClasses[size].box}
      border-2 rounded-md
      transition-all duration-200 ease-in-out
      ${
        checked
          ? `${variantClasses[variant].bg} border-transparent`
          : `bg-white border-gray-300 ${variantClasses[variant].border}`
      }
      ${disabled ? "" : "group-hover:shadow-md"}
      focus-within:ring-4 ${variantClasses[variant].ring}
    `}
  >
    <!-- Checkmark icon with animation -->
    <svg
      class={`
        ${sizeClasses[size].check}
        text-white transition-all duration-200
        ${checked ? "scale-100 opacity-100" : "scale-0 opacity-0"}
      `}
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="3"
        d="M5 13l4 4L19 7"
      />
    </svg>
  </div>

  <!-- Label text -->
  {#if $$slots.default}
    <span class="ml-3 text-sm text-gray-700 dark:text-gray-300">
      <slot />
    </span>
  {/if}
</label>
