<script lang="ts">
  interface Props {
    name?: string;
    icon?: string;
    size?: "sm" | "md" | "lg";
    online?: boolean;
    initials?: string;
  }

  let { name, icon, size = "md", online = false, initials }: Props = $props();

  const sizeClasses = {
    sm: "w-8 h-8 text-xs",
    md: "w-12 h-12 text-sm",
    lg: "w-16 h-16 text-lg",
  };

  const colors = [
    "bg-gradient-to-br from-blue-400 to-blue-600",
    "bg-gradient-to-br from-purple-400 to-purple-600",
    "bg-gradient-to-br from-pink-400 to-pink-600",
    "bg-gradient-to-br from-green-400 to-green-600",
    "bg-gradient-to-br from-orange-400 to-orange-600",
    "bg-gradient-to-br from-red-400 to-red-600",
  ];

  const getColor = (str?: string) => {
    if (!str) return colors[0];
    const index = str.charCodeAt(0) % colors.length;
    return colors[index];
  };
</script>

<div class="relative inline-block">
  <div
    class={`
      rounded-full font-semibold text-white flex items-center justify-center overflow-hidden
      ${sizeClasses[size]} ${getColor(initials || name)}
    `}
  >
    {#if icon}
      <i class={`bi ${icon}`}></i>
    {:else if initials}
      {initials}
    {:else if name}
      {name
        .split(" ")
        .map((n) => n[0])
        .join("")}
    {:else}
      <i class="bi bi-person"></i>
    {/if}
  </div>

  {#if online}
    <div
      class="absolute bottom-0 right-0 w-3 h-3 bg-green-400 border-2 border-white rounded-full"
    ></div>
  {/if}
</div>
