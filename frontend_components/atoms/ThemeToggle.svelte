<script lang="ts">
  import { theme } from "../stores/theme.ts";
  import { onMount } from "svelte";

  interface Props {
    class?: string;
  }

  let { class: customClass = "" }: Props = $props();

  onMount(() => {
    theme.init();
  });

  function toggleTheme() {
    theme.toggle();
  }
</script>

<button
  onclick={toggleTheme}
  class={`
    p-2 rounded-lg transition-all duration-200
    hover:bg-gray-200 dark:hover:bg-gray-700
    active:scale-95
    ${customClass}
  `}
  aria-label="Toggle theme"
  title="Toggle dark/light mode"
>
  {#if $theme === "dark"}
    <i class="bi bi-sun-fill text-xl text-yellow-400"></i>
  {:else}
    <i class="bi bi-moon-fill text-xl text-blue-600"></i>
  {/if}
</button>
