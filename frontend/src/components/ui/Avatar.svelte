<script>
  export let name = '';
  export let imageUrl = '';
  export let size = 'medium'; // small, medium, large, xlarge

  const sizes = {
    small: 32,
    medium: 40,
    large: 56,
    xlarge: 96
  };

  const fontSize = {
    small: 14,
    medium: 16,
    large: 20,
    xlarge: 32
  };

  function getInitials(name) {
    if (!name) return '?';
    const parts = name.trim().split(' ');
    if (parts.length >= 2) {
      return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
    }
    return name.substring(0, 2).toUpperCase();
  }

  function getAvatarColor(name) {
    const colors = [
      'rgb(255, 87, 34)',   // Deep Orange
      'rgb(103, 80, 164)',  // Deep Purple
      'rgb(0, 188, 212)',   // Cyan
      'rgb(76, 175, 80)',   // Green
      'rgb(255, 152, 0)',   // Orange
      'rgb(233, 30, 99)',   // Pink
      'rgb(63, 81, 181)',   // Indigo
      'rgb(0, 150, 136)',   // Teal
    ];
    
    let hash = 0;
    for (let i = 0; i < name.length; i++) {
      hash = name.charCodeAt(i) + ((hash << 5) - hash);
    }
    return colors[Math.abs(hash) % colors.length];
  }

  $: bgColor = imageUrl ? 'transparent' : getAvatarColor(name);
  $: initials = getInitials(name);
</script>

<div
  class="avatar {size}"
  style="--avatar-size: {sizes[size]}px; --avatar-font-size: {fontSize[size]}px; --avatar-bg: {bgColor};"
>
  {#if imageUrl}
    <img src={imageUrl} alt={name} class="avatar-image" />
  {:else}
    <span class="avatar-initials">{initials}</span>
  {/if}
</div>

<style>
  .avatar {
    width: var(--avatar-size);
    height: var(--avatar-size);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--avatar-bg);
    overflow: hidden;
    flex-shrink: 0;
    position: relative;
  }

  .avatar-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-initials {
    font-size: var(--avatar-font-size);
    font-weight: 600;
    color: white;
    user-select: none;
  }

  .small {
    font-size: 14px;
  }

  .medium {
    font-size: 16px;
  }

  .large {
    font-size: 20px;
  }

  .xlarge {
    font-size: 32px;
  }
</style>
