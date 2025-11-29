<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * FormField - Consistent form field wrapper with label, input, and error
   * @prop {string} label - Field label
   * @prop {string} name - Field name/id
   * @prop {string} type - Input type (text/email/password/textarea/select)
   * @prop {string} value - Field value (bindable)
   * @prop {string} placeholder - Placeholder text
   * @prop {boolean} required - Required field
   * @prop {boolean} disabled - Disabled field
   * @prop {string} error - Error message
   * @prop {string} hint - Helper text
   * @prop {string} icon - Bootstrap icon (left side)
   * @prop {string} size - Field size (sm/md/lg)
   * @prop {Array} options - Select options [{value, label}]
   * @prop {number} rows - Textarea rows
   */

  let {
    label = "",
    name = "",
    type = "text",
    value = $bindable(""),
    placeholder = "",
    required = false,
    disabled = false,
    error = "",
    hint = "",
    icon = "",
    size = "md",
    options = [],
    rows = 4,
    class: className = "",
    oninput = null,
    onchange = null,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const sizeClasses = {
    sm: "px-3 py-1.5 text-sm h-9",
    md: "px-4 py-2 text-base h-10",
    lg: "px-5 py-3 text-lg h-12",
  };

  const fieldId = $derived(name || `field-${Math.random().toString(36).substr(2, 9)}`);

  function handleInput(e) {
    value = e.target.value;
    if (oninput) oninput(e);
  }

  function handleChange(e) {
    value = e.target.value;
    if (onchange) onchange(e);
  }
</script>

<div class="form-field {className}">
  <!-- Label -->
  {#if label}
    <label for={fieldId} class="field-label">
      {label}
      {#if required}
        <span class="required-indicator">*</span>
      {/if}
    </label>
  {/if}

  <!-- Input Container -->
  <div class="field-container">
    {#if icon}
      <div class="field-icon">
        <i class="bi bi-{icon}"></i>
      </div>
    {/if}

    {#if type === "textarea"}
      <!-- Textarea -->
      <textarea
        id={fieldId}
        name={name}
        class="field-input {sizeClasses[size]}"
        class:has-icon={icon}
        class:has-error={error}
        placeholder={placeholder}
        required={required}
        disabled={disabled}
        rows={rows}
        value={value}
        oninput={handleInput}
        onchange={handleChange}
      ></textarea>
    {:else if type === "select"}
      <!-- Select -->
      <select
        id={fieldId}
        name={name}
        class="field-input {sizeClasses[size]}"
        class:has-icon={icon}
        class:has-error={error}
        required={required}
        disabled={disabled}
        value={value}
        onchange={handleChange}
      >
        {#if placeholder}
          <option value="" disabled>{placeholder}</option>
        {/if}
        {#each options as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
      <div class="select-arrow">
        <i class="bi bi-chevron-down"></i>
      </div>
    {:else}
      <!-- Regular Input -->
      <input
        id={fieldId}
        name={name}
        type={type}
        class="field-input {sizeClasses[size]}"
        class:has-icon={icon}
        class:has-error={error}
        placeholder={placeholder}
        required={required}
        disabled={disabled}
        value={value}
        oninput={handleInput}
        onchange={handleChange}
      />
    {/if}
  </div>

  <!-- Hint or Error -->
  {#if error}
    <div class="field-error">
      <i class="bi bi-exclamation-circle"></i>
      {error}
    </div>
  {:else if hint}
    <div class="field-hint">{hint}</div>
  {/if}
</div>

<style>
  .form-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .field-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(55 65 81);
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  :global(.dark) .field-label {
    color: rgb(209 213 219);
  }

  .required-indicator {
    color: rgb(239 68 68);
  }

  .field-container {
    position: relative;
  }

  .field-icon {
    position: absolute;
    left: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: rgb(156 163 175);
    pointer-events: none;
    z-index: 1;
  }

  .field-input {
    width: 100%;
    border: 1px solid rgb(229 231 235);
    border-radius: 8px;
    background: white;
    color: rgb(17 24 39);
    transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
    font-family: inherit;
  }

  :global(.dark) .field-input {
    background: rgb(31 41 55);
    border-color: rgb(55 65 81);
    color: rgb(243 244 246);
  }

  .field-input.has-icon {
    padding-left: 2.75rem;
  }

  .field-input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .field-input.has-error {
    border-color: rgb(239 68 68);
  }

  .field-input.has-error:focus {
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
  }

  .field-input:disabled {
    background: rgb(243 244 246);
    color: rgb(156 163 175);
    cursor: not-allowed;
  }

  :global(.dark) .field-input:disabled {
    background: rgb(17 24 39);
    color: rgb(75 85 99);
  }

  /* Textarea */
  textarea.field-input {
    resize: vertical;
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
    min-height: auto;
  }

  /* Select */
  select.field-input {
    appearance: none;
    padding-right: 2.5rem;
    cursor: pointer;
  }

  .select-arrow {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: rgb(156 163 175);
    pointer-events: none;
  }

  /* Error Message */
  .field-error {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
    color: rgb(239 68 68);
  }

  .field-error i {
    font-size: 1rem;
  }

  /* Hint Text */
  .field-hint {
    font-size: 0.875rem;
    color: rgb(107 114 128);
  }

  :global(.dark) .field-hint {
    color: rgb(156 163 175);
  }

  /* Responsive */
  @media (max-width: 640px) {
    .field-input {
      font-size: 16px; /* Prevent zoom on iOS */
    }
  }
</style>
