<script>
  import { comments, tags } from "../stores/comments";
  import { auth } from "../stores/auth";
  import Icon from "./ui/Icon.svelte";

  export let file = null;
  export let visible = false;

  let newComment = "";
  let newTag = "";
  let editingCommentId = null;
  let editText = "";

  $: filePath = file ? `${file.path || ""}${file.name}` : "";
  $: fileComments = $comments[filePath] || [];
  $: fileTags = tags.getTags(filePath, $tags);
  $: allTagNames = tags.getAllTagNames($tags);
  $: currentUser = $auth.user?.username || "Anonymous";

  function handleAddComment() {
    if (!newComment.trim() || !file) return;

    comments.addComment(filePath, currentUser, newComment.trim());
    newComment = "";
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
    comments.editComment(filePath, commentId, editText.trim());
    editingCommentId = null;
    editText = "";
  }

  function handleCancelEdit() {
    editingCommentId = null;
    editText = "";
  }

  function handleDeleteComment(commentId) {
    if (confirm("Delete this comment?")) {
      comments.deleteComment(filePath, commentId);
    }
  }

  function handleAddTag() {
    if (!newTag.trim() || !file) return;

    tags.addTag(filePath, newTag.trim());
    newTag = "";
  }

  function handleRemoveTag(tagName) {
    tags.removeTag(filePath, tagName);
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
  }
</script>

