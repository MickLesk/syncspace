<script>
  import { onMount, onDestroy } from "svelte";
  import * as THREE from "three";

  let { file, previewUrl } = $props();

  let container = $state(null);
  let scene = $state(null);
  let camera = $state(null);
  let renderer = $state(null);
  let model = $state(null);
  let controls = $state({ rotating: true, zoom: 1 });
  let animationId = $state(null);
  let loading = $state(true);
  let error = $state(null);
  let mouseDown = $state(false);
  let lastMouseX = $state(0);
  let lastMouseY = $state(0);

  onMount(async () => {
    if (!container || !previewUrl) return;

    try {
      await initScene();
      await loadModel();
      animate();
    } catch (err) {
      console.error("3D Model loading error:", err);
      error = err.message || "Failed to load 3D model";
    }
  });

  onDestroy(() => {
    if (animationId) cancelAnimationFrame(animationId);
    if (renderer) {
      renderer.dispose();
      renderer.forceContextLoss();
    }
    if (scene) {
      scene.traverse((obj) => {
        if (obj.geometry) obj.geometry.dispose();
        if (obj.material) {
          if (Array.isArray(obj.material)) {
            obj.material.forEach((m) => m.dispose());
          } else {
            obj.material.dispose();
          }
        }
      });
    }
  });

  async function initScene() {
    const width = container.clientWidth;
    const height = 500;

    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1a2e);

    // Camera
    camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    camera.position.set(0, 2, 5);

    // Renderer
    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    container.appendChild(renderer.domElement);

    // Lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 7.5);
    directionalLight.castShadow = true;
    scene.add(directionalLight);

    const pointLight = new THREE.PointLight(0x667eea, 0.5);
    pointLight.position.set(-5, 5, -5);
    scene.add(pointLight);

    // Grid helper
    const gridHelper = new THREE.GridHelper(10, 10, 0x444444, 0x222222);
    scene.add(gridHelper);
  }

  async function loadModel() {
    loading = true;
    const ext = file?.name?.split(".").pop()?.toLowerCase();

    try {
      const response = await fetch(previewUrl);
      const text = await response.text();

      if (ext === "stl") {
        await loadSTL(text);
      } else if (ext === "obj") {
        await loadOBJ(text);
      } else {
        throw new Error(`Unsupported format: .${ext}`);
      }

      loading = false;
    } catch (err) {
      loading = false;
      throw err;
    }
  }

  async function loadSTL(text) {
    // Simple STL ASCII parser
    const geometry = new THREE.BufferGeometry();
    const vertices = [];
    const normals = [];

    // Check if binary or ASCII
    if (text.startsWith("solid")) {
      // ASCII STL
      const lines = text.split("\n");
      let currentNormal = null;

      for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith("facet normal")) {
          const parts = trimmed.split(/\s+/);
          currentNormal = [
            parseFloat(parts[2]),
            parseFloat(parts[3]),
            parseFloat(parts[4]),
          ];
        } else if (trimmed.startsWith("vertex")) {
          const parts = trimmed.split(/\s+/);
          vertices.push(
            parseFloat(parts[1]),
            parseFloat(parts[2]),
            parseFloat(parts[3])
          );
          if (currentNormal) {
            normals.push(...currentNormal);
          }
        }
      }
    }

    if (vertices.length === 0) {
      // Create a fallback cube if parsing fails
      const cubeGeom = new THREE.BoxGeometry(2, 2, 2);
      const material = new THREE.MeshStandardMaterial({
        color: 0x667eea,
        metalness: 0.3,
        roughness: 0.7,
      });
      model = new THREE.Mesh(cubeGeom, material);
      model.castShadow = true;
      model.receiveShadow = true;
      scene.add(model);
      return;
    }

    geometry.setAttribute(
      "position",
      new THREE.Float32BufferAttribute(vertices, 3)
    );
    if (normals.length > 0) {
      geometry.setAttribute(
        "normal",
        new THREE.Float32BufferAttribute(normals, 3)
      );
    } else {
      geometry.computeVertexNormals();
    }
    geometry.center();

    const material = new THREE.MeshStandardMaterial({
      color: 0x667eea,
      metalness: 0.3,
      roughness: 0.7,
      flatShading: true,
    });

    model = new THREE.Mesh(geometry, material);
    model.castShadow = true;
    model.receiveShadow = true;

    // Normalize scale
    const box = new THREE.Box3().setFromObject(model);
    const size = box.getSize(new THREE.Vector3());
    const maxDim = Math.max(size.x, size.y, size.z);
    model.scale.multiplyScalar(3 / maxDim);

    scene.add(model);
  }

  async function loadOBJ(text) {
    const geometry = new THREE.BufferGeometry();
    const vertices = [];
    const vertexData = [];

    const lines = text.split("\n");
    for (const line of lines) {
      const parts = line.trim().split(/\s+/);
      if (parts[0] === "v") {
        vertexData.push([
          parseFloat(parts[1]),
          parseFloat(parts[2]),
          parseFloat(parts[3]),
        ]);
      } else if (parts[0] === "f") {
        for (let i = 1; i <= 3 && i < parts.length; i++) {
          const idx = parseInt(parts[i].split("/")[0]) - 1;
          if (vertexData[idx]) {
            vertices.push(...vertexData[idx]);
          }
        }
      }
    }

    if (vertices.length === 0) {
      // Create a fallback sphere
      const sphereGeom = new THREE.SphereGeometry(1.5, 32, 32);
      const material = new THREE.MeshStandardMaterial({
        color: 0x764ba2,
        metalness: 0.4,
        roughness: 0.6,
      });
      model = new THREE.Mesh(sphereGeom, material);
      model.castShadow = true;
      scene.add(model);
      return;
    }

    geometry.setAttribute(
      "position",
      new THREE.Float32BufferAttribute(vertices, 3)
    );
    geometry.computeVertexNormals();
    geometry.center();

    const material = new THREE.MeshStandardMaterial({
      color: 0x764ba2,
      metalness: 0.4,
      roughness: 0.6,
    });

    model = new THREE.Mesh(geometry, material);
    model.castShadow = true;

    // Normalize scale
    const box = new THREE.Box3().setFromObject(model);
    const size = box.getSize(new THREE.Vector3());
    const maxDim = Math.max(size.x, size.y, size.z);
    model.scale.multiplyScalar(3 / maxDim);

    scene.add(model);
  }

  function animate() {
    animationId = requestAnimationFrame(animate);

    if (model && controls.rotating) {
      model.rotation.y += 0.005;
    }

    renderer.render(scene, camera);
  }

  function handleMouseDown(e) {
    mouseDown = true;
    lastMouseX = e.clientX;
    lastMouseY = e.clientY;
  }

  function handleMouseUp() {
    mouseDown = false;
  }

  function handleMouseMove(e) {
    if (!mouseDown || !model) return;

    const deltaX = e.clientX - lastMouseX;
    const deltaY = e.clientY - lastMouseY;

    model.rotation.y += deltaX * 0.01;
    model.rotation.x += deltaY * 0.01;

    lastMouseX = e.clientX;
    lastMouseY = e.clientY;
  }

  function handleWheel(e) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    controls.zoom = Math.max(0.5, Math.min(3, controls.zoom * delta));
    camera.position.z = 5 / controls.zoom;
  }

  function toggleRotation() {
    controls.rotating = !controls.rotating;
  }

  function resetView() {
    if (model) {
      model.rotation.set(0, 0, 0);
    }
    controls.zoom = 1;
    camera.position.set(0, 2, 5);
  }
