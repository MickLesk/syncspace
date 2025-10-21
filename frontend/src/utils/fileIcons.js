// File type to Bootstrap icon mapping
// See: https://icons.getbootstrap.com/
export function getFileIcon(filename, isDirectory = false) {
  if (isDirectory) {
    return 'folder-fill';
  }

  const ext = filename.split('.').pop()?.toLowerCase();

  const iconMap = {
    // Documents
    pdf: 'file-earmark-pdf',
    doc: 'file-earmark-word',
    docx: 'file-earmark-word',
    txt: 'file-earmark-text',
    rtf: 'file-earmark-text',
    odt: 'file-earmark-text',
    
    // Spreadsheets
    xls: 'file-earmark-excel',
    xlsx: 'file-earmark-excel',
    csv: 'file-earmark-spreadsheet',
    ods: 'file-earmark-spreadsheet',
    
    // Presentations
    ppt: 'file-earmark-ppt',
    pptx: 'file-earmark-ppt',
    odp: 'file-earmark-slides',
    
    // Images
    jpg: 'file-earmark-image',
    jpeg: 'file-earmark-image',
    png: 'file-earmark-image',
    gif: 'file-earmark-image',
    bmp: 'file-earmark-image',
    svg: 'file-image',
    webp: 'file-earmark-image',
    ico: 'file-earmark-image',
    
    // Videos
    mp4: 'file-earmark-play',
    avi: 'file-earmark-play',
    mov: 'file-earmark-play',
    wmv: 'file-earmark-play',
    flv: 'file-earmark-play',
    mkv: 'file-earmark-play',
    webm: 'file-earmark-play',
    
    // Audio
    mp3: 'file-earmark-music',
    wav: 'file-earmark-music',
    ogg: 'file-earmark-music',
    flac: 'file-earmark-music',
    m4a: 'file-earmark-music',
    wma: 'file-earmark-music',
    
    // Archives
    zip: 'file-earmark-zip',
    rar: 'file-earmark-zip',
    '7z': 'file-earmark-zip',
    tar: 'file-earmark-zip',
    gz: 'file-earmark-zip',
    
    // Code
    js: 'file-earmark-code',
    ts: 'file-earmark-code',
    jsx: 'file-earmark-code',
    tsx: 'file-earmark-code',
    py: 'file-earmark-code',
    java: 'file-earmark-code',
    cpp: 'file-earmark-code',
    c: 'file-earmark-code',
    h: 'file-earmark-code',
    cs: 'file-earmark-code',
    php: 'file-earmark-code',
    rb: 'file-earmark-code',
    go: 'file-earmark-code',
    rs: 'file-earmark-code',
    swift: 'file-earmark-code',
    kt: 'file-earmark-code',
    
    // Web
    html: 'filetype-html',
    css: 'filetype-css',
    scss: 'filetype-scss',
    sass: 'filetype-sass',
    json: 'filetype-json',
    xml: 'filetype-xml',
    yaml: 'filetype-yml',
    yml: 'filetype-yml',
    
    // Executable
    exe: 'file-earmark-binary',
    msi: 'file-earmark-binary',
    app: 'app',
    dmg: 'file-earmark-binary',
    deb: 'file-earmark-binary',
    rpm: 'file-earmark-binary',
    
    // Fonts
    ttf: 'file-earmark-font',
    otf: 'file-earmark-font',
    woff: 'file-earmark-font',
    woff2: 'file-earmark-font',
  };

  return iconMap[ext] || 'file-earmark'; // Default file icon
}

// Get file type for preview
export function getFileType(filename) {
  const ext = filename.split('.').pop()?.toLowerCase();
  
  const imageTypes = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'svg', 'webp'];
  const videoTypes = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv', 'webm'];
  const audioTypes = ['mp3', 'wav', 'ogg', 'flac', 'm4a'];
  const textTypes = ['txt', 'md', 'json', 'xml', 'html', 'css', 'js', 'ts', 'jsx', 'tsx', 'py', 'java', 'c', 'cpp', 'h', 'cs', 'php', 'rb', 'go', 'rs'];
  
  if (imageTypes.includes(ext)) return 'image';
  if (videoTypes.includes(ext)) return 'video';
  if (audioTypes.includes(ext)) return 'audio';
  if (textTypes.includes(ext)) return 'text';
  if (ext === 'pdf') return 'pdf';
  
  return 'unknown';
}

// Check if file can be previewed
export function isPreviewable(filename) {
  const type = getFileType(filename);
  return ['image', 'video', 'audio', 'text', 'pdf'].includes(type);
}
