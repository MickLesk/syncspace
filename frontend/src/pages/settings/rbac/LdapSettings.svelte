<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import api from "../../../lib/api.js";
  import { showToast } from "../../../stores/toast.js";
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";
  import StandardButton from "../../../components/ui/StandardButton.svelte";
  import StandardModal from "../../../components/ui/StandardModal.svelte";
  import StandardInput from "../../../components/ui/StandardInput.svelte";
  import StandardSelect from "../../../components/ui/StandardSelect.svelte";
  import StandardToggle from "../../../components/ui/StandardToggle.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let testing = $state(false);
  let syncing = $state(false);
  let error = $state("");
  let success = $state("");

  // LDAP Configurations
  let configs = $state([]);

  // Modern Modal State
  let showConfigModal = $state(false);
  let editingConfig = $state(null);
  let configForm = $state({
    name: "",
    host: "",
    port: 389,
    use_ssl: false,
    bind_dn: "",
    bind_password: "",
    base_dn: "",
    user_filter: "(objectClass=person)",
    username_attribute: "uid",
    email_attribute: "mail",
    display_name_attribute: "cn",
    group_filter: "(objectClass=group)",
    group_membership_attribute: "memberOf",
    enabled: true,
    sync_interval_minutes: 60,
  });

  const syncIntervalOptions = [
    { value: 15, label: "15 Minuten" },
    { value: 30, label: "30 Minuten" },
    { value: 60, label: "1 Stunde" },
    { value: 240, label: "4 Stunden" },
    { value: 1440, label: "24 Stunden" }
  ];

  onMount(async () => {
    await loadConfigs();
  });

  async function loadConfigs() {
    loading = true;
    error = "";
    try {
      configs = (await api.ldap.listConfigs()) || [];
    } catch (e) {
      console.error("Failed to load LDAP configs:", e);
      error = "Failed to load LDAP configurations";
    } finally {
      loading = false;
    }
  }

  function openAddConfig() {
    editingConfig = null;
    configForm = {
      name: "",
      host: "",
      port: 389,
      use_ssl: false,
      bind_dn: "",
      bind_password: "",
      base_dn: "",
      user_filter: "(objectClass=person)",
      username_attribute: "uid",
      email_attribute: "mail",
      display_name_attribute: "cn",
      group_filter: "(objectClass=group)",
      group_membership_attribute: "memberOf",
      enabled: true,
      sync_interval_minutes: 60,
    };
    showConfigModal = true;
  }

  function openEditConfig(config) {
    editingConfig = config;
    configForm = {
      name: config.name || "",
      host: config.host || "",
      port: config.port || 389,
      use_ssl: config.use_ssl || false,
      bind_dn: config.bind_dn || "",
      bind_password: "", // Don't show existing password
      base_dn: config.base_dn || "",
      user_filter: config.user_filter || "(objectClass=person)",
      username_attribute: config.username_attribute || "uid",
      email_attribute: config.email_attribute || "mail",
      display_name_attribute: config.display_name_attribute || "cn",
      group_filter: config.group_filter || "(objectClass=group)",
      group_membership_attribute:
        config.group_membership_attribute || "memberOf",
      enabled: config.enabled ?? true,
      sync_interval_minutes: config.sync_interval_minutes || 60,
    };
    showConfigModal = true;
  }

  async function saveConfig() {
    if (!configForm.name || !configForm.host || !configForm.base_dn) {
      showToast("Name, Host und Base DN sind erforderlich", "error");
      return;
    }

    saving = true;
    try {
      const data = { ...configForm };
      // Don't send empty password on update
      if (editingConfig && !data.bind_password) {
        delete data.bind_password;
      }

      if (editingConfig) {
        await api.ldap.updateConfig(editingConfig.id, data);
        showToast("LDAP-Konfiguration erfolgreich aktualisiert", "success");
      } else {
        await api.ldap.createConfig(data);
        showToast("LDAP-Konfiguration erfolgreich erstellt", "success");
      }
      showConfigModal = false;
      await loadConfigs();
    } catch (e) {
      showToast(e.message || "Fehler beim Speichern der Konfiguration", "error");
    } finally {
      saving = false;
    }
  }

  function openAddConfig() {
    editingConfig = null;
    configForm = {
      name: "",
      host: "",
      port: 389,
      use_ssl: false,
      bind_dn: "",
      bind_password: "",
      base_dn: "",
      user_filter: "(objectClass=person)",
      username_attribute: "uid",
      email_attribute: "mail",
      display_name_attribute: "cn",
      group_filter: "(objectClass=group)",
      group_membership_attribute: "memberOf",
      enabled: true,
      sync_interval_minutes: 60,
    };
    showConfigModal = true;
  }

  async function deleteConfig(config) {
    if (!confirm(`LDAP-Konfiguration "${config.name}" löschen?`)) return;
    try {
      await api.ldap.deleteConfig(config.id);
      success = "Configuration deleted successfully";
      await loadConfigs();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to delete configuration";
    }
  }

  async function testConnection(config) {
    testing = true;
    error = "";
    try {
      const result = await api.ldap.testConnection(config.id);
      if (result.success) {
        success = `Connection successful! Found ${result.users_found || 0} users`;
      } else {
        error = result.error || "Connection test failed";
      }
      setTimeout(() => {
        success = "";
        error = "";
      }, 5000);
    } catch (e) {
      error = e.message || "Connection test failed";
    } finally {
      testing = false;
    }
  }

  async function syncUsers(config) {
    syncing = true;
    error = "";
    try {
      const result = await api.ldap.sync(config.id);
      success = `Sync complete: ${result.synced || 0} users synced, ${result.created || 0} created, ${result.updated || 0} updated`;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Sync failed";
    } finally {
      syncing = false;
    }
  }

  async function syncAllConfigs() {
    syncing = true;
    error = "";
    try {
      const result = await api.ldap.syncAll();
      success = `Sync complete: ${result.total_synced || 0} users synced across all configurations`;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Sync failed";
    } finally {
      syncing = false;
    }
  }

  function closeModal() {
    showConfigModal = false;
    editingConfig = null;
  }

  function handlePortChange() {
    // Auto-adjust port based on SSL toggle
    if (configForm.use_ssl && configForm.port === 389) {
      configForm.port = 636;
    } else if (!configForm.use_ssl && configForm.port === 636) {
      configForm.port = 389;
    }
  }
