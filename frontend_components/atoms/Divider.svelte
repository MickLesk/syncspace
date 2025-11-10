<script lang="ts">
  interface Props {
    variant?: "horizontal" | "vertical";
    color?: "default" | "gray" | "primary" | "danger";
    class?: string;
  }

  let {
    variant = "horizontal",
    color = "default",
    class: customClass = "",
  }: Props = $props();

  const colorClasses = {
    default: "bg-gray-200",
    gray: "bg-gray-300",
    primary: "bg-blue-300",
    danger: "bg-red-300",
  };

  const classes = `${
    variant === "horizontal" ? "w-full h-px" : "h-full w-px"
  } ${colorClasses[color]} ${customClass}`;
</script>

<div class={classes} role="separator" />
