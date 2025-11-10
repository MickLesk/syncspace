<script lang="ts">
  interface FilePreview {
    type: "text" | "code" | "image" | "pdf" | "video" | "audio" | "file";
    url?: string;
    name?: string;
    size?: number;
    content?: string;
  }

  interface Props {
    file: FilePreview;
  }

  let { file }: Props = $props();

  function formatFileSize(bytes?: number) {
    if (!bytes) return "Unknown";
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function getLanguage(filename?: string) {
    if (!filename) return "text";
    const ext = filename.split(".").pop() || "text";
    const langMap: { [key: string]: string } = {
      js: "javascript",
      ts: "typescript",
      py: "python",
      rb: "ruby",
      go: "go",
      rs: "rust",
      java: "java",
      cpp: "cpp",
      c: "c",
      sh: "bash",
      html: "html",
      css: "css",
      xml: "xml",
      json: "json",
      yaml: "yaml",
      yml: "yaml",
      sql: "sql",
      md: "markdown",
    };
    return langMap[ext] || ext;
  }
</script>

<div class="bg-white rounded-lg border border-gray-200 overflow-hidden">
  <!-- Header -->
  <div
    class="px-6 py-4 border-b border-gray-200 flex items-center justify-between"
  >
    <div class="flex items-center gap-3">
      <i class="bi bi-file text-gray-400 text-xl"></i>
      <div>
        <p class="font-semibold text-gray-900">{file.name || "File"}</p>
        <p class="text-xs text-gray-500">{formatFileSize(file.size)}</p>
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="p-6">
    {#if file.type === "image" && file.url}
      <img
        src={file.url}
        alt={file.name}
        class="max-w-full h-auto rounded-lg"
      />
    {:else if file.type === "video" && file.url}
      <video src={file.url} controls class="max-w-full h-auto rounded-lg" />
    {:else if file.type === "audio" && file.url}
      <audio src={file.url} controls class="w-full" />
    {:else if (file.type === "text" || file.type === "code") && file.content}
      <pre class="bg-gray-100 p-4 rounded-lg overflow-x-auto text-xs"><code
          >{file.content}</code
        ></pre>
    {:else if file.type === "pdf" && file.url}
      <iframe
        src={file.url}
        class="w-full h-96 rounded-lg border border-gray-200"
        title="PDF Viewer"
      ></iframe>
    {:else}
      <div class="text-center py-12 text-gray-500">
        <i class="bi bi-file-text text-4xl mb-3 block"></i>
        <p>File preview not available</p>
      </div>
    {/if}
  </div>
</div>
