<script>
  /**
   * Performance Testing Component for Virtual Scrolling
   * Generates test datasets and measures rendering performance
   */
  import VirtualList from "../ui/VirtualList.svelte";
  import FileCard from "../files/FileCard.svelte";

  let testSize = $state(1000);
  let itemHeight = $state(72);
  let viewMode = $state("list");
  let useVirtual = $state(true);
  let testData = $state([]);
  let renderTime = $state(0);
  let scrollFPS = $state(0);
  let memoryUsage = $state(0);

  // Generate test file data
  function generateTestData(count) {
    const start = performance.now();
    const data = [];
    const types = ['folder', 'pdf', 'image', 'video', 'document', 'text'];
    const extensions = {
      pdf: '.pdf',
      image: '.jpg',
      video: '.mp4',
      document: '.docx',
      text: '.txt',
      folder: ''
    };

    for (let i = 0; i < count; i++) {
      const type = types[Math.floor(Math.random() * types.length)];
      const ext = extensions[type];
      data.push({
        name: `${type === 'folder' ? 'Folder' : 'File'}_${i}${ext}`,
        file_path: `/test/path/${i}${ext}`,
        size: Math.floor(Math.random() * 10000000),
        modified_at: new Date(Date.now() - Math.random() * 31536000000).toISOString(),
        is_dir: type === 'folder',
        mime_type: type === 'folder' ? null : `application/${type}`,
      });
    }
    const end = performance.now();
    renderTime = Math.round(end - start);
    testData = data;
    measureMemory();
  }

  // Measure memory usage (if available)
  function measureMemory() {
    if (performance.memory) {
      memoryUsage = Math.round(performance.memory.usedJSHeapSize / 1024 / 1024);
    }
  }

  // Measure scroll FPS
  let frameCount = 0;
  let lastTime = 0;
  function measureScrollFPS() {
    const now = performance.now();
    frameCount++;
    
    if (now - lastTime >= 1000) {
      scrollFPS = frameCount;
      frameCount = 0;
      lastTime = now;
    }
    
    requestAnimationFrame(measureScrollFPS);
  }

  // Start FPS monitoring
  $effect(() => {
    measureScrollFPS();
  });

  // Preset test sizes
  const presets = [
    { label: '100 items', value: 100 },
    { label: '1K items', value: 1000 },
    { label: '10K items', value: 10000 },
    { label: '100K items', value: 100000 },
  ];
</script>

<div class="performance-test-container p-6 bg-base-200 rounded-lg">
  <h2 class="text-2xl font-bold mb-4">Virtual Scrolling Performance Test</h2>
  
  <!-- Controls -->
  <div class="controls grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
    <div class="form-control">
      <label class="label" for="test-size">
        <span class="label-text">Test Size</span>
      </label>
      <input
        id="test-size"
        type="number"
        class="input input-bordered w-full"
        bind:value={testSize}
        min="10"
        max="1000000"
        step="100"
      />
    </div>

    <div class="form-control">
      <label class="label" for="item-height">
        <span class="label-text">Item Height (px)</span>
      </label>
      <input
        id="item-height"
        type="number"
        class="input input-bordered w-full"
        bind:value={itemHeight}
        min="40"
        max="400"
        step="10"
      />
    </div>

    <div class="form-control">
      <label class="label" for="view-mode">
        <span class="label-text">View Mode</span>
      </label>
      <select
        id="view-mode"
        class="select select-bordered w-full"
        bind:value={viewMode}
      >
        <option value="list">List</option>
        <option value="grid">Grid</option>
      </select>
    </div>

    <div class="form-control">
      <label class="label" for="use-virtual">
        <span class="label-text">Virtualization</span>
      </label>
      <select
        id="use-virtual"
        class="select select-bordered w-full"
        bind:value={useVirtual}
      >
        <option value={true}>Enabled</option>
        <option value={false}>Disabled</option>
      </select>
    </div>
  </div>

  <!-- Preset Buttons -->
  <div class="presets flex flex-wrap gap-2 mb-6">
    {#each presets as preset}
      <button
        class="btn btn-outline btn-sm"
        onclick={() => {
          testSize = preset.value;
          generateTestData(preset.value);
        }}
      >
        {preset.label}
      </button>
    {/each}
    <button
      class="btn btn-primary btn-sm"
      onclick={() => generateTestData(testSize)}
    >
      Generate Test Data
    </button>
  </div>

  <!-- Metrics -->
  <div class="metrics grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
    <div class="stat bg-base-100 rounded-lg shadow">
      <div class="stat-title">Items Rendered</div>
      <div class="stat-value text-2xl">{testData.length.toLocaleString()}</div>
    </div>
    <div class="stat bg-base-100 rounded-lg shadow">
      <div class="stat-title">Render Time</div>
      <div class="stat-value text-2xl">{renderTime}ms</div>
    </div>
    <div class="stat bg-base-100 rounded-lg shadow">
      <div class="stat-title">Scroll FPS</div>
      <div class="stat-value text-2xl">{scrollFPS}</div>
    </div>
    <div class="stat bg-base-100 rounded-lg shadow">
      <div class="stat-title">Memory (MB)</div>
      <div class="stat-value text-2xl">{memoryUsage}</div>
    </div>
  </div>

  <!-- Test Area -->
  <div class="test-area bg-base-100 rounded-lg shadow-lg p-4">
    {#if testData.length > 0}
      {#if useVirtual}
        <div style="height: 600px;">
          <VirtualList
            items={testData}
            {itemHeight}
            isGrid={viewMode === "grid"}
          >
            {#snippet children(file, index)}
              <FileCard {file} {viewMode} />
            {/snippet}
          </VirtualList>
        </div>
      {:else}
        <div
          style="height: 600px; overflow-y: auto;"
          class={viewMode === "grid"
            ? "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
            : "flex flex-col gap-2"}
        >
          {#each testData as file (file.file_path)}
            <FileCard {file} {viewMode} />
          {/each}
        </div>
      {/if}
    {:else}
      <div class="text-center py-12 text-base-content/60">
        <p class="text-lg">Generate test data to start performance testing</p>
      </div>
    {/if}
  </div>

  <!-- Instructions -->
  <div class="instructions mt-6 p-4 bg-info/10 rounded-lg">
    <h3 class="text-lg font-semibold mb-2">How to Test:</h3>
    <ol class="list-decimal list-inside space-y-1">
      <li>Select a preset size or enter a custom number</li>
      <li>Click "Generate Test Data" to create the dataset</li>
      <li>Observe render time and memory usage</li>
      <li>Scroll through the list and watch the FPS counter</li>
      <li>Toggle virtualization on/off to compare performance</li>
      <li>Try different view modes (list vs grid)</li>
    </ol>
    <p class="mt-2 text-sm text-base-content/70">
      <strong>Expected Results:</strong> With virtualization enabled, even 100K items should scroll smoothly at 60 FPS.
      Without virtualization, performance degrades rapidly above 1K items.
    </p>
  </div>
</div>

<style>
  .performance-test-container {
    max-width: 1400px;
    margin: 0 auto;
  }
</style>
