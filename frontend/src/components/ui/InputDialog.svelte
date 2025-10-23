<script>
  import Dialog from "./Dialog.svelte";
  import Input from "./Input.svelte";
  import { createEventDispatcher } from "svelte";

  export let open = false;
  export let title = "";
  export let label = "";
  export let placeholder = "";
  export let initialValue = "";
  export let type = "text";
  export let required = true;
  export let confirmText = "OK";
  export let cancelText = "Abbrechen";

  const dispatch = createEventDispatcher();
  let value = initialValue;
  let error = "";

  $: if (open) {
    value = initialValue;
    error = "";
  }

  function handleConfirm() {
    if (required && !value.trim()) {
      error = "Dieses Feld ist erforderlich";
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
