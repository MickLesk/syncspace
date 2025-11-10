<script lang="ts">
  interface Props {
    title?: string;
    description?: string;
    footer?: string;
    hoverable?: boolean;
    bordered?: boolean;
    shadow?: boolean;
    class?: string;
  }

  let {
    title,
    description,
    footer,
    hoverable = false,
    bordered = true,
    shadow = true,
    class: customClass = "",
  }: Props = $props();
</script>

<div
  class={`
    rounded-lg bg-white p-6
    ${bordered ? "border border-gray-200" : ""}
    ${shadow ? "shadow-md" : ""}
    ${hoverable ? "hover:shadow-lg transition-shadow cursor-pointer" : ""}
    ${customClass}
  `}
>
  {#if title}
    <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
  {/if}

  {#if description}
    <p class="text-gray-600 text-sm mb-4">{description}</p>
  {/if}

  <div class="mb-4">
    <slot />
  </div>

  {#if footer}
    <div class="pt-4 border-t border-gray-200 mt-4">
      <p class="text-xs text-gray-500">{footer}</p>
    </div>
  {/if}

  <div class="pt-4 border-t border-gray-200 mt-4">
    <slot name="footer" />
  </div>
</div>
