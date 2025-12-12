<script>
  import PageWrapper from "../../components/PageWrapper.svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let accounts = $state([]);
  let showAddModal = $state(false);
  let editingAccount = $state(null);
  let testingId = $state(null);
  let fetchingId = $state(null);
  let viewingMessages = $state(null);
  let messages = $state([]);
  let loadingMessages = $state(false);

  // Form state
  let formData = $state({
    name: "",
    email_address: "",
    protocol: "imap",
    server: "",
    port: 993,
    username: "",
    password: "",
    use_tls: true,
    auto_fetch: false,
    fetch_interval_minutes: 30,
    store_attachments: true,
    attachment_folder: "/email_attachments",
  });

  const protocols = [
    { value: "imap", label: "IMAP", defaultPort: 993 },
    { value: "pop3", label: "POP3", defaultPort: 995 },
  ];

  $effect(() => {
    loadAccounts();
  });

  async function loadAccounts() {
    loading = true;
    try {
      accounts = await api.email.list();
    } catch (e) {
      console.error("Failed to load email accounts:", e);
      errorToast("Failed to load email accounts");
    } finally {
      loading = false;
    }
  }

  function openAddModal() {
    editingAccount = null;
    formData = {
      name: "",
      email_address: "",
      protocol: "imap",
      server: "",
      port: 993,
      username: "",
      password: "",
      use_tls: true,
      auto_fetch: false,
      fetch_interval_minutes: 30,
      store_attachments: true,
      attachment_folder: "/email_attachments",
    };
    showAddModal = true;
  }

  function openEditModal(account) {
    editingAccount = account;
    formData = {
      name: account.name,
      email_address: account.email_address,
      protocol: account.protocol,
      server: account.server,
      port: account.port,
      username: account.username,
      password: "",
      use_tls: account.use_tls,
      auto_fetch: account.auto_fetch,
      fetch_interval_minutes: account.fetch_interval_minutes,
      store_attachments: account.store_attachments,
      attachment_folder: account.attachment_folder,
    };
    showAddModal = true;
  }

  function closeModal() {
    showAddModal = false;
    editingAccount = null;
  }

  function onProtocolChange() {
    const proto = protocols.find((p) => p.value === formData.protocol);
    if (proto) {
      formData.port = proto.defaultPort;
    }
  }

  async function saveAccount() {
    try {
      if (editingAccount) {
        await api.email.update(editingAccount.id, formData);
        success("Email account updated");
      } else {
        await api.email.create(formData);
        success("Email account created");
      }
      closeModal();
      loadAccounts();
    } catch (e) {
      console.error("Failed to save email account:", e);
      errorToast("Failed to save account");
    }
  }

  async function deleteAccount(id) {
    if (!confirm("Are you sure you want to delete this email account?")) return;
    try {
      await api.email.delete(id);
      success("Account deleted");
      loadAccounts();
    } catch (e) {
      console.error("Failed to delete account:", e);
      errorToast("Failed to delete account");
    }
  }

  async function testConnection(id) {
    testingId = id;
    try {
      const result = await api.email.test(id);
      if (result.success) {
        success("Connection successful!");
      } else {
        errorToast(result.message || "Connection failed");
      }
    } catch (e) {
      console.error("Connection test failed:", e);
      errorToast("Connection test failed");
    } finally {
      testingId = null;
    }
  }

  async function fetchEmails(id) {
    fetchingId = id;
    try {
      const result = await api.email.fetch(id);
      success(`Fetched ${result.messages_fetched || 0} new messages`);
      loadAccounts();
    } catch (e) {
      console.error("Fetch failed:", e);
      errorToast("Failed to fetch emails");
    } finally {
      fetchingId = null;
    }
  }

  async function viewMessages(account) {
    viewingMessages = account;
    loadingMessages = true;
    try {
      messages = await api.email.getMessages(account.id);
    } catch (e) {
      console.error("Failed to load messages:", e);
      errorToast("Failed to load messages");
      messages = [];
    } finally {
      loadingMessages = false;
    }
  }

  function closeMessagesModal() {
    viewingMessages = null;
    messages = [];
  }

  function formatDate(dateStr) {
    if (!dateStr) return "Never";
    return new Date(dateStr).toLocaleString();
  }

  function getProtocolBadge(protocol) {
    return protocol.toUpperCase();
  }
