<script lang="ts">
  interface Props {
    disabled?: boolean;
    variant?: "primary" | "danger" | "success" | "warning";
    class?: string;
  }

  let {
    disabled = false,
    variant = "primary",
    class: customClass = "",
  }: Props = $props();

  const variantClasses = {
    primary: "peer-checked:bg-blue-500",
    danger: "peer-checked:bg-red-500",
    success: "peer-checked:bg-green-500",
    warning: "peer-checked:bg-yellow-500",
  };
</script>

<label class={`flex items-center cursor-pointer select-none ${customClass}`}>
  <input
    type="checkbox"
    {disabled}
    class="sr-only peer"
    onchange
  />
  <div
    class={`
      relative w-11 h-6 bg-gray-300 rounded-full transition-colors
      ${variantClasses[variant]} peer-checked:bg-blue-500 peer-disabled:opacity-50
    `}
  >
    <span
      class="
        absolute left-1 top-1 bg-white w-4 h-4 rounded-full shadow transition-transform
        peer-checked:translate-x-5
      "
    />
  </div>
  <span class="ml-3 text-sm text-gray-700">
    <slot />
  </span>
</label>
