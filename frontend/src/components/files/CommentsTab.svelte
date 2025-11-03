<script>
  import { success } from "../../stores/toast";

  let { file } = $props();
  let newComment = $state("");
  let comments = $state([]);

  function addComment() {
    if (!newComment.trim()) return;
    success("💬 Comments feature coming soon!");
    newComment = "";
  }
</script>

<div class="space-y-4">
  <!-- Comment Input -->
  <div class="card bg-base-200">
    <div class="card-body p-4">
      <textarea
        bind:value={newComment}
        placeholder="Write a comment about this file..."
        class="textarea textarea-bordered w-full"
        rows="3"
        onkeydown={(e) => {
          if (e.key === "Enter" && e.ctrlKey) addComment();
        }}
      ></textarea>
      <div class="flex justify-between items-center">
        <span class="text-xs text-base-content/60">
          <kbd class="kbd kbd-xs">Ctrl</kbd> +
          <kbd class="kbd kbd-xs">Enter</kbd> to send
        </span>
        <button onclick={addComment} class="btn btn-primary btn-sm gap-2">
          <i class="bi bi-send"></i>
          Add Comment
        </button>
      </div>
    </div>
  </div>

  <!-- Empty State -->
  {#if comments.length === 0}
    <div class="text-center py-16 text-base-content/60">
      <i class="bi bi-chat-dots text-6xl mb-4 block opacity-30"></i>
      <h3 class="text-xl font-semibold mb-2">No comments yet</h3>
      <p>Be the first to comment on this file!</p>
      <div class="alert alert-info mt-6 max-w-md mx-auto">
        <i class="bi bi-info-circle"></i>
        <span>Comments feature will be implemented with backend API</span>
      </div>
    </div>
  {/if}
</div>
