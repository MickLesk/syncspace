<script>
  import { onMount } from "svelte";
  import { comments, tags } from "../../stores/comments";
  import { auth } from "../../stores/auth";
  import { currentLang } from "../../stores/ui.js";
  import Icon from "./ui/Icon.svelte";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { file = null, visible = false } = $props();

  let newComment = $state("");
  let newTag = $state("");
  let editingCommentId = $state(null);
  let editText = $state("");
  let loading = $state(false);
  let errorMessage = $state("");
  let lastFilePath = $state(null);

  const filePath = $derived(file ? `${file.path || ""}${file.name}` : "");
  const fileComments = $derived($comments[filePath] || []);
  const fileTags = $derived(tags.getTags(filePath, $tags));
  const allTagNames = $derived(tags.getAllTagNames($tags));
  const currentUser = $derived($auth.user?.username || "Anonymous");

  // Load data only when file path changes and is visible
  $effect(() => {
    if (filePath && filePath !== lastFilePath && visible) {
      lastFilePath = filePath;
      loadData();
    }
  });

  // Reset when visibility changes
  $effect(() => {
    if (!visible) {
      lastFilePath = null;
    }
  });

  async function loadData() {
    if (!file) return;

    loading = true;
    errorMessage = "";

    try {
      await Promise.all([comments.loadForFile(filePath), tags.loadAll()]);
    } catch (e) {
      console.error("Failed to load data:", e);
      errorMessage = tr("failedToLoadComments");
    } finally {
      loading = false;
    }
  }

  async function handleAddComment() {
    if (!newComment.trim() || !file) return;

    loading = true;
    errorMessage = "";

    try {
      await comments.addComment(filePath, currentUser, newComment.trim());
      newComment = "";
    } catch (e) {
      console.error("Failed to add comment:", e);
      errorMessage = t(lang, "failedToAddComment");
    } finally {
      loading = false;
    }
  }

  function handleEditComment(commentId) {
    const comment = fileComments.find((c) => c.id === commentId);
    if (comment) {
      editingCommentId = commentId;
      editText = comment.text;
    }
  }

  function handleSaveEdit(commentId) {
    if (!editText.trim()) return;
    // Edit not implemented in backend yet
    console.warn("Edit comment not yet implemented");
    editingCommentId = null;
    editText = "";
  }

  function handleCancelEdit() {
    editingCommentId = null;
    editText = "";
  }

  async function handleDeleteComment(commentId) {
    if (!confirm("Delete this comment?")) return;

    loading = true;
    errorMessage = "";

    try {
      await comments.deleteComment(filePath, commentId);
    } catch (e) {
      console.error("Failed to delete comment:", e);
      errorMessage = t(lang, "failedToDeleteComment");
    } finally {
      loading = false;
    }
  }

  async function handleAddTag() {
    if (!newTag.trim() || !file) return;

    loading = true;
    errorMessage = "";

    try {
      await tags.addTag(filePath, newTag.trim());
      newTag = "";
    } catch (e) {
      console.error("Failed to add tag:", e);
      errorMessage = t(lang, "failedToAddTag");
    } finally {
      loading = false;
    }
  }

  async function handleRemoveTag(tagName) {
    loading = true;
    errorMessage = "";

    try {
      await tags.removeTag(filePath, tagName);
    } catch (e) {
      console.error("Failed to remove tag:", e);
      errorMessage = t(lang, "failedToRemoveTag");
    } finally {
      loading = false;
    }
  }

  function handleSelectTag(tagName) {
    newTag = tagName;
  }

  function formatTimestamp(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return "Just now";
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;

    return date.toLocaleDateString();
  }

  function handleKeydown(e) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      if (editingCommentId) {
        handleSaveEdit(editingCommentId);
      } else {
        handleAddComment();
      }
    } else if (e.key === "Escape") {
      handleCancelEdit();
    }
  }

  function handleClose() {
    visible = false;
    errorMessage = "";
  }
