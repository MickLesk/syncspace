<!--
  TextareaV2.svelte - Modern Textarea Component using DaisyUI
  
  A flexible textarea with auto-resize, character counter, and validation.
  
  Props:
  - value: string - Textarea value
  - placeholder: string - Placeholder text
  - label: string - Input label
  - error: string - Error message
  - success: string - Success message
  - helpText: string - Help text
  - disabled: boolean - Disable textarea
  - required: boolean - Required field
  - rows: number - Initial rows (default: 3)
  - maxlength: number - Maximum characters
  - showCharCount: boolean - Show character counter (default: false)
  - autoResize: boolean - Auto-resize height (default: false)
  - size: "xs" | "sm" | "md" | "lg" (default: "md")
  - glass: boolean - Glass effect (default: false)
  
  Events:
  - on:input, on:change, on:focus, on:blur, on:keydown, on:keyup
  
  Usage:
  <TextareaV2 
    bind:value={message}
    label="Message"
    placeholder="Type your message..."
    maxlength={500}
    showCharCount
    autoResize
  />
-->

<script>
  export let value = "";
  export let placeholder = "";
  export let label = "";
  export let error = "";
  export let success = "";
  export let helpText = "";
  export let disabled = false;
  export let required = false;
  export let rows = 3;
  export let maxlength = null;
  export let showCharCount = false;
  export let autoResize = false;
  export let size = "md";
  export let glass = false;

  let textareaElement;

  $: hasError = !!error;
  $: hasSuccess = !!success && !hasError;
  $: charCount = value?.length || 0;

  $: sizeClasses = {
    xs: "textarea-xs",
    sm: "textarea-sm",
    md: "textarea-md",
    lg: "textarea-lg",
  };

  $: computedClass = [
    "textarea",
    "textarea-bordered",
    "w-full",
    sizeClasses[size],
    hasError ? "textarea-error" : "",
    hasSuccess ? "textarea-success" : "",
    glass ? "glass-textarea" : "",
  ]
    .filter(Boolean)
    .join(" ");

  function handleInput(e) {
    value = e.target.value;

    if (autoResize) {
      resize();
    }
  }

  function resize() {
    if (textareaElement) {
      textareaElement.style.height = "auto";
      textareaElement.style.height = textareaElement.scrollHeight + "px";
    }
  }

  // Auto-resize on mount
  $: if (autoResize && textareaElement && value) {
    resize();
  }
</script>

<div class="form-control">
  {#if label}
    <label class="label">
      <span class="label-text">
        {label}
        {#if required}<span class="text-error">*</span>{/if}
      </span>
      {#if showCharCount && maxlength}
        <span class="label-text-alt" class:text-error={charCount > maxlength}>
          {charCount}/{maxlength}
        </span>
      {/if}
    </label>
  {/if}

  <textarea
    bind:this={textareaElement}
    class={computedClass}
    {placeholder}
    {disabled}
    {required}
    {rows}
    maxlength={maxlength || undefined}
    bind:value
    on:input={handleInput}
    on:change
    on:focus
    on:blur
    on:keydown
    on:keyup
    {...$$restProps}
  ></textarea>

  {#if error}
    <label class="label">
      <span class="label-text-alt text-error">
        <i class="bi bi-exclamation-circle-fill mr-1"></i>
        {error}
      </span>
    </label>
  {:else if success}
    <label class="label">
      <span class="label-text-alt text-success">
        <i class="bi bi-check-circle-fill mr-1"></i>
        {success}
      </span>
    </label>
  {:else if helpText}
    <label class="label">
      <span class="label-text-alt">{helpText}</span>
    </label>
  {/if}
</div>

<style>
  .glass-textarea {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
  }

  textarea {
    transition: all var(--duration-200) var(--ease-standard);
    resize: vertical;
  }

  textarea:focus {
    transform: translateY(-2px);
    box-shadow: var(--shadow-lg);
  }

  textarea.textarea-lg {
    min-height: 120px;
  }

  textarea.textarea-md {
    min-height: 80px;
  }

  textarea.textarea-sm {
    min-height: 60px;
  }

  textarea.textarea-xs {
    min-height: 48px;
  }

  /* Scrollbar Styling */
  textarea::-webkit-scrollbar {
    width: 8px;
  }

  textarea::-webkit-scrollbar-track {
    background: var(--color-surface-container-low);
    border-radius: var(--radius-full);
  }

  textarea::-webkit-scrollbar-thumb {
    background: var(--color-outline);
    border-radius: var(--radius-full);
  }

  textarea::-webkit-scrollbar-thumb:hover {
    background: var(--color-primary);
  }
</style>
