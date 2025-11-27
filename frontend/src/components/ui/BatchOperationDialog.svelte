<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { toast } from "../../stores/ui";
  import { t } from "../../lib/i18n";

  let isOpen = $state(false);
  let operationType = $state("copy"); // 'copy' | 'move'
  let items = $state([]);
  let destinationPath = $state("");
  let progress = $state(0);
  let currentFile = $state("");
  let isProcessing = $state(false);
  let conflicts = $state([]);
  let conflictResolutions = $state({});
  let showConflictDialog = $state(false);
  let currentConflictIndex = $state(0);
  let totalItems = $state(0);
  let processedItems = $state(0);
  let skippedItems = $state(0);
  let failedItems = $state([]);
  let errors = $state({});

  // Conflict resolution strategies
  const CONFLICT_STRATEGIES = {
    SKIP: "skip",
    OVERWRITE: "overwrite",
    RENAME: "rename",
  };

  export let onComplete = null;
  export let onCancel = null;

  function open(type, filesToMove, destination) {
    operationType = type;
    items = filesToMove;
    destinationPath = destination;
    conflicts = [];
    conflictResolutions = {};
    failedItems = [];
    errors = {};
    progress = 0;
    totalItems = filesToMove.length;
    processedItems = 0;
    skippedItems = 0;
    isOpen = true;
    currentConflictIndex = 0;

    // Start detecting conflicts
    detectConflicts();
  }

  async function detectConflicts() {
    try {
      // Load destination directory to find conflicts
      const destItems = await api.files.listDirectory(destinationPath);
      const destNames = new Set(destItems.data?.map((i) => i.name) || []);

      conflicts = items.filter((item) => destNames.has(item.name));

      // If no conflicts, start operation directly
      if (conflicts.length === 0) {
        startOperation();
      } else {
        showConflictDialog = true;
      }
    } catch (error) {
      console.error("Error detecting conflicts:", error);
      startOperation(); // Try anyway
    }
  }

  function handleConflictResolution(strategy, customName = null) {
    const conflict = conflicts[currentConflictIndex];

    conflictResolutions[conflict.path] = {
      strategy,
      customName,
      originalName: conflict.name,
    };

    if (currentConflictIndex < conflicts.length - 1) {
      currentConflictIndex++;
    } else {
      showConflictDialog = false;
      startOperation();
    }
  }

  async function startOperation() {
    isProcessing = true;
    processedItems = 0;
    skippedItems = 0;
    failedItems = [];
    errors = {};

    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      currentFile = item.name;

      try {
        const resolution = conflictResolutions[item.path];

        // Check if this item should be skipped
        if (resolution?.strategy === CONFLICT_STRATEGIES.SKIP) {
          skippedItems++;
          processedItems++;
          progress = Math.round((processedItems / totalItems) * 100);
          continue;
        }

        let targetName = item.name;

        // Handle rename conflict
        if (resolution?.strategy === CONFLICT_STRATEGIES.RENAME) {
          targetName = resolution.customName;
        }

        const targetPath = `${destinationPath}/${targetName}`;

        if (operationType === "copy") {
          await api.files.copy(item.path, targetPath);
        } else if (operationType === "move") {
          await api.files.move(item.path, targetPath);
        }

        processedItems++;
        progress = Math.round((processedItems / totalItems) * 100);
      } catch (error) {
        console.error(`Error ${operationType}ing ${item.name}:`, error);
        failedItems.push(item.name);
        errors[item.path] = error.message || t("errors.operationFailed");
        processedItems++;
        progress = Math.round((processedItems / totalItems) * 100);
      }
    }

    isProcessing = false;
    completeOperation();
  }

  function completeOperation() {
    const message =
      operationType === "copy"
        ? t("batch.copyComplete", { count: processedItems })
        : t("batch.moveComplete", { count: processedItems });

    toast.show(message, "success");

    if (onComplete) {
      onComplete({
        type: operationType,
        destination: destinationPath,
        processedItems,
        skippedItems,
        failedItems,
        errors,
      });
    }

    close();
  }

  function close() {
    isOpen = false;
    if (onCancel && isProcessing) {
      onCancel();
    }
  }

  function handleKeyDown(e) {
    if (e.key === "Escape" && !isProcessing) {
      close();
    }
  }

  function getSuggestedName(originalName) {
    const parts = originalName.split(".");
    const ext = parts.pop();
    const name = parts.join(".");
    return `${name} - copy.${ext}`;
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    role="dialog"
    aria-labelledby="batch-op-title"
    aria-modal="true"
    onkeydown={handleKeyDown}
  >
    <div
      class="w-full max-w-2xl rounded-lg bg-white shadow-xl dark:bg-gray-800"
    >
      <!-- Header -->
      <div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
        <h2
          id="batch-op-title"
          class="text-lg font-semibold text-gray-900 dark:text-white"
        >
          {operationType === "copy"
            ? t("batch.copyFiles")
            : t("batch.moveFiles")}
        </h2>
      </div>

      <!-- Content -->
      <div class="p-6">
        {#if showConflictDialog}
          <!-- Conflict Resolution -->
          <div class="space-y-4">
            <div class="text-sm text-gray-600 dark:text-gray-400">
              {t("batch.conflictDetected", {
                current: currentConflictIndex + 1,
                total: conflicts.length,
              })}
            </div>

            <div
              class="rounded border border-yellow-200 bg-yellow-50 p-4 dark:border-yellow-800 dark:bg-yellow-900/20"
            >
              <div class="flex gap-3">
                <i
                  class="bi bi-exclamation-triangle text-yellow-600 dark:text-yellow-400 mt-0.5"
                ></i>
                <div>
                  <div class="font-medium text-gray-900 dark:text-white">
                    {conflicts[currentConflictIndex]?.name}
                  </div>
                  <div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                    {t("batch.fileAlreadyExists")}
                  </div>
                </div>
              </div>
            </div>

            <!-- Resolution Options -->
            <div class="space-y-2">
              <button
                onclick={() =>
                  handleConflictResolution(CONFLICT_STRATEGIES.SKIP)}
                class="w-full rounded border border-gray-300 px-4 py-2 text-left transition-colors hover:bg-gray-50 dark:border-gray-600 dark:hover:bg-gray-700 text-gray-900 dark:text-white"
              >
                <div class="font-medium">{t("batch.skip")}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  {t("batch.skipDescription")}
                </div>
              </button>

              <button
                onclick={() =>
                  handleConflictResolution(CONFLICT_STRATEGIES.OVERWRITE)}
                class="w-full rounded border border-gray-300 px-4 py-2 text-left transition-colors hover:bg-gray-50 dark:border-gray-600 dark:hover:bg-gray-700 text-gray-900 dark:text-white"
              >
                <div class="font-medium">{t("batch.overwrite")}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  {t("batch.overwriteDescription")}
                </div>
              </button>

              <div class="space-y-2">
                <div
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                >
                  {t("batch.rename")}
                </div>
                <input
                  type="text"
                  value={getSuggestedName(
                    conflicts[currentConflictIndex]?.name
                  )}
                  onchange={(e) =>
                    handleConflictResolution(
                      CONFLICT_STRATEGIES.RENAME,
                      e.target.value
                    )}
                  class="w-full rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                  placeholder={t("batch.enterNewName")}
                />
              </div>
            </div>
          </div>
        {:else}
          <!-- Progress View -->
          <div class="space-y-4">
            <!-- Stats -->
            <div class="grid grid-cols-4 gap-2 text-center">
              <div>
                <div class="text-2xl font-bold text-gray-900 dark:text-white">
                  {processedItems}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {t("common.processed")}
                </div>
              </div>
              <div>
                <div class="text-2xl font-bold text-gray-900 dark:text-white">
                  {totalItems - processedItems}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {t("common.remaining")}
                </div>
              </div>
              <div>
                <div
                  class="text-2xl font-bold text-yellow-600 dark:text-yellow-400"
                >
                  {skippedItems}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {t("common.skipped")}
                </div>
              </div>
              <div>
                <div class="text-2xl font-bold text-red-600 dark:text-red-400">
                  {failedItems.length}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {t("common.failed")}
                </div>
              </div>
            </div>

            <!-- Progress Bar -->
            <div>
              <div class="mb-2 flex items-center justify-between">
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                >
                  {currentFile || t("batch.preparing")}
                </span>
                <span class="text-sm text-gray-500 dark:text-gray-400"
                  >{progress}%</span
                >
              </div>
              <div
                class="h-2 w-full rounded-full bg-gray-200 dark:bg-gray-700 overflow-hidden"
              >
                <div
                  class="h-full bg-blue-500 transition-all duration-300"
                  style="width: {progress}%"
                ></div>
              </div>
            </div>

            <!-- Failed Items -->
            {#if failedItems.length > 0}
              <div
                class="rounded border border-red-200 bg-red-50 p-3 dark:border-red-800 dark:bg-red-900/20"
              >
                <div
                  class="text-sm font-medium text-red-900 dark:text-red-200 mb-2"
                >
                  {t("batch.failedItems", { count: failedItems.length })}
                </div>
                <ul class="space-y-1 max-h-32 overflow-y-auto">
                  {#each failedItems as item}
                    <li class="text-xs text-red-800 dark:text-red-300">
                      • {item}
                      {#if errors[item]}
                        <div class="text-red-700 dark:text-red-400">
                          {errors[item]}
                        </div>
                      {/if}
                    </li>
                  {/each}
                </ul>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      {#if !showConflictDialog}
        <div
          class="border-t border-gray-200 flex gap-3 px-6 py-4 dark:border-gray-700"
        >
          <button
            onclick={close}
            disabled={isProcessing}
            class="flex-1 rounded border border-gray-300 px-4 py-2 font-medium text-gray-700 transition-colors hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
          >
            {isProcessing ? t("common.cancel") : t("common.close")}
          </button>
          {#if progress === 100}
            <button
              onclick={close}
              class="flex-1 rounded bg-blue-500 px-4 py-2 font-medium text-white transition-colors hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700"
            >
              {t("common.done")}
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}
