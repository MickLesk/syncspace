<script>
  /**
   * Tags & Comments Panel Component
   * Shows tags and comments for a file with full CRUD
   *
   * Props:
   * - filePath: string - Current file path
   * - readOnly: boolean - Disable editing if true
   */
  import { onMount } from "svelte";
  import {
    currentFileMetadata,
    loading,
    error,
    loadTags,
    loadComments,
    addTag,
    removeTag,
    updateTag,
    addComment,
    editComment,
    deleteComment,
    addReaction,
    getCommentThread,
  } from "../stores/tagsComments.js";
  import { serverState } from "../stores/serverState.js";

  export let filePath = "";
  export let readOnly = false;

  let activeTab = "tags"; // 'tags' | 'comments'
  let newTagName = "";
  let newTagColor = "#3b82f6";
  let newCommentText = "";
  let replyingToId = null;
  let editingCommentId = null;
  let editingText = "";
  let showEmojiPicker = false;
  let selectedCommentId = null;

  const tagColors = [
    "#ef4444", // red
    "#f97316", // orange
    "#eab308", // yellow
    "#22c55e", // green
    "#06b6d4", // cyan
    "#3b82f6", // blue
    "#8b5cf6", // purple
    "#ec4899", // pink
  ];

  onMount(async () => {
    if (filePath) {
      await Promise.all([loadTags(filePath), loadComments(filePath)]);
    }
  });

  // Watch filePath changes
  $: if (filePath) {
    Promise.all([loadTags(filePath), loadComments(filePath)]);
  }

  async function handleAddTag() {
    if (!newTagName.trim()) return;

    await addTag(filePath, {
      name: newTagName,
      color: newTagColor,
    });

    newTagName = "";
    newTagColor = "#3b82f6";
  }

  async function handleDeleteTag(tagId) {
    if (confirm("Delete this tag?")) {
      await removeTag(filePath, tagId);
    }
  }

  async function handleAddComment() {
    if (!newCommentText.trim()) return;

    await addComment(filePath, newCommentText, replyingToId);

    newCommentText = "";
    replyingToId = null;
  }

  async function handleEditComment() {
    if (!editingText.trim()) return;

    await editComment(filePath, editingCommentId, editingText);

    editingCommentId = null;
    editingText = "";
  }

  async function handleDeleteComment(commentId) {
    if (confirm("Delete this comment?")) {
      await deleteComment(filePath, commentId);
    }
  }

  function startReply(commentId) {
    replyingToId = commentId;
    document.getElementById("comment-input")?.focus();
  }

  function startEdit(comment) {
    editingCommentId = comment.id;
    editingText = comment.text;
  }

  function formatDate(date) {
    return new Date(date).toLocaleDateString("de-DE", {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function renderCommentTree(comments, depth = 0) {
    if (depth > 5) return []; // Prevent infinite nesting
    return comments.filter((c) => !c.parentCommentId || depth > 0);
  }
</script>

<div
  class="flex flex-col h-full bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700"
>
  <!-- Header -->
  <div class="flex gap-4 p-4 border-b border-slate-200 dark:border-slate-700">
    <button
      class:active={activeTab === "tags"}
      class="px-4 py-2 rounded text-sm font-medium transition-colors"
      class:bg-blue-100={activeTab === "tags"}
      class:dark:bg-blue-900={activeTab === "tags"}
      class:text-blue-700={activeTab === "tags"}
      class:dark:text-blue-200={activeTab === "tags"}
      class:text-slate-600={activeTab !== "tags"}
      class:dark:text-slate-400={activeTab !== "tags"}
      on:click={() => (activeTab = "tags")}
    >
      <i class="bi bi-tag mr-2" />
      Tags ({$currentFileMetadata.tags.length})
    </button>

    <button
      class:active={activeTab === "comments"}
      class="px-4 py-2 rounded text-sm font-medium transition-colors"
      class:bg-blue-100={activeTab === "comments"}
      class:dark:bg-blue-900={activeTab === "comments"}
      class:text-blue-700={activeTab === "comments"}
      class:dark:text-blue-200={activeTab === "comments"}
      class:text-slate-600={activeTab !== "comments"}
      class:dark:text-slate-400={activeTab !== "comments"}
      on:click={() => (activeTab = "comments")}
    >
      <i class="bi bi-chat-dots mr-2" />
      Comments ({$currentFileMetadata.comments.length})
    </button>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if $loading}
      <div class="flex items-center justify-center h-full">
        <div class="animate-spin">
          <i class="bi bi-hourglass text-2xl text-blue-500" />
        </div>
      </div>
    {:else if $error}
      <div
        class="bg-red-50 dark:bg-red-900/20 p-3 rounded text-red-700 dark:text-red-200 text-sm"
      >
        <i class="bi bi-exclamation-circle mr-2" />
        {$error}
      </div>
    {:else if activeTab === "tags"}
      <!-- TAGS TAB -->
      <div class="space-y-4">
        {#if !readOnly}
          <!-- Add Tag Form -->
          <div class="bg-slate-50 dark:bg-slate-700/50 p-3 rounded space-y-3">
            <label
              for="tagNameInput"
              class="block text-sm font-medium text-slate-700 dark:text-slate-300"
            >
              Add Tag
            </label>

            <div class="flex gap-2">
              <input
                id="tagNameInput"
                type="text"
                placeholder="Tag name..."
                bind:value={newTagName}
                class="flex-1 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
              />

              <div class="flex gap-1 flex-wrap">
                {#each tagColors as color}
                  <button
                    class="w-8 h-8 rounded border-2 transition-all"
                    class:border-slate-900={newTagColor === color}
                    class:border-transparent={newTagColor !== color}
                    style="background-color: {color}"
                    on:click={() => (newTagColor = color)}
                    title={color}
                  />
                {/each}
              </div>

              <button
                on:click={handleAddTag}
                disabled={!newTagName.trim()}
                class="px-4 py-2 bg-blue-500 hover:bg-blue-600 disabled:bg-slate-300 text-white rounded text-sm font-medium transition-colors"
              >
                <i class="bi bi-plus" />
              </button>
            </div>
          </div>
        {/if}

        <!-- Tags List -->
        {#if $currentFileMetadata.tags.length === 0}
          <p
            class="text-center text-slate-500 dark:text-slate-400 text-sm py-8"
          >
            No tags yet
          </p>
        {:else}
          <div class="flex flex-wrap gap-2">
            {#each $currentFileMetadata.tags as tag (tag.id)}
              <div
                class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-sm text-white"
                style="background-color: {tag.color}"
              >
                <span>{tag.name}</span>

                {#if !readOnly}
                  <button
                    on:click={() => handleDeleteTag(tag.id)}
                    class="hover:opacity-75 transition-opacity"
                    title="Remove tag"
                  >
                    <i class="bi bi-x" />
                  </button>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {:else if activeTab === "comments"}
      <!-- COMMENTS TAB -->
      <div class="space-y-4">
        <!-- Comments List -->
        {#if $currentFileMetadata.comments.length === 0}
          <p
            class="text-center text-slate-500 dark:text-slate-400 text-sm py-8"
          >
            No comments yet
          </p>
        {:else}
          <div class="space-y-3">
            {#each $currentFileMetadata.comments.filter((c) => !c.parentCommentId) as comment (comment.id)}
              <div
                class="border border-slate-200 dark:border-slate-700 rounded p-3 space-y-2"
              >
                <!-- Comment Header -->
                <div class="flex items-start justify-between">
                  <div class="flex items-center gap-2">
                    <div
                      class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-xs font-bold"
                    >
                      {comment.author?.[0]?.toUpperCase() || "?"}
                    </div>
                    <div>
                      <p
                        class="text-sm font-medium text-slate-900 dark:text-white"
                      >
                        {comment.author || "Unknown"}
                      </p>
                      <p class="text-xs text-slate-500 dark:text-slate-400">
                        {formatDate(comment.createdAt)}
                      </p>
                    </div>
                  </div>

                  {#if !readOnly && comment.author === $serverState.profile.displayName}
                    <div class="flex gap-1">
                      <button
                        on:click={() => startEdit(comment)}
                        class="text-blue-500 hover:text-blue-700 text-sm"
                        title="Edit"
                      >
                        <i class="bi bi-pencil" />
                      </button>
                      <button
                        on:click={() => handleDeleteComment(comment.id)}
                        class="text-red-500 hover:text-red-700 text-sm"
                        title="Delete"
                      >
                        <i class="bi bi-trash" />
                      </button>
                    </div>
                  {/if}
                </div>

                <!-- Comment Text -->
                {#if editingCommentId === comment.id}
                  <div class="space-y-2">
                    <textarea
                      bind:value={editingText}
                      class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
                      rows="3"
                    />
                    <div class="flex gap-2">
                      <button
                        on:click={handleEditComment}
                        class="px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm"
                      >
                        Save
                      </button>
                      <button
                        on:click={() => (editingCommentId = null)}
                        class="px-3 py-1 bg-slate-300 dark:bg-slate-600 text-slate-900 dark:text-white rounded text-sm"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                {:else}
                  <p class="text-slate-700 dark:text-slate-300 text-sm">
                    {comment.text}
                  </p>

                  {#if comment.isPending}
                    <p class="text-xs text-slate-500 dark:text-slate-400">
                      <i class="bi bi-hourglass mr-1 animate-spin" />
                      Sending...
                    </p>
                  {/if}
                {/if}

                <!-- Comment Actions -->
                <div
                  class="flex gap-3 pt-2 border-t border-slate-200 dark:border-slate-700"
                >
                  {#if !readOnly}
                    <button
                      on:click={() => startReply(comment.id)}
                      class="text-sm text-blue-500 hover:text-blue-700 flex items-center gap-1"
                    >
                      <i class="bi bi-reply" />
                      Reply
                    </button>
                  {/if}
                </div>

                <!-- Replies -->
                {#each $currentFileMetadata.comments.filter((c) => c.parentCommentId === comment.id) as reply (reply.id)}
                  <div
                    class="ml-4 mt-2 border-l-2 border-slate-200 dark:border-slate-700 pl-3 space-y-2"
                  >
                    <div class="flex items-start justify-between">
                      <div class="flex items-center gap-2">
                        <div
                          class="w-6 h-6 rounded-full bg-gradient-to-br from-green-400 to-green-600 flex items-center justify-center text-white text-xs font-bold"
                        >
                          {reply.author?.[0]?.toUpperCase() || "?"}
                        </div>
                        <div>
                          <p
                            class="text-xs font-medium text-slate-900 dark:text-white"
                          >
                            {reply.author || "Unknown"}
                          </p>
                          <p class="text-xs text-slate-500 dark:text-slate-400">
                            {formatDate(reply.createdAt)}
                          </p>
                        </div>
                      </div>

                      {#if !readOnly && reply.author === $serverState.profile.displayName}
                        <button
                          on:click={() => handleDeleteComment(reply.id)}
                          class="text-red-500 hover:text-red-700 text-xs"
                        >
                          <i class="bi bi-trash" />
                        </button>
                      {/if}
                    </div>
                    <p class="text-slate-700 dark:text-slate-300 text-xs">
                      {reply.text}
                    </p>
                  </div>
                {/each}
              </div>
            {/each}
          </div>
        {/if}

        <!-- Add Comment Form -->
        {#if !readOnly}
          <div
            class="border-t border-slate-200 dark:border-slate-700 pt-4 mt-4"
          >
            {#if replyingToId}
              <div
                class="mb-3 p-2 bg-blue-50 dark:bg-blue-900/20 rounded text-sm text-blue-700 dark:text-blue-200"
              >
                Replying to comment...
                <button
                  on:click={() => (replyingToId = null)}
                  class="ml-2 underline hover:no-underline"
                >
                  Cancel
                </button>
              </div>
            {/if}

            <textarea
              id="comment-input"
              placeholder="Add a comment..."
              bind:value={newCommentText}
              class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
              rows="3"
            />

            <div class="flex gap-2 mt-2">
              <button
                on:click={handleAddComment}
                disabled={!newCommentText.trim()}
                class="px-4 py-2 bg-blue-500 hover:bg-blue-600 disabled:bg-slate-300 text-white rounded text-sm font-medium"
              >
                Post Comment
              </button>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  :global(.active) {
    font-weight: 600;
  }
</style>
