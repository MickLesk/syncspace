<script>
  import { onMount } from "svelte";
  import { workflow } from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import WorkflowRuleCard from "../../components/workflow/WorkflowRuleCard.svelte";
  import WorkflowEditor from "../../components/workflow/WorkflowEditor.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";

  let rules = $state([]);
  let triggerTypes = $state([]);
  let actionTypes = $state([]);
  let loading = $state(true);
  let showEditor = $state(false);
  let editingRule = $state(null);
  let filterTriggerType = $state("");
  let filterActionType = $state("");
  let filterStatus = $state("all");

  onMount(async () => {
    await Promise.all([loadRules(), loadTriggerTypes(), loadActionTypes()]);
    loading = false;
  });

  async function loadRules() {
    try {
      const isActive =
        filterStatus === "all" ? null : filterStatus === "active";
      const response = await workflow.listRules(
        filterTriggerType || null,
        filterActionType || null,
        isActive,
        true // Include stats
      );
      rules = response || [];
    } catch (error) {
      console.error("Failed to load workflow rules:", error);
    }
  }

  async function loadTriggerTypes() {
    try {
      const response = await workflow.listTriggerTypes();
      triggerTypes = response.trigger_types || [];
    } catch (error) {
      console.error("Failed to load trigger types:", error);
    }
  }

  async function loadActionTypes() {
    try {
      const response = await workflow.listActionTypes();
      actionTypes = response.action_types || [];
    } catch (error) {
      console.error("Failed to load action types:", error);
    }
  }

  function openCreateRule() {
    editingRule = null;
    showEditor = true;
  }

  function openEditRule(rule) {
    editingRule = rule;
    showEditor = true;
  }

  async function handleRuleSaved() {
    showEditor = false;
    editingRule = null;
    await loadRules();
  }

  async function handleDeleteRule(rule) {
    if (
      !confirm(
        t($currentLang, "workflow.confirmDeleteRule").replace(
          "{name}",
          rule.display_name
        )
      )
    ) {
      return;
    }

    try {
      await workflow.deleteRule(rule.id);
      await loadRules();
    } catch (error) {
      console.error("Failed to delete workflow rule:", error);
      alert(t($currentLang, "workflow.deleteRuleError"));
    }
  }

  async function handleToggleRule(rule) {
    try {
      await workflow.toggleRule(rule.id);
      await loadRules();
    } catch (error) {
      console.error("Failed to toggle workflow rule:", error);
    }
  }

  async function applyFilters() {
    await loadRules();
  }

  async function clearFilters() {
    filterTriggerType = "";
    filterActionType = "";
    filterStatus = "all";
    await loadRules();
  }

  $effect(() => {
    // Reactive loading when filters change
    if (!loading) {
      loadRules();
    }
  });
</script>

<PageWrapper>
  <div class="p-6 max-w-7xl mx-auto">
    <!-- Header -->
    <div class="mb-6">
      <h1 class="text-3xl font-bold mb-2">
        {t($currentLang, "workflow.title")}
      </h1>
      <p class="text-base-content/70">
        {t($currentLang, "workflow.description")}
      </p>
    </div>

    <!-- Filters and Actions -->
    <div class="flex flex-wrap gap-4 mb-6">
      <div class="flex-1 min-w-[200px]">
        <label class="label">
          <span class="label-text"
            >{t($currentLang, "workflow.filterByTrigger")}</span
          >
        </label>
        <select
          bind:value={filterTriggerType}
          class="select select-bordered w-full"
        >
          <option value="">{t($currentLang, "workflow.allTriggers")}</option>
          {#each triggerTypes as triggerType}
            <option value={triggerType.value}>{triggerType.label}</option>
          {/each}
        </select>
      </div>

      <div class="flex-1 min-w-[200px]">
        <label class="label">
          <span class="label-text"
            >{t($currentLang, "workflow.filterByAction")}</span
          >
        </label>
        <select
          bind:value={filterActionType}
          class="select select-bordered w-full"
        >
          <option value="">{t($currentLang, "workflow.allActions")}</option>
          {#each actionTypes as actionType}
            <option value={actionType.value}>{actionType.label}</option>
          {/each}
        </select>
      </div>

      <div class="flex-1 min-w-[200px]">
        <label class="label">
          <span class="label-text"
            >{t($currentLang, "workflow.filterByStatus")}</span
          >
        </label>
        <select bind:value={filterStatus} class="select select-bordered w-full">
          <option value="all">{t($currentLang, "workflow.allStatus")}</option>
          <option value="active"
            >{t($currentLang, "workflow.activeRules")}</option
          >
          <option value="inactive"
            >{t($currentLang, "workflow.inactiveRules")}</option
          >
        </select>
      </div>

      <div class="flex items-end gap-2">
        <button onclick={clearFilters} class="btn btn-ghost">
          {t($currentLang, "workflow.clearFilters")}
        </button>
        <button onclick={openCreateRule} class="btn btn-primary">
          <i class="bi bi-plus-lg"></i>
          {t($currentLang, "workflow.createRule")}
        </button>
      </div>
    </div>

    <!-- Rules Grid -->
    {#if loading}
      <div class="flex justify-center items-center py-12">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
    {:else if rules.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-lg">
        <i class="bi bi-gear-wide text-6xl text-base-content/30 mb-4"></i>
        <p class="text-lg text-base-content/70 mb-4">
          {t($currentLang, "workflow.noRules")}
        </p>
        <button onclick={openCreateRule} class="btn btn-primary">
          <i class="bi bi-plus-lg"></i>
          {t($currentLang, "workflow.createFirstRule")}
        </button>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each rules as rule (rule.id)}
          <WorkflowRuleCard
            {rule}
            {triggerTypes}
            {actionTypes}
            onEdit={() => openEditRule(rule)}
            onDelete={() => handleDeleteRule(rule)}
            onToggle={() => handleToggleRule(rule)}
          />
        {/each}
      </div>
    {/if}
  </div>
</PageWrapper>

{#if showEditor}
  <WorkflowEditor
    rule={editingRule}
    {triggerTypes}
    {actionTypes}
    onSave={handleRuleSaved}
    onCancel={() => {
      showEditor = false;
      editingRule = null;
    }}
  />
{/if}

<style>
  .label {
    padding: 0.25rem 0;
  }
</style>
