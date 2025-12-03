<script>
  import { onMount, onDestroy } from "svelte";

  let { file, previewUrl } = $props();

  let videoElement = $state(null);
  let isPlaying = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1);
  let isMuted = $state(false);
  let isFullscreen = $state(false);
  let playbackRate = $state(1);
  let showControls = $state(true);
  let loading = $state(true);
  let error = $state(null);
  let buffered = $state(0);
  let controlsTimeout = $state(null);

  onMount(() => {
    if (videoElement) {
      videoElement.addEventListener("loadedmetadata", handleLoadedMetadata);
      videoElement.addEventListener("timeupdate", handleTimeUpdate);
      videoElement.addEventListener("progress", handleProgress);
      videoElement.addEventListener("play", () => (isPlaying = true));
      videoElement.addEventListener("pause", () => (isPlaying = false));
      videoElement.addEventListener("ended", () => {
        isPlaying = false;
        currentTime = 0;
      });
      videoElement.addEventListener("error", handleError);
      videoElement.addEventListener("waiting", () => (loading = true));
      videoElement.addEventListener("canplay", () => (loading = false));
    }
  });

  onDestroy(() => {
    if (controlsTimeout) clearTimeout(controlsTimeout);
  });

  function handleLoadedMetadata() {
    duration = videoElement.duration;
    loading = false;
  }

  function handleTimeUpdate() {
    currentTime = videoElement.currentTime;
  }

  function handleProgress() {
    if (videoElement.buffered.length > 0) {
      buffered =
        (videoElement.buffered.end(videoElement.buffered.length - 1) /
          duration) *
        100;
    }
  }

  function handleError(e) {
    loading = false;
    error = "Failed to load video";
    console.error("Video error:", e);
  }

  function togglePlay() {
    if (videoElement.paused) {
      videoElement.play();
    } else {
      videoElement.pause();
    }
  }

  function seek(e) {
    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    videoElement.currentTime = percent * duration;
  }

  function handleVolumeChange(e) {
    volume = parseFloat(e.target.value);
    videoElement.volume = volume;
    isMuted = volume === 0;
  }

  function toggleMute() {
    isMuted = !isMuted;
    videoElement.muted = isMuted;
  }

  function skip(seconds) {
    videoElement.currentTime = Math.max(
      0,
      Math.min(duration, videoElement.currentTime + seconds)
    );
  }

  function setPlaybackRate(rate) {
    playbackRate = rate;
    videoElement.playbackRate = rate;
  }

  function toggleFullscreen() {
    const container = videoElement.parentElement;
    if (!document.fullscreenElement) {
      container.requestFullscreen();
      isFullscreen = true;
    } else {
      document.exitFullscreen();
      isFullscreen = false;
    }
  }

  function handleMouseMove() {
    showControls = true;
    if (controlsTimeout) clearTimeout(controlsTimeout);
    controlsTimeout = setTimeout(() => {
      if (isPlaying) showControls = false;
    }, 3000);
  }

  function handleKeydown(e) {
    switch (e.key) {
      case " ":
      case "k":
        e.preventDefault();
        togglePlay();
        break;
      case "ArrowLeft":
        skip(-5);
        break;
      case "ArrowRight":
        skip(5);
        break;
      case "ArrowUp":
        volume = Math.min(1, volume + 0.1);
        videoElement.volume = volume;
        break;
      case "ArrowDown":
        volume = Math.max(0, volume - 0.1);
        videoElement.volume = volume;
        break;
      case "m":
        toggleMute();
        break;
      case "f":
        toggleFullscreen();
        break;
    }
  }

  function formatTime(seconds) {
    if (!seconds || isNaN(seconds)) return "0:00";
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    if (hours > 0) {
      return `${hours}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
    }
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  function getVideoType() {
    const ext = file?.name?.split(".").pop()?.toLowerCase();
    const types = {
      mp4: "video/mp4",
      webm: "video/webm",
      ogg: "video/ogg",
      mov: "video/quicktime",
      avi: "video/x-msvideo",
      mkv: "video/x-matroska",
    };
    return types[ext] || "video/mp4";
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="video-player-container relative bg-black rounded-xl overflow-hidden shadow-2xl"
  onmousemove={handleMouseMove}
  onmouseleave={() => {
    if (isPlaying) showControls = false;
  }}
  role="region"
  aria-label="Video Player"
  tabindex="0"
  onkeydown={handleKeydown}
>
  {#if error}
    <div class="flex flex-col items-center justify-center h-96 text-white">
      <i
        class="bi bi-exclamation-triangle text-6xl text-error mb-4"
        aria-hidden="true"
      ></i>
      <p class="text-lg">{error}</p>
    </div>
  {:else}
    <!-- Video Element -->
    <video
      bind:this={videoElement}
      class="w-full max-h-[70vh] cursor-pointer"
      onclick={togglePlay}
      preload="metadata"
    >
      <source src={previewUrl} type={getVideoType()} />
      <track kind="captions" />
      Your browser does not support video playback.
    </video>

    <!-- Loading overlay -->
    {#if loading}
      <div
        class="absolute inset-0 flex items-center justify-center bg-black/50"
      >
        <span class="loading loading-spinner loading-lg text-white"></span>
      </div>
    {/if}

    <!-- Big play button overlay -->
    {#if !isPlaying && !loading}
      <div
        class="absolute inset-0 flex items-center justify-center cursor-pointer"
        onclick={togglePlay}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === " " && togglePlay()}
      >
        <div
          class="w-20 h-20 rounded-full bg-primary/80 flex items-center justify-center shadow-lg hover:bg-primary transition-colors"
        >
          <i class="bi bi-play-fill text-4xl text-white ml-1" aria-hidden="true"
          ></i>
        </div>
      </div>
    {/if}

    <!-- Controls overlay -->
    <div
      class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent p-4 transition-opacity duration-300"
      class:opacity-0={!showControls}
      class:opacity-100={showControls}
    >
      <!-- Progress bar -->
      <div
        class="relative h-1.5 bg-white/20 rounded-full mb-4 cursor-pointer group"
        onclick={seek}
        role="slider"
        onkeydown={(e) => {
          if (e.key === "ArrowLeft" || e.key === "ArrowRight") handleSeek(e);
        }}
        aria-label="Video progress"
        aria-valuemin="0"
        aria-valuemax={duration}
        aria-valuenow={currentTime}
        tabindex="0"
      >
        <!-- Buffered -->
        <div
          class="absolute h-full bg-white/30 rounded-full transition-all"
          style="width: {buffered}%"
        ></div>
        <!-- Progress -->
        <div
          class="absolute h-full bg-primary rounded-full transition-all"
          style="width: {duration ? (currentTime / duration) * 100 : 0}%"
        ></div>
        <!-- Hover indicator -->
        <div
          class="absolute top-1/2 -translate-y-1/2 w-4 h-4 bg-primary rounded-full shadow-lg opacity-0 group-hover:opacity-100 transition-opacity"
          style="left: calc({duration
            ? (currentTime / duration) * 100
            : 0}% - 8px)"
        ></div>
      </div>

      <!-- Controls row -->
      <div class="flex items-center justify-between text-white">
        <div class="flex items-center gap-3">
          <!-- Play/Pause -->
          <button
            aria-label={isPlaying ? "Pause" : "Play"}
            onclick={togglePlay}
            class="btn btn-circle btn-sm btn-ghost text-white hover:bg-white/20"
            ><i class="bi" aria-hidden="true"></i><span class="sr-only"
              >Toggle play</span
            ></button
          >

          <!-- Skip buttons -->
          <button
            onclick={() => skip(-10)}
            class="btn btn-circle btn-sm btn-ghost text-white hover:bg-white/20"
            title="Back 10s"
          >
            <i class="bi bi-skip-backward text-lg" aria-hidden="true"></i>
          </button>
          <button
            onclick={() => skip(10)}
            class="btn btn-circle btn-sm btn-ghost text-white hover:bg-white/20"
            title="Forward 10s"
          >
            <i class="bi bi-skip-forward text-lg" aria-hidden="true"></i>
          </button>

          <!-- Volume -->
          <div class="flex items-center gap-2 group">
            <button
              onclick={toggleMute}
              class="btn btn-circle btn-sm btn-ghost text-white hover:bg-white/20"
              ><i class="bi" aria-hidden="true"></i><span class="sr-only"
                >Toggle mute</span
              ></button
            >
            <input
              type="range"
              min="0"
              max="1"
              step="0.05"
              value={volume}
              onchange={handleVolumeChange}
              class="w-0 group-hover:w-20 transition-all duration-200 accent-primary"
            />
          </div>

          <!-- Time -->
          <span class="text-sm font-mono">
            {formatTime(currentTime)} / {formatTime(duration)}
          </span>
        </div>

        <div class="flex items-center gap-2">
          <!-- Playback speed -->
          <div class="dropdown dropdown-top dropdown-end">
            <button
              tabindex="0"
              class="btn btn-sm btn-ghost text-white hover:bg-white/20"
            >
              {playbackRate}x
            </button>
            <ul
              class="dropdown-content menu bg-base-300 rounded-box shadow-xl mb-2 z-10"
            >
              {#each [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2] as rate}
                <li>
                  <button
                    onclick={() => setPlaybackRate(rate)}
                    class:font-bold={playbackRate === rate}
                  >
                    {rate}x
                  </button>
                </li>
              {/each}
            </ul>
          </div>

          <!-- Fullscreen -->
          <button
            onclick={toggleFullscreen}
            class="btn btn-circle btn-sm btn-ghost text-white hover:bg-white/20"
            title="Fullscreen (F)"
          >
            <i
              class="bi {isFullscreen
                ? 'bi-fullscreen-exit'
                : 'bi-fullscreen'} text-lg"
            ></i>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- Keyboard shortcuts info -->
<div
  class="mt-4 text-xs text-base-content/50 flex flex-wrap gap-3 justify-center"
>
  <span><kbd class="kbd kbd-xs">Space</kbd> Play/Pause</span>
  <span
    ><kbd class="kbd kbd-xs">←</kbd><kbd class="kbd kbd-xs">→</kbd> Skip 5s</span
  >
  <span
    ><kbd class="kbd kbd-xs">↑</kbd><kbd class="kbd kbd-xs">↓</kbd> Volume</span
  >
  <span><kbd class="kbd kbd-xs">M</kbd> Mute</span>
  <span><kbd class="kbd kbd-xs">F</kbd> Fullscreen</span>
</div>

<style>
  .video-player-container:focus {
    outline: 2px solid oklch(var(--p));
    outline-offset: 2px;
  }

  input[type="range"] {
    height: 4px;
    border-radius: 2px;
    appearance: none;
    background: rgba(255, 255, 255, 0.3);
  }

  input[type="range"]::-webkit-slider-thumb {
    appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: white;
    cursor: pointer;
  }
</style>
