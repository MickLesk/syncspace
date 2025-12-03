<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import ModernCard from "./ModernCard.svelte";
  import ModernButton from "./ModernButton.svelte";

  const dispatch = createEventDispatcher();
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { open = $bindable(false) } = $props();

  const faqs = [
    {
      question: "How do I upload files?",
      answer:
        "You can upload files by dragging and dropping them into the file view, or by clicking the upload button in the toolbar. Multiple files can be uploaded simultaneously.",
    },
    {
      question: "How do I enable Two-Factor Authentication?",
      answer:
        "Go to Settings → Security tab, click 'Enable 2FA', scan the QR code with your authenticator app, and enter the verification code to complete setup.",
    },
    {
      question: "Can I change my default theme?",
      answer:
        "Yes! Go to Settings → General tab and choose between Light, Dark, or Auto theme. Auto follows your system preference.",
    },
    {
      question: "How do I search for files?",
      answer:
        "Use the search bar in the top navigation. The search uses Tantivy full-text search with fuzzy matching, so it can find files even with typos.",
    },
    {
      question: "What file formats can I preview?",
      answer:
        "You can preview images (JPG, PNG, GIF, WebP, SVG), videos (MP4, WebM, OGG), PDFs, text files (TXT, MD, JSON, JS, CSS, HTML, XML, CSV), DOCX documents, and Excel files (XLSX, XLS).",
    },
    {
      question: "How do I organize files?",
      answer:
        "Use folders to organize files. Right-click for context menus, drag files to move them, and use multi-select mode for bulk operations.",
    },
    {
      question: "Is my data secure?",
      answer:
        "Yes! All data is stored locally on your server. Passwords are hashed with Argon2, JWT tokens are used for authentication, and you can enable 2FA for extra security.",
    },
    {
      question: "How do I change my password?",
      answer:
        "Go to Settings → Security tab and click 'Change Password'. You'll need your current password to set a new one.",
    },
  ];

  function closeDialog() {
    open = false;
    dispatch("close");
  }

  function handleBackdropClick(event) {
    if (event.target === event.currentTarget) {
      closeDialog();
    }
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in"
    onclick={handleBackdropClick}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Escape" && closeDialog()}
  >
    <!-- Dialog -->
    <div
      class="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden animate-scale-in"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.key === "Escape" && closeDialog()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="help-dialog-title"
      tabindex="-1"
    >
      <!-- Header -->
      <div
        class="bg-gradient-to-r from-primary-500 to-secondary-500 p-6 text-white"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <i class="bi bi-question-circle-fill text-3xl" aria-hidden="true"
            ></i>
            <div>
              <h2 id="help-dialog-title" class="text-2xl font-bold">
                Help & Support
              </h2>
              <p class="text-white/80 text-sm">Frequently Asked Questions</p>
            </div>
          </div>
          <button
            aria-label="Close dialog"
            onclick={closeDialog}
            class="p-2 hover:bg-white/20 rounded-lg transition-colors"
            ><i class="bi bi-x" aria-hidden="true"></i></button
          >
        </div>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-200px)]">
        <div class="space-y-4">
          {#each faqs as faq, index}
            <details class="group">
              <summary
                class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-xl cursor-pointer transition-colors"
              >
                <i
                  class="bi bi-chevron-right group-open:rotate-90 transition-transform text-primary-600 dark:text-primary-400"
                ></i>
                <span
                  class="font-semibold text-gray-900 dark:text-gray-100 flex-1"
                >
                  {faq.question}
                </span>
              </summary>
              <div class="p-4 pl-11 text-gray-700 dark:text-gray-300">
                {faq.answer}
              </div>
            </details>
          {/each}
        </div>

        <!-- Additional Resources -->
        <div
          class="mt-8 p-6 bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-xl border border-blue-200 dark:border-blue-800"
        >
          <h3
            class="text-lg font-bold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2"
          >
            <i
              class="bi bi-book text-blue-600 dark:text-blue-400"
              aria-hidden="true"
            ></i>
            Need More Help?
          </h3>
          <p class="text-gray-700 dark:text-gray-300 mb-4">
            Check out our comprehensive documentation for detailed guides and
            tutorials.
          </p>
          <div class="flex flex-wrap gap-3">
            <a
              href="#/docs"
              class="inline-flex items-center gap-2 px-4 py-2 bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors font-medium"
            >
              <i class="bi bi-file-earmark-text" aria-hidden="true"></i>
              Documentation
            </a>
            <a
              href="https://github.com/MickLesk/syncspace/issues"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center gap-2 px-4 py-2 bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors font-medium"
            >
              <i class="bi bi-github" aria-hidden="true"></i>
              GitHub Issues
            </a>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800"
      >
        <div class="flex justify-end gap-3">
          <ModernButton variant="secondary" onclick={closeDialog}>
            Close
          </ModernButton>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  details summary {
    list-style: none;
  }

  details summary::-webkit-details-marker {
    display: none;
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes scale-in {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }

  .animate-fade-in {
    animation: fade-in 0.2s ease-out;
  }

  .animate-scale-in {
    animation: scale-in 0.3s ease-out;
  }
</style>
