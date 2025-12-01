<script>
  import CommentsPanel from "../comments/CommentsPanel.svelte";

  let { file } = $props();
</script>

<div class="h-full flex flex-col">
  <CommentsPanel filePath={file?.path || file?.name || ""} />
</div>