</script>

<PageWrapper>
  <PageHeader
    title="Email Integration"
    subtitle="Connect email accounts to automatically save attachments"
    icon="bi-envelope"
  >
    <ModernButton variant="primary" onclick={openAddModal}>
      <i class="bi bi-plus-lg me-2"></i>
      Add Account
    </ModernButton>
  </PageHeader>

  {#if loading}
    <LoadingState message="Loading email accounts..." />
  {:else if accounts.length === 0}
    <EmptyState
      icon="bi-envelope-slash"
      title="No Email Accounts"
      description="Add an email account to automatically fetch and save attachments."
    >
      <ModernButton variant="primary" onclick={openAddModal}>
        <i class="bi bi-plus-lg me-2"></i>
        Add Account
      </ModernButton>
    </EmptyState>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      {#each accounts as account}
        <ModernCard>
          <div class="p-4">
            <div class="flex items-start justify-between mb-3">
              <div class="flex items-center gap-3">
                <div class="p-2 rounded-lg bg-secondary/10">
                  <i class="bi bi-envelope text-xl text-secondary"></i>
                </div>
                <div>
                  <h3 class="font-semibold text-base-content">
                    {account.name}
                  </h3>
                  <p class="text-sm text-base-content/60">
                    {account.email_address}
                  </p>
                </div>
              </div>
              <div class="dropdown dropdown-end">
                <button
                  tabindex="0"
                  class="btn btn-ghost btn-sm btn-square"
                  aria-label="More options"
                >
                  <i class="bi bi-three-dots-vertical"></i>
                </button>
                <ul
                  class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-40"
                >
                  <li>
                    <button onclick={() => openEditModal(account)}>Edit</button>
                  </li>
                  <li>
                    <button onclick={() => viewMessages(account)}
                      >View Messages</button
                    >
                  </li>
                  <li>
                    <button
                      onclick={() => deleteAccount(account.id)}
                      class="text-error">Delete</button
                    >
                  </li>
                </ul>
              </div>
            </div>

            <div class="space-y-2 text-sm mb-4">
              <div class="flex justify-between">
                <span class="text-base-content/60">Protocol:</span>
                <span class="badge badge-sm badge-outline"
                  >{getProtocolBadge(account.protocol)}</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Server:</span>
                <span class="font-mono text-xs"
                  >{account.server}:{account.port}</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">TLS:</span>
                {#if account.use_tls}
                  <i class="bi bi-check-circle-fill text-success"></i>
                {:else}
                  <i class="bi bi-x-circle text-base-content/40"></i>
                {/if}
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Auto Fetch:</span>
                <span
                  class="badge badge-sm"
                  class:badge-success={account.auto_fetch}
                  class:badge-ghost={!account.auto_fetch}
                >
                  {account.auto_fetch
                    ? `Every ${account.fetch_interval_minutes}m`
                    : "Manual"}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Last Fetch:</span>
                <span class="text-xs">{formatDate(account.last_fetch_at)}</span>
              </div>
              {#if account.last_fetch_status}
                <div class="flex justify-between">
                  <span class="text-base-content/60">Status:</span>
                  <span
                    class="badge badge-sm"
                    class:badge-success={account.last_fetch_status ===
                      "success"}
                    class:badge-error={account.last_fetch_status === "failed"}
                  >
                    {account.last_fetch_status}
                  </span>
                </div>
              {/if}
            </div>

            <div class="flex gap-2">
              <ModernButton
                variant="outline"
                size="sm"
                onclick={() => testConnection(account.id)}
                disabled={testingId === account.id}
              >
                {#if testingId === account.id}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-wifi me-1"></i>
                {/if}
                Test
              </ModernButton>
              <ModernButton
                variant="primary"
                size="sm"
                onclick={() => fetchEmails(account.id)}
                disabled={fetchingId === account.id}
              >
                {#if fetchingId === account.id}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-cloud-download me-1"></i>
                {/if}
                Fetch
              </ModernButton>
            </div>
          </div>
        </ModernCard>
      {/each}
    </div>
  {/if}
</PageWrapper>

<!-- Add/Edit Modal -->
{#if showAddModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        {editingAccount ? "Edit Email Account" : "Add Email Account"}
      </h3>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          saveAccount();
        }}
      >
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <UIInput
  label="Account Name"
  bind:value={formData.name}
  placeholder="Work Email"
  required
/>
          </div>

          <div class="form-control">
            <UIInput
  label="Email Address"
  type="email"
  bind:value={formData.email_address}
  placeholder="user@example.com"
  required
/>
          </div>

          <div class="form-control">
            <UISelect
  label="Protocol"
  bind:value={formData.protocol}
/>
          </div>

          <div class="form-control">
            <UIInput
  label="Server"
  bind:value={formData.server}
  placeholder="imap.gmail.com"
  required
/>
          </div>

          <div class="form-control">
            <UIInput
  label="Port"
  type="number"
  bind:value={formData.port}
/>
          </div>

          <div class="form-control">
            <UIInput
  label="Username"
  bind:value={formData.username}
  required
/>
          </div>

          <div class="form-control md:col-span-2">
            <label class="label" for="email-password">
              <span class="label-text"
                >Password {editingAccount
                  ? "(leave empty to keep current)"
                  : ""}</span
              >
            </label>
            <input
              type="password"
              id="email-password"
              class="input input-bordered"
              bind:value={formData.password}
              required={!editingAccount}
            />
          </div>

          <div class="form-control">
            <UIInput
  label="Attachment Folder"
  bind:value={formData.attachment_folder}
  placeholder="/email_attachments"
/>
          </div>

          <div class="form-control">
            <UIInput
  label="Fetch Interval (minutes)"
  type="number"
  bind:value={formData.fetch_interval_minutes}
  disabled
/>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={formData.use_tls}
              />
              <span class="label-text">Use TLS/SSL</span>
            </label>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={formData.store_attachments}
              />
              <span class="label-text">Store Attachments</span>
            </label>
          </div>

          <div class="form-control md:col-span-2">
            <label class="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                bind:checked={formData.auto_fetch}
              />
              <span class="label-text">Enable Auto Fetch</span>
            </label>
          </div>
        </div>

        <div class="modal-action">
          <button type="button" class="btn" onclick={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">
            {editingAccount ? "Update" : "Create"}
          </button>
        </div>
      </form>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeModal}
      onkeydown={(e) => e.key === "Escape" && closeModal()}
    ></div>
  </div>
{/if}

<!-- Messages Modal -->
{#if viewingMessages}
  <div class="modal modal-open">
    <div class="modal-box max-w-4xl max-h-[80vh]">
      <div class="flex justify-between items-center mb-4">
        <h3 class="font-bold text-lg">
          Messages - {viewingMessages.name}
        </h3>
        <button
          class="btn btn-ghost btn-sm btn-circle"
          onclick={closeMessagesModal}
          aria-label="Close"
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>

      {#if loadingMessages}
        <div class="flex justify-center py-8">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if messages.length === 0}
        <div class="text-center py-8 text-base-content/60">
          <i class="bi bi-inbox text-4xl mb-2"></i>
          <p>No messages found</p>
        </div>
      {:else}
        <div class="overflow-auto max-h-[60vh]">
          <table class="table table-zebra">
            <thead>
              <tr>
                <th>Subject</th>
                <th>From</th>
                <th>Date</th>
                <th>Attachments</th>
              </tr>
            </thead>
            <tbody>
              {#each messages as msg}
                <tr>
                  <td class="max-w-xs truncate"
                    >{msg.subject || "(No Subject)"}</td
                  >
                  <td class="text-sm">{msg.from_address}</td>
                  <td class="text-sm">{formatDate(msg.date)}</td>
                  <td>
                    {#if msg.has_attachments}
                      <i class="bi bi-paperclip text-primary"></i>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      <div class="modal-action">
        <button class="btn" onclick={closeMessagesModal}>Close</button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeMessagesModal}
      onkeydown={(e) => e.key === "Escape" && closeMessagesModal()}
    ></div>
  </div>
{/if}
