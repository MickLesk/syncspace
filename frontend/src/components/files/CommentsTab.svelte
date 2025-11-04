<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";

  let { file } = $props();
  let newComment = $state("");
  let comments = $state([]);
  let loading = $state(false);
  let editingId = $state(null);
  let editText = $state("");

  onMount(() => {
    loadComments();
  });

  async function loadComments() {
    if (!file?.path && !file?.name) return;
    loading = true;
    try {
      const filePath = file.path || file.name;
      const data = await api.comments.list(filePath);
      comments = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error("Failed to load comments:", err);
      errorToast("Failed to load comments");
    } finally {
      loading = false;
    }
  }

  async function addComment() {
    if (!newComment.trim()) return;
    try {
      const filePath = file.path || file.name;
      await api.comments.create(filePath, newComment.trim());
      success("💬 Comment added!");
      newComment = "";
      await loadComments();
    } catch (err) {
      console.error("Failed to add comment:", err);
      errorToast("Failed to add comment");
    }
  }

  function startEdit(comment) {
    editingId = comment.id;
    editText = comment.text;
  }

  function cancelEdit() {
    editingId = null;
    editText = "";
  }

  async function saveEdit(commentId) {
    if (!editText.trim()) return;
    try {
      const filePath = file.path || file.name;
      await api.comments.update(commentId, filePath, editText.trim());
      success("✏️ Comment updated!");
      editingId = null;
      editText = "";
      await loadComments();
    } catch (err) {
      console.error("Failed to update comment:", err);
      errorToast("Failed to update comment");
    }
  }

  async function deleteComment(commentId) {
    if (!confirm("Delete this comment?")) return;
    try {
      await api.comments.delete(commentId);
      success("🗑️ Comment deleted!");
      await loadComments();
    } catch (err) {
      console.error("Failed to delete comment:", err);
      errorToast("Failed to delete comment");
    }
  }

  function formatDate(dateStr) {
    try {
      const date = new Date(dateStr);
      return date.toLocaleString();
    } catch {
      return dateStr;
    }
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
        <button
          onclick={addComment}
          class="btn btn-primary btn-sm gap-2"
          disabled={!newComment.trim()}
        >
          <i class="bi bi-send"></i>
          Add Comment
        </button>
      </div>
    </div>
  </div>

  <!-- Loading State -->
  {#if loading}
    <div class="flex justify-center py-8">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if comments.length === 0}
    <!-- Empty State -->
    <div class="text-center py-16 text-base-content/60">
      <i class="bi bi-chat-dots text-6xl mb-4 block opacity-30"></i>
      <h3 class="text-xl font-semibold mb-2">No comments yet</h3>
      <p>Be the first to comment on this file!</p>
    </div>
  {:else}
    <!-- Comments List -->
    <div class="space-y-3">
      {#each comments as comment (comment.id)}
        <div class="card bg-base-100 border border-base-300">
          <div class="card-body p-4">
            <div class="flex justify-between items-start gap-2">
              <div class="flex items-center gap-2 flex-1 min-w-0">
                <div class="avatar placeholder">
                  <div
                    class="bg-primary text-primary-content rounded-full w-8 h-8"
                  >
                    <span class="text-xs"
                      >{comment.author_id.slice(0, 2).toUpperCase()}</span
                    >
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <div class="font-semibold text-sm truncate">
                    {comment.author_id}
                  </div>
                  <div class="text-xs text-base-content/60">
                    {formatDate(comment.created_at)}
                  </div>
                </div>
              </div>
              <div class="dropdown dropdown-end">
                <button
                  tabindex="0"
                  class="btn btn-ghost btn-xs btn-square"
                  aria-label="Comment options"
                >
                  <i class="bi bi-three-dots-vertical"></i>
                </button>
                <ul
                  class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-32"
                >
                  <li>
                    <button onclick={() => startEdit(comment)}
                      ><i class="bi bi-pencil"></i> Edit</button
                    >
                  </li>
                  <li>
                    <button
                      onclick={() => deleteComment(comment.id)}
                      class="text-error"
                      ><i class="bi bi-trash"></i> Delete</button
                    >
                  </li>
                </ul>
              </div>
            </div>

            {#if editingId === comment.id}
              <!-- Edit Mode -->
              <div class="mt-3 space-y-2">
                <textarea
                  bind:value={editText}
                  class="textarea textarea-bordered w-full"
                  rows="3"
                ></textarea>
                <div class="flex gap-2 justify-end">
                  <button onclick={cancelEdit} class="btn btn-ghost btn-sm"
                    >Cancel</button
                  >
                  <button
                    onclick={() => saveEdit(comment.id)}
                    class="btn btn-primary btn-sm">Save</button
                  >
                </div>
              </div>
            {:else}
              <!-- Display Mode -->
              <div class="mt-2 text-sm whitespace-pre-wrap break-words">
                {comment.text}
              </div>
              {#if comment.edited_at}
                <div class="text-xs text-base-content/50 italic mt-1">
                  Edited {formatDate(comment.edited_at)}
                </div>
              {/if}
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
