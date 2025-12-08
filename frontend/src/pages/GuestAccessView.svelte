<script>
  import { onMount } from "svelte";
  import { t } from "../i18n.js";
  import { guests } from "../lib/api.js";
  import { showToast } from "../stores/toast.js";

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
      showToast($t("guests.loadError"), "error");
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
      showToast($t("guests.nameRequired"), "error");
      return;
    }

    try {
      if (editingGuest) {
        await guests.update(editingGuest.id, guestForm);
        showToast($t("guests.guestUpdated"), "success");
      } else {
        await guests.create(guestForm);
        showToast($t("guests.guestCreated"), "success");
      }
      showGuestModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to save guest:", error);
      showToast($t("guests.createError"), "error");
    }
  }

  async function deleteGuest(guest) {
    if (!confirm($t("guests.deleteGuestConfirm"))) return;

    try {
      await guests.delete(guest.id);
      showToast($t("guests.guestDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete guest:", error);
      showToast($t("guests.deleteError"), "error");
    }
  }

  async function viewGuestActivity(guest) {
    selectedGuestActivity = guest;
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
      showToast($t("guests.filePathRequired"), "error");
      return;
    }

    try {
      if (editingLink) {
        await guests.updateLink(editingLink.id, linkForm);
        showToast($t("guests.linkUpdated"), "success");
      } else {
        await guests.createLink(linkForm);
        showToast($t("guests.linkCreated"), "success");
      }
      showLinkModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to save link:", error);
      showToast($t("guests.createError"), "error");
    }
  }

  async function toggleLink(link) {
    try {
      await guests.toggleLink(link.id);
      showToast($t("guests.linkToggled"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to toggle link:", error);
      showToast($t("guests.updateError"), "error");
    }
  }

  async function deleteLink(link) {
    if (!confirm($t("guests.deleteLinkConfirm"))) return;

    try {
      await guests.deleteLink(link.id);
      showToast($t("guests.linkDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete link:", error);
      showToast($t("guests.deleteError"), "error");
    }
  }

  function copyLink(link) {
    const url = `${window.location.origin}/guest/${link.token}`;
    navigator.clipboard.writeText(url);
    showToast($t("guests.linkCopied"), "success");
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
      showToast($t("guests.emailRequired"), "error");
      return;
    }

    try {
      await guests.createInvitation(invitationForm);
      showToast($t("guests.invitationCreated"), "success");
      showInvitationModal = false;
      await loadData();
    } catch (error) {
      console.error("Failed to send invitation:", error);
      showToast($t("guests.createError"), "error");
    }
  }

  async function resendInvitation(invitation) {
    try {
      await guests.resendInvitation(invitation.id);
      showToast($t("guests.invitationResent"), "success");
    } catch (error) {
      console.error("Failed to resend invitation:", error);
      showToast($t("guests.updateError"), "error");
    }
  }

  async function deleteInvitation(invitation) {
    if (!confirm($t("guests.deleteInvitationConfirm"))) return;

    try {
      await guests.deleteInvitation(invitation.id);
      showToast($t("guests.invitationDeleted"), "success");
      await loadData();
    } catch (error) {
      console.error("Failed to delete invitation:", error);
      showToast($t("guests.deleteError"), "error");
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return $t("guests.never");
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

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-2xl font-bold text-base-content">{$t("guests.title")}</h1>
    <p class="text-base-content/60 mt-1">{$t("guests.subtitle")}</p>
  </div>

  <!-- Stats Cards -->
  {#if stats}
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
      <div class="bg-base-200 rounded-xl p-4">
        <div class="text-3xl font-bold text-primary">
          {stats.active_guests || 0}
        </div>
        <div class="text-sm text-base-content/60">
          {$t("guests.activeGuests")}
        </div>
      </div>
      <div class="bg-base-200 rounded-xl p-4">
        <div class="text-3xl font-bold text-secondary">
          {stats.active_links || 0}
        </div>
        <div class="text-sm text-base-content/60">
          {$t("guests.activeLinks")}
        </div>
      </div>
      <div class="bg-base-200 rounded-xl p-4">
        <div class="text-3xl font-bold text-accent">
          {stats.pending_invitations || 0}
        </div>
        <div class="text-sm text-base-content/60">
          {$t("guests.pendingInvitations")}
        </div>
      </div>
      <div class="bg-base-200 rounded-xl p-4">
        <div class="text-3xl font-bold text-info">
          {stats.total_accesses || 0}
        </div>
        <div class="text-sm text-base-content/60">
          {$t("guests.totalAccesses")}
        </div>
      </div>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="tabs tabs-boxed mb-6 bg-base-200 p-1">
    <button
      class="tab {activeTab === 'users'
        ? 'tab-active bg-primary text-primary-content'
        : ''}"
      onclick={() => (activeTab = "users")}
    >
      <i class="bi bi-person-badge mr-2"></i>
      {$t("guests.tabUsers")}
    </button>
    <button
      class="tab {activeTab === 'links'
        ? 'tab-active bg-primary text-primary-content'
        : ''}"
      onclick={() => (activeTab = "links")}
    >
      <i class="bi bi-link-45deg mr-2"></i>
      {$t("guests.tabLinks")}
    </button>
    <button
      class="tab {activeTab === 'invitations'
        ? 'tab-active bg-primary text-primary-content'
        : ''}"
      onclick={() => (activeTab = "invitations")}
    >
      <i class="bi bi-envelope mr-2"></i>
      {$t("guests.tabInvitations")}
    </button>
  </div>

  <!-- Content -->
  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if activeTab === "users"}
    <!-- Guest Users Tab -->
    <div class="flex justify-end mb-4">
      <button class="btn btn-primary" onclick={() => openGuestModal()}>
        <i class="bi bi-plus-lg mr-2"></i>
        {$t("guests.createGuest")}
      </button>
    </div>

    {#if guestUsers.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-person-badge text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{$t("guests.noGuests")}</p>
        <p class="text-sm text-base-content/40">{$t("guests.noGuestsHint")}</p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{$t("guests.guestName")}</th>
              <th>{$t("guests.guestEmail")}</th>
              <th>{$t("guests.permissions")}</th>
              <th>{$t("guests.accessCount")}</th>
              <th>{$t("guests.expiresIn")}</th>
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
                      >{$t("guests.expired")}</span
                    >
                  {:else}
                    {formatDate(guest.expires_at)}
                  {/if}
                </td>
                <td>
                  <div class="flex gap-1">
                    <button
                      class="btn btn-ghost btn-xs"
                      title={$t("guests.viewActivity")}
                      onclick={() => viewGuestActivity(guest)}
                    >
                      <i class="bi bi-activity"></i>
                    </button>
                    <button
                      class="btn btn-ghost btn-xs"
                      title={$t("guests.editGuest")}
                      onclick={() => openGuestModal(guest)}
                    >
                      <i class="bi bi-pencil"></i>
                    </button>
                    <button
                      class="btn btn-ghost btn-xs text-error"
                      title={$t("guests.deleteGuest")}
                      onclick={() => deleteGuest(guest)}
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
  {:else if activeTab === "links"}
    <!-- Access Links Tab -->
    <div class="flex justify-end mb-4">
      <button class="btn btn-primary" onclick={() => openLinkModal()}>
        <i class="bi bi-plus-lg mr-2"></i>
        {$t("guests.createLink")}
      </button>
    </div>

    {#if accessLinks.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-link-45deg text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{$t("guests.noLinks")}</p>
        <p class="text-sm text-base-content/40">{$t("guests.noLinksHint")}</p>
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
                      {$t("guests.passwordProtected")}
                    </span>
                  {/if}
                </div>
                <div class="flex gap-4 text-sm text-base-content/60">
                  <span>
                    <i class="bi bi-eye mr-1"></i>
                    {link.access_count}
                    {$t("guests.accessCount").toLowerCase()}
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
                  title={$t("guests.copyLink")}
                >
                  <i class="bi bi-clipboard"></i>
                </button>
                <input
                  type="checkbox"
                  class="toggle toggle-primary toggle-sm"
                  checked={link.is_active}
                  onchange={() => toggleLink(link)}
                  title={$t("guests.toggleActive")}
                />
                <button
                  class="btn btn-ghost btn-sm"
                  onclick={() => openLinkModal(link)}
                  title={$t("guests.editLink")}
                >
                  <i class="bi bi-pencil"></i>
                </button>
                <button
                  class="btn btn-ghost btn-sm text-error"
                  onclick={() => deleteLink(link)}
                  title={$t("guests.deleteLink")}
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
        {$t("guests.sendInvitation")}
      </button>
    </div>

    {#if invitations.length === 0}
      <div class="text-center py-12 bg-base-200 rounded-xl">
        <i class="bi bi-envelope text-5xl text-base-content/30"></i>
        <p class="mt-4 text-base-content/60">{$t("guests.noInvitations")}</p>
        <p class="text-sm text-base-content/40">
          {$t("guests.noInvitationsHint")}
        </p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{$t("guests.inviteEmail")}</th>
              <th>{$t("guests.permissions")}</th>
              <th>{$t("guests.expiresIn")}</th>
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
                      >{$t("guests.expired")}</span
                    >
                  {:else}
                    {formatDate(invitation.expires_at)}
                  {/if}
                </td>
                <td>
                  <div class="flex gap-1">
                    <button
                      class="btn btn-ghost btn-xs"
                      title={$t("guests.resendInvitation")}
                      onclick={() => resendInvitation(invitation)}
                      disabled={isExpired(invitation.expires_at)}
                    >
                      <i class="bi bi-arrow-repeat"></i>
                    </button>
                    <button
                      class="btn btn-ghost btn-xs text-error"
                      title={$t("guests.deleteInvitation")}
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
</div>

<!-- Guest User Modal -->
{#if showGuestModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {editingGuest ? $t("guests.editGuest") : $t("guests.createGuest")}
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="guest-name">
            <span class="label-text">{$t("guests.guestName")} *</span>
          </label>
          <input
            id="guest-name"
            type="text"
            class="input input-bordered"
            bind:value={guestForm.display_name}
            placeholder="John Doe"
          />
        </div>

        <div class="form-control">
          <label class="label" for="guest-email">
            <span class="label-text">{$t("guests.guestEmail")}</span>
          </label>
          <input
            id="guest-email"
            type="email"
            class="input input-bordered"
            bind:value={guestForm.email}
            placeholder="john@example.com"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label" for="guest-expires">
              <span class="label-text">{$t("guests.expiresIn")}</span>
            </label>
            <select
              id="guest-expires"
              class="select select-bordered"
              bind:value={guestForm.expires_in_days}
            >
              <option value={7}>7 {$t("guests.days")}</option>
              <option value={14}>14 {$t("guests.days")}</option>
              <option value={30}>30 {$t("guests.days")}</option>
              <option value={90}>90 {$t("guests.days")}</option>
              <option value={365}>365 {$t("guests.days")}</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label" for="guest-max">
              <span class="label-text">{$t("guests.maxAccesses")}</span>
            </label>
            <input
              id="guest-max"
              type="number"
              class="input input-bordered"
              bind:value={guestForm.max_accesses}
              placeholder={$t("guests.unlimited")}
              min="1"
            />
          </div>
        </div>

        <div class="form-control">
          <label class="label" for="guest-notes">
            <span class="label-text">{$t("guests.guestNotes")}</span>
          </label>
          <textarea
            id="guest-notes"
            class="textarea textarea-bordered"
            bind:value={guestForm.notes}
            rows="2"
          ></textarea>
        </div>

        <fieldset class="form-control">
          <legend class="label">
            <span class="label-text">{$t("guests.permissions")}</span>
          </legend>
          <div class="flex flex-wrap gap-4">
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={guestForm.can_view}
              />
              <span class="label-text">{$t("guests.canView")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={guestForm.can_download}
              />
              <span class="label-text">{$t("guests.canDownload")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={guestForm.can_upload}
              />
              <span class="label-text">{$t("guests.canUpload")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={guestForm.can_comment}
              />
              <span class="label-text">{$t("guests.canComment")}</span>
            </label>
          </div>
        </fieldset>
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => (showGuestModal = false)}>
          {$t("guests.cancel")}
        </button>
        <button class="btn btn-primary" onclick={saveGuest}>
          {editingGuest ? $t("guests.save") : $t("guests.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="0"
      aria-label="Close modal"
      onclick={() => (showGuestModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showGuestModal = false)}
    ></div>
  </div>
{/if}

<!-- Link Modal -->
{#if showLinkModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {editingLink ? $t("guests.editLink") : $t("guests.createLink")}
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="link-path">
            <span class="label-text">{$t("guests.filePath")} *</span>
          </label>
          <input
            id="link-path"
            type="text"
            class="input input-bordered font-mono"
            bind:value={linkForm.file_path}
            placeholder="/documents/report.pdf"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label" for="link-type">
              <span class="label-text">{$t("guests.accessType")}</span>
            </label>
            <select
              id="link-type"
              class="select select-bordered"
              bind:value={linkForm.access_type}
            >
              <option value="file">{$t("guests.typeFile")}</option>
              <option value="folder">{$t("guests.typeFolder")}</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label" for="link-expires">
              <span class="label-text">{$t("guests.expiresIn")}</span>
            </label>
            <select
              id="link-expires"
              class="select select-bordered"
              bind:value={linkForm.expires_in_days}
            >
              <option value={1}>1 {$t("guests.days")}</option>
              <option value={7}>7 {$t("guests.days")}</option>
              <option value={14}>14 {$t("guests.days")}</option>
              <option value={30}>30 {$t("guests.days")}</option>
              <option value={null}>{$t("guests.never")}</option>
            </select>
          </div>
        </div>

        <div class="form-control">
          <label class="label" for="link-password">
            <span class="label-text">{$t("guests.password")}</span>
          </label>
          <input
            id="link-password"
            type="password"
            class="input input-bordered"
            bind:value={linkForm.password}
            placeholder={$t("guests.noPassword")}
          />
        </div>

        <div class="form-control">
          <label class="label" for="link-max">
            <span class="label-text">{$t("guests.maxAccesses")}</span>
          </label>
          <input
            id="link-max"
            type="number"
            class="input input-bordered"
            bind:value={linkForm.max_accesses}
            placeholder={$t("guests.unlimited")}
            min="1"
          />
        </div>

        <div class="form-control">
          <span class="label-text">{$t("guests.permissions")}</span>
          <div class="flex flex-wrap gap-4 mt-2">
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={linkForm.can_view}
              />
              <span class="label-text">{$t("guests.canView")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={linkForm.can_download}
              />
              <span class="label-text">{$t("guests.canDownload")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={linkForm.can_upload}
              />
              <span class="label-text">{$t("guests.canUpload")}</span>
            </label>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => (showLinkModal = false)}>
          {$t("guests.cancel")}
        </button>
        <button class="btn btn-primary" onclick={saveLink}>
          {editingLink ? $t("guests.save") : $t("guests.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="0"
      aria-label="Close modal"
      onclick={() => (showLinkModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showLinkModal = false)}
    ></div>
  </div>
{/if}

<!-- Invitation Modal -->
{#if showInvitationModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">{$t("guests.sendInvitation")}</h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="invite-email">
            <span class="label-text">{$t("guests.inviteEmail")} *</span>
          </label>
          <input
            id="invite-email"
            type="email"
            class="input input-bordered"
            bind:value={invitationForm.email}
            placeholder="guest@example.com"
          />
        </div>

        <div class="form-control">
          <label class="label" for="invite-expires">
            <span class="label-text">{$t("guests.expiresIn")}</span>
          </label>
          <select
            id="invite-expires"
            class="select select-bordered"
            bind:value={invitationForm.expires_in_days}
          >
            <option value={1}>1 {$t("guests.days")}</option>
            <option value={7}>7 {$t("guests.days")}</option>
            <option value={14}>14 {$t("guests.days")}</option>
            <option value={30}>30 {$t("guests.days")}</option>
          </select>
        </div>

        <div class="form-control">
          <label class="label" for="invite-message">
            <span class="label-text">{$t("guests.inviteMessage")}</span>
          </label>
          <textarea
            id="invite-message"
            class="textarea textarea-bordered"
            bind:value={invitationForm.message}
            rows="3"
            placeholder="Optional personal message..."
          ></textarea>
        </div>

        <div class="form-control">
          <span class="label-text">{$t("guests.permissions")}</span>
          <div class="flex flex-wrap gap-4 mt-2">
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={invitationForm.can_view}
              />
              <span class="label-text">{$t("guests.canView")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={invitationForm.can_download}
              />
              <span class="label-text">{$t("guests.canDownload")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={invitationForm.can_upload}
              />
              <span class="label-text">{$t("guests.canUpload")}</span>
            </label>
            <label class="label cursor-pointer gap-2">
              <input
                type="checkbox"
                class="checkbox checkbox-primary checkbox-sm"
                bind:checked={invitationForm.can_comment}
              />
              <span class="label-text">{$t("guests.canComment")}</span>
            </label>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => (showInvitationModal = false)}>
          {$t("guests.cancel")}
        </button>
        <button class="btn btn-primary" onclick={sendInvitation}>
          {$t("guests.sendInvitation")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="0"
      aria-label="Close modal"
      onclick={() => (showInvitationModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showInvitationModal = false)}
    ></div>
  </div>
{/if}

<!-- Activity Modal -->
{#if selectedGuestActivity}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        {$t("guests.guestActivity")}: {selectedGuestActivity.display_name}
      </h3>

      {#if guestActivity.length === 0}
        <div class="text-center py-8 text-base-content/60">
          <i class="bi bi-activity text-3xl mb-2"></i>
          <p>No activity recorded</p>
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

      <div class="modal-action">
        <button class="btn" onclick={() => (selectedGuestActivity = null)}>
          Close
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="0"
      aria-label={$t("common.close")}
      onclick={() => (selectedGuestActivity = null)}
      onkeydown={(e) =>
        (e.key === "Enter" || e.key === " ") && (selectedGuestActivity = null)}
    ></div>
  </div>
{/if}