</script>

{#if visible && file}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-[1000] animate-fadeIn" onclick={handleClose} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="bg-white dark:bg-gray-800 rounded-3xl w-[90%] max-w-[600px] max-h-[80vh] flex flex-col shadow-2xl animate-slideUp"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <div class="flex items-center justify-between px-6 py-5 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-3">
          <Icon name="💬" size="20px" />
          <h3 class="m-0 text-xl font-medium text-gray-900 dark:text-gray-100 truncate max-w-[400px]">{file.name}</h3>
        </div>
        <button class="bg-transparent border-none w-8 h-8 rounded-full flex items-center justify-center cursor-pointer text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-100 transition-all" onclick={handleClose} aria-label="Close">
          <Icon name="✕" size="18px" />
        </button>
      </div>

      <div class="p-6 overflow-y-auto flex-1 relative">
        {#if errorMessage}
          <div class="flex items-center gap-3 px-4 py-3 bg-red-100 dark:bg-red-500/20 text-red-800 dark:text-red-300 rounded-xl mb-4 text-sm">
            <Icon name="⚠️" size="16px" />
            <span class="flex-1">{errorMessage}</span>
            <button class="bg-transparent border-none p-1 rounded-full cursor-pointer text-red-800 dark:text-red-300 opacity-70 hover:opacity-100 transition-opacity" onclick={() => (errorMessage = "")} aria-label="Dismiss">
              <Icon name="✕" size="14px" />
            </button>
          </div>
        {/if}

        {#if loading}
          <div class="absolute inset-0 bg-black/30 backdrop-blur-sm flex items-center justify-center rounded-3xl z-10">
            <div class="w-10 h-10 border-4 border-gray-300 dark:border-gray-600 border-t-blue-500 rounded-full animate-spin"></div>
          </div>
        {/if}

        <!-- Tags Section -->
        <div class="mb-8">
          <h4 class="flex items-center gap-2 text-base font-medium text-gray-900 dark:text-gray-100 m-0 mb-4">
            <Icon name="🏷️" size="16px" />
            Tags
          </h4>

          <div class="flex flex-wrap gap-2 mb-4 min-h-[32px]">
            {#each fileTags as tag}
              <span
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full border text-sm font-medium transition-all"
                style="background-color: {tag.color}20; color: {tag.color}; border-color: {tag.color}"
              >
                {tag.name}
                <button class="bg-transparent border-none p-0 w-4 h-4 rounded-full flex items-center justify-center cursor-pointer opacity-70 hover:opacity-100 transition-opacity" onclick={() => handleRemoveTag(tag.name)} aria-label="Remove tag">
                  <Icon name="✕" size="12px" />
                </button>
              </span>
            {/each}
          </div>

          <div class="flex gap-2">
            <input
              type="text"
              class="flex-1 px-4 py-2.5 border border-gray-200 dark:border-gray-600 rounded-xl text-sm bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 focus:bg-white dark:focus:bg-gray-600 transition-all"
              placeholder="Add a tag..."
              bind:value={newTag}
              onkeydown={(e) => e.key === "Enter" && handleAddTag()}
              list="tag-suggestions"
            />
            <datalist id="tag-suggestions">
              {#each allTagNames as tagName}
                <option value={tagName}></option>
              {/each}
            </datalist>
            <button
              class="px-4 py-2.5 border-none rounded-xl bg-blue-500 text-white text-sm font-medium cursor-pointer flex items-center gap-1 hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              onclick={handleAddTag}
              disabled={!newTag.trim()}
              aria-label="Add tag"
            >
              <Icon name="➕" size="14px" />
            </button>
          </div>
        </div>

        <!-- Comments Section -->
        <div>
          <h4 class="flex items-center gap-2 text-base font-medium text-gray-900 dark:text-gray-100 m-0 mb-4">
            <Icon name="💬" size="16px" />
            Comments ({fileComments.length})
          </h4>

          <div class="mb-4 max-h-[400px] overflow-y-auto">
            {#each fileComments as comment (comment.id)}
              <div class="p-4 rounded-xl bg-gray-50 dark:bg-gray-700 mb-3 hover:bg-gray-100 dark:hover:bg-gray-600 transition-all">
                <div class="flex items-center gap-2 mb-2">
                  <span class="text-sm font-semibold text-blue-600 dark:text-blue-400">{comment.user}</span>
                  <span class="text-xs text-gray-500 dark:text-gray-400">{formatTimestamp(comment.timestamp)}</span>
                  {#if comment.edited}
                    <span class="text-[11px] text-gray-500 dark:text-gray-400 italic">(edited)</span>
                  {/if}
                </div>

                {#if editingCommentId === comment.id}
                  <div class="mt-2">
                    <!-- svelte-ignore a11y_autofocus -->
                    <textarea
                      class="w-full px-4 py-3 border border-gray-200 dark:border-gray-600 rounded-xl text-sm font-inherit bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 resize-y focus:outline-none focus:border-blue-500 transition-all"
                      bind:value={editText}
                      onkeydown={handleKeydown}
                      rows="3"
                      autofocus
                    ></textarea>
                    <div class="flex gap-2 mt-2">
                      <button class="px-4 py-2 border-none rounded-lg text-sm font-medium cursor-pointer bg-blue-500 text-white hover:bg-blue-600 transition-all" onclick={() => handleSaveEdit(comment.id)}>Save</button>
                      <button class="px-4 py-2 border-none rounded-lg text-sm font-medium cursor-pointer bg-gray-200 dark:bg-gray-600 text-gray-900 dark:text-gray-100 hover:bg-gray-300 dark:hover:bg-gray-500 transition-all" onclick={handleCancelEdit}>Cancel</button>
                    </div>
                  </div>
                {:else}
                  <div class="text-sm text-gray-900 dark:text-gray-100 leading-relaxed whitespace-pre-wrap break-words mb-2">{comment.text}</div>
                  {#if comment.user === currentUser}
                    <div class="flex gap-3">
                      <button class="bg-transparent border-none px-2 py-1 rounded-lg flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400 cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-500 hover:text-gray-900 dark:hover:text-gray-100 transition-all" onclick={() => handleEditComment(comment.id)}>
                        <Icon name="✏️" size="12px" />
                        Edit
                      </button>
                      <button class="bg-transparent border-none px-2 py-1 rounded-lg flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400 cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-500 hover:text-gray-900 dark:hover:text-gray-100 transition-all" onclick={() => handleDeleteComment(comment.id)}>
                        <Icon name="🗑️" size="12px" />
                        Delete
                      </button>
                    </div>
                  {/if}
                {/if}
              </div>
            {/each}

            {#if fileComments.length === 0}
              <div class="text-center py-10 text-gray-500 dark:text-gray-400">
                <Icon name="💭" size="48px" />
                <p class="my-2 text-sm">No comments yet</p>
                <p class="text-xs opacity-70">Be the first to comment!</p>
              </div>
            {/if}
          </div>

          <div class="flex flex-col gap-3">
            <textarea
              class="w-full px-4 py-3 border border-gray-200 dark:border-gray-600 rounded-xl text-sm font-inherit bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 resize-y focus:outline-none focus:border-blue-500 transition-all"
              placeholder="Add a comment..."
              bind:value={newComment}
              onkeydown={handleKeydown}
              rows="3"
            ></textarea>
            <button
              class="self-end px-5 py-2.5 border-none rounded-xl bg-blue-500 text-white text-sm font-medium cursor-pointer flex items-center gap-2 hover:bg-blue-600 hover:-translate-y-0.5 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none disabled:shadow-none transition-all"
              onclick={handleAddComment}
              disabled={!newComment.trim()}
            >
              <Icon name="📨" size="16px" />
              Comment
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
