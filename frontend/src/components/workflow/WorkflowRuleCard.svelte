<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let { rule, triggerTypes, actionTypes, onEdit, onDelete, onToggle } =
    $props();

  function getTriggerLabel(value) {
    const type = triggerTypes.find((t) => t.value === value);
    return type ? type.label : value;
  }

  function getActionLabel(value) {
    const type = actionTypes.find((a) => a.value === value);
    return type ? type.label : value;
  }

  function getStatusBadge() {
    return rule.is_active ? "badge-success" : "badge-ghost";
  }

  function getSuccessRate() {
    if (rule.execution_count === 0) return 0;
    return Math.round((rule.success_count / rule.execution_count) * 100);
  }
</script>

<div class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all hover:shadow-lg transition-shadow">
  <div class="card-body p-4">
    <!-- Header -->
    <div class="flex justify-between items-start mb-3">
      <div class="flex-1">
        <h3 class="font-semibold text-lg">{rule.display_name}</h3>
        {#if rule.description}
          <p class="text-sm text-base-content/70 mt-1">{rule.description}</p>
        {/if}
      </div>
      <span class="badge {getStatusBadge()} ml-2">
        {rule.is_active
          ? t($currentLang, "workflow.active")
          : t($currentLang, "workflow.inactive")}
      </span>
    </div>

    <!-- Workflow Flow -->
    <div class="flex items-center gap-2 mb-3 text-sm">
      <div class="badge badge-primary badge-outline">
        <i class="bi bi-lightning-charge mr-1" aria-hidden="true"></i>
        {getTriggerLabel(rule.trigger_type)}
      </div>
      <i class="bi bi-arrow-right text-base-content/50" aria-hidden="true"></i>
      <div class="badge badge-secondary badge-outline">
        <i class="bi bi-gear mr-1" aria-hidden="true"></i>
        {getActionLabel(rule.action_type)}
      </div>
    </div>

    <!-- Stats -->
    <div class="grid grid-cols-3 gap-2 text-center mb-3 text-sm">
      <div class="bg-base-200 p-2 rounded">
        <div class="font-semibold">{rule.execution_count || 0}</div>
        <div class="text-xs text-base-content/70">
          {t($currentLang, "workflow.executions")}
        </div>
      </div>
      <div class="bg-base-200 p-2 rounded">
        <div class="font-semibold text-success">{rule.success_count || 0}</div>
        <div class="text-xs text-base-content/70">
          {t($currentLang, "workflow.success")}
        </div>
      </div>
      <div class="bg-base-200 p-2 rounded">
        <div class="font-semibold text-error">{rule.failed_count || 0}</div>
        <div class="text-xs text-base-content/70">
          {t($currentLang, "workflow.failed")}
        </div>
      </div>
    </div>

    {#if rule.execution_count > 0}
      <div class="mb-3">
        <div class="flex justify-between text-xs mb-1">
          <span>{t($currentLang, "workflow.successRate")}</span>
          <span class="font-semibold">{getSuccessRate()}%</span>
        </div>
        <progress
          class="progress progress-success w-full"
          value={getSuccessRate()}
          max="100"
        ></progress>
      </div>
    {/if}

    {#if rule.last_execution}
      <div class="text-xs text-base-content/70 mb-3">
        {t($currentLang, "workflow.lastRun")}: {new Date(
          rule.last_execution
        ).toLocaleString()}
      </div>
    {/if}

    <!-- Actions -->
    <div class="flex gap-2">
      <button
        onclick={onToggle}
        class="btn btn-sm flex-1"
        title={rule.is_active
          ? t($currentLang, "workflow.disable")
          : t($currentLang, "workflow.enable")}
      >
        <i
          class="bi bi-{rule.is_active ? 'pause' : 'play'}-circle"
          aria-hidden="true"
        ></i>
        {rule.is_active
          ? t($currentLang, "workflow.disable")
          : t($currentLang, "workflow.enable")}
      </button>
      <button
        onclick={onEdit}
        class="btn btn-sm btn-ghost"
        title={t($currentLang, "workflow.editRule")}
      >
        <i class="bi bi-pencil" aria-hidden="true"></i>
      </button>
      <button
        aria-label="Delete"
        onclick={onDelete}
        class="btn btn-sm btn-ghost text-error"
        title={t($currentLang, "workflow.deleteRule")}
        ><i class="bi bi-trash" aria-hidden="true"></i></button
      >
    </div>
  </div>
</div>
