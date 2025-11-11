<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Modal from "../organisms/Modal.svelte";
  import FileViewer from "../organisms/FileViewer.svelte";

  let modalOpen = $state(false);
  let largeModalOpen = $state(false);
  let expressiveModalOpen = $state(false);
  let heroModalOpen = $state(false);
  let fileViewerOpen = $state(false);

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

  let currentFileIndex = $state(0);
  let selectedFile = $state(sampleFiles[0]);

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

      <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-8 mb-8">
        <!-- Standard Modal Button -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Standard Modal
          </h3>
          <p class="text-slate-400 text-sm mb-6">
            Opens a standard modal dialog.
          </p>
          <Button onclick={() => (modalOpen = true)}>
            <i class="bi bi-box-arrow-up-right mr-2"></i>Open Modal
          </Button>
        </div>

        <!-- Large Modal Button -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Large Modal</h3>
          <p class="text-slate-400 text-sm mb-6">
            Opens a larger modal dialog.
          </p>
          <Button variant="secondary" onclick={() => (largeModalOpen = true)}>
            <i class="bi bi-window-maximize mr-2"></i>Open Large
          </Button>
        </div>

        <!-- Enhanced Modal Button -->
        <div
          class="bg-gradient-to-br from-blue-500/10 to-purple-500/10 border border-blue-500/30 rounded-xl p-8"
        >
          <div
            class="inline-flex items-center gap-2 px-2.5 py-1 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-full text-xs font-bold mb-3"
          >
            <i class="bi bi-stars"></i>
            ENHANCED
          </div>
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Enhanced Modal
          </h3>
          <p class="text-slate-400 text-sm mb-6">
            Bold typography & extra-rounded shape.
          </p>
          <Button
            variant="primary"
            onclick={() => (expressiveModalOpen = true)}
          >
            <i class="bi bi-lightning-charge mr-2"></i>Open Enhanced
          </Button>
        </div>

        <!-- Hero Moment Modal Button -->
        <div
          class="bg-gradient-to-br from-purple-500/10 to-pink-500/10 border border-purple-500/30 rounded-xl p-8"
        >
          <div
            class="inline-flex items-center gap-2 px-2.5 py-1 bg-gradient-to-r from-purple-600 to-pink-600 text-white rounded-full text-xs font-bold mb-3"
          >
            <i class="bi bi-stars"></i>
            HERO MOMENT
          </div>
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Hero Modal</h3>
          <p class="text-slate-400 text-sm mb-6">
            Dramatic entrance animation.
          </p>
          <Button variant="primary" onclick={() => (heroModalOpen = true)}>
            <i class="bi bi-rocket-takeoff mr-2"></i>Open Hero
          </Button>
        </div>
      </div>

      <!-- Modal Size Reference -->
      <div
        class="bg-slate-700/30 border border-slate-600 rounded-lg p-4 text-slate-300 text-sm"
      >
        <i class="bi bi-info-circle mr-2"></i>
        <strong>Modal Sizes:</strong> sm (28rem), md (32rem), lg (36rem), xl
        (42rem), 2xl (56rem)
        <br />
        <strong>Advanced Features:</strong> emphasized (bold title), shapeStyle (extra-rounded,
        squircle), heroMoment (dramatic entrance), motion (bouncy, smooth, gentle,
        energetic)
      </div>
    </section>

    <!-- Standard Modals -->
    <Modal bind:open={modalOpen} title="Standard Modal" size="md">
      <div class="space-y-4">
        <p class="text-slate-300">
          This is a standard modal dialog. It's perfect for simple interactions
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
      {#snippet footer()}
        <Button
          variant="secondary"
          size="sm"
          onclick={() => (modalOpen = false)}>Cancel</Button
        >
        <Button variant="primary" size="sm" onclick={() => (modalOpen = false)}
          >Confirm</Button
        >
      {/snippet}
    </Modal>

    <Modal
      bind:open={largeModalOpen}
      title="Large Modal with Features"
      size="lg"
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
      {#snippet footer()}
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
      {/snippet}
    </Modal>

    <!-- Enhanced Modals -->
    <Modal
      bind:open={expressiveModalOpen}
      title="Enhanced Modal Title"
      size="lg"
      emphasized={true}
      shapeStyle="extra-extra-large"
      motion="bouncy"
    >
      <div class="space-y-4">
        <div
          class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-full shadow-lg"
        >
          <i class="bi bi-stars"></i>
          <span class="text-sm font-bold">ENHANCED FEATURES</span>
        </div>

        <p class="text-slate-300 text-lg">
          This modal showcases <strong>advanced design</strong> with:
        </p>

        <div class="grid md:grid-cols-2 gap-4">
          <div
            class="bg-gradient-to-br from-blue-500/10 to-purple-500/10 border border-blue-500/30 rounded-2xl p-5"
          >
            <div
              class="w-10 h-10 rounded-xl bg-blue-500/20 flex items-center justify-center mb-3"
            >
              <i class="bi bi-type-bold text-blue-400 text-xl"></i>
            </div>
            <h4 class="text-white font-bold mb-2">Bold Typography</h4>
            <p class="text-slate-400 text-sm">
              Emphasized title with font-bold (700)
            </p>
          </div>

          <div
            class="bg-gradient-to-br from-purple-500/10 to-pink-500/10 border border-purple-500/30 rounded-2xl p-5"
          >
            <div
              class="w-10 h-10 rounded-xl bg-purple-500/20 flex items-center justify-center mb-3"
            >
              <i class="bi bi-square text-purple-400 text-xl"></i>
            </div>
            <h4 class="text-white font-bold mb-2">Extra-Extra-Large Shape</h4>
            <p class="text-slate-400 text-sm">
              32px rounded corners for modern look
            </p>
          </div>

          <div
            class="bg-gradient-to-br from-green-500/10 to-teal-500/10 border border-green-500/30 rounded-2xl p-5"
          >
            <div
              class="w-10 h-10 rounded-xl bg-green-500/20 flex items-center justify-center mb-3"
            >
              <i class="bi bi-arrow-down-up text-green-400 text-xl"></i>
            </div>
            <h4 class="text-white font-bold mb-2">Bouncy Spring Physics</h4>
            <p class="text-slate-400 text-sm">
              cubic-bezier(0.34, 1.56, 0.64, 1) animation
            </p>
          </div>

          <div
            class="bg-gradient-to-br from-yellow-500/10 to-orange-500/10 border border-yellow-500/30 rounded-2xl p-5"
          >
            <div
              class="w-10 h-10 rounded-xl bg-yellow-500/20 flex items-center justify-center mb-3"
            >
              <i class="bi bi-palette text-yellow-400 text-xl"></i>
            </div>
            <h4 class="text-white font-bold mb-2">Gradient Accents</h4>
            <p class="text-slate-400 text-sm">
              Header & footer with gradient backgrounds
            </p>
          </div>
        </div>

        <div class="bg-slate-700/30 border border-slate-600 rounded-2xl p-4">
          <p class="text-slate-400 text-sm">
            <i class="bi bi-lightbulb text-yellow-400 mr-2"></i>
            <strong>Research-backed:</strong> Enhanced UI design improves comprehension
            by 4x and reduces task completion time by 32% (based on industry research
            with 18,000+ participants).
          </p>
        </div>
      </div>
      {#snippet footer()}
        <Button
          variant="secondary"
          size="sm"
          onclick={() => (expressiveModalOpen = false)}>Cancel</Button
        >
        <Button
          variant="primary"
          size="sm"
          emphasized
          shapeStyle="extra-rounded"
          onclick={() => (expressiveModalOpen = false)}
        >
          <i class="bi bi-check-circle mr-2"></i>Looks Amazing!
        </Button>
      {/snippet}
    </Modal>

    <Modal
      bind:open={heroModalOpen}
      title="Hero Moment Modal"
      size="xl"
      emphasized={true}
      shapeStyle="extra-extra-extra-large"
      heroMoment={true}
      motion="energetic"
      glass={true}
    >
      <div class="space-y-6">
        <!-- Hero Header -->
        <div class="text-center">
          <div
            class="w-20 h-20 mx-auto rounded-3xl bg-gradient-to-br from-purple-600 to-pink-600 flex items-center justify-center mb-4 shadow-2xl shadow-purple-500/50"
          >
            <i class="bi bi-rocket-takeoff-fill text-4xl text-white"></i>
          </div>
          <h3
            class="text-2xl font-black text-transparent bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text mb-2"
          >
            Welcome to Hero Moment!
          </h3>
          <p class="text-slate-400">
            A dramatic entrance with enhanced animations and glassmorphism
          </p>
        </div>

        <!-- Features Grid -->
        <div class="grid grid-cols-3 gap-4">
          <div
            class="text-center p-4 bg-purple-500/10 rounded-2xl border border-purple-500/30"
          >
            <div class="text-3xl font-black text-purple-400 mb-1">600ms</div>
            <div class="text-xs text-slate-400">Animation Duration</div>
          </div>
          <div
            class="text-center p-4 bg-pink-500/10 rounded-2xl border border-pink-500/30"
          >
            <div class="text-3xl font-black text-pink-400 mb-1">40px</div>
            <div class="text-xs text-slate-400">Corner Radius</div>
          </div>
          <div
            class="text-center p-4 bg-blue-500/10 rounded-2xl border border-blue-500/30"
          >
            <div class="text-3xl font-black text-blue-400 mb-1">70%</div>
            <div class="text-xs text-slate-400">Backdrop Darkness</div>
          </div>
        </div>

        <!-- Hero Content -->
        <div
          class="bg-gradient-to-br from-purple-500/10 via-pink-500/10 to-blue-500/10 border border-purple-500/30 rounded-2xl p-6"
        >
          <p class="text-slate-300 mb-4">
            <strong class="text-white">Hero Moment</strong> creates a memorable first
            impression with:
          </p>
          <ul class="space-y-2 text-slate-400 text-sm">
            <li class="flex items-start gap-2">
              <i class="bi bi-check-circle-fill text-green-400 mt-0.5"></i>
              <span
                ><strong>Energetic motion:</strong> cubic-bezier(0.68, -0.55, 0.265,
                1.55)</span
              >
            </li>
            <li class="flex items-start gap-2">
              <i class="bi bi-check-circle-fill text-green-400 mt-0.5"></i>
              <span
                ><strong>Glassmorphism:</strong> 80% opacity with backdrop-blur-2xl</span
              >
            </li>
            <li class="flex items-start gap-2">
              <i class="bi bi-check-circle-fill text-green-400 mt-0.5"></i>
              <span
                ><strong>Extra-large shadow:</strong> shadow-2xl with hover scale</span
              >
            </li>
            <li class="flex items-start gap-2">
              <i class="bi bi-check-circle-fill text-green-400 mt-0.5"></i>
              <span
                ><strong>Longer animation:</strong> 600ms for dramatic effect</span
              >
            </li>
          </ul>
        </div>

        <!-- Call to Action -->
        <div
          class="text-center bg-slate-700/30 rounded-2xl p-5 border border-slate-600"
        >
          <p class="text-slate-300 text-sm">
            Perfect for important announcements, onboarding flows, or
            celebrating achievements! 🎉
          </p>
        </div>
      </div>
      {#snippet footer()}
        <Button
          variant="secondary"
          size="md"
          onclick={() => (heroModalOpen = false)}>Maybe Later</Button
        >
        <Button
          variant="primary"
          size="md"
          emphasized
          shapeStyle="extra-rounded"
          motion="bouncy"
          onclick={() => (heroModalOpen = false)}
        >
          <i class="bi bi-stars mr-2"></i>Let's Go!
        </Button>
      {/snippet}
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
