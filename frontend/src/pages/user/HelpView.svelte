<script>
  let searchQuery = $state("");

  const faqItems = [
    {
      question: "How do I upload files?",
      answer:
        'Click the "Upload" button in the toolbar, or drag and drop files directly into the file list.',
      category: "Files",
    },
    {
      question: "How do I share files with others?",
      answer:
        'Right-click on a file and select "Share" to create a shareable link.',
      category: "Sharing",
    },
    {
      question: "How do I change my password?",
      answer: 'Go to Settings > Security and use the "Change Password" option.',
      category: "Account",
    },
    {
      question: "What file types are supported?",
      answer:
        "SyncSpace supports all file types. Preview is available for images, PDFs, and text files.",
      category: "Files",
    },
    {
      question: "How do I recover deleted files?",
      answer:
        "Check the Trash section. Files are kept for 30 days before permanent deletion.",
      category: "Files",
    },
  ];

  let filteredFAQ = $derived(
    searchQuery.trim() === ""
      ? faqItems
      : faqItems.filter(
          (item) =>
            item.question.toLowerCase().includes(searchQuery.toLowerCase()) ||
            item.answer.toLowerCase().includes(searchQuery.toLowerCase())
        )
  );
</script>

<div class="container mx-auto px-4 py-8 max-w-4xl">
  <div class="mb-8">
    <h1
      class="text-3xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-3"
    >
      <i class="bi bi-question-circle text-primary-600 dark:text-primary-400"
      ></i>
      Help & Support
    </h1>
    <p class="text-gray-600 dark:text-gray-400 mt-2">
      Find answers to common questions and get support
    </p>
  </div>

  <div class="space-y-6">
    <!-- Search -->
    <div class="form-control">
      <div class="relative">
        <input
          type="text"
          placeholder="Search help articles..."
          class="input input-bordered w-full rounded-xl pl-12"
          bind:value={searchQuery}
        />
        <i
          class="bi bi-search absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
        ></i>
      </div>
    </div>

    <!-- Quick Links -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <a
        href="#getting-started"
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-2xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-rocket text-4xl text-primary-600"></i>
          <h3 class="card-title text-lg">Getting Started</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Quick start guide
          </p>
        </div>
      </a>

      <a
        href="#documentation"
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-2xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-book text-4xl text-secondary-600"></i>
          <h3 class="card-title text-lg">Documentation</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Full documentation
          </p>
        </div>
      </a>

      <a
        href="#contact"
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-2xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-envelope text-4xl text-accent-600"></i>
          <h3 class="card-title text-lg">Contact Us</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">Get in touch</p>
        </div>
      </a>
    </div>

    <!-- FAQ -->
    <div class="card bg-white dark:bg-gray-800 shadow-xl rounded-2xl">
      <div class="card-body">
        <h2 class="card-title text-xl mb-6">
          <i class="bi bi-chat-question text-primary-600"></i>
          Frequently Asked Questions
        </h2>

        <div class="space-y-2">
          {#each filteredFAQ as item}
            <div
              class="collapse collapse-arrow bg-base-200 dark:bg-gray-700 rounded-xl"
            >
              <input type="radio" name="faq-accordion" />
              <div class="collapse-title font-medium">
                <span class="badge badge-primary badge-sm mr-2"
                  >{item.category}</span
                >
                {item.question}
              </div>
              <div class="collapse-content">
                <p class="text-gray-600 dark:text-gray-400">{item.answer}</p>
              </div>
            </div>
          {/each}
        </div>

        {#if filteredFAQ.length === 0}
          <div class="text-center py-8">
            <i class="bi bi-search text-4xl text-gray-400 mb-4"></i>
            <p class="text-gray-600 dark:text-gray-400">
              No results found for "{searchQuery}"
            </p>
          </div>
        {/if}
      </div>
    </div>

    <!-- Contact Support -->
    <div
      class="card bg-gradient-to-br from-primary-600 to-primary-400 text-white shadow-xl rounded-2xl"
    >
      <div class="card-body">
        <h2 class="card-title text-xl">
          <i class="bi bi-headset"></i>
          Need More Help?
        </h2>
        <p class="opacity-90">
          Can't find what you're looking for? Our support team is here to help!
        </p>
        <div class="card-actions justify-end mt-4">
          <button class="btn btn-white rounded-xl">
            <i class="bi bi-envelope"></i>
            Contact Support
          </button>
        </div>
      </div>
    </div>

    <!-- Version Info -->
    <div class="text-center text-sm text-gray-600 dark:text-gray-400">
      <p>SyncSpace v0.1.0 • Backend API v0.1.0</p>
      <p class="mt-1">
        <a
          href="https://github.com/MickLesk/syncspace"
          target="_blank"
          class="link link-primary"
        >
          View on GitHub
        </a>
      </p>
    </div>
  </div>
</div>
