<script lang="ts">
  interface Props {
    label?: string;
    required?: boolean;
    error?: string;
    hint?: string;
    class?: string;
  }

  let {
    label,
    required = false,
    error,
    hint,
    class: customClass = "",
  }: Props = $props();
</script>

<label class={`block text-sm font-medium text-gray-700 mb-2 ${customClass}`}>
  {label}
  {#if required}
    <span class="text-red-500 ml-1">*</span>
  {/if}
  {#if hint}
    <span class="text-gray-500 font-normal text-xs ml-2">({hint})</span>
  {/if}
</label>

{#if error}
  <p class="text-red-500 text-xs mt-1">{error}</p>
{/if}

<style>
  :global(.form-group) {
    @apply mb-4;
  }
</style>
