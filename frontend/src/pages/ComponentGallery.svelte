<script>
  /**
   * Component Gallery - Showcase for all DaisyUI components
   * Created: October 24, 2025
   */
  import Button from "../components/ui/Button.svelte";
  import Input from "../components/ui/Input.svelte";
  import Modal from "../components/ui/Modal.svelte";
  import Badge from "../components/ui/Badge.svelte";
  import Avatar from "../components/ui/Avatar.svelte";
  import ProgressBar from "../components/ui/ProgressBar.svelte";
  import Chip from "../components/ui/Chip.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";

  let testValue = "";
  let emailValue = "";
  let passwordValue = "";
  let searchValue = "";
  let errorValue = "";

  
  let successValue = "john@example.com";

  // Modal states
  let showBasicModal = false;
  let showLargeModal = false;
  let showConfirmModal = false;

  // Select options
  let selectedCountry = "";
  let selectedLanguages = [];
  const countries = [
    { value: "us", label: "United States" },
    { value: "uk", label: "United Kingdom" },
    { value: "de", label: "Germany" },
    { value: "fr", label: "France" },
    { value: "es", label: "Spain" },
    { value: "it", label: "Italy" },
  ];

  // Textarea
  let messageValue = "";
  let bioValue = "";

  // Checkboxes
  let agreeTerms = false;
  let receiveNewsletter = true;
  let indeterminateState = false;

  function handleClick() {
    alert("Button clicked!");
  }

  function handleAsyncAction() {
    return new Promise((resolve) => {
      setTimeout(() => {
        alert("Async action completed!");
        resolve();
      }, 2000);
    });
  }
</script>

<PageHeader
  title="Component Gallery"
  description="Showcase of all DaisyUI-based components"
  icon="palette"
  gradient="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
/>

