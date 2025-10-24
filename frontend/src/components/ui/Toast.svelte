<script>
  import { toasts, removeToast } from "../../stores/toast.js";

  function getIcon(type) {
    switch (type) {
      case "success":
        return "check-circle-fill";
      case "error":
        return "x-circle-fill";
      case "warning":
        return "exclamation-triangle-fill";
      case "info":
        return "info-circle-fill";
      default:
        return "info-circle-fill";
    }
  }

  function getAlertClass(type) {
    switch (type) {
      case "success":
        return "alert-success";
      case "error":
        return "alert-error";
      case "warning":
        return "alert-warning";
      case "info":
        return "alert-info";
      default:
        return "alert-info";
    }
  }
</script>

<div class="toast toast-end toast-bottom z-[9999]">
  {#each $toasts as toast (toast.id)}
    <div class="alert {getAlertClass(toast.type)} shadow-lg">
      <div class="flex items-center gap-3">
        <i class="bi bi-{getIcon(toast.type)} text-xl"></i>
        <span class="font-medium">{toast.message}</span>
      </div>
      <button
        class="btn btn-ghost btn-sm btn-square"
        on:click={() => removeToast(toast.id)}
        aria-label="Close"
      >
        <i class="bi bi-x-lg"></i>
      </button>
    </div>
  {/each}
</div>

<style>
  .alert {
    min-width: 320px;
    max-width: 500px;
    animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @media (max-width: 640px) {
    .alert {
      min-width: unset;
      max-width: calc(100vw - 2rem);
    }
  }
</style>
