<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api.js";
  import * as monaco from "monaco-editor";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  let { isOpen = $bindable(false), filePath, fileId, onSave } = $props();

  let editor = $state(null);
  let editorContainer = $state(null);
  let loading = $state(true);
  let saving = $state(false);
  let originalContent = $state("");
  let hasUnsavedChanges = $state(false);
  let autoSaveTimeout = $state(null);
  let editorMode = $state("single"); // 'single' or 'split'

  // Detect language from file extension
  function detectLanguage(path) {
    const ext = path.split(".").pop()?.toLowerCase();
    const languageMap = {
      js: "javascript",
      jsx: "javascript",
      ts: "typescript",
      tsx: "typescript",
      py: "python",
      rs: "rust",
      go: "go",
      java: "java",
      cpp: "cpp",
      c: "c",
      cs: "csharp",
      php: "php",
      rb: "ruby",
      swift: "swift",
      kt: "kotlin",
      scala: "scala",
      sh: "shell",
      bash: "shell",
      sql: "sql",
      html: "html",
      css: "css",
      scss: "scss",
      sass: "sass",
      less: "less",
      json: "json",
      xml: "xml",
      yaml: "yaml",
      yml: "yaml",
      md: "markdown",
      txt: "plaintext",
      log: "plaintext",
    };
    return languageMap[ext] || "plaintext";
  }

  async function loadFile() {
    if (!filePath) return;

    loading = true;
    try {
      const response = await fetch(
        `/api/files/download/${encodeURIComponent(filePath)}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) throw new Error("Failed to load file");

      const content = await response.text();
      originalContent = content;

      if (editor) {
        editor.setValue(content);
        hasUnsavedChanges = false;
      }
    } catch (err) {
      console.error("Failed to load file:", err);
      errorToast(tr("failedToLoadFile"));
    } finally {
      loading = false;
    }
  }

  async function saveFile() {
    if (!editor || saving) return;

    saving = true;
    try {
      const content = editor.getValue();
      const blob = new Blob([content], { type: "text/plain" });
      const formData = new FormData();
      formData.append("file", blob, filePath.split("/").pop());
      formData.append("path", filePath);

      const response = await fetch(
        `/api/upload/${encodeURIComponent(filePath)}`,
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
          body: formData,
        }
      );

      if (!response.ok) throw new Error("Failed to save file");

      originalContent = content;
      hasUnsavedChanges = false;
      success(tr("fileSavedSuccessfully"));
      onSave?.();
    } catch (err) {
      console.error("Failed to save file:", err);
      errorToast(tr("failedToSaveFile"));
    } finally {
      saving = false;
    }
  }

  function scheduleAutoSave() {
    if (autoSaveTimeout) clearTimeout(autoSaveTimeout);
    autoSaveTimeout = setTimeout(() => {
      if (hasUnsavedChanges) {
        saveFile();
      }
    }, 3000); // 3 seconds debounce
  }

  function handleKeyboardShortcuts(e) {
    // Cmd/Ctrl + S to save
    if ((e.metaKey || e.ctrlKey) && e.key === "s") {
      e.preventDefault();
      saveFile();
    }
    // Cmd/Ctrl + F is handled by Monaco
  }

  function closeModal() {
    if (hasUnsavedChanges) {
      const confirmed = confirm(tr("unsavedChangesWarning"));
      if (!confirmed) return;
    }
    isOpen = false;
  }

  onMount(async () => {
    if (!editorContainer) return;

    // Initialize Monaco Editor
    const language = detectLanguage(filePath);
    editor = monaco.editor.create(editorContainer, {
      value: "",
      language: language,
      theme: document.documentElement.classList.contains("dark")
        ? "vs-dark"
        : "vs",
      automaticLayout: true,
      fontSize: 14,
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      wordWrap: "on",
      lineNumbers: "on",
      folding: true,
      renderWhitespace: "selection",
      tabSize: 2,
      insertSpaces: true,
    });

    // Track changes
    editor.onDidChangeModelContent(() => {
      const currentContent = editor.getValue();
      hasUnsavedChanges = currentContent !== originalContent;
      if (hasUnsavedChanges) {
        scheduleAutoSave();
      }
    });

    // Load file content
    await loadFile();

    // Add keyboard shortcuts
    document.addEventListener("keydown", handleKeyboardShortcuts);
  });

  onDestroy(() => {
    if (autoSaveTimeout) clearTimeout(autoSaveTimeout);
    if (editor) editor.dispose();
    document.removeEventListener("keydown", handleKeyboardShortcuts);
  });
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    tabindex="0"
  >
    <div
      class="w-full h-full max-w-7xl max-h-[90vh] bg-white dark:bg-gray-900 rounded-2xl shadow-2xl flex flex-col"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <i
            class="bi bi-file-earmark-code text-2xl text-green-500"
            aria-hidden="true"
          ></i>
          <div>
            <h2 class="text-lg font-bold text-gray-900 dark:text-white">
              {filePath?.split("/").pop() || tr("fileEditor")}
            </h2>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {filePath}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <!-- Save Status -->
          {#if hasUnsavedChanges}
            <span
              class="text-sm text-orange-500 dark:text-orange-400 flex items-center gap-2"
            >
              <i class="bi bi-dot animate-pulse" aria-hidden="true"></i>
              {tr("unsavedChanges")}
            </span>
          {:else if !loading}
            <span
              class="text-sm text-green-500 dark:text-green-400 flex items-center gap-2"
            >
              <i class="bi bi-check-circle" aria-hidden="true"></i>
              {tr("allChangesSaved")}
            </span>
          {/if}

          <!-- Save Button -->
          <button
            onclick={saveFile}
            disabled={saving || !hasUnsavedChanges}
            class="px-4 py-2 bg-green-500 hover:bg-green-600 disabled:bg-gray-300 dark:disabled:bg-gray-700 text-white rounded-lg transition-colors flex items-center gap-2"
          >
            {#if saving}
              <i class="bi bi-arrow-repeat animate-spin" aria-hidden="true"></i>
              {tr("saving")}
            {:else}
              <i class="bi bi-save" aria-hidden="true"></i>
              {tr("save")}
            {/if}
          </button>

          <!-- Close Button -->
          <button
            aria-label={tr("close")}
            onclick={closeModal}
            class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
            ><i class="bi bi-x" aria-hidden="true"></i></button
          >
        </div>
      </div>

      <!-- Editor Container -->
      <div class="flex-1 relative">
        {#if loading}
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div
                class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-500 mb-4"
              ></div>
              <p class="text-gray-600 dark:text-gray-400">
                {tr("loadingFile")}
              </p>
            </div>
          </div>
        {/if}
        <div bind:this={editorContainer} class="w-full h-full"></div>
      </div>

      <!-- Footer -->
      <div
        class="px-6 py-3 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex items-center justify-between text-sm"
      >
        <div class="flex items-center gap-4 text-gray-600 dark:text-gray-400">
          <span>{tr("language")}: {detectLanguage(filePath)}</span>
          <span>•</span>
          <span>{tr("autoSave")}: {tr("enabled")}</span>
        </div>
        <div class="flex items-center gap-2 text-gray-500 dark:text-gray-400">
          <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded"
            >Ctrl+S</kbd
          >
          <span>{tr("toSave")}</span>
          <span class="mx-2">•</span>
          <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded"
            >Ctrl+F</kbd
          >
          <span>{tr("toSearch")}</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  kbd {
    font-family: ui-monospace, monospace;
    font-size: 0.75rem;
  }
</style>
