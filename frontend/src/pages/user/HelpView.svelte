<script>
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let searchQuery = $state("");

  const faqItems = $derived([
    {
      question: tr("faqUploadQuestion"),
      answer: tr("faqUploadAnswer"),
      category: tr("files"),
    },
    {
      question: tr("faqShareQuestion"),
      answer: tr("faqShareAnswer"),
      category: tr("sharing"),
    },
    {
      question: tr("faqPasswordQuestion"),
      answer: tr("faqPasswordAnswer"),
      category: tr("account"),
    },
    {
      question: tr("faqFileTypesQuestion"),
      answer: tr("faqFileTypesAnswer"),
      category: tr("files"),
    },
    {
      question: tr("faqRecoverQuestion"),
      answer: tr("faqRecoverAnswer"),
      category: tr("files"),
    },
  ]);

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
       aria-hidden="true"></i>
      {tr("helpAndSupport")}
    </h1>
    <p class="text-gray-600 dark:text-gray-400 mt-2">
      {tr("findAnswersToCommonQuestions")}
    </p>
  </div>

  <div class="space-y-6">
    <!-- Search -->
    <div class="form-control">
      <div class="relative">
        <input
          type="text"
          placeholder={tr("searchHelpArticles")}
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
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-rocket text-4xl text-primary-600" aria-hidden="true"></i>
          <h3 class="card-title text-lg">{tr("gettingStarted")}</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {tr("quickStartGuide")}
          </p>
        </div>
      </a>

      <a
        href="#documentation"
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-book text-4xl text-secondary-600" aria-hidden="true"></i>
          <h3 class="card-title text-lg">{tr("documentation")}</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {tr("fullDocumentation")}
          </p>
        </div>
      </a>

      <a
        href="#contact"
        class="card bg-white dark:bg-gray-800 shadow-lg hover:shadow-xl transition-shadow rounded-xl"
      >
        <div class="card-body items-center text-center">
          <i class="bi bi-envelope text-4xl text-accent-600" aria-hidden="true"></i>
          <h3 class="card-title text-lg">{tr("contactUs")}</h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {tr("getInTouch")}
          </p>
        </div>
      </a>
    </div>

    <!-- FAQ -->
    <div class="card bg-white dark:bg-gray-800 shadow-xl rounded-xl">
      <div class="card-body">
        <h2 class="card-title text-xl mb-6">
          <i class="bi bi-chat-question text-primary-600" aria-hidden="true"></i>
          {tr("frequentlyAskedQuestions")}
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
            <i class="bi bi-search text-4xl text-gray-400 mb-4" aria-hidden="true"></i>
            <p class="text-gray-600 dark:text-gray-400">
              {tr("noResultsFoundFor", searchQuery)}
            </p>
          </div>
        {/if}
      </div>
    </div>

    <!-- Contact Support -->
    <div
      class="card bg-gradient-to-br from-primary-600 to-primary-400 text-white shadow-xl rounded-xl"
    >
      <div class="card-body">
        <h2 class="card-title text-xl">
          <i class="bi bi-headset" aria-hidden="true"></i>
          {tr("needMoreHelp")}
        </h2>
        <p class="opacity-90">
          {tr("cantFindWhatYoureLookingFor")}
        </p>
        <div class="card-actions justify-end mt-4">
          <button class="btn btn-white rounded-xl">
            <i class="bi bi-envelope" aria-hidden="true"></i>
            {tr("contactSupport")}
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
          {tr("viewOnGitHub")}
        </a>
      </p>
    </div>
  </div>
</div>
