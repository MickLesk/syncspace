<!-- UIFileUpload.svelte - File upload component with drag & drop -->
<script>
  let {
    label = "",
    accept = "",
    multiple = false,
    maxSize = 0,
    disabled = false,
    error = "",
    onUpload = (files) => {},
    class: className = "",
  } = $props();

  let dragover = $state(false);
  let fileInput;
  let selectedFiles = $state([]);

  function handleDrop(e) {
    e.preventDefault();
    dragover = false;

    if (disabled) return;

    const files = Array.from(e.dataTransfer.files);
    handleFiles(files);
  }

  function handleFileSelect(e) {
    const files = Array.from(e.target.files);
    handleFiles(files);
  }

  function handleFiles(files) {
    // Filter by accept attribute
    if (accept) {
      const acceptTypes = accept.split(",").map((t) => t.trim());
      files = files.filter((f) =>
        acceptTypes.some((type) => {
          if (type.startsWith(".")) {
            return f.name.endsWith(type);
          }
          return f.type.match(type.replace("*", ".*"));
        })
      );
    }

    // Check max size
    if (maxSize > 0) {
      files = files.filter((f) => f.size <= maxSize);
    }

    // Apply multiple restriction
    if (!multiple && files.length > 0) {
      files = [files[0]];
    }

    selectedFiles = files;
    onUpload(files);
  }

  function formatFileSize(bytes) {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }
</script>

<div class="form-control w-full {className}">
  {#if label}
    <label class="label">
      <span class="label-text text-white font-medium">{label}</span>
    </label>
  {/if}

  <div
    role="button"
    tabindex="0"
    class="border-2 border-dashed rounded-lg p-8 text-center transition-all cursor-pointer
           {dragover
      ? 'border-blue-400 bg-blue-500/10'
      : 'border-white/20 bg-white/5'}
           {disabled
      ? 'opacity-50 cursor-not-allowed'
      : 'hover:border-white/40 hover:bg-white/10'}
           {error ? 'border-red-400' : ''}"
    ondragover={(e) => {
      e.preventDefault();
      if (!disabled) dragover = true;
    }}
    ondragleave={() => (dragover = false)}
    ondrop={handleDrop}
    onclick={() => !disabled && fileInput.click()}
    onkeydown={(e) => e.key === "Enter" && !disabled && fileInput.click()}
  >
    <i class="bi bi-cloud-upload text-4xl text-white/40 mb-3"></i>
    <p class="text-white/80 mb-1">
      {dragover
        ? "Dateien hier ablegen"
        : "Dateien hierher ziehen oder klicken zum Auswählen"}
    </p>
    {#if accept}
      <p class="text-sm text-white/60">Erlaubte Formate: {accept}</p>
    {/if}
    {#if maxSize > 0}
      <p class="text-sm text-white/60">Max. Größe: {formatFileSize(maxSize)}</p>
    {/if}

    <input
      bind:this={fileInput}
      type="file"
      {accept}
      {multiple}
      {disabled}
      onchange={handleFileSelect}
      class="hidden"
    />
  </div>

  {#if selectedFiles.length > 0}
    <div class="mt-3 space-y-2">
      {#each selectedFiles as file}
        <div
          class="flex items-center gap-2 p-2 bg-white/5 rounded text-sm text-white/80"
        >
          <i class="bi bi-file-earmark text-white/60"></i>
          <span class="flex-1 truncate">{file.name}</span>
          <span class="text-white/60">{formatFileSize(file.size)}</span>
        </div>
      {/each}
    </div>
  {/if}

  {#if error}
    <label class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </label>
  {/if}
</div>