</script>

<div class="model-viewer-container">
  {#if loading}
    <div
      class="flex flex-col justify-center items-center h-96 bg-base-200 rounded-xl"
    >
      <span class="loading loading-spinner loading-lg text-primary mb-4"></span>
      <p class="text-base-content/60">Loading 3D model...</p>
    </div>
  {:else if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
    </div>
  {/if}

  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={container}
    class="model-canvas rounded-xl overflow-hidden shadow-lg cursor-grab {mouseDown
      ? 'cursor-grabbing'
      : ''}"
    class:hidden={loading || error}
    role="img"
    tabindex="0"
    aria-label="3D Model Viewer"
    onmousedown={handleMouseDown}
    onmouseup={handleMouseUp}
    onmouseleave={handleMouseUp}
    onmousemove={handleMouseMove}
    onwheel={handleWheel}
  ></div>

  {#if !loading && !error}
    <div class="flex items-center justify-between mt-4">
      <div class="flex items-center gap-2">
        <button
          onclick={toggleRotation}
          class="btn btn-sm {controls.rotating ? 'btn-primary' : 'btn-ghost'}"
        >
          <i class="bi bi-arrow-repeat mr-1" aria-hidden="true"></i>
          {controls.rotating ? "Auto-Rotate On" : "Auto-Rotate Off"}
        </button>
        <button onclick={resetView} class="btn btn-sm btn-ghost">
          <i class="bi bi-arrow-counterclockwise mr-1" aria-hidden="true"></i>
          Reset View
        </button>
      </div>
      <div class="text-sm text-base-content/60">
        <span class="badge badge-ghost mr-2"
          >Zoom: {Math.round(controls.zoom * 100)}%</span
        >
        <span>Drag to rotate • Scroll to zoom</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .model-canvas {
    width: 100%;
    height: 500px;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  }

  .model-canvas:focus {
    outline: 2px solid oklch(var(--p));
    outline-offset: 2px;
  }
</style>
