<script lang="ts">
  interface Props {
    disabled?: boolean;
    variant?: "primary" | "secondary" | "danger";
    error?: boolean;
    class?: string;
    oninput?: (event: Event) => void;
    onchange?: (event: Event) => void;
    onblur?: (event: Event) => void;
  }

  let {
    disabled = false,
    variant = "primary",
    error = false,
    class: customClass = "",
    oninput,
    onchange,
    onblur,
  }: Props = $props();

  const variantClasses = {
    primary: "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
    secondary: "border-gray-400 focus:border-gray-600",
    danger: "border-red-500 focus:ring-red-500",
  };
</script>

<input
  type="text"
  {disabled}
  class={`
    w-full px-4 py-2 border rounded-lg text-gray-900 placeholder-gray-400
    focus:outline-none focus:ring-2 transition-colors
    disabled:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50
    ${error ? "border-red-500 focus:ring-red-500" : variantClasses[variant]}
    ${customClass}
  `}
  {oninput}
  {onchange}
  {onblur}
/>