{#if visible && file}
  <div class="comments-panel-overlay" on:click={handleClose}>
    <div class="comments-panel" on:click|stopPropagation>
      <div class="panel-header">
        <div class="panel-title">
          <Icon name="💬" size="20px" />
          <h3>{file.name}</h3>
        </div>
        <button class="btn-close" on:click={handleClose}>
          <Icon name="✕" size="18px" />
        </button>
      </div>

      <div class="panel-content">
        <!-- Tags Section -->
        <div class="section">
          <h4 class="section-title">
            <Icon name="🏷️" size="16px" />
            Tags
          </h4>

          <div class="tags-list">
            {#each fileTags as tag}
              <span
                class="tag"
                style="background-color: {tag.color}20; color: {tag.color}; border-color: {tag.color}"
              >
                {tag.name}
                <button
                  class="tag-remove"
                  on:click={() => handleRemoveTag(tag.name)}
                >
                  <Icon name="✕" size="12px" />
                </button>
              </span>
            {/each}
          </div>

          <div class="tag-input-group">
            <input
              type="text"
              class="input-tag"
              placeholder="Add a tag..."
              bind:value={newTag}
              on:keydown={(e) => e.key === "Enter" && handleAddTag()}
              list="tag-suggestions"
            />
            <datalist id="tag-suggestions">
              {#each allTagNames as tagName}
                <option value={tagName} />
              {/each}
            </datalist>
            <button
              class="btn-add-tag"
              on:click={handleAddTag}
              disabled={!newTag.trim()}
            >
              <Icon name="➕" size="14px" />
            </button>
          </div>
        </div>

        <!-- Comments Section -->
        <div class="section">
          <h4 class="section-title">
            <Icon name="💬" size="16px" />
            Comments ({fileComments.length})
          </h4>

          <div class="comments-list">
            {#each fileComments as comment (comment.id)}
              <div class="comment-item">
                <div class="comment-header">
                  <span class="comment-user">{comment.user}</span>
                  <span class="comment-time"
                    >{formatTimestamp(comment.timestamp)}</span
                  >
                  {#if comment.edited}
                    <span class="comment-edited">(edited)</span>
                  {/if}
                </div>

                {#if editingCommentId === comment.id}
                  <div class="comment-edit">
                    <textarea
                      class="textarea-edit"
                      bind:value={editText}
                      on:keydown={handleKeydown}
                      rows="3"
                      autofocus
                    />
                    <div class="edit-actions">
                      <button
                        class="btn-save"
                        on:click={() => handleSaveEdit(comment.id)}
                      >
                        Save
                      </button>
                      <button class="btn-cancel" on:click={handleCancelEdit}>
                        Cancel
                      </button>
                    </div>
                  </div>
                {:else}
                  <div class="comment-text">{comment.text}</div>
                  {#if comment.user === currentUser}
                    <div class="comment-actions">
                      <button
                        class="btn-action"
                        on:click={() => handleEditComment(comment.id)}
                      >
                        <Icon name="✏️" size="12px" />
                        Edit
                      </button>
                      <button
                        class="btn-action"
                        on:click={() => handleDeleteComment(comment.id)}
                      >
                        <Icon name="🗑️" size="12px" />
                        Delete
                      </button>
                    </div>
                  {/if}
                {/if}
              </div>
            {/each}

            {#if fileComments.length === 0}
              <div class="empty-state">
                <Icon name="💭" size="48px" />
                <p>No comments yet</p>
                <p class="empty-hint">Be the first to comment!</p>
              </div>
            {/if}
          </div>

          <div class="comment-input-group">
            <textarea
              class="textarea-comment"
              placeholder="Add a comment..."
              bind:value={newComment}
              on:keydown={handleKeydown}
              rows="3"
            />
            <button
              class="btn-add-comment"
              on:click={handleAddComment}
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
  .comments-panel-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .comments-panel {
    background: var(--md-sys-color-surface);
    border-radius: 24px;
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 24px;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .panel-title {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .panel-title h3 {
    margin: 0;
    font-size: 20px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 400px;
  }

  .btn-close {
    background: none;
    border: none;
    width: 32px;
    height: 32px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.2s ease;
  }

  .btn-close:hover {
    background: var(--md-sys-color-surface-container-high);
    color: var(--md-sys-color-on-surface);
  }

  .panel-content {
    padding: 24px;
    overflow-y: auto;
    flex: 1;
  }

  .section {
    margin-bottom: 32px;
  }

  .section:last-child {
    margin-bottom: 0;
  }

  .section-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 16px 0;
  }

  /* Tags Styles */
  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 16px;
    min-height: 32px;
  }

  .tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 16px;
    border: 1px solid;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .tag-remove {
    background: none;
    border: none;
    padding: 0;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0.7;
    transition: opacity 0.2s ease;
  }

  .tag-remove:hover {
    opacity: 1;
  }

  .tag-input-group {
    display: flex;
    gap: 8px;
  }

  .input-tag {
    flex: 1;
    padding: 10px 16px;
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 12px;
    font-size: 14px;
    background: var(--md-sys-color-surface-container-lowest);
    color: var(--md-sys-color-on-surface);
    transition: all 0.2s ease;
  }

  .input-tag:focus {
    outline: none;
    border-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-surface-container);
  }

  .btn-add-tag {
    padding: 10px 16px;
    border: none;
    border-radius: 12px;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .btn-add-tag:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .btn-add-tag:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Comments Styles */
  .comments-list {
    margin-bottom: 16px;
    max-height: 400px;
    overflow-y: auto;
  }

  .comment-item {
    padding: 16px;
    border-radius: 12px;
    background: var(--md-sys-color-surface-container-lowest);
    margin-bottom: 12px;
    transition: all 0.2s ease;
  }

  .comment-item:hover {
    background: var(--md-sys-color-surface-container);
  }

  .comment-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .comment-user {
    font-size: 13px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
  }

  .comment-time {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .comment-edited {
    font-size: 11px;
    color: var(--md-sys-color-on-surface-variant);
    font-style: italic;
  }

  .comment-text {
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    line-height: 1.5;
    white-space: pre-wrap;
    word-wrap: break-word;
    margin-bottom: 8px;
  }

  .comment-actions {
    display: flex;
    gap: 12px;
  }

  .btn-action {
    background: none;
    border: none;
    padding: 4px 8px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-action:hover {
    background: var(--md-sys-color-surface-container-high);
    color: var(--md-sys-color-on-surface);
  }

  .comment-edit {
    margin-top: 8px;
  }

  .textarea-edit,
  .textarea-comment {
    width: 100%;
    padding: 12px 16px;
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 12px;
    font-size: 14px;
    font-family: inherit;
    background: var(--md-sys-color-surface-container-lowest);
    color: var(--md-sys-color-on-surface);
    resize: vertical;
    transition: all 0.2s ease;
  }

  .textarea-edit:focus,
  .textarea-comment:focus {
    outline: none;
    border-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-surface-container);
  }

  .edit-actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .btn-save,
  .btn-cancel {
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-save {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }

  .btn-save:hover {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .btn-cancel {
    background: var(--md-sys-color-surface-container-high);
    color: var(--md-sys-color-on-surface);
  }

  .btn-cancel:hover {
    background: var(--md-sys-color-surface-container-highest);
  }

  .comment-input-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .btn-add-comment {
    align-self: flex-end;
    padding: 10px 20px;
    border: none;
    border-radius: 12px;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-add-comment:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .btn-add-comment:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .empty-state {
    text-align: center;
    padding: 40px 20px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .empty-state p {
    margin: 8px 0;
    font-size: 14px;
  }

  .empty-hint {
    font-size: 12px;
    opacity: 0.7;
  }

  /* Mobile Responsive */
  @media (max-width: 768px) {
    .comments-panel {
      width: 95%;
      max-height: 90vh;
    }

    .panel-header {
      padding: 16px;
    }

    .panel-title h3 {
      font-size: 18px;
      max-width: 250px;
    }

    .panel-content {
      padding: 16px;
    }
  }
</style>
