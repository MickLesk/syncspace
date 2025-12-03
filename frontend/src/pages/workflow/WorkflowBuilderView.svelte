<script>
  import { onMount, onDestroy } from "svelte";
  import { workflow } from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import WorkflowRuleCard from "../../components/workflow/WorkflowRuleCard.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import { modals, modalEvents } from "../../stores/modals.js";

  let rules = $state([]);
  let triggerTypes = $state([]);
  let actionTypes = $state([]);
  let loading = $state(true);
  let filterTriggerType = $state("");
  let filterActionType = $state("");
  let filterStatus = $state("all");

  let unsubscribe;

  onMount(async () => {
    await Promise.all([loadRules(), loadTriggerTypes(), loadActionTypes()]);
    loading = false;

    // Listen for modal save events
    unsubscribe = modalEvents.on("workflowRuleSaved", async () => {
      await loadRules();
    });
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
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
    modals.openWorkflowEditor(null, triggerTypes, actionTypes);
  }

  function openEditRule(rule) {
    modals.openWorkflowEditor(rule, triggerTypes, actionTypes);
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
  <div class="min-h-screen bg-gradient-to-br from-base-100 to-base-200 p-6">
    <div class="max-w-7xl mx-auto">
      <!-- Modern Header with Glass Effect -->
      <div
        class="mb-8 bg-base-100/50 backdrop-blur-xl rounded-2xl shadow-2xl border border-base-300 p-8"
      >
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-4">
            <div
              class="p-4 bg-gradient-to-br from-cyan-500 to-blue-500 rounded-xl shadow-lg"
            >
              <i class="bi bi-diagram-3-fill text-3xl text-white" aria-hidden="true"></i>
            </div>
            <div>
              <h1
                class="text-4xl font-bold bg-gradient-to-r from-cyan-600 to-blue-600 bg-clip-text text-transparent"
              >
                {t($currentLang, "workflow.title")}
              </h1>
              <p class="text-base-content/60 mt-1 text-lg">
                {t($currentLang, "workflow.description")}
              </p>
            </div>
          </div>
          <button class="btn btn-primary gap-2" onclick={openCreateRule}>
            <i class="bi bi-plus-circle" aria-hidden="true"></i>
            {t($currentLang, "workflow.createRule")}
          </button>
        </div>

        <!-- Modern Filters in Grid -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold"
                >{t($currentLang, "workflow.filterByTrigger")}</span
              >
            </label>
            <select
              bind:value={filterTriggerType}
              class="select select-bordered bg-base-100/80 backdrop-blur"
            >
              <option value="">{t($currentLang, "workflow.allTriggers")}</option
              >
              {#each triggerTypes as triggerType}
                <option value={triggerType.value}>{triggerType.label}</option>
              {/each}
            </select>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold"
                >{t($currentLang, "workflow.filterByAction")}</span
              >
            </label>
            <select
              bind:value={filterActionType}
              class="select select-bordered bg-base-100/80 backdrop-blur"
            >
              <option value="">{t($currentLang, "workflow.allActions")}</option>
              {#each actionTypes as actionType}
                <option value={actionType.value}>{actionType.label}</option>
              {/each}
            </select>
          </div>

          <div class="form-control">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold"
                  >{t($currentLang, "workflow.filterByStatus")}</span
                >
              </label>
              <select
                bind:value={filterStatus}
                class="select select-bordered bg-base-100/80 backdrop-blur"
              >
                <option value="all"
                  >{t($currentLang, "workflow.allStatus")}</option
                >
                <option value="active"
                  >{t($currentLang, "workflow.activeRules")}</option
                >
                <option value="inactive"
                  >{t($currentLang, "workflow.inactiveRules")}</option
                >
              </select>
            </div>

            <div class="form-control">
              <div class="label opacity-0">Actions</div>
              <button onclick={clearFilters} class="btn btn-outline w-full">
                <i class="bi bi-x-circle" aria-hidden="true"></i>
                {t($currentLang, "workflow.clearFilters")}
              </button>
            </div>
          </div>
        </div>

        <!-- Rules Grid -->
        {#if loading}
          <div class="flex justify-center items-center py-12">
            <span class="loading loading-spinner loading-lg"></span>
          </div>
        {:else if rules.length === 0}
          <div class="text-center py-12 bg-base-200 rounded-lg">
            <i class="bi bi-gear-wide text-6xl text-base-content/30 mb-4" aria-hidden="true"></i>
            <p class="text-lg text-base-content/70 mb-4">
              {t($currentLang, "workflow.noRules")}
            </p>
            <button aria-label="Add" onclick={openCreateRule} class="btn btn-primary"><i class="bi bi-plus-lg" aria-hidden="true"></i>
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
    </div>
  </div></PageWrapper
>

<style>
  .label {
    padding: 0.25rem 0;
  }
</style>
