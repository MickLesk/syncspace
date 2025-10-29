// File type to Bootstrap icon mapping
// See: https://icons.getbootstrap.com/
export function getFileIcon(filename, isDirectory = false) {
  if (isDirectory) {
    return 'folder-fill';
  }

  const ext = filename.split('.').pop()?.toLowerCase();

  const iconMap = {
    // Documents
    pdf: 'file-earmark-pdf-fill',
    doc: 'file-earmark-word-fill',
    docx: 'file-earmark-word-fill',
    txt: 'file-earmark-text-fill',
    rtf: 'file-earmark-text-fill',
    odt: 'file-earmark-text-fill',
    
    // Spreadsheets
    xls: 'file-earmark-excel-fill',
    xlsx: 'file-earmark-excel-fill',
    csv: 'file-earmark-spreadsheet-fill',
    ods: 'file-earmark-spreadsheet-fill',
    
    // Presentations
    ppt: 'file-earmark-ppt-fill',
    pptx: 'file-earmark-ppt-fill',
    odp: 'file-earmark-slides-fill',
    
    // Images
    jpg: 'file-earmark-image-fill',
    jpeg: 'file-earmark-image-fill',
    png: 'file-earmark-image-fill',
    gif: 'file-earmark-image-fill',
    bmp: 'file-earmark-image-fill',
    svg: 'file-image-fill',
    webp: 'file-earmark-image-fill',
    ico: 'file-earmark-image-fill',
    
    // Videos
    mp4: 'file-earmark-play-fill',
    avi: 'file-earmark-play-fill',
    mov: 'file-earmark-play-fill',
    wmv: 'file-earmark-play-fill',
    flv: 'file-earmark-play-fill',
    mkv: 'file-earmark-play-fill',
    webm: 'file-earmark-play-fill',
    
    // Audio
    mp3: 'file-earmark-music-fill',
    wav: 'file-earmark-music-fill',
    ogg: 'file-earmark-music-fill',
    flac: 'file-earmark-music-fill',
    m4a: 'file-earmark-music-fill',
    wma: 'file-earmark-music-fill',
    
    // Archives
    zip: 'file-earmark-zip-fill',
    rar: 'file-earmark-zip-fill',
    '7z': 'file-earmark-zip-fill',
    tar: 'file-earmark-zip-fill',
    gz: 'file-earmark-zip-fill',
    
    // Code
    js: 'file-earmark-code-fill',
    ts: 'file-earmark-code-fill',
    jsx: 'file-earmark-code-fill',
    tsx: 'file-earmark-code-fill',
    py: 'file-earmark-code-fill',
    java: 'file-earmark-code-fill',
    cpp: 'file-earmark-code-fill',
    c: 'file-earmark-code-fill',
    h: 'file-earmark-code-fill',
    cs: 'file-earmark-code-fill',
    php: 'file-earmark-code-fill',
    rb: 'file-earmark-code-fill',
    go: 'file-earmark-code-fill',
    rs: 'file-earmark-code-fill',
    swift: 'file-earmark-code-fill',
    kt: 'file-earmark-code-fill',
    
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
    exe: 'file-earmark-binary-fill',
    msi: 'file-earmark-binary-fill',
    app: 'app-fill',
    dmg: 'file-earmark-binary-fill',
    deb: 'file-earmark-binary-fill',
    rpm: 'file-earmark-binary-fill',
    
    // Fonts
    ttf: 'file-earmark-font-fill',
    otf: 'file-earmark-font-fill',
    woff: 'file-earmark-font-fill',
    woff2: 'file-earmark-font-fill',
  };

  return iconMap[ext] || 'file-earmark-fill'; // Default file icon
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

// Get color class for file type
export function getFileIconColor(filename) {
  const ext = filename.split('.').pop()?.toLowerCase();
  
  // Image files - purple
  if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'ico'].includes(ext)) {
    return 'text-purple-600';
  }
  // Video files - pink
  if (['mp4', 'avi', 'mkv', 'mov', 'webm', 'flv', 'wmv'].includes(ext)) {
    return 'text-pink-600';
  }
  // Audio files - yellow
  if (['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac', 'wma'].includes(ext)) {
    return 'text-yellow-600';
  }
  // Documents - blue/red
  if (['pdf'].includes(ext)) {
    return 'text-red-600';
  }
  if (['doc', 'docx', 'txt', 'md'].includes(ext)) {
    return 'text-blue-600';
  }
  // Spreadsheets - green
  if (['xls', 'xlsx', 'csv'].includes(ext)) {
    return 'text-green-600';
  }
  // Presentations - orange
  if (['ppt', 'pptx'].includes(ext)) {
    return 'text-orange-600';
  }
  // Archive - brown/amber
  if (['zip', 'rar', '7z', 'tar', 'gz', 'bz2'].includes(ext)) {
    return 'text-amber-700';
  }
  // Code files - cyan/varied
  if (['js', 'ts', 'jsx', 'tsx', 'json'].includes(ext)) {
    return 'text-cyan-600';
  }
  if (['html'].includes(ext)) {
    return 'text-orange-500';
  }
  if (['css', 'scss', 'sass'].includes(ext)) {
    return 'text-blue-500';
  }
  if (['py', 'java', 'c', 'cpp', 'cs', 'php', 'rb', 'go', 'rs', 'swift'].includes(ext)) {
    return 'text-green-600';
  }
  // Text files - gray
  if (['txt', 'md', 'log'].includes(ext)) {
    return 'text-gray-600';
  }
  
  // Default - gray
  return 'text-gray-500';
}
