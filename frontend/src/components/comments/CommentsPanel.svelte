<script>
  import { onMount } from "svelte";
  import { comments as commentsApi } from "../../lib/api.js";
  import { t } from "../../i18n.js";
  import { auth } from "../../stores/auth.js";

  let { filePath = "" } = $props();

  let comments = $state([]);
  let loading = $state(false);
  let error = $state(null);
  let newComment = $state("");
  let posting = $state(false);
  let expandedComments = $state(new Set());
  let replyingTo = $state(null);
  let replyText = $state("");

  onMount(async () => {
    await loadComments();
  });

  async function loadComments() {
    if (!filePath) return;
    loading = true;
    error = null;
    try {
      comments = await commentsApi.listForFile(filePath);
      // Sort by created_at descending
      comments.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
    } catch (e) {
      console.error("Failed to load comments:", e);
      error = $t("comments.loadError");
    } finally {
      loading = false;
    }
  }

  async function postComment() {
    if (!newComment.trim() || !filePath) return;

    posting = true;
    error = null;
    try {
      const comment = await commentsApi.create(filePath, {
        content: newComment.trim(),
        mentions: extractMentions(newComment),
      });
      comments = [comment, ...comments];
      newComment = "";
    } catch (e) {
      console.error("Failed to post comment:", e);
      error = $t("comments.postError");
    } finally {
      posting = false;
    }
  }

  async function postReply() {
    if (!replyText.trim() || !replyingTo) return;

    posting = true;
    error = null;
    try {
      const reply = await commentsApi.create(filePath, {
        content: replyText.trim(),
        parent_comment_id: replyingTo.id,
        mentions: extractMentions(replyText),
      });

      // Add reply to parent comment's replies
      const parentIdx = comments.findIndex((c) => c.id === replyingTo.id);
      if (parentIdx !== -1) {
        if (!comments[parentIdx].replies) {
          comments[parentIdx].replies = [];
        }
        comments[parentIdx].replies = [reply, ...comments[parentIdx].replies];
      }

      replyText = "";
      replyingTo = null;
    } catch (e) {
      console.error("Failed to post reply:", e);
      error = $t("comments.postError");
    } finally {
      posting = false;
    }
  }

  async function deleteComment(commentId) {
    if (!confirm($t("comments.deleteConfirm"))) return;

    try {
      await commentsApi.delete(filePath, commentId);
      comments = comments.filter((c) => c.id !== commentId);
    } catch (e) {
      console.error("Failed to delete comment:", e);
      error = $t("comments.deleteError");
    }
  }

  async function addReaction(commentId, emoji) {
    try {
      await commentsApi.addReaction(filePath, commentId, emoji);
      await loadComments();
    } catch (e) {
      console.error("Failed to add reaction:", e);
    }
  }

  async function removeReaction(commentId, emoji) {
    try {
      await commentsApi.removeReaction(filePath, commentId, emoji);
      await loadComments();
    } catch (e) {
      console.error("Failed to remove reaction:", e);
    }
  }

  function extractMentions(text) {
    const mentions = [];
    const regex = /@(\w+)/g;
    let match;
    while ((match = regex.exec(text)) !== null) {
      if (!mentions.includes(match[1])) {
        mentions.push(match[1]);
      }
    }
    return mentions;
  }

  function formatDate(dateStr) {
    const date = new Date(dateStr);
    const now = new Date();
    const diff = now - date;
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return $t("comments.justNow");
    if (minutes < 60) return `${minutes}m`;
    if (hours < 24) return `${hours}h`;
    if (days < 7) return `${days}d`;

    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function toggleExpanded(commentId) {
    if (expandedComments.has(commentId)) {
      expandedComments.delete(commentId);
    } else {
      expandedComments.add(commentId);
    }
    expandedComments = expandedComments;
  }

  const commonEmojis = ["👍", "❤️", "😂", "🔥", "👀", "🎉"];
</script>

<div class="flex flex-col h-full bg-base-100">
  <!-- Header -->
  <div class="border-b border-base-300 p-4">
    <h3 class="font-semibold text-lg flex items-center gap-2">
      <i class="bi bi-chat-dots"></i>
      {$t("comments.title")}
    </h3>
  </div>

  <!-- Comments Area -->
  <div class="flex-1 overflow-y-auto p-4 space-y-4">
    {#if error}
      <div class="alert alert-error gap-2">
        <i class="bi bi-exclamation-triangle"></i>
        <span class="text-sm">{error}</span>
      </div>
    {/if}

    {#if loading}
      <div class="flex justify-center py-8">
        <span class="loading loading-spinner loading-md text-primary"></span>
      </div>
    {:else if comments.length === 0}
      <div class="text-center py-8 text-base-content/60">
        <i class="bi bi-chat text-3xl mb-2 block"></i>
        <p class="text-sm">{$t("comments.noComments")}</p>
      </div>
    {:else}
      {#each comments as comment (comment.id)}
        <div class="card bg-base-200/50 border border-base-300">
          <!-- Comment Header -->
          <div class="card-body p-3">
            <div class="flex items-start justify-between gap-2">
              <div class="flex items-center gap-2 flex-1 min-w-0">
                {#if comment.user_avatar}
                  <img
                    src={`data:image/png;base64,${comment.user_avatar}`}
                    alt={comment.username}
                    class="w-8 h-8 rounded-full object-cover flex-shrink-0"
                  />
                {:else}
                  <div
                    class="w-8 h-8 rounded-full bg-primary/20 flex items-center justify-center flex-shrink-0 text-xs font-bold"
                  >
                    {comment.username?.charAt(0).toUpperCase()}
                  </div>
                {/if}
                <div class="min-w-0">
                  <p class="font-medium text-sm truncate">{comment.username}</p>
                  <p class="text-xs text-base-content/60">
                    {formatDate(comment.created_at)}
                  </p>
                </div>
              </div>

              {#if $auth.user?.id === comment.user_id}
                <button
                  onclick={() => deleteComment(comment.id)}
                  class="btn btn-ghost btn-xs text-error"
                  title={$t("delete")}
                >
                  <i class="bi bi-trash"></i>
                </button>
              {/if}
            </div>

            <!-- Comment Content -->
            <p
              class="text-sm text-base-content mt-2 break-words whitespace-pre-wrap"
            >
              {comment.content}
            </p>

            <!-- Mentions -->
            {#if comment.mentions && comment.mentions.length > 0}
              <p class="text-xs text-primary mt-1">
                {$t("comments.mentions")}: {comment.mentions.join(", ")}
              </p>
            {/if}

            <!-- Reactions -->
            {#if comment.reactions && comment.reactions.length > 0}
              <div class="flex flex-wrap gap-1 mt-2">
                {#each Object.entries(comment.reactions).filter(([_, count]) => count > 0) as [emoji, count]}
                  <button
                    onclick={() => removeReaction(comment.id, emoji)}
                    class="btn btn-xs btn-outline gap-1"
                  >
                    {emoji}
                    <span class="text-xs">{count}</span>
                  </button>
                {/each}
              </div>
            {/if}

            <!-- Reactions & Actions -->
            <div
              class="flex items-center gap-2 mt-2 pt-2 border-t border-base-300"
            >
              <div class="dropdown dropdown-hover">
                <button class="btn btn-ghost btn-xs gap-1">
                  <i class="bi bi-emoji-smile text-sm"></i>
                  {$t("comments.react")}
                </button>
                <ul
                  class="dropdown-content bg-base-100 border border-base-300 rounded-lg p-2 flex gap-1"
                >
                  {#each commonEmojis as emoji}
                    <button
                      onclick={() => addReaction(comment.id, emoji)}
                      class="btn btn-ghost btn-sm text-lg p-1 h-auto min-h-auto"
                    >
                      {emoji}
                    </button>
                  {/each}
                </ul>
              </div>

              <button
                onclick={() => {
                  if (replyingTo?.id === comment.id) {
                    replyingTo = null;
                  } else {
                    replyingTo = comment;
                  }
                }}
                class="btn btn-ghost btn-xs gap-1"
              >
                <i class="bi bi-reply"></i>
                {$t("comments.reply")}
              </button>

              {#if comment.replies && comment.replies.length > 0}
                <button
                  onclick={() => toggleExpanded(comment.id)}
                  class="btn btn-ghost btn-xs gap-1 ml-auto"
                >
                  {expandedComments.has(comment.id) ? $t("hide") : $t("show")}
                  ({comment.replies.length})
                </button>
              {/if}
            </div>

            <!-- Replies -->
            {#if comment.replies && comment.replies.length > 0 && expandedComments.has(comment.id)}
              <div class="mt-3 ml-4 pl-3 border-l-2 border-base-300 space-y-2">
                {#each comment.replies as reply (reply.id)}
                  <div class="card bg-base-100 border border-base-200">
                    <div class="card-body p-2">
                      <div class="flex items-start justify-between gap-2">
                        <div class="flex items-center gap-2 flex-1 min-w-0">
                          {#if reply.user_avatar}
                            <img
                              src={`data:image/png;base64,${reply.user_avatar}`}
                              alt={reply.username}
                              class="w-6 h-6 rounded-full object-cover flex-shrink-0"
                            />
                          {:else}
                            <div
                              class="w-6 h-6 rounded-full bg-primary/20 flex items-center justify-center flex-shrink-0 text-xs font-bold"
                            >
                              {reply.username?.charAt(0).toUpperCase()}
                            </div>
                          {/if}
                          <div class="min-w-0">
                            <p class="font-medium text-xs truncate">
                              {reply.username}
                            </p>
                            <p class="text-xs text-base-content/60">
                              {formatDate(reply.created_at)}
                            </p>
                          </div>
                        </div>
                        {#if $auth.user?.id === reply.user_id}
                          <button aria-label="Delete" onclick={() => deleteComment(reply.id)} class="btn btn-ghost btn-xs text-error"><i class="bi bi-trash" aria-hidden="true"></i></button>
                        {/if}
                      </div>
                      <p
                        class="text-xs text-base-content mt-1 break-words whitespace-pre-wrap"
                      >
                        {reply.content}
                      </p>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}

            <!-- Reply Input -->
            {#if replyingTo?.id === comment.id}
              <div class="mt-3 ml-4 pl-3 border-l-2 border-primary space-y-2">
                <textarea
                  bind:value={replyText}
                  placeholder={$t("comments.replyPlaceholder")}
                  class="textarea textarea-bordered textarea-sm w-full"
                  rows="2"
                ></textarea>
                <div class="flex gap-2">
                  <button
                    onclick={postReply}
                    disabled={!replyText.trim() || posting}
                    class="btn btn-primary btn-sm gap-2 flex-1"
                  >
                    {#if posting}
                      <span class="loading loading-spinner loading-xs"></span>
                    {:else}
                      <i class="bi bi-send"></i>
                    {/if}
                    {$t("send")}
                  </button>
                  <button
                    onclick={() => {
                      replyingTo = null;
                      replyText = "";
                    }}
                    class="btn btn-ghost btn-sm"
                  >
                    {$t("cancel")}
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- New Comment Input -->
  <div class="border-t border-base-300 p-4 space-y-2">
    <textarea
      bind:value={newComment}
      placeholder={$t("comments.placeholder")}
      class="textarea textarea-bordered w-full"
      rows="3"
    ></textarea>
    <div class="flex gap-2">
      <button
        onclick={postComment}
        disabled={!newComment.trim() || posting}
        class="btn btn-primary btn-sm gap-2 flex-1"
      >
        {#if posting}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <i class="bi bi-send"></i>
        {/if}
        {$t("comments.post")}
      </button>
      <button onclick={() => (newComment = "")} class="btn btn-ghost btn-sm">
        {$t("clear")}
      </button>
    </div>
    <p class="text-xs text-base-content/60">
      {$t("comments.mentionTip")}
    </p>
  </div>
</div>
