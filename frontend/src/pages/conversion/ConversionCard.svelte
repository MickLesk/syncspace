<script>
  import { language } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { formatFileSize } from "../../lib/i18nFormatting.js";

  let { job, oncancel, ondelete } = $props();

  function getStatusIcon(status) {
    switch (status) {
      case "pending":
        return "bi-hourglass";
      case "processing":
        return "bi-arrow-repeat";
      case "completed":
        return "bi-check-circle-fill";
      case "failed":
        return "bi-x-circle-fill";
      case "cancelled":
        return "bi-dash-circle-fill";
      default:
        return "bi-question-circle";
    }
  }

  function getStatusClass(status) {
    switch (status) {
      case "pending":
        return "badge-warning";
      case "processing":
        return "badge-info";
      case "completed":
        return "badge-success";
      case "failed":
        return "badge-error";
      case "cancelled":
        return "badge-ghost";
      default:
        return "badge-ghost";
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    const date = new Date(dateStr);
    return date.toLocaleString($language);
  }

  function downloadFile() {
    if (job.output_path) {
      const link = document.createElement("a");
      link.href = `/api/files/download/${encodeURIComponent(job.output_path)}`;
      link.download = job.output_path.split("/").pop();
      link.click();
    }
  }
</script>

<div class="card bg-base-200 shadow-sm">
  <div class="card-body p-4">
    <!-- Header -->
    <div class="mb-3 flex items-start justify-between">
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2">
          <i
            class="bi {getStatusIcon(job.status)} text-lg"
            class:text-success={job.status === "completed"}
            class:text-error={job.status === "failed"}
            class:text-warning={job.status === "pending"}
            class:text-info={job.status === "processing"}
          ></i>
          <span class="badge {getStatusClass(job.status)} badge-sm">
            {t(
              $language,
              `conversion.status${job.status.charAt(0).toUpperCase() + job.status.slice(1)}`
            )}
          </span>
        </div>
        <h3
          class="mt-2 truncate text-sm font-semibold text-base-content"
          title={job.source_path}
        >
          {job.source_path.split("/").pop()}
        </h3>
        <p class="text-xs text-base-content/60">
          → {job.target_format.toUpperCase()}
        </p>
      </div>
    </div>

    <!-- Progress Bar (for processing) -->
    {#if job.status === "processing" && job.progress !== null && job.progress !== undefined}
      <div class="mb-3">
        <div class="flex justify-between text-xs text-base-content/60 mb-1">
          <span>{t($language, "conversion.progress")}</span>
          <span>{Math.round(job.progress)}%</span>
        </div>
        <progress
          class="progress progress-primary w-full"
          value={job.progress}
          max="100"
        ></progress>
      </div>
    {/if}

    <!-- Error Message -->
    {#if job.error_message}
      <div class="alert alert-error mb-3 p-2 text-xs">
        <i class="bi bi-exclamation-triangle"></i>
        <span class="truncate">{job.error_message}</span>
      </div>
    {/if}

    <!-- Details -->
    <div class="space-y-1 text-xs text-base-content/60">
      <div class="flex justify-between">
        <span>{t($language, "conversion.createdAt")}:</span>
        <span>{formatDate(job.created_at)}</span>
      </div>
      {#if job.completed_at}
        <div class="flex justify-between">
          <span>{t($language, "conversion.completedAt")}:</span>
          <span>{formatDate(job.completed_at)}</span>
        </div>
      {/if}
      {#if job.output_path}
        <div class="flex justify-between">
          <span>{t($language, "conversion.outputFile")}:</span>
          <span class="truncate max-w-[180px]" title={job.output_path}>
            {job.output_path.split("/").pop()}
          </span>
        </div>
      {/if}
    </div>

    <!-- Actions -->
    <div class="card-actions mt-4 justify-end gap-2">
      {#if job.status === "completed" && job.output_path}
        <button
          onclick={downloadFile}
          class="btn btn-success btn-xs"
          aria-label={t($language, "conversion.download")}
        >
          <i class="bi bi-download"></i>
          {t($language, "conversion.download")}
        </button>
      {/if}
      {#if job.status === "pending" || job.status === "processing"}
        <button
          onclick={oncancel}
          class="btn btn-warning btn-xs"
          aria-label={t($language, "conversion.cancel")}
        >
          <i class="bi bi-x-lg"></i>
          {t($language, "conversion.cancel")}
        </button>
      {/if}
      <button
        onclick={ondelete}
        class="btn btn-error btn-xs"
        aria-label={t($language, "conversion.delete")}
      >
        <i class="bi bi-trash"></i>
        {t($language, "conversion.delete")}
      </button>
    </div>
  </div>
</div>
