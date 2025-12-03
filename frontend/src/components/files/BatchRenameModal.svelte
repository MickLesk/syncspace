<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import api from "../../lib/api.js";
  import Modal from "../ui/Modal.svelte";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Props
  let {
    isOpen = $bindable(false),
    files = [],
    onClose = () => {},
    onComplete = () => {},
  } = $props();

  // State
  let renameMode = $state("prefix"); // prefix, suffix, replace, sequential, pattern
  let prefixValue = $state("");
  let suffixValue = $state("");
  let searchValue = $state("");
  let replaceValue = $state("");
  let sequentialStart = $state(1);
  let sequentialPadding = $state(0);
  let patternValue = $state("[ORIGINAL]");
  let previewMode = $state(true);
  let isProcessing = $state(false);

  // Preview of renamed files
  const preview = $derived.by(() => {
    if (!files || files.length === 0) return [];

    return files.map((file, idx) => {
      let newName = file.name;
      const ext =
        newName.lastIndexOf(".") > 0
          ? newName.substring(newName.lastIndexOf("."))
          : "";
      const nameWithoutExt = ext
        ? newName.substring(0, newName.lastIndexOf("."))
        : newName;

      switch (renameMode) {
        case "prefix":
          newName = prefixValue + nameWithoutExt + ext;
          break;
        case "suffix":
          newName = nameWithoutExt + suffixValue + ext;
          break;
        case "replace":
          newName = nameWithoutExt.replaceAll(searchValue, replaceValue) + ext;
          break;
        case "sequential":
          const paddedNum = String(sequentialStart + idx).padStart(
            sequentialPadding,
            "0"
          );
          newName = paddedNum + "_" + nameWithoutExt + ext;
          break;
        case "pattern":
          let result = patternValue;
          result = result.replace("[ORIGINAL]", nameWithoutExt);
          result = result.replace("[INDEX]", idx + 1);
          result = result.replace(
            "[PADDED]",
            String(sequentialStart + idx).padStart(sequentialPadding, "0")
          );
          result = result.replace(
            "[DATE]",
            new Date().toISOString().split("T")[0]
          );
          newName = result + ext;
          break;
      }

      return {
        original: file.name,
        new: newName,
        file: file,
      };
    });
  });

  const hasChanges = $derived(preview.some((p) => p.original !== p.new));

  async function performRename() {
    if (!hasChanges) {
      errorToast(tr("batchRename.noChanges"));
      return;
    }

    isProcessing = true;
    try {
      let successCount = 0;
      let errorCount = 0;

      for (const item of preview) {
        if (item.original === item.new) continue;

        try {
          const response = await api.files.rename(item.file.path, item.new);

          if (response.ok) {
            successCount++;
          } else {
            errorCount++;
            console.error(`Failed to rename ${item.original}`);
          }
        } catch (err) {
          errorCount++;
          console.error(`Failed to rename ${item.original}:`, err);
        }
      }

      if (successCount > 0) {
        success(tr("batchRename.success", successCount));
      }
      if (errorCount > 0) {
        errorToast(tr("batchRename.errors", errorCount));
      }

      onComplete?.();
      isOpen = false;
    } finally {
      isProcessing = false;
    }
  }

  function resetValues() {
    prefixValue = "";
    suffixValue = "";
    searchValue = "";
    replaceValue = "";
    sequentialStart = 1;
    sequentialPadding = 0;
    patternValue = "[ORIGINAL]";
  }
</script>

