<script lang="ts">
  type DividerVariant = "horizontal" | "vertical";
  type DividerColor = "default" | "gray" | "primary" | "danger" | "gradient";
  type DividerStyle = "solid" | "dashed" | "dotted" | "double" | "gradient";

  interface Props {
    variant?: DividerVariant;
    color?: DividerColor;

    // 🆕 ADVANCED FEATURES
    style?: DividerStyle; // Line style
    thickness?: 1 | 2 | 3 | 4; // Line thickness
    gradient?: boolean; // Gradient effect
    glowing?: boolean; // Glow effect

    class?: string;
  }

  let {
    variant = "horizontal",
    color = "default",
    style = "solid",
    thickness = 1,
    gradient = false,
    glowing = false,
    class: customClass = "",
  }: Props = $props();

  // 🆕 ADVANCED: Thickness variants
  const thicknessClasses = {
    horizontal: {
      1: "h-px",
      2: "h-0.5",
      3: "h-1",
      4: "h-1.5",
    },
    vertical: {
      1: "w-px",
      2: "w-0.5",
      3: "w-1",
      4: "w-1.5",
    },
  };

  // 🆕 ADVANCED: Color classes with gradient option
  const colorClasses = gradient
    ? {
        default:
          "bg-gradient-to-r from-transparent via-gray-300 to-transparent dark:via-gray-700",
        gray: "bg-gradient-to-r from-transparent via-gray-400 to-transparent dark:via-gray-600",
        primary:
          "bg-gradient-to-r from-transparent via-blue-500 to-transparent",
        danger: "bg-gradient-to-r from-transparent via-red-500 to-transparent",
        gradient: "bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500",
      }
    : {
        default: "bg-gray-200 dark:bg-gray-700",
        gray: "bg-gray-300 dark:bg-gray-600",
        primary: "bg-blue-300 dark:bg-blue-700",
        danger: "bg-red-300 dark:bg-red-700",
        gradient: "bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500",
      };

  // 🆕 ADVANCED: Line style
  const styleClasses = {
    solid: "",
    dashed: "border-dashed",
    dotted: "border-dotted",
    double: "border-double",
    gradient: "",
  };

  // 🆕 ADVANCED: Glow effect
  const glowClasses = glowing
    ? {
        default: "shadow-sm shadow-gray-300 dark:shadow-gray-700",
        gray: "shadow-sm shadow-gray-400 dark:shadow-gray-600",
        primary: "shadow-md shadow-blue-500/50",
        danger: "shadow-md shadow-red-500/50",
        gradient: "shadow-lg shadow-purple-500/50",
      }
    : {
        default: "",
        gray: "",
        primary: "",
        danger: "",
        gradient: "",
      };

  const classes = `
    ${variant === "horizontal" ? "w-full" : "h-full"}
    ${thicknessClasses[variant][thickness]}
    ${colorClasses[color]}
    ${styleClasses[style]}
    ${glowClasses[color]}
    ${customClass}
  `;
</script>

<div class={classes} role="separator"></div>
