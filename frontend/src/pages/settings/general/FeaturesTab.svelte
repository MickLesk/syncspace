<script>
  import { favoritesEnabled, currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<div class="settings-grid">
  <!-- Features Card -->
  <div class="card">
    <div class="card-header">
      <div class="card-icon rose"><i class="bi bi-toggles"></i></div>
      <div class="card-title">
        <h3>{tr("features")}</h3>
        <p>{tr("customizeFeatures")}</p>
      </div>
    </div>
    <div class="toggle-list">
      <div class="toggle-item">
        <div class="toggle-info">
          <span class="toggle-label">{tr("enableFavoritesSystem")}</span>
          <span class="toggle-desc">{tr("showFavoritesInSidebar")}</span>
        </div>
        <button
          class="toggle-switch"
          class:active={$favoritesEnabled}
          onclick={() => favoritesEnabled.set(!$favoritesEnabled)}
          role="switch"
          aria-checked={$favoritesEnabled}
          aria-label="Toggle favorites system"
        >
          <span class="toggle-knob"></span>
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 0.875rem;
    margin-bottom: 1.25rem;
  }
  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .card-icon.rose {
    background: #ffe4e6;
    color: #e11d48;
  }
  :global(.dark) .card-icon.rose {
    background: rgba(225, 29, 72, 0.2);
    color: #fb7185;
  }

  .card-title {
    flex: 1;
  }
  .card-title h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }
  .card-title p {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .card-title h3 {
    color: #f9fafb;
  }
  :global(.dark) .card-title p {
    color: #9ca3af;
  }

  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .toggle-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .toggle-item {
    background: #374151;
  }
  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .toggle-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .toggle-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .toggle-label {
    color: #f9fafb;
  }
  :global(.dark) .toggle-desc {
    color: #9ca3af;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    background: #d1d5db;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
  }
  .toggle-switch.active {
    background: #22c55e;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s;
  }
  .toggle-switch.active .toggle-knob {
    transform: translateX(20px);
  }
  :global(.dark) .toggle-switch {
    background: #4b5563;
  }
</style>
