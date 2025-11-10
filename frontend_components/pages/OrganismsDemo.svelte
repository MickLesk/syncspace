<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Modal from "../organisms/Modal.svelte";
  import FileViewer from "../organisms/FileViewer.svelte";

  let modalOpen = false;
  let largeModalOpen = false;
  let fileViewerOpen = false;

  const sampleFiles = [
    {
      type: "text",
      name: "example.txt",
      size: "2.4 KB",
      content:
        "Hello, World!\n\nThis is a sample text file demonstrating the FileViewer component.",
    },
    {
      type: "code",
      name: "index.js",
      size: "1.8 KB",
      content: `function helloWorld() {
  console.log('Hello, World!');
  return true;
}

const result = helloWorld();`,
    },
    {
      type: "image",
      name: "sample.png",
      size: "45 KB",
      url: "https://via.placeholder.com/400x300?text=Sample+Image",
    },
  ];

  let currentFileIndex = 0;
  let selectedFile = sampleFiles[0];

  function openFile(index: number) {
    currentFileIndex = index;
    selectedFile = sampleFiles[index];
    fileViewerOpen = true;
  }

  function nextFile() {
    currentFileIndex = (currentFileIndex + 1) % sampleFiles.length;
    selectedFile = sampleFiles[currentFileIndex];
  }

  function previousFile() {
    currentFileIndex =
      (currentFileIndex - 1 + sampleFiles.length) % sampleFiles.length;
    selectedFile = sampleFiles[currentFileIndex];
  }
</script>