<div class="gallery-container">
  <!-- Buttons Section -->
  <section class="gallery-section">
    <h2 class="section-title">Buttons</h2>

    <div class="subsection">
      <h3>Variants</h3>
      <div class="component-row">
        <Button>Default</Button>
        <Button variant="primary">Primary</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="accent">Accent</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="outline">Outline</Button>
        <Button variant="error">Error</Button>
        <Button variant="success">Success</Button>
        <Button variant="link">Link</Button>
      </div>
    </div>

    <div class="subsection">
      <h3>Sizes</h3>
      <div class="component-row">
        <Button size="xs">Extra Small</Button>
        <Button size="sm">Small</Button>
        <Button size="md">Medium</Button>
        <Button size="lg">Large</Button>
      </div>
    </div>

    <div class="subsection">
      <h3>With Icons</h3>
      <div class="component-row">
        <Button variant="primary" iconLeft="upload">Upload</Button>
        <Button variant="secondary" iconRight="download">Download</Button>
        <Button variant="error" iconLeft="trash">Delete</Button>
        <Button variant="success" iconLeft="plus-lg">Create</Button>
      </div>
    </div>

    <div class="subsection">
      <h3>States</h3>
      <div class="component-row">
        <Button variant="primary" loading>Loading...</Button>
        <Button variant="secondary" disabled>Disabled</Button>
        <Button variant="accent" glass>Glass Effect</Button>
      </div>
    </div>

    <div class="subsection">
      <h3>Full Width</h3>
      <Button variant="primary" fullWidth iconLeft="save">Save Changes</Button>
    </div>
  </section>

  <!-- Inputs Section -->
  <section class="gallery-section">
    <h2 class="section-title">Inputs</h2>

    <div class="subsection">
      <h3>Basic Inputs</h3>
      <div class="input-grid">
        <InputV2
          label="Name"
          placeholder="Enter your name"
          bind:value={testValue}
          helpText="This is a help text"
        />

        <InputV2
          label="Email"
          type="email"
          placeholder="you@example.com"
          bind:value={emailValue}
          iconLeft="envelope"
          required
        />

        <InputV2
          label="Password"
          type="password"
          placeholder="••••••••"
          bind:value={passwordValue}
          iconLeft="lock"
          required
        />

        <InputV2
          label="Search"
          placeholder="Search files..."
          bind:value={searchValue}
          iconLeft="search"
          iconRight="x-lg"
        />
      </div>
    </div>

    <div class="subsection">
      <h3>Sizes</h3>
      <div class="input-grid">
        <InputV2
          label="Extra Small"
          size="xs"
          placeholder="Extra small input"
        />
        <InputV2 label="Small" size="sm" placeholder="Small input" />
        <InputV2 label="Medium" size="md" placeholder="Medium input" />
        <InputV2 label="Large" size="lg" placeholder="Large input" />
      </div>
    </div>

    <div class="subsection">
      <h3>Validation States</h3>
      <div class="input-grid">
        <InputV2
          label="With Error"
          bind:value={errorValue}
          error="This field is required"
          iconLeft="exclamation-triangle"
        />

        <InputV2
          label="With Success"
          bind:value={successValue}
          success="Email is valid"
          iconLeft="check-circle"
        />

        <InputV2
          label="With Character Count"
          placeholder="Type something..."
          maxlength={50}
          showCharCount
        />

        <InputV2 label="Disabled" placeholder="Disabled input" disabled />
      </div>
    </div>

    <div class="subsection">
      <h3>Glass Effect</h3>
      <InputV2
        label="Glass Input"
        placeholder="Glass effect input"
        glass
        iconLeft="sparkles"
      />
    </div>
  </section>

  <!-- Select Section -->
  <section class="gallery-section">
    <h2 class="section-title">Select Dropdowns</h2>

    <div class="subsection">
      <h3>Basic Select</h3>
      <div class="input-grid">
        <SelectV2
          bind:value={selectedCountry}
          options={countries}
          label="Country"
          placeholder="Select a country"
        />

        <SelectV2
          bind:value={selectedCountry}
          options={countries}
          label="Searchable Select"
          placeholder="Search countries..."
          searchable
        />

        <SelectV2
          bind:value={selectedLanguages}
          options={[
            { value: "en", label: "English" },
            { value: "de", label: "German" },
            { value: "fr", label: "French" },
            { value: "es", label: "Spanish" },
          ]}
          label="Multiple Select"
          placeholder="Select languages"
          multiple
          searchable
        />
      </div>
    </div>
  </section>

  <!-- Textarea Section -->
  <section class="gallery-section">
    <h2 class="section-title">Textareas</h2>

    <div class="subsection">
      <h3>Textarea Variants</h3>
      <div class="input-grid">
        <textarea
          bind:value={messageValue}
          label="Message"
          placeholder="Type your message..."
          helpText="Maximum 500 characters"
          maxlength={500}
          showCharCount
        />

        <textarea
          bind:value={bioValue}
          label="Bio (Auto-resize)"
          placeholder="Tell us about yourself..."
          autoResize
          rows={2}
        />

        <textarea
          label="Large Textarea"
          placeholder="Larger textarea..."
          size="lg"
        />
      </div>
    </div>
  </section>

  <!-- Checkbox Section -->
  <section class="gallery-section">
    <h2 class="section-title">Checkboxes</h2>

    <div class="subsection">
      <h3>Checkbox Variants</h3>
      <div class="space-y-4">
        <input
          bind:checked={agreeTerms}
          label="I agree to the terms and conditions"
          description="Please read our privacy policy and terms of service"
        />

        <input
          bind:checked={receiveNewsletter}
          label="Receive newsletter"
          description="Get weekly updates about new features"
          color="secondary"
        />

        <input
          bind:checked={indeterminateState}
          indeterminate={true}
          label="Indeterminate state"
          description="This checkbox is in an indeterminate state"
          color="accent"
        />

        <div class="flex gap-4">
          <input checked={true} label="Extra Small" size="xs" />
          <input checked={true} label="Small" size="sm" />
          <input checked={true} label="Medium" size="md" />
          <input checked={true} label="Large" size="lg" />
        </div>

        <div class="flex gap-4">
          <input checked={true} label="Primary" color="primary" />
          <input checked={true} label="Secondary" color="secondary" />
          <input checked={true} label="Accent" color="accent" />
          <input checked={true} label="Success" color="success" />
          <input checked={true} label="Error" color="error" />
        </div>
      </div>
    </div>
  </section>

  <!-- Cards Section -->
  <section class="gallery-section">
    <h2 class="section-title">Cards</h2>

    <div class="subsection">
      <h3>Basic Cards</h3>
      <div class="card-grid">
        <CardV2>
          <svelte:fragment slot="title">Default Card</svelte:fragment>
          <p>This is a basic card with title and content.</p>
          <svelte:fragment slot="actions">
            <Button variant="ghost" size="sm">Cancel</Button>
            <Button variant="primary" size="sm">Action</Button>
          </svelte:fragment>
        </CardV2>

        <CardV2 variant="bordered">
          <svelte:fragment slot="title">Bordered Card</svelte:fragment>
          <p>This card has a border for better separation.</p>
          <svelte:fragment slot="actions">
            <Button variant="outline" size="sm" iconLeft="heart">Like</Button>
          </svelte:fragment>
        </CardV2>

        <CardV2 variant="compact">
          <svelte:fragment slot="title">Compact Card</svelte:fragment>
          <p>Smaller padding for dense layouts.</p>
        </CardV2>
      </div>
    </div>

    <div class="subsection">
      <h3>Interactive Cards</h3>
      <div class="card-grid">
        <CardV2 hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-star-fill text-warning"></i>
            Hoverable Card
          </svelte:fragment>
          <p>Hover over this card to see the lift effect.</p>
        </CardV2>

        <CardV2 clickable hoverable on:cardclick={() => alert("Card clicked!")}>
          <svelte:fragment slot="title">
            <i class="bi bi-hand-index-thumb text-primary"></i>
            Clickable Card
          </svelte:fragment>
          <p>This card is fully clickable and interactive.</p>
        </CardV2>

        <CardV2 glass hoverable>
          <svelte:fragment slot="title">
            <i class="bi bi-droplet text-info"></i>
            Glass Card
          </svelte:fragment>
          <p>Beautiful glassmorphism effect with blur.</p>
        </CardV2>
      </div>
    </div>

    <div class="subsection">
      <h3>Cards with Images</h3>
      <div class="card-grid">
        <CardV2 imageTop="https://picsum.photos/400/200?random=1" hoverable>
          <svelte:fragment slot="title">Image Top</svelte:fragment>
          <p>Card with image at the top.</p>
          <svelte:fragment slot="actions">
            <Button variant="primary" size="sm" iconLeft="eye">View</Button>
          </svelte:fragment>
        </CardV2>

        <CardV2
          variant="side"
          imageSide="https://picsum.photos/200/200?random=2"
        >
          <svelte:fragment slot="title">Side Image</svelte:fragment>
          <p>Card with image on the side - great for list items.</p>
          <svelte:fragment slot="actions">
            <Button variant="ghost" size="sm">Details</Button>
          </svelte:fragment>
        </CardV2>
      </div>
    </div>
  </section>

  <!-- Modals Section -->
  <section class="gallery-section">
    <h2 class="section-title">Modals</h2>

    <div class="subsection">
      <h3>Modal Variants</h3>
      <div class="component-row">
        <Button variant="primary" on:click={() => (showBasicModal = true)}>
          Open Basic Modal
        </Button>
        <Button variant="secondary" on:click={() => (showLargeModal = true)}>
          Open Large Modal
        </Button>
        <Button variant="accent" on:click={() => (showConfirmModal = true)}>
          Open Confirm Dialog
        </Button>
      </div>
    </div>
  </section>

  <!-- Theme Toggle Section -->
  <section class="gallery-section">
    <h2 class="section-title">Theme Switching</h2>
    <div class="subsection">
      <p class="mb-4">Click buttons to switch themes:</p>
      <div class="component-row">
        <Button
          variant="primary"
          on:click={() =>
            document.documentElement.setAttribute("data-theme", "syncspace")}
        >
          Light Theme
        </Button>
        <Button
          variant="secondary"
          on:click={() =>
            document.documentElement.setAttribute(
              "data-theme",
              "syncspace-dark"
            )}
        >
          Dark Theme
        </Button>
      </div>
    </div>
  </section>

  <!-- Code Examples Section -->
  <section class="gallery-section">
    <h2 class="section-title">Usage Examples</h2>

    <div class="subsection">
      <h3>Button Example</h3>
      <pre><code
          >{`<Button variant="primary" iconLeft="upload">
  Upload File
</Button>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Input Example</h3>
      <pre><code
          >{`<InputV2 
  label="Email" 
  type="email" 
  placeholder="you@example.com"
  iconLeft="envelope"
  bind:value={emailValue}
  required
/>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Card Example</h3>
      <pre><code
          >{`<CardV2 hoverable>
  <svelte:fragment slot="title">Card Title</svelte:fragment>
  <p>Card content goes here</p>
  <svelte:fragment slot="actions">
    <Button variant="primary">Action</Button>
  </svelte:fragment>
</CardV2>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Modal Example</h3>
      <pre><code
          >{`<ModalV2 bind:open={showModal} title="Confirm">
  <p>Are you sure?</p>
  <svelte:fragment slot="actions">
    <Button variant="ghost">Cancel</Button>
    <Button variant="primary">Confirm</Button>
  </svelte:fragment>
</ModalV2>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Select Example</h3>
      <pre><code
          >{`<SelectV2 
  bind:value={selected}
  options={[
    { value: "1", label: "Option 1" },
    { value: "2", label: "Option 2" }
  ]}
  searchable
/>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Textarea Example</h3>
      <pre><code
          >{`<textarea 
  bind:value={message}
  label="Message"
  maxlength={500}
  showCharCount
  autoResize
/>`}</code
        ></pre>
    </div>

    <div class="subsection">
      <h3>Checkbox Example</h3>
      <pre><code
          >{`<input 
  bind:checked={agreed}
  label="I agree to terms"
  description="Read our policy"
/>`}</code
        ></pre>
    </div>
  </section>
</div>

<!-- Modal Examples -->
<ModalV2 bind:open={showBasicModal} title="Basic Modal">
  <p class="mb-4">
    This is a basic modal with a title and content. It can be closed by clicking
    the X button, pressing ESC, or clicking the backdrop.
  </p>
  <p>
    Modals automatically trap focus and restore it when closed. They also
    prevent body scrolling while open.
  </p>
  <svelte:fragment slot="actions">
    <Button variant="ghost" on:click={() => (showBasicModal = false)}>
      Close
    </Button>
    <Button variant="primary" on:click={() => (showBasicModal = false)}>
      Got it!
    </Button>
  </svelte:fragment>
</ModalV2>

<ModalV2 bind:open={showLargeModal} size="lg" title="Large Modal">
  <div class="space-y-4">
    <p>
      This is a larger modal that can contain more content. Perfect for forms,
      detailed information, or complex interactions.
    </p>

    <CardV2>
      <svelte:fragment slot="title">Card Inside Modal</svelte:fragment>
      <p>You can nest components inside modals!</p>
    </CardV2>

    <InputV2
      label="Email"
      type="email"
      placeholder="Enter your email"
      iconLeft="envelope"
    />

    <InputV2
      label="Message"
      placeholder="Type your message..."
      iconLeft="chat"
    />
  </div>

  <svelte:fragment slot="actions">
    <Button variant="ghost" on:click={() => (showLargeModal = false)}>
      Cancel
    </Button>
    <Button
      variant="primary"
      iconLeft="send"
      on:click={() => {
        alert("Form submitted!");
        showLargeModal = false;
      }}
    >
      Submit
    </Button>
  </svelte:fragment>
</ModalV2>

<ModalV2 bind:open={showConfirmModal} size="sm" title="Confirm Action">
  <div class="text-center">
    <div class="text-6xl mb-4">⚠️</div>
    <p class="text-lg font-semibold mb-2">Are you sure?</p>
    <p class="text-sm opacity-70">
      This action cannot be undone. Please confirm that you want to proceed.
    </p>
  </div>

  <svelte:fragment slot="actions">
    <Button
      variant="ghost"
      fullWidth
      on:click={() => (showConfirmModal = false)}
    >
      Cancel
    </Button>
    <Button
      variant="error"
      fullWidth
      iconLeft="trash"
      on:click={() => {
        alert("Action confirmed!");
        showConfirmModal = false;
      }}
    >
      Delete
    </Button>
  </svelte:fragment>
</ModalV2>

<style>
  .gallery-container {
    max-width: 1400px;
    margin: 0 auto;
    padding: var(--spacing-6);
  }

  .gallery-section {
    margin-bottom: var(--spacing-12);
    padding: var(--spacing-6);
    background: var(--color-surface-container);
    border-radius: var(--radius-2xl);
    box-shadow: var(--shadow-md);
  }

  .section-title {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-on-surface);
    margin-bottom: var(--spacing-6);
    background: var(--color-brand-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subsection {
    margin-bottom: var(--spacing-8);
  }

  .subsection h3 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-on-surface);
    margin-bottom: var(--spacing-4);
  }

  .component-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--spacing-4);
    align-items: center;
  }

  .input-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: var(--spacing-4);
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: var(--spacing-6);
  }

  .space-y-4 > * + * {
    margin-top: var(--spacing-4);
  }

  .text-center {
    text-align: center;
  }

  .text-6xl {
    font-size: 4rem;
  }

  .text-lg {
    font-size: var(--font-size-lg);
  }

  .text-sm {
    font-size: var(--font-size-sm);
  }

  .font-semibold {
    font-weight: var(--font-weight-semibold);
  }

  .opacity-70 {
    opacity: 0.7;
  }

  .mb-2 {
    margin-bottom: var(--spacing-2);
  }

  pre {
    background: var(--color-surface-container-low);
    padding: var(--spacing-4);
    border-radius: var(--radius-lg);
    overflow-x: auto;
    margin-top: var(--spacing-2);
  }

  code {
    font-family: "Fira Code", "Courier New", monospace;
    font-size: var(--font-size-sm);
    color: var(--color-primary);
  }

  .mb-4 {
    margin-bottom: var(--spacing-4);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .gallery-container {
      padding: var(--spacing-4);
    }

    .gallery-section {
      padding: var(--spacing-4);
    }

    .component-row {
      flex-direction: column;
      align-items: stretch;
    }

    .input-grid {
      grid-template-columns: 1fr;
    }

    .card-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
