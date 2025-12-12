<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import GeneralSettingsTab from "./general/GeneralSettingsTab.svelte";
  import NotificationsUnified from "./general/NotificationsUnified.svelte";
  import SecurityUnified from "./general/SecurityUnified.svelte";
  import FeaturesTab from "./general/FeaturesTab.svelte";
  import ThemeCustomizationView from "./general/ThemeCustomizationView.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    {
      id: "general",
      icon: "sliders",
      label: "Allgemein",
      component: GeneralSettingsTab,
    },
    {
      id: "theme",
      icon: "palette-fill",
      label: "Design",
      component: ThemeCustomizationView,
    },
    {
      id: "notifications",
      icon: "bell-fill",
      label: "Benachrichtigungen",
      component: NotificationsUnified,
    },
    {
      id: "security",
      icon: "shield-lock-fill",
      label: "Sicherheit",
      component: SecurityUnified,
    },
    {
      id: "features",
      icon: "toggles",
      label: "Features",
      component: FeaturesTab,
    },
  ];

  let activeTab = $state("general");
  let ActiveComponent = $derived(
    tabs.find((tab) => tab.id === activeTab)?.component || GeneralSettingsTab
  );
</script>

<PageWrapper gradient>
  <ModernCard>
    <!-- Header -->
    <div class="flex justify-between items-center mb-8">
      <div>
        <h1 class="text-3xl font-bold text-neutral-content">
          Allgemeine Einstellungen
        </h1>
        <p class="text-base-content/70 mt-2">
          Konfiguriere die grundlegenden Einstellungen deiner Instanz
        </p>
      </div>
    </div>

    <!-- Tab Navigation -->
    <div class="flex flex-wrap gap-2 mb-6 p-1 bg-base-200 rounded-xl">
      {#each tabs as tab}
        <ModernButton
          variant={activeTab === tab.id ? "primary" : "ghost"}
          onclick={() => (activeTab = tab.id)}
        >
          <i class="bi bi-{tab.icon}"></i>
          <span>{tab.label}</span>
        </ModernButton>
      {/each}
    </div>

    <!-- Tab Content -->
    <div class="mt-6">
      <ActiveComponent />
    </div>
  </ModernCard>
</PageWrapper>