<div class="min-h-screen bg-slate-900 py-12">
  <div class="max-w-7xl mx-auto px-6">
    <!-- Header -->
    <div class="mb-12">
      <h1 class="text-4xl font-bold text-white mb-2">
        Organisms Component Demo
      </h1>
      <p class="text-slate-400">Complex, feature-rich components</p>
    </div>

    <!-- Modal Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-window-fullscreen mr-3 text-blue-400"></i>Modal
      </h2>

      <div class="grid md:grid-cols-2 gap-8 mb-8">
        <!-- Small Modal Button -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Small Modal</h3>
          <p class="text-slate-400 text-sm mb-6">
            Opens a compact modal dialog (28rem width).
          </p>
          <Button onclick={() => (modalOpen = true)}>
            <i class="bi bi-box-arrow-up-right mr-2"></i>Open Modal
          </Button>
        </div>

        <!-- Large Modal Button -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Large Modal</h3>
          <p class="text-slate-400 text-sm mb-6">
            Opens a larger modal dialog (36rem width).
          </p>
          <Button variant="secondary" onclick={() => (largeModalOpen = true)}>
            <i class="bi bi-window-maximize mr-2"></i>Open Large Modal
          </Button>
        </div>
      </div>

      <!-- Modal Size Reference -->
      <div
        class="bg-slate-700/30 border border-slate-600 rounded-lg p-4 text-slate-300 text-sm"
      >
        <i class="bi bi-info-circle mr-2"></i>
        <strong>Modal Sizes:</strong> sm (28rem), md (32rem), lg (36rem), xl (42rem)
      </div>
    </section>

    <!-- Modals -->
    <Modal
      open={modalOpen}
      title="Small Modal"
      size="sm"
      on:close={() => (modalOpen = false)}
    >
      <div class="space-y-4">
        <p class="text-slate-300">
          This is a small modal dialog. It's perfect for simple interactions
          like confirmations or form inputs.
        </p>
        <div class="bg-slate-700/30 border border-slate-600 rounded-lg p-4">
          <p class="text-slate-400 text-sm">
            <i class="bi bi-lightbulb mr-2"></i>
            <strong>Tip:</strong> Modals overlay content and prevent interaction
            with the rest of the page.
          </p>
        </div>
      </div>
      <div slot="footer" class="flex gap-3 justify-end">
        <Button
          variant="secondary"
          size="sm"
          onclick={() => (modalOpen = false)}>Cancel</Button
        >
        <Button variant="primary" size="sm" onclick={() => (modalOpen = false)}
          >Confirm</Button
        >
      </div>
    </Modal>

    <Modal
      open={largeModalOpen}
      title="Large Modal with Features"
      size="lg"
      on:close={() => (largeModalOpen = false)}
    >
      <div class="space-y-4">
        <p class="text-slate-300">
          This is a larger modal with more content. Great for forms, settings,
          or detailed information displays.
        </p>

        <div class="grid md:grid-cols-2 gap-4">
          <div class="bg-slate-700/30 border border-slate-600 rounded-lg p-4">
            <h4 class="text-white font-semibold mb-2 flex items-center">
              <i class="bi bi-check-circle text-green-400 mr-2"></i>Feature 1
            </h4>
            <p class="text-slate-400 text-sm">
              Fully customizable and responsive
            </p>
          </div>
          <div class="bg-slate-700/30 border border-slate-600 rounded-lg p-4">
            <h4 class="text-white font-semibold mb-2 flex items-center">
              <i class="bi bi-star text-yellow-400 mr-2"></i>Feature 2
            </h4>
            <p class="text-slate-400 text-sm">
              Smooth animations and transitions
            </p>
          </div>
        </div>

        <div
          class="bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/30 rounded-lg p-4"
        >
          <p class="text-slate-300 text-sm">
            <i class="bi bi-info-circle text-blue-400 mr-2"></i>
            This modal demonstrates the larger size and layout flexibility.
          </p>
        </div>
      </div>
      <div slot="footer" class="flex gap-3 justify-end">
        <Button
          variant="secondary"
          size="sm"
          onclick={() => (largeModalOpen = false)}>Discard</Button
        >
        <Button
          variant="primary"
          size="sm"
          onclick={() => (largeModalOpen = false)}>Save Changes</Button
        >
      </div>
    </Modal>

    <Divider variant="horizontal" color="slate" />

    <!-- FileViewer Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-file-earmark-text mr-3 text-purple-400"></i>File Viewer
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-6">
          Supported File Types
        </h3>

        <div class="grid md:grid-cols-3 gap-4">
          {#each sampleFiles as file, index (index)}
            <button
              onclick={() => openFile(index)}
              class="bg-slate-700/50 hover:bg-slate-700 border border-slate-600 rounded-lg p-4 text-left transition-colors group"
            >
              <div class="flex items-start justify-between mb-2">
                <div class="flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <i class="bi bi-file-earmark text-blue-400"></i>
                    <p class="text-white font-medium truncate">{file.name}</p>
                  </div>
                  <p class="text-slate-400 text-sm">{file.size}</p>
                </div>
                <i
                  class="bi bi-arrow-right text-slate-500 group-hover:text-blue-400 transition-colors"
                ></i>
              </div>
              <p class="text-slate-500 text-xs">
                {file.type.charAt(0).toUpperCase() + file.type.slice(1)}
              </p>
            </button>
          {/each}
        </div>
      </div>

      <!-- File Type Support Info -->
      <div
        class="bg-slate-700/30 border border-slate-600 rounded-lg p-4 text-slate-300 text-sm space-y-2"
      >
        <p class="font-semibold text-white">Supported Types:</p>
        <ul class="list-disc list-inside space-y-1 text-xs">
          <li>Text files (.txt, .md, .json, .csv)</li>
          <li>Code files (JavaScript, Python, HTML, CSS, XML)</li>
          <li>Images (JPG, PNG, GIF, WebP, SVG, BMP)</li>
          <li>Videos (MP4, WebM, OGG)</li>
          <li>Audio files (MP3, WAV, OGG)</li>
          <li>PDF documents</li>
          <li>Generic file fallback</li>
        </ul>
      </div>
    </section>

    <!-- File Viewer Modal -->
    <Modal
      open={fileViewerOpen}
      title={selectedFile?.name || "File Viewer"}
      size="lg"
      on:close={() => (fileViewerOpen = false)}
    >
      {#if selectedFile}
        <FileViewer file={selectedFile} />
      {/if}

      <div slot="footer" class="flex items-center justify-between">
        <div class="flex gap-2">
          <Button
            variant="secondary"
            size="sm"
            disabled={currentFileIndex === 0}
            onclick={previousFile}
          >
            <i class="bi bi-chevron-left mr-1"></i>Previous
          </Button>
          <Button
            variant="secondary"
            size="sm"
            disabled={currentFileIndex === sampleFiles.length - 1}
            onclick={nextFile}
          >
            Next<i class="bi bi-chevron-right ml-1"></i>
          </Button>
        </div>
        <Button
          variant="primary"
          size="sm"
          onclick={() => (fileViewerOpen = false)}>Close</Button
        >
      </div>
    </Modal>

    <Divider variant="horizontal" color="slate" />

    <!-- Info Box -->
    <div
      class="bg-blue-500/10 border border-blue-500/30 rounded-xl p-6 text-slate-300"
    >
      <i class="bi bi-info-circle text-blue-400 mr-2"></i>
      <strong>Tip:</strong> Organisms are complete, self-contained features that
      combine multiple atoms and molecules. They typically handle complex interactions
      and state management.
    </div>
  </div>
</div>
