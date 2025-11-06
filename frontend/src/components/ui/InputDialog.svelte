<script>
  import Dialog from "./Dialog.svelte";
  import Input from "./Input.svelte";
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    open = $bindable(false),
    title = "",
    label = "",
    placeholder = "",
    initialValue = "",
    type = "text",
    required = true,
    confirmText = "OK",
    cancelText = "Abbrechen",
  } = $props();

  const dispatch = createEventDispatcher();
  let value = $state(initialValue);
  let error = $state("");

  $effect(() => {
    if (open) {
      value = initialValue;
      error = "";
    }
  });

  function handleConfirm() {
    if (required && !value.trim()) {
      error = tr("fieldRequired");
      return;
    }
    dispatch("confirm", value);
    open = false;
  }

  function handleCancel() {
    dispatch("cancel");
    open = false;
  }
</script>

<Dialog
  {open}
  {title}
  {confirmText}
  {cancelText}
  on:confirm={handleConfirm}
  on:cancel={handleCancel}
>
  <Input
    bind:value
    {label}
    {placeholder}
    {type}
    {required}
    {error}
    oninput={() => (error = "")}
  />
</Dialog>
