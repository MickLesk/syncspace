<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UITextarea from "../../../components/ui/UITextarea.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import { guests } from "../../../lib/api.js";
  import { showToast } from "../../../stores/toast.js";
  import PageWrapper from "../../../components/PageWrapper.svelte";
  import ModernCard from "../../../components/ui/ModernCard.svelte";
  import ModernButton from "../../../components/ui/ModernButton.svelte";
  import EmptyState from "../../../components/ui/EmptyState.svelte";
  import LoadingState from "../../../components/ui/LoadingState.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // State
  let activeTab = $state("users");
  let loading = $state(true);
  let stats = $state(null);

  // Guest users
  let guestUsers = $state([]);
  let showGuestModal = $state(false);
  let editingGuest = $state(null);
  let guestForm = $state({
    display_name: "",
    email: "",
    expires_in_days: 30,
    max_accesses: null,
    notes: "",
    can_view: true,
    can_download: true,
    can_upload: false,
    can_comment: false,
  });

  // Access links
  let accessLinks = $state([]);
  let showLinkModal = $state(false);
  let editingLink = $state(null);
  let linkForm = $state({
    file_path: "",
    guest_id: null,
    access_type: "file",
    expires_in_days: 7,
    password: "",
    max_accesses: null,
    can_view: true,
    can_download: true,
    can_upload: false,
  });

  // Invitations
  let invitations = $state([]);
  let showInvitationModal = $state(false);
  let invitationForm = $state({
    email: "",
    expires_in_days: 7,
    message: "",
    can_view: true,
    can_download: true,
    can_upload: false,
    can_comment: false,
  });

  // Activity
  let selectedGuestActivity = $state(null);
  let showActivityModal = $state(false);
  let guestActivity = $state([]);

  onMount(() => {
    loadData();
  });

  async function loadData() {
    loading = true;
    try {
      const [statsRes, guestsRes, linksRes, invitationsRes] = await Promise.all(
        [
          guests.getStats().catch(() => null),
          guests.list({ includeExpired: false, includeInactive: false }),
          guests.listLinks({ includeExpired: false, includeInactive: false }),
          guests.listInvitations({
            includeExpired: false,
            includeAccepted: false,
          }),
        ]
      );

      stats = statsRes;
      guestUsers = guestsRes?.guests || [];
      accessLinks = linksRes?.links || [];
      invitations = invitationsRes?.invitations || [];
    } catch (error) {
      console.error("Failed to load guest data:", error);
      showToast(tr("guests.loadError"), "error");
    } finally {
      loading = false;
    }
  }

  // Guest User Functions
  function openGuestModal(guest = null) {
    editingGuest = guest;
    if (guest) {
      guestForm = {
        display_name: guest.display_name,
        email: guest.email || "",
        expires_in_days: 30,
        max_accesses: guest.max_accesses,
        notes: guest.notes || "",
        can_view: guest.can_view,
        can_download: guest.can_download,
        can_upload: guest.can_upload,
        can_comment: guest.can_comment,
      };
    } else {
      guestForm = {
        display_name: "",
        email: "",
        expires_in_days: 30,
        max_accesses: null,
        notes: "",
        can_view: true,
        can_download: true,
        can_upload: false,
        can_comment: false,
      };
    }
    showGuestModal = true;
  }

  async function saveGuest() {
    if (!guestForm.display_name.trim()) {
      showToast(tr("guests.nameRequired"), "error");
      return;
    }

    try {
      if (editingGuest) {
        await guests.update(editingGuest.id, guestForm);
        showToast(tr("guests.guestUpdated"), "success");
      } else {
        await guests.create(guestForm);
        showToast(tr("guests.guestCreated"), "success");
      }
      showGuestModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to save guest:", error);
      showToast(tr("guests.createError"), "error");
    }
  }

  async function deleteGuest(guest) {
    if (!confirm(tr("guests.deleteGuestConfirm"))) return;

    try {
      await guests.delete(guest.id);
      showToast(tr("guests.guestDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete guest:", error);
      showToast(tr("guests.deleteError"), "error");
    }
  }

  async function viewGuestActivity(guest) {
    selectedGuestActivity = guest;
    showActivityModal = true;
    try {
      const res = await guests.getActivity(guest.id, { limit: 50 });
      guestActivity = res?.activity || [];
    } catch (error) {
      console.error("Failed to load activity:", error);
      guestActivity = [];
    }
  }

  // Link Functions
  function openLinkModal(link = null) {
    editingLink = link;
    if (link) {
      linkForm = {
        file_path: link.file_path,
        guest_id: link.guest_id,
        access_type: link.access_type,
        expires_in_days: 7,
        password: "",
        max_accesses: link.max_accesses,
        can_view: link.can_view,
        can_download: link.can_download,
        can_upload: link.can_upload,
      };
    } else {
      linkForm = {
        file_path: "",
        guest_id: null,
        access_type: "file",
        expires_in_days: 7,
        password: "",
        max_accesses: null,
        can_view: true,
        can_download: true,
        can_upload: false,
      };
    }
    showLinkModal = true;
  }

  async function saveLink() {
    if (!linkForm.file_path.trim()) {
      showToast(tr("guests.filePathRequired"), "error");
      return;
    }

    try {
      if (editingLink) {
        await guests.updateLink(editingLink.id, linkForm);
        showToast(tr("guests.linkUpdated"), "success");
      } else {
        await guests.createLink(linkForm);
        showToast(tr("guests.linkCreated"), "success");
      }
      showLinkModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to save link:", error);
      showToast(tr("guests.createError"), "error");
    }
  }

  async function toggleLink(link) {
    try {
      await guests.toggleLink(link.id);
      showToast(tr("guests.linkToggled"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to toggle link:", error);
      showToast(tr("guests.updateError"), "error");
    }
  }

  async function deleteLink(link) {
    if (!confirm(tr("guests.deleteLinkConfirm"))) return;

    try {
      await guests.deleteLink(link.id);
      showToast(tr("guests.linkDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete link:", error);
      showToast(tr("guests.deleteError"), "error");
    }
  }

  function copyLink(link) {
    const url = `${window.location.origin}/guest/${link.token}`;
    navigator.clipboard.writeText(url);
    showToast(tr("guests.linkCopied"), "success");
  }

  // Invitation Functions
  function openInvitationModal() {
    invitationForm = {
      email: "",
      expires_in_days: 7,
      message: "",
      can_view: true,
      can_download: true,
      can_upload: false,
      can_comment: false,
    };
    showInvitationModal = true;
  }

  async function sendInvitation() {
    if (!invitationForm.email.trim()) {
      showToast(tr("guests.emailRequired"), "error");
      return;
    }

    try {
      await guests.createInvitation(invitationForm);
      showToast(tr("guests.invitationCreated"), "success");
      showInvitationModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to send invitation:", error);
      showToast(tr("guests.createError"), "error");
    }
  }

  async function resendInvitation(invitation) {
    try {
      await guests.resendInvitation(invitation.id);
      showToast(tr("guests.invitationResent"), "success");
    } catch (error) {
      console.error("Failed to resend invitation:", error);
      showToast(tr("guests.updateError"), "error");
    }
  }

  async function deleteInvitation(invitation) {
    if (!confirm(tr("guests.deleteInvitationConfirm"))) return;

    try {
      await guests.deleteInvitation(invitation.id);
      showToast(tr("guests.invitationDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete invitation:", error);
      showToast(tr("guests.deleteError"), "error");
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return tr("guests.never");
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function isExpired(dateStr) {
    if (!dateStr) return false;
    return new Date(dateStr) < new Date();
  }
</script>

<PageWrapper gradient>
  <!-- Header -->
  <ModernCard variant="glass" class="mb-6">
    <div class="p-6">
      <div class="flex items-center justify-between flex-wrap gap-4">
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            <i class="bi bi-person-check-fill mr-2" aria-hidden="true"></i>
            {tr("guests.title")}
          </h1>
          <p class="text-gray-600 dark:text-gray-400">
            {tr("guests.subtitle")}
          </p>
        </div>
      </div>
    </div>
  </ModernCard>

  <!-- Stats Cards -->
  {#if stats}
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
      <ModernCard variant="glass">
        <div class="p-4">
          <div
            class="text-3xl font-bold text-primary-600 dark:text-primary-400"
          >
            {stats.active_guests || 0}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">
            {tr("guests.activeGuests")}
          </div>
        </div>
      </ModernCard>
      <ModernCard variant="glass">
        <div class="p-4">
          <div
            class="text-3xl font-bold text-secondary-600 dark:text-secondary-400"
          >
            {stats.active_links || 0}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">
            {tr("guests.activeLinks")}
          </div>
        </div>
      </ModernCard>
      <ModernCard variant="glass">
        <div class="p-4">
          <div class="text-3xl font-bold text-accent-600 dark:text-accent-400">
            {stats.pending_invitations || 0}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">
            {tr("guests.pendingInvitations")}
          </div>
        </div>
      </ModernCard>
      <ModernCard variant="glass">
        <div class="p-4">
          <div class="text-3xl font-bold text-info-600 dark:text-info-400">
            {stats.total_accesses || 0}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">
            {tr("guests.totalAccesses")}
          </div>
        </div>
      </ModernCard>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="flex gap-2 mb-6">
    <ModernButton
      variant={activeTab === "users" ? "primary" : "ghost"}
      onclick={() => (activeTab = "users")}
    >
      <i class="bi bi-person-badge mr-2"></i>
      {tr("guests.tabUsers")}
    </ModernButton>
    <ModernButton
      variant={activeTab === "links" ? "primary" : "ghost"}
      onclick={() => (activeTab = "links")}
    >
      <i class="bi bi-link-45deg mr-2"></i>
      {tr("guests.tabLinks")}
    </ModernButton>
    <ModernButton
      variant={activeTab === "invitations" ? "primary" : "ghost"}
      onclick={() => (activeTab = "invitations")}
    >
      <i class="bi bi-envelope mr-2"></i>
      {tr("guests.tabInvitations")}
    </ModernButton>
  </div>

  <!-- Content -->
  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if activeTab === "users"}
    <!-- Guest Users Tab -->
    <div class="flex justify-end mb-4">
      <ModernButton variant="primary" onclick={() => openGuestModal()}>
        <i class="bi bi-plus-lg mr-2"></i>
        {tr("guests.createGuest")}
      </ModernButton>
    </div>

    {#if guestUsers.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-person-badge text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{tr("guests.noGuests")}</p>
        <p class="text-sm text-base-content/40">{tr("guests.noGuestsHint")}</p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{tr("guests.guestName")}</th>
              <th>{tr("guests.guestEmail")}</th>
              <th>{tr("guests.permissions")}</th>
              <th>{tr("guests.accessCount")}</th>
              <th>{tr("guests.expiresIn")}</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each guestUsers as guest}
              <tr class={isExpired(guest.expires_at) ? "opacity-50" : ""}>
                <td class="font-medium">{guest.display_name}</td>
                <td>{guest.email || "-"}</td>
                <td>
                  <div class="flex gap-1">
                    {#if guest.can_view}<span
                        class="badge badge-sm badge-outline">View</span
                      >{/if}
                    {#if guest.can_download}<span
                        class="badge badge-sm badge-outline">DL</span
                      >{/if}
                    {#if guest.can_upload}<span
                        class="badge badge-sm badge-outline">UL</span
                      >{/if}
                    {#if guest.can_comment}<span
                        class="badge badge-sm badge-outline">Comment</span
                      >{/if}
                  </div>
                </td>
                <td>
                  {guest.access_count}
                  {#if guest.max_accesses}/ {guest.max_accesses}{/if}
                </td>
                <td>
                  {#if isExpired(guest.expires_at)}
                    <span class="badge badge-error badge-sm"
                      >{tr("guests.expired")}</span
                    >
                  {:else}
                    {formatDate(guest.expires_at)}
                  {/if}
                </td>
                <td>
                  <div class="flex gap-1">
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      title={tr("guests.viewActivity")}
                      onclick={() => viewGuestActivity(guest)}
                    >
                      <i class="bi bi-activity"></i>
                    </ModernButton>
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      title={tr("guests.editGuest")}
                      onclick={() => openGuestModal(guest)}
                    >
                      <i class="bi bi-pencil"></i>
                    </ModernButton>
                    <ModernButton
                      variant="danger"
                      size="sm"
                      title={tr("guests.deleteGuest")}
                      onclick={() => deleteGuest(guest)}
                    >
                      <i class="bi bi-trash"></i>
                    </ModernButton>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {:else if activeTab === "links"}
    <!-- Access Links Tab -->
    <div class="flex justify-end mb-4">
      <ModernButton variant="primary" onclick={() => openLinkModal()}>
        <i class="bi bi-plus-lg mr-2"></i>
        {tr("guests.createLink")}
      </ModernButton>
    </div>

    {#if accessLinks.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-link-45deg text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{tr("guests.noLinks")}</p>
        <p class="text-sm text-base-content/40">{tr("guests.noLinksHint")}</p>
      </div>
    {:else}
      <div class="space-y-3">
        {#each accessLinks as link}
          <div
            class="bg-base-200 rounded-xl p-4 {!link.is_active
              ? 'opacity-50'
              : ''}"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-2">
                  <i
                    class="bi {link.access_type === 'folder'
                      ? 'bi-folder'
                      : 'bi-file-earmark'} text-primary"
                  ></i>
                  <span class="font-mono text-sm">{link.file_path}</span>
                  {#if link.password_hash}
                    <span class="badge badge-warning badge-sm">
                      <i class="bi bi-lock-fill mr-1"></i>
                      {tr("guests.passwordProtected")}
                    </span>
                  {/if}
                </div>
                <div class="flex gap-4 text-sm text-base-content/60">
                  <span>
                    <i class="bi bi-eye mr-1"></i>
                    {link.access_count}
                    {tr("guests.accessCount").toLowerCase()}
                  </span>
                  {#if link.expires_at}
                    <span>
                      <i class="bi bi-clock mr-1"></i>
                      {formatDate(link.expires_at)}
                    </span>
                  {/if}
                </div>
              </div>
              <div class="flex items-center gap-2">
                <button
                  class="btn btn-ghost btn-sm"
                  onclick={() => copyLink(link)}
                  title={tr("guests.copyLink")}
                >
                  <i class="bi bi-clipboard"></i>
                </button>
                <input
                  type="checkbox"
                  class="toggle toggle-primary toggle-sm"
                  checked={link.is_active}
                  onchange={() => toggleLink(link)}
                  title={tr("guests.toggleActive")}
                />
                <button
                  class="btn btn-ghost btn-sm"
                  onclick={() => openLinkModal(link)}
                  title={tr("guests.editLink")}
                >
                  <i class="bi bi-pencil"></i>
                </button>
                <button
                  class="btn btn-ghost btn-sm text-error"
                  onclick={() => deleteLink(link)}
                  title={tr("guests.deleteLink")}
                >
                  <i class="bi bi-trash"></i>
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {:else if activeTab === "invitations"}
    <!-- Invitations Tab -->
    <div class="flex justify-end mb-4">
      <button class="btn btn-primary" onclick={() => openInvitationModal()}>
        <i class="bi bi-plus-lg mr-2"></i>
        {tr("guests.sendInvitation")}
      </button>
    </div>

    {#if invitations.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-envelope text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{tr("guests.noInvitations")}</p>
        <p class="text-sm text-base-content/40">
          {tr("guests.noInvitationsHint")}
        </p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{tr("guests.inviteEmail")}</th>
              <th>{tr("guests.permissions")}</th>
              <th>{tr("guests.expiresIn")}</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each invitations as invitation}
              <tr class={isExpired(invitation.expires_at) ? "opacity-50" : ""}>
                <td>{invitation.email}</td>
                <td>
                  <div class="flex gap-1">
                    {#if invitation.can_view}<span
                        class="badge badge-sm badge-outline">View</span
                      >{/if}
                    {#if invitation.can_download}<span
                        class="badge badge-sm badge-outline">DL</span
                      >{/if}
                    {#if invitation.can_upload}<span
                        class="badge badge-sm badge-outline">UL</span
                      >{/if}
                    {#if invitation.can_comment}<span
                        class="badge badge-sm badge-outline">Comment</span
                      >{/if}
                  </div>
                </td>
                <td>
                  {#if isExpired(invitation.expires_at)}
                    <span class="badge badge-error badge-sm"
                      >{tr("guests.expired")}</span
                    >
                  {:else}
                    {formatDate(invitation.expires_at)}
                  {/if}
                </td>
                <td>
                  <div class="flex gap-1">
                    <button
                      class="btn btn-ghost btn-xs"
                      title={tr("guests.resendInvitation")}
                      onclick={() => resendInvitation(invitation)}
                      disabled={isExpired(invitation.expires_at)}
                    >
                      <i class="bi bi-arrow-repeat"></i>
                    </button>
                    <button
                      class="btn btn-ghost btn-xs text-error"
                      title={tr("guests.deleteInvitation")}
                      onclick={() => deleteInvitation(invitation)}
                    >
                      <i class="bi bi-trash"></i>
                    </button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}

  <!-- Guest User Modal -->
  <UIModal
    bind:show={showGuestModal}
    title={editingGuest ? tr("guests.editGuest") : tr("guests.createGuest")}
    size="lg"
    actions={[
      {
        label: tr("guests.cancel"),
        variant: "secondary",
        onClick: () => (showGuestModal = false),
      },
      {
        label: editingGuest ? tr("guests.save") : tr("guests.create"),
        variant: "primary",
        onClick: saveGuest,
      },
    ]}
  >
    <div class="space-y-4">
      <UIInput
        label="{tr('guests.guestName')} *"
        bind:value={guestForm.display_name}
        placeholder="John Doe"
      />

      <UIInput
        label={tr("guests.guestEmail")}
        type="email"
        bind:value={guestForm.email}
        placeholder="john@example.com"
      />

      <div class="grid grid-cols-2 gap-4">
        <UISelect
          label={tr("guests.expiresIn")}
          bind:value={guestForm.expires_in_days}
        />

        <UIInput
          label={tr("guests.maxAccesses")}
          type="number"
          bind:value={guestForm.max_accesses}
        />
      </div>

      <UITextarea
        label={tr("guests.guestNotes")}
        bind:value={guestForm.notes}
        rows={2}
      />

      <div>
        <div class="mb-2">
          <span class="text-sm font-medium">{tr("guests.permissions")}</span>
        </div>
        <div class="flex flex-wrap gap-4">
          <UICheckbox
            bind:checked={guestForm.can_view}
            label={tr("guests.canView")}
          />
          <UICheckbox
            bind:checked={guestForm.can_download}
            label={tr("guests.canDownload")}
          />
          <UICheckbox
            bind:checked={guestForm.can_upload}
            label={tr("guests.canUpload")}
          />
          <UICheckbox
            bind:checked={guestForm.can_comment}
            label={tr("guests.canComment")}
          />
        </div>
      </div>
    </div>
  </UIModal>

  <!-- Link Modal -->
  <UIModal
    bind:show={showLinkModal}
    title={editingLink ? tr("guests.editLink") : tr("guests.createLink")}
    size="lg"
    actions={[
      {
        label: tr("guests.cancel"),
        variant: "secondary",
        onClick: () => (showLinkModal = false),
      },
      {
        label: editingLink ? tr("guests.save") : tr("guests.create"),
        variant: "primary",
        onClick: saveLink,
      },
    ]}
  >
    <div class="space-y-4">
      <UIInput
        label="{tr('guests.filePath')} *"
        bind:value={linkForm.file_path}
        placeholder="/documents/report.pdf"
      />

      <div class="grid grid-cols-2 gap-4">
        <UISelect
          label={tr("guests.accessType")}
          bind:value={linkForm.access_type}
          options={[
            { value: "file", label: tr("guests.typeFile") },
            { value: "folder", label: tr("guests.typeFolder") },
          ]}
        />

        <UISelect
          label={tr("guests.expiresIn")}
          bind:value={linkForm.expires_in_days}
        />
      </div>

      <UIInput
        label={tr("guests.password")}
        type="password"
        bind:value={linkForm.password}
      />

      <UIInput
        label={tr("guests.maxAccesses")}
        type="number"
        bind:value={linkForm.max_accesses}
      />

      <div>
        <div class="mb-2">
          <span class="text-sm font-medium">{tr("guests.permissions")}</span>
        </div>
        <div class="flex flex-wrap gap-4">
          <UICheckbox
            bind:checked={linkForm.can_view}
            label={tr("guests.canView")}
          />
          <UICheckbox
            bind:checked={linkForm.can_download}
            label={tr("guests.canDownload")}
          />
          <UICheckbox
            bind:checked={linkForm.can_upload}
            label={tr("guests.canUpload")}
          />
        </div>
      </div>
    </div>
  </UIModal>

  <!-- Invitation Modal -->
  <UIModal
    bind:show={showInvitationModal}
    title={tr("guests.sendInvitation")}
    size="lg"
    actions={[
      {
        label: tr("guests.cancel"),
        variant: "secondary",
        onClick: () => (showInvitationModal = false),
      },
      {
        label: tr("guests.sendInvitation"),
        variant: "primary",
        onClick: sendInvitation,
      },
    ]}
  >
    <div class="space-y-4">
      <UIInput
        label="{tr('guests.inviteEmail')} *"
        type="email"
        bind:value={invitationForm.email}
        placeholder="guest@example.com"
      />

      <UISelect
        label={tr("guests.expiresIn")}
        bind:value={invitationForm.expires_in_days}
      />

      <UITextarea
        label={tr("guests.inviteMessage")}
        bind:value={invitationForm.message}
        rows={3}
        placeholder="Optional personal message..."
      />

      <div>
        <div class="mb-2">
          <span class="text-sm font-medium">{tr("guests.permissions")}</span>
        </div>
        <div class="flex flex-wrap gap-4">
          <UICheckbox
            bind:checked={invitationForm.can_view}
            label={tr("guests.canView")}
          />
          <UICheckbox
            bind:checked={invitationForm.can_download}
            label={tr("guests.canDownload")}
          />
          <UICheckbox
            bind:checked={invitationForm.can_upload}
            label={tr("guests.canUpload")}
          />
          <UICheckbox
            bind:checked={invitationForm.can_comment}
            label={tr("guests.canComment")}
          />
        </div>
      </div>
    </div>
  </UIModal>

  <!-- Activity Modal -->
  <UIModal
    bind:show={showActivityModal}
    title="{tr('guests.guestActivity')}: {selectedGuestActivity?.display_name ||
      ''}"
    size="xl"
    actions={[
      {
        label: "Close",
        variant: "secondary",
        onClick: () => {
          showActivityModal = false;
          selectedGuestActivity = null;
        },
      },
    ]}
  >
    {#if guestActivity.length === 0}
      <div class="text-center py-8 text-base-content/60">
        <i class="bi bi-activity text-3xl mb-2"></i>
        <p>{tr("guests.noActivity")}</p>
      </div>
    {:else}
      <div class="space-y-2 max-h-96 overflow-y-auto">
        {#each guestActivity as activity}
          <div class="flex items-center gap-3 p-2 bg-base-200 rounded-lg">
            <i class="bi bi-clock text-base-content/40"></i>
            <div class="flex-1">
              <span class="font-medium">{activity.action}</span>
              {#if activity.file_path}
                <span class="text-sm text-base-content/60 ml-2"
                  >{activity.file_path}</span
                >
              {/if}
            </div>
            <span class="text-xs text-base-content/40"
              >{formatDate(activity.accessed_at)}</span
            >
          </div>
        {/each}
      </div>
    {/if}
  </UIModal>
</PageWrapper>