<Modal {isOpen} onClose={() => (isOpen = false)} size="lg">
  <div class="space-y-4">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h2
        class="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2"
      >
        <i class="bi bi-type text-indigo-500" aria-hidden="true"></i>
        {tr("batchRename.title")}
      </h2>
      <span class="text-sm text-gray-500 dark:text-gray-400"
        >{files.length} {tr("files")}</span
      >
    </div>

    <!-- Mode Selection -->
    <div class="space-y-3">
      <div class="text-sm font-medium text-gray-900 dark:text-white">
        {tr("batchRename.mode")}
      </div>

      <div class="grid grid-cols-2 gap-2 md:grid-cols-3">
        {#each [{ value: "prefix", label: tr("batchRename.prefix"), icon: "bi-arrow-left" }, { value: "suffix", label: tr("batchRename.suffix"), icon: "bi-arrow-right" }, { value: "replace", label: tr("batchRename.replace"), icon: "bi-arrow-left-right" }, { value: "sequential", label: tr("batchRename.sequential"), icon: "bi-list-ol" }, { value: "pattern", label: tr("batchRename.pattern"), icon: "bi-regex" }] as mode}
          <button
            onclick={() => (renameMode = mode.value)}
            class="px-3 py-2 rounded-lg border transition flex items-center gap-2 justify-center text-sm {renameMode ===
            mode.value
              ? 'bg-indigo-500 border-indigo-500 text-white'
              : 'bg-white dark:bg-gray-700 border-gray-200 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:border-indigo-300'}"
          >
            <i class="bi {mode.icon}"></i>
            <span class="hidden sm:inline">{mode.label}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Mode-specific inputs -->
    <div class="space-y-3 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
      {#if renameMode === "prefix"}
        <div>
          <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            {tr("batchRename.prefixValue")}
          </div>
          <input
            bind:value={prefixValue}
            type="text"
            placeholder="e.g., 2025-"
            class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
          />
        </div>
      {:else if renameMode === "suffix"}
        <div>
          <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            {tr("batchRename.suffixValue")}
          </div>
          <input
            bind:value={suffixValue}
            type="text"
            placeholder="e.g., _final"
            class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
          />
        </div>
      {:else if renameMode === "replace"}
        <div class="grid grid-cols-2 gap-3">
          <div>
            <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {tr("batchRename.search")}
            </div>
            <input
              bind:value={searchValue}
              type="text"
              placeholder="Find..."
              class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
            />
          </div>
          <div>
            <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {tr("batchRename.replace")}
            </div>
            <input
              bind:value={replaceValue}
              type="text"
              placeholder="Replace..."
              class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
            />
          </div>
        </div>
      {:else if renameMode === "sequential"}
        <div class="grid grid-cols-2 gap-3">
          <div>
            <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {tr("batchRename.startNumber")}
            </div>
            <input
              bind:value={sequentialStart}
              type="number"
              min="1"
              class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
            />
          </div>
          <div>
            <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {tr("batchRename.padding")}
            </div>
            <input
              bind:value={sequentialPadding}
              type="number"
              min="0"
              max="10"
              class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500"
            />
          </div>
        </div>
      {:else if renameMode === "pattern"}
        <div>
          <div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            {tr("batchRename.pattern")}
          </div>
          <input
            bind:value={patternValue}
            type="text"
            placeholder="[ORIGINAL]_[INDEX]"
            class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-indigo-500 mb-2"
          />
          <div class="text-xs text-gray-600 dark:text-gray-400 space-y-1">
            <div>
              <strong>[ORIGINAL]</strong> - {tr("batchRename.patternOriginal")}
            </div>
            <div>
              <strong>[INDEX]</strong> - {tr("batchRename.patternIndex")}
            </div>
            <div>
              <strong>[PADDED]</strong> - {tr("batchRename.patternPadded")}
            </div>
            <div><strong>[DATE]</strong> - {tr("batchRename.patternDate")}</div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Preview -->
    {#if previewMode && preview.length > 0}
      <div class="space-y-2">
        <div class="flex items-center justify-between">
          <h3 class="text-sm font-medium text-gray-900 dark:text-white">
            {tr("batchRename.preview")}
          </h3>
          <span class="text-xs text-gray-500 dark:text-gray-400">
            {preview.filter((p) => p.original !== p.new)
              .length}/{preview.length}
            {tr("batchRename.willChange")}
          </span>
        </div>

        <div
          class="max-h-48 overflow-y-auto space-y-1 bg-gray-50 dark:bg-gray-700/50 p-2 rounded"
        >
          {#each preview.slice(0, 10) as item}
            <div class="text-xs font-mono flex items-center gap-2">
              {#if item.original === item.new}
                <i class="bi bi-dash-circle text-gray-400" aria-hidden="true"></i>
                <span class="text-gray-600 dark:text-gray-400"
                  >{item.original}</span
                >
              {:else}
                <i class="bi bi-arrow-right text-green-500" aria-hidden="true"></i>
                <span class="text-gray-600 dark:text-gray-400 line-through"
                  >{item.original}</span
                >
                <span class="text-green-600 dark:text-green-400"
                  >{item.new}</span
                >
              {/if}
            </div>
          {/each}
          {#if preview.length > 10}
            <div class="text-xs text-gray-500 dark:text-gray-400 p-1">
              ... {tr("batchRename.and")}
              {preview.length - 10}
              {tr("batchRename.more")}
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Actions -->
    <div class="flex gap-2 justify-end pt-4">
      <button
        onclick={() => (previewMode = !previewMode)}
        class="px-3 py-2 text-sm bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition"
      >
        {previewMode ? tr("hide") : tr("show")}
        {tr("preview")}
      </button>

      <button
        onclick={resetValues}
        class="px-3 py-2 text-sm bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition"
      >
        {tr("reset")}
      </button>

      <button
        onclick={() => (isOpen = false)}
        class="px-4 py-2 text-sm bg-gray-300 dark:bg-gray-600 text-gray-900 dark:text-white rounded-lg hover:bg-gray-400 dark:hover:bg-gray-500 transition"
      >
        {tr("cancel")}
      </button>

      <button
        onclick={performRename}
        disabled={!hasChanges || isProcessing}
        class="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition flex items-center gap-2"
      >
        {#if isProcessing}
          <i class="bi bi-hourglass-split animate-spin" aria-hidden="true"></i>
          {tr("processing")}
        {:else}
          <i class="bi bi-check2" aria-hidden="true"></i>
          {tr("batchRename.apply")}
        {/if}
      </button>
    </div>
  </div>
</Modal>
