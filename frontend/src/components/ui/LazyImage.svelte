<script>
  import { onMount } from "svelte";

  let {
    src = "",
    alt = "",
    placeholder = "",
    width = null,
    height = null,
    class: className = "",
    loading = "eager",
  } = $props();

  let isLoaded = $state(false);
  let imgRef = $state(null);
  let observer = $state(null);

  onMount(() => {
    if (loading === "lazy" && imgRef && "IntersectionObserver" in window) {
      observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              imgRef.src = src;
              imgRef.loading = "eager";
              observer.unobserve(imgRef);
            }
          });
        },
        { rootMargin: "50px" }
      );

      observer.observe(imgRef);
    }

    return () => {
      if (observer) {
        observer.disconnect();
      }
    };
  });
</script>

<img
  bind:this={imgRef}
  src={loading === "eager" ? src : placeholder || "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='1' height='1'%3E%3C/svg%3E"}
  alt={alt}
  {width}
  {height}
  class="{className} transition-opacity duration-300"
  class:opacity-0={!isLoaded && loading === "lazy"}
  loading={loading === "lazy" ? "lazy" : "eager"}
  onload={() => (isLoaded = true)}
  onerror={() => console.error(`Failed to load image: ${src}`)}
/>

<style>
  img {
    display: block;
  }
</style>
