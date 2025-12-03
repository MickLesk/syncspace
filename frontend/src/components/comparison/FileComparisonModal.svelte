<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";
  import DiffMatchPatch from "diff-match-patch";

  let {
    show = $bindable(false),
    fileA = null,
    fileB = null,
    versionA = null,
    versionB = null,
    onClose = () => {},
  } = $props();

  let loading = $state(true);
  let error = $state(null);
  let comparisonData = $state(null);
  let viewMode = $state("side-by-side"); // side-by-side, unified, split
  let imageViewMode = $state("side-by-side"); // side-by-side, overlay, swipe
  let overlayOpacity = $state(50);

  const dmp = new DiffMatchPatch();

  onMount(async () => {
    if (show) {
      await loadComparison();
    }
  });

  async function loadComparison() {
    loading = true;
    error = null;

    try {
      const params = new URLSearchParams();
      if (versionA) params.append("version_a", versionA);
      if (versionB) params.append("version_b", versionB);
      if (fileB) params.append("file_b", fileB);

      const path = fileA.path || fileA;
      const response = await fetch(
        `http://localhost:8080/api/compare/${encodeURIComponent(path)}?${params}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) throw new Error("Failed to load comparison");

      comparisonData = await response.json();
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function close() {
    show = false;
    onClose();
  }

  function handleKeydown(e) {
    if (e.key === "Escape") close();
  }

  // Generate HTML diff for text files
  function generateDiff() {
    if (!comparisonData?.file_a?.content || !comparisonData?.file_b?.content) {
      return null;
    }

    const diffs = dmp.diff_main(
      comparisonData.file_a.content,
      comparisonData.file_b.content
    );
    dmp.diff_cleanupSemantic(diffs);

    return diffs;
  }

  // Convert diffs to side-by-side format
  function getSideBySideDiff() {
    const diffs = generateDiff();
    if (!diffs) return { left: [], right: [] };

    const left = [];
    const right = [];
    let leftLine = "";
    let rightLine = "";

    for (const [op, text] of diffs) {
      const lines = text.split("\n");

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const isLastLine = i === lines.length - 1;

        if (op === 0) {
          // Equal
          leftLine += line;
          rightLine += line;
        } else if (op === -1) {
          // Deletion
          leftLine += line;
        } else if (op === 1) {
          // Addition
          rightLine += line;
        }

        if (!isLastLine) {
          left.push({
            text: leftLine,
            type: op === -1 ? "removed" : "unchanged",
          });
          right.push({
            text: rightLine,
            type: op === 1 ? "added" : "unchanged",
          });
          leftLine = "";
          rightLine = "";
        }
      }
    }

    if (leftLine || rightLine) {
      left.push({ text: leftLine, type: leftLine ? "removed" : "empty" });
      right.push({ text: rightLine, type: rightLine ? "added" : "empty" });
    }

    return { left, right };
  }

  // Convert diffs to unified format
  function getUnifiedDiff() {
    const diffs = generateDiff();
    if (!diffs) return [];

    const lines = [];
    let currentLine = "";

    for (const [op, text] of diffs) {
      const textLines = text.split("\n");

      for (let i = 0; i < textLines.length; i++) {
        const line = textLines[i];
        const isLastLine = i === textLines.length - 1;

        currentLine += line;

        if (!isLastLine) {
          lines.push({
            text: currentLine,
            type: op === -1 ? "removed" : op === 1 ? "added" : "unchanged",
          });
          currentLine = "";
        }
      }
    }

    if (currentLine) {
      lines.push({
        text: currentLine,
        type: "unchanged",
      });
    }

    return lines;
  }

  function getLineClass(type) {
    switch (type) {
      case "added":
        return "bg-green-50 dark:bg-green-900/20 border-l-4 border-green-500";
      case "removed":
        return "bg-red-50 dark:bg-red-900/20 border-l-4 border-red-500";
      case "unchanged":
        return "bg-white dark:bg-gray-800";
      case "empty":
        return "bg-gray-100 dark:bg-gray-900";
      default:
        return "";
    }
  }

  $effect(() => {
    if (show && comparisonData === null) {
      loadComparison();
    }
  });
</script>

{#if show}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    role="dialog"
    tabindex="0"
    onclick={close}
    onkeydown={handleKeydown}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-7xl w-full max-h-[90vh] flex flex-col"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="button"
      tabindex="0"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700"
      >
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {$t("comparison.title")}
          </h2>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {comparisonData ? $t("comparison.comparing") : $t("loading")}
          </p>
        </div>

        <!-- View Mode Selector (for text files) -->
        {#if comparisonData?.file_a?.is_text}
          <div class="flex gap-2">
            <button
              onclick={() => (viewMode = "side-by-side")}
              class="px-3 py-1.5 text-sm rounded-lg transition-colors {viewMode ===
              'side-by-side'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}"
            >
              <i class="bi bi-layout-split" aria-hidden="true"></i>
              {$t("comparison.sideBySide")}
            </button>
            <button
              onclick={() => (viewMode = "unified")}
              class="px-3 py-1.5 text-sm rounded-lg transition-colors {viewMode ===
              'unified'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}"
            >
              <i class="bi bi-list-ul" aria-hidden="true"></i>
              {$t("comparison.unified")}
            </button>
          </div>
        {/if}

        <button
          aria-label="Close"
          onclick={close}
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
          ><i class="bi bi-x-lg" aria-hidden="true"></i></button
        >
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-auto p-6">
        {#if loading}
          <div class="flex items-center justify-center h-64">
            <div
              class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"
            ></div>
          </div>
        {:else if error}
          <div class="bg-red-50 border border-red-200 rounded-lg p-4">
            <p class="text-red-800">{error}</p>
          </div>
        {:else if comparisonData}
          <!-- Text File Comparison -->
          {#if comparisonData.file_a.is_text && comparisonData.file_b.is_text}
            {#if viewMode === "side-by-side"}
              {@const diff = getSideBySideDiff()}
              <div class="grid grid-cols-2 gap-4">
                <!-- Left Side -->
                <div
                  class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden"
                >
                  <div
                    class="bg-gray-100 dark:bg-gray-900 px-4 py-2 border-b border-gray-200 dark:border-gray-700"
                  >
                    <h3 class="font-semibold text-gray-900 dark:text-white">
                      {comparisonData.file_a.name}
                    </h3>
                    <p class="text-xs text-gray-500">
                      {comparisonData.file_a.version ||
                        $t("comparison.current")}
                    </p>
                  </div>
                  <div class="font-mono text-sm">
                    {#each diff.left as line, idx}
                      <div class="px-4 py-1 {getLineClass(line.type)}">
                        <span class="text-gray-400 mr-4">{idx + 1}</span>
                        <span class="whitespace-pre-wrap"
                          >{line.text || " "}</span
                        >
                      </div>
                    {/each}
                  </div>
                </div>

                <!-- Right Side -->
                <div
                  class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden"
                >
                  <div
                    class="bg-gray-100 dark:bg-gray-900 px-4 py-2 border-b border-gray-200 dark:border-gray-700"
                  >
                    <h3 class="font-semibold text-gray-900 dark:text-white">
                      {comparisonData.file_b.name}
                    </h3>
                    <p class="text-xs text-gray-500">
                      {comparisonData.file_b.version ||
                        $t("comparison.current")}
                    </p>
                  </div>
                  <div class="font-mono text-sm">
                    {#each diff.right as line, idx}
                      <div class="px-4 py-1 {getLineClass(line.type)}">
                        <span class="text-gray-400 mr-4">{idx + 1}</span>
                        <span class="whitespace-pre-wrap"
                          >{line.text || " "}</span
                        >
                      </div>
                    {/each}
                  </div>
                </div>
              </div>
            {:else if viewMode === "unified"}
              {@const lines = getUnifiedDiff()}
              <div
                class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden"
              >
                <div
                  class="bg-gray-100 dark:bg-gray-900 px-4 py-2 border-b border-gray-200 dark:border-gray-700"
                >
                  <h3 class="font-semibold text-gray-900 dark:text-white">
                    {$t("comparison.unifiedDiff")}
                  </h3>
                </div>
                <div class="font-mono text-sm">
                  {#each lines as line, idx}
                    <div class="px-4 py-1 {getLineClass(line.type)}">
                      <span class="text-gray-400 mr-4">{idx + 1}</span>
                      <span class="whitespace-pre-wrap">{line.text}</span>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          {:else}
            <!-- Binary/Image File Info -->
            <div class="text-center py-12">
              <i
                class="bi bi-file-earmark-binary text-6xl text-gray-300 dark:text-gray-600"
              ></i>
              <p class="mt-4 text-gray-500 dark:text-gray-400">
                {$t("comparison.binaryNotSupported")}
              </p>
              <div class="mt-6 grid grid-cols-2 gap-4 max-w-2xl mx-auto">
                <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
                  <h4 class="font-semibold text-gray-900 dark:text-white">
                    {comparisonData.file_a.name}
                  </h4>
                  <p class="text-sm text-gray-500 mt-2">
                    {$t("size")}: {(
                      comparisonData.file_a.size_bytes / 1024
                    ).toFixed(2)} KB
                  </p>
                  <p class="text-sm text-gray-500">
                    {$t("type")}: {comparisonData.file_a.mime_type}
                  </p>
                </div>
                <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
                  <h4 class="font-semibold text-gray-900 dark:text-white">
                    {comparisonData.file_b.name}
                  </h4>
                  <p class="text-sm text-gray-500 mt-2">
                    {$t("size")}: {(
                      comparisonData.file_b.size_bytes / 1024
                    ).toFixed(2)} KB
                  </p>
                  <p class="text-sm text-gray-500">
                    {$t("type")}: {comparisonData.file_b.mime_type}
                  </p>
                </div>
              </div>
            </div>
          {/if}
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex justify-end gap-2 p-6 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          onclick={close}
          class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
        >
          {$t("close")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Custom scrollbar */
  .overflow-auto::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .overflow-auto::-webkit-scrollbar-track {
    background: rgb(243 244 246);
  }

  .overflow-auto::-webkit-scrollbar-thumb {
    background: rgb(209 213 219);
    border-radius: 4px;
  }

  :global(.dark) .overflow-auto::-webkit-scrollbar-track {
    background: rgb(31 41 55);
  }

  :global(.dark) .overflow-auto::-webkit-scrollbar-thumb {
    background: rgb(55 65 81);
  }
</style>