</script>

<!-- Modern Header -->
<div class="mb-8 flex items-center justify-between">
  <div>
    <h1 class="text-3xl font-bold text-white mb-2">
      <i class="bi bi-diagram-3-fill mr-3"></i>
      LDAP & Active Directory
    </h1>
    <p class="text-white/80">Verwalten Sie LDAP/AD-Verbindungen für Benutzer-Synchronisation</p>
  </div>
  <StandardButton variant="primary" onclick={openAddConfig}>
    <i class="bi bi-plus-lg mr-2"></i>
    Neue Konfiguration
  </StandardButton>
</div>

{#if loading}
  <StandardGlassCard>
    <div class="flex items-center justify-center py-12">
      <div class="loading loading-spinner loading-lg"></div>
      <span class="ml-4 text-white/80">Lade LDAP-Konfigurationen...</span>
    </div>
  </StandardGlassCard>
{:else if configs.length === 0}
  <StandardGlassCard>
    <div class="text-center py-16">
      <div class="w-16 h-16 bg-white/10 rounded-full flex items-center justify-center mx-auto mb-4">
        <i class="bi bi-diagram-3 text-2xl text-white/60"></i>
      </div>
      <h3 class="text-xl font-semibold text-white mb-2">Keine LDAP-Konfigurationen</h3>
      <p class="text-white/60 mb-6">Fügen Sie einen LDAP/AD-Server hinzu, um Benutzer zu synchronisieren</p>
      <StandardButton variant="primary" onclick={openAddConfig}>
        <i class="bi bi-plus-lg mr-2"></i>
        Erste Konfiguration erstellen
      </StandardButton>
    </div>
  </StandardGlassCard>
{:else}
  <!-- Configuration Tips -->
  <StandardGlassCard class="mb-6">
    <div class="p-6">
      <h3 class="text-lg font-semibold text-white mb-4">
        <i class="bi bi-lightbulb mr-2"></i>
        Konfigurationstipps
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-white/70">
        <div>
          <strong class="text-white">Active Directory:</strong> Port 389 (LDAP) oder 636 (LDAPS)
        </div>
        <div>
          <strong class="text-white">Bind DN Format:</strong> cn=admin,dc=example,dc=com
        </div>
        <div>
          <strong class="text-white">Benutzer Filter:</strong> (objectClass=user)
        </div>
        <div>
          <strong class="text-white">Gruppen Filter:</strong> (objectClass=group)
        </div>
      </div>
    </div>
  </StandardGlassCard>

  <!-- LDAP Configurations -->
  <div class="grid grid-cols-1 gap-6">
    {#each configs as config (config.id)}
      <StandardGlassCard>
          <!-- Configuration Header -->
          <div class="flex items-start justify-between mb-4">
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <h3 class="text-xl font-semibold text-white">{config.name}</h3>
                {#if config.enabled}
                  <span class="badge badge-success badge-sm">Aktiv</span>
                {:else}
                  <span class="badge badge-ghost badge-sm">Inaktiv</span>
                {/if}
              </div>
              <p class="text-white/60 text-sm">{config.host}:{config.port}</p>
            </div>
            
            <!-- Action Buttons -->
            <div class="flex gap-2">
              <StandardButton
                variant="ghost"
                size="sm"
                onclick={() => testConnection(config)}
                disabled={testing}
                title="Verbindung testen"
              >
                {#if testing}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-plug"></i>
                {/if}
              </StandardButton>
              <StandardButton
                variant="ghost"
                size="sm"
                onclick={() => syncUsers(config)}
                disabled={syncing}
                title="Benutzer synchronisieren"
              >
                {#if syncing}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-arrow-repeat"></i>
                {/if}
              </StandardButton>
              <StandardButton
                variant="ghost"
                size="sm"
                onclick={() => openEditConfig(config)}
                title="Konfiguration bearbeiten"
              >
                <i class="bi bi-pencil"></i>
              </StandardButton>
              <StandardButton
                variant="outline-error"
                size="sm"
                onclick={() => deleteConfig(config)}
                title="Konfiguration löschen"
              >
                <i class="bi bi-trash"></i>
              </StandardButton>
            </div>
          </div>

          <!-- Configuration Details -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
            <div class="bg-white/5 rounded-lg p-3">
              <div class="text-white/60 text-xs uppercase tracking-wider mb-1">Base DN</div>
              <div class="text-white font-mono text-sm break-all">{config.base_dn}</div>
            </div>
            <div class="bg-white/5 rounded-lg p-3">
              <div class="text-white/60 text-xs uppercase tracking-wider mb-1">SSL/TLS</div>
              <div class="text-white">{config.use_ssl ? "Aktiviert" : "Deaktiviert"}</div>
            </div>
            <div class="bg-white/5 rounded-lg p-3">
              <div class="text-white/60 text-xs uppercase tracking-wider mb-1">Sync-Intervall</div>
              <div class="text-white">{config.sync_interval_minutes} Minuten</div>
            </div>
          </div>

          {#if config.last_sync}
            <div class="text-xs text-white/50 border-t border-white/10 pt-3">
              Zuletzt synchronisiert: {new Date(config.last_sync).toLocaleString("de-DE")}
            </div>
          {/if}
        </div>
      </StandardGlassCard>
    {/each}
  </div>
{/if}

<!-- Modern LDAP Configuration Modal -->
<StandardModal 
  bind:showModal={showConfigModal} 
  title={editingConfig ? "LDAP-Konfiguration bearbeiten" : "Neue LDAP-Konfiguration"}
  size="lg"
>
  <div class="space-y-6">
    <!-- Grundeinstellungen -->
    <div class="space-y-4">
      <h4 class="text-white font-medium flex items-center gap-2 border-b border-white/10 pb-2">
        <i class="bi bi-gear"></i>
        Grundeinstellungen
      </h4>
      
      <StandardInput
        label="Konfigurationsname"
        placeholder="z.B. Unternehmens-AD"
        bind:value={configForm.name}
        required
      />
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <StandardInput
          label="Host"
          placeholder="ldap.example.com"
          bind:value={configForm.host}
          required
        />
        
        <StandardInput
          type="number"
          label="Port"
          bind:value={configForm.port}
        />
      </div>
      
      <StandardToggle
        bind:checked={configForm.use_ssl}
        label="SSL/TLS verwenden (LDAPS)"
        description="Aktiviert verschlüsselte Verbindung über Port 636"
      />
    </div>

    <!-- Authentifizierung -->
    <div class="space-y-4">
      <h4 class="text-white font-medium flex items-center gap-2 border-b border-white/10 pb-2">
        <i class="bi bi-key"></i>
        Authentifizierung
      </h4>
      
      <StandardInput
        label="Bind DN"
        placeholder="cn=admin,dc=example,dc=com"
        bind:value={configForm.bind_dn}
        class="font-mono"
      />
      
      <StandardInput
        type="password"
        label="Bind-Passwort"
        placeholder={editingConfig ? "Leer lassen für bestehendes" : "Passwort eingeben"}
        bind:value={configForm.bind_password}
      />
      
      <StandardInput
        label="Base DN"
        placeholder="dc=example,dc=com"
        bind:value={configForm.base_dn}
        required
        class="font-mono"
      />
    </div>

    <!-- Benutzer-Mapping -->
    <div class="space-y-4">
      <h4 class="text-white font-medium flex items-center gap-2 border-b border-white/10 pb-2">
        <i class="bi bi-person"></i>
        Benutzer-Mapping
      </h4>
      
      <StandardInput
        label="Benutzer-Filter"
        placeholder="(objectClass=person)"
        bind:value={configForm.user_filter}
        class="font-mono"
      />
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <StandardInput
          label="Benutzername-Attribut"
          placeholder="uid"
          bind:value={configForm.username_attribute}
        />
        
        <StandardInput
          label="E-Mail-Attribut"
          placeholder="mail"
          bind:value={configForm.email_attribute}
        />
      </div>
      
      <StandardInput
        label="Anzeigename-Attribut"
        placeholder="cn"
        bind:value={configForm.display_name_attribute}
      />
    </div>

    <!-- Synchronisation -->
    <div class="space-y-4">
      <h4 class="text-white font-medium flex items-center gap-2 border-b border-white/10 pb-2">
        <i class="bi bi-arrow-repeat"></i>
        Synchronisation
      </h4>
      
      <StandardSelect
        label="Sync-Intervall"
        options={syncIntervalOptions}
        bind:value={configForm.sync_interval_minutes}
      />
      
      <StandardToggle
        bind:checked={configForm.enabled}
        label="Konfiguration aktivieren"
        description="Automatische Synchronisation aktivieren"
      />
    </div>
  </div>

  <svelte:fragment slot="actions">
    <StandardButton variant="ghost" onclick={() => showConfigModal = false}>
      Abbrechen
    </StandardButton>
    <StandardButton 
      variant="primary" 
      onclick={saveConfig} 
      disabled={saving}
      class={saving ? "loading" : ""}
    >
      {saving ? "Speichere..." : editingConfig ? "Aktualisieren" : "Erstellen"}
    </StandardButton>
  </svelte:fragment>
</StandardModal>
