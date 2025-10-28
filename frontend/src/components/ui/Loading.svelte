<script>
  let loadingText = $state("Loading");
  let dots = $state("");

  $effect(() => {
    const interval = setInterval(() => {
      dots = dots.length >= 3 ? "" : dots + ".";
    }, 500);

    return () => clearInterval(interval);
  });
</script>

<!-- Loading Page with Glassmorphism -->
<div class="loading-container">
  <!-- Animated background blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Loading Card -->
  <div class="loading-card">
    <!-- Logo with Spin -->
    <div class="logo-wrapper">
      <div class="logo-circle-outer">
        <div class="logo-circle-inner">
          <i class="bi bi-cloud-fill text-5xl text-white"></i>
        </div>
      </div>
    </div>

    <!-- Brand -->
    <h1 class="brand-name">SyncSpace</h1>

    <!-- Loading Text -->
    <p class="loading-text">
      {loadingText}{dots}
    </p>

    <!-- Progress Bar -->
    <div class="progress-bar">
      <div class="progress-fill"></div>
    </div>

    <!-- Tagline -->
    <p class="tagline">Secure Cloud Storage Platform</p>
  </div>
</div>

<style>
  /* Main Container with Gradient Background */
  .loading-container {
    position: relative;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
    overflow: hidden;
  }

  /* Animated Background Blobs */
  .blob {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    animation: float 20s infinite ease-in-out;
  }

  .blob-1 {
    width: 500px;
    height: 500px;
    background: linear-gradient(135deg, #667eea, #764ba2);
    top: -10%;
    left: -10%;
    animation-delay: 0s;
  }

  .blob-2 {
    width: 400px;
    height: 400px;
    background: linear-gradient(135deg, #f093fb, #f5576c);
    top: 40%;
    right: -5%;
    animation-delay: 5s;
  }

  .blob-3 {
    width: 350px;
    height: 350px;
    background: linear-gradient(135deg, #4facfe, #00f2fe);
    bottom: -10%;
    left: 30%;
    animation-delay: 10s;
  }

  /* Glassmorphism Card */
  .loading-card {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 400px;
    padding: 3rem 2.5rem;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px) saturate(180%);
    border-radius: 24px;
    box-shadow:
      0 8px 32px 0 rgba(31, 38, 135, 0.15),
      0 0 0 1px rgba(255, 255, 255, 0.18) inset;
    animation: slideUp 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  /* Dark mode card */
  :global(.dark) .loading-card {
    background: rgba(17, 24, 39, 0.95);
    box-shadow:
      0 8px 32px 0 rgba(0, 0, 0, 0.3),
      0 0 0 1px rgba(255, 255, 255, 0.08) inset;
  }

  /* Logo with Nested Circles */
  .logo-wrapper {
    margin-bottom: 2rem;
  }

  .logo-circle-outer {
    width: 140px;
    height: 140px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea, #764ba2);
    border-radius: 50%;
    animation: rotate 3s linear infinite;
    box-shadow:
      0 10px 40px rgba(102, 126, 234, 0.4),
      0 0 0 8px rgba(102, 126, 234, 0.1),
      0 0 0 16px rgba(102, 126, 234, 0.05);
  }

  .logo-circle-inner {
    width: 120px;
    height: 120px;
    background: linear-gradient(135deg, #764ba2, #667eea);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: rotate-reverse 2s linear infinite;
  }

  /* Brand Name */
  .brand-name {
    font-size: 2.5rem;
    font-weight: 800;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #667eea, #764ba2);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  /* Loading Text */
  .loading-text {
    font-size: 1.125rem;
    color: rgb(107 114 128);
    margin-bottom: 1.5rem;
    font-weight: 500;
    min-width: 120px;
    text-align: center;
  }

  :global(.dark) .loading-text {
    color: rgb(156 163 175);
  }

  /* Progress Bar */
  .progress-bar {
    width: 100%;
    height: 6px;
    background: rgb(229 231 235);
    border-radius: 100px;
    overflow: hidden;
    margin-bottom: 2rem;
  }

  :global(.dark) .progress-bar {
    background: rgb(55 65 81);
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #667eea, #764ba2, #f093fb);
    background-size: 200% 100%;
    border-radius: 100px;
    animation: progress 2s ease-in-out infinite;
  }

  /* Tagline */
  .tagline {
    font-size: 0.875rem;
    color: rgb(156 163 175);
    font-weight: 500;
  }

  :global(.dark) .tagline {
    color: rgb(107 114 128);
  }

  /* Animations */
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(60px) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes float {
    0%,
    100% {
      transform: translate(0, 0) rotate(0deg);
    }
    33% {
      transform: translate(30px, -30px) rotate(120deg);
    }
    66% {
      transform: translate(-20px, 20px) rotate(240deg);
    }
  }

  @keyframes rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes rotate-reverse {
    from {
      transform: rotate(360deg);
    }
    to {
      transform: rotate(0deg);
    }
  }

  @keyframes progress {
    0% {
      width: 0%;
      background-position: 0% 0%;
    }
    50% {
      width: 70%;
      background-position: 100% 0%;
    }
    100% {
      width: 100%;
      background-position: 200% 0%;
    }
  }

  /* Responsive */
  @media (max-width: 640px) {
    .loading-card {
      padding: 2rem 1.5rem;
      margin: 1rem;
    }

    .logo-circle-outer {
      width: 110px;
      height: 110px;
    }

    .logo-circle-inner {
      width: 95px;
      height: 95px;
    }

    .logo-circle-inner i {
      font-size: 3rem !important;
    }

    .brand-name {
      font-size: 2rem;
    }
  }
</style>
