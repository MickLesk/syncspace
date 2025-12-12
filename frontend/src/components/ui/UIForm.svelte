<!-- UIForm.svelte - Einheitlicher Form-Container -->
<script>
  let { 
    title = "",
    subtitle = "",
    onSubmit = () => {},
    loading = false,
    class: className = "",
    children
  } = $props();
</script>

<UICard class={className}>
  <form on:submit|preventDefault={onSubmit} class="p-6 space-y-6">
    {#if title || subtitle}
      <div class="border-b border-white/10 pb-4">
        {#if title}
          <h3 class="text-xl font-semibold text-white mb-2">{title}</h3>
        {/if}
        {#if subtitle}
          <p class="text-white/70 text-sm">{subtitle}</p>
        {/if}
      </div>
    {/if}
    
    <div class="space-y-4">
      {@render children()}
    </div>
    
    {#if loading}
      <div class="flex justify-center py-4">
        <div class="loading loading-spinner loading-lg"></div>
      </div>
    {/if}
  </form>
</UICard>

<script>
  import UICard from './UICard.svelte';
</script>