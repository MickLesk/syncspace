<script>
  import { onMount, onDestroy } from "svelte";
  import WaveSurfer from "wavesurfer.js";

  let { file, previewUrl } = $props();

  let waveformContainer = $state(null);
  let wavesurfer = $state(null);
  let isPlaying = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(0.8);
  let loading = $state(true);
  let error = $state(null);
  let playbackRate = $state(1);

  onMount(async () => {
    if (!waveformContainer || !previewUrl) return;

    try {
      wavesurfer = WaveSurfer.create({
        container: waveformContainer,
        waveColor: "rgba(102, 126, 234, 0.5)",
        progressColor: "#667eea",
        cursorColor: "#764ba2",
        cursorWidth: 2,
        barWidth: 3,
        barGap: 2,
        barRadius: 3,
        responsive: true,
        height: 128,
        normalize: true,
        backend: "WebAudio",
      });

      wavesurfer.on("ready", () => {
        loading = false;
        duration = wavesurfer.getDuration();
        wavesurfer.setVolume(volume);
      });

      wavesurfer.on("audioprocess", () => {
        currentTime = wavesurfer.getCurrentTime();
      });

      wavesurfer.on("play", () => {
        isPlaying = true;
      });

      wavesurfer.on("pause", () => {
        isPlaying = false;
      });

      wavesurfer.on("finish", () => {
        isPlaying = false;
        currentTime = 0;
      });

      wavesurfer.on("error", (err) => {
        loading = false;
        error = err.message || "Failed to load audio";
        console.error("WaveSurfer error:", err);
      });

      await wavesurfer.load(previewUrl);
    } catch (err) {
      loading = false;
      error = err.message || "Failed to initialize audio player";
      console.error("Audio initialization error:", err);
    }
  });

  onDestroy(() => {
    if (wavesurfer) {
      wavesurfer.destroy();
    }
  });

  function togglePlay() {
    if (wavesurfer) {
      wavesurfer.playPause();
    }
  }

  function stop() {
    if (wavesurfer) {
      wavesurfer.stop();
      isPlaying = false;
      currentTime = 0;
    }
  }

  function skipBackward() {
    if (wavesurfer) {
      wavesurfer.skip(-10);
    }
  }

  function skipForward() {
    if (wavesurfer) {
      wavesurfer.skip(10);
    }
  }

  function handleVolumeChange(e) {
    volume = parseFloat(e.target.value);
    if (wavesurfer) {
      wavesurfer.setVolume(volume);
    }
  }

  function handleSpeedChange(speed) {
    playbackRate = speed;
    if (wavesurfer) {
      wavesurfer.setPlaybackRate(speed);
    }
  }

  function formatTime(seconds) {
    if (!seconds || isNaN(seconds)) return "0:00";
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  function getAudioIcon() {
    const ext = file?.name?.split(".").pop()?.toLowerCase();
    switch (ext) {
      case "mp3":
        return "bi-file-music";
      case "wav":
        return "bi-soundwave";
      case "flac":
        return "bi-disc";
      case "aac":
        return "bi-music-note-beamed";
      case "m4a":
        return "bi-music-note";
      default:
        return "bi-music-note-beamed";
    }
  }
</script>

<div
  class="audio-player bg-gradient-to-br from-base-200 to-base-300 rounded-xl p-6 shadow-lg"
>
  <!-- Header with file info -->
  <div class="flex items-center gap-4 mb-6">
    <div
      class="w-20 h-20 rounded-xl bg-gradient-to-br from-primary to-secondary flex items-center justify-center shadow-lg"
    >
      <i class="{getAudioIcon()} text-4xl text-white"></i>
    </div>
    <div class="flex-1 min-w-0">
      <h3 class="text-lg font-bold truncate">{file?.name || "Audio File"}</h3>
      <p class="text-sm text-base-content/60">
        {file?.size ? Math.round(file.size / 1024) : 0} KB • {file?.name
          ?.split(".")
          .pop()
          ?.toUpperCase()}
      </p>
    </div>
  </div>

  {#if loading}
    <div class="flex flex-col items-center justify-center py-8">
      <span class="loading loading-spinner loading-lg text-primary mb-4"></span>
      <p class="text-base-content/60">Loading waveform...</p>
    </div>
  {:else if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
    </div>
  {/if}

  <!-- Waveform -->
  <div
    bind:this={waveformContainer}
    class="waveform-container bg-base-100 rounded-xl p-4 mb-4 shadow-inner"
    class:hidden={loading || error}
  ></div>

  {#if !loading && !error}
    <!-- Time display -->
    <div class="flex justify-between text-sm text-base-content/60 mb-4 px-2">
      <span>{formatTime(currentTime)}</span>
      <span>{formatTime(duration)}</span>
    </div>

    <!-- Main controls -->
    <div class="flex items-center justify-center gap-4 mb-6">
      <button
        onclick={skipBackward}
        class="btn btn-circle btn-ghost btn-lg"
        title="Skip back 10s"
      >
        <i class="bi bi-skip-backward-fill text-2xl" aria-hidden="true"></i>
      </button>

      <button
        aria-label={isPlaying ? "Pause" : "Play"}
        onclick={togglePlay}
        class="btn btn-circle btn-primary btn-xl shadow-lg hover:shadow-xl transition-shadow"
        title={isPlaying ? "Pause" : "Play"}
        ><i class="bi" aria-hidden="true"></i><span class="sr-only"
          >Toggle play</span
        ></button
      >

      <button
        onclick={skipForward}
        class="btn btn-circle btn-ghost btn-lg"
        title="Skip forward 10s"
      >
        <i class="bi bi-skip-forward-fill text-2xl" aria-hidden="true"></i>
      </button>

      <button
        onclick={stop}
        class="btn btn-circle btn-ghost btn-lg"
        title="Stop"
      >
        <i class="bi bi-stop-fill text-2xl" aria-hidden="true"></i>
      </button>
    </div>

    <!-- Secondary controls -->
    <div class="flex items-center justify-between gap-4 px-2">
      <!-- Volume control -->
      <div class="flex items-center gap-2 flex-1 max-w-xs">
        <i
          class="bi {volume === 0
            ? 'bi-volume-mute'
            : volume < 0.5
              ? 'bi-volume-down'
              : 'bi-volume-up'} text-lg text-base-content/60"
        ></i>
        <input
          type="range"
          min="0"
          max="1"
          step="0.05"
          value={volume}
          onchange={handleVolumeChange}
          class="range range-primary range-xs flex-1"
        />
        <span class="text-xs text-base-content/60 w-8"
          >{Math.round(volume * 100)}%</span
        >
      </div>

      <!-- Playback speed -->
      <div class="flex items-center gap-1">
        <span class="text-xs text-base-content/60 mr-2">Speed:</span>
        {#each [0.5, 0.75, 1, 1.25, 1.5, 2] as speed}
          <button
            onclick={() => handleSpeedChange(speed)}
            class="btn btn-xs {playbackRate === speed
              ? 'btn-primary'
              : 'btn-ghost'}"
          >
            {speed}x
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .btn-xl {
    width: 4rem;
    height: 4rem;
    min-height: 4rem;
  }

  .waveform-container {
    min-height: 128px;
  }

  /* Custom styling for the waveform cursor */
  :global(.waveform-container wave) {
    overflow: hidden !important;
  }
</style>
