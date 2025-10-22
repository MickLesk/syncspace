/**
 * Duplicate File Detector
 * Client-side duplicate detection using file content hashing
 */

/**
 * Calculate SHA-256 hash of file content
 */
async function calculateFileHash(file) {
  const buffer = await file.arrayBuffer();
  const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

/**
 * Calculate fast hash based on file size + first/last KB
 * For quick initial deduplication before full hash
 */
async function calculateQuickHash(file) {
  const CHUNK_SIZE = 1024; // 1KB
  
  // For small files, use full content
  if (file.size <= CHUNK_SIZE * 2) {
    return calculateFileHash(file);
  }

  // For larger files, hash first KB + last KB + size
  const firstChunk = file.slice(0, CHUNK_SIZE);
  const lastChunk = file.slice(file.size - CHUNK_SIZE, file.size);
  
  const firstBuffer = await firstChunk.arrayBuffer();
  const lastBuffer = await lastChunk.arrayBuffer();
  
  const combined = new Uint8Array(firstBuffer.byteLength + lastBuffer.byteLength + 8);
  combined.set(new Uint8Array(firstBuffer), 0);
  combined.set(new Uint8Array(lastBuffer), firstBuffer.byteLength);
  
  // Add file size as 8 bytes
  const sizeBytes = new ArrayBuffer(8);
  new DataView(sizeBytes).setBigUint64(0, BigInt(file.size), true);
  combined.set(new Uint8Array(sizeBytes), firstBuffer.byteLength + lastBuffer.byteLength);

  const hashBuffer = await crypto.subtle.digest('SHA-256', combined);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

/**
 * Find duplicate files in a list
 * Returns groups of duplicates
 */
export async function findDuplicates(files, onProgress = null) {
  const hashMap = new Map(); // hash -> [files]
  const quickHashMap = new Map(); // quick hash -> [files]
  const duplicateGroups = [];

  // Step 1: Quick hash all files (fast)
  for (let i = 0; i < files.length; i++) {
    const file = files[i];
    
    if (onProgress) {
      onProgress({ phase: 'quick-scan', current: i + 1, total: files.length });
    }

    try {
      const quickHash = await calculateQuickHash(file);
      
      if (!quickHashMap.has(quickHash)) {
        quickHashMap.set(quickHash, []);
      }
      quickHashMap.get(quickHash).push(file);
    } catch (e) {
      console.error(`Failed to hash ${file.name}:`, e);
    }
  }

  // Step 2: Full hash for potential duplicates (slower)
  const potentialDuplicates = Array.from(quickHashMap.values()).filter(g => g.length > 1);
  let processed = 0;
  
  for (const group of potentialDuplicates) {
    for (const file of group) {
      if (onProgress) {
        processed++;
        onProgress({ 
          phase: 'full-scan', 
          current: processed, 
          total: potentialDuplicates.reduce((sum, g) => sum + g.length, 0)
        });
      }

      try {
        const fullHash = await calculateFileHash(file);
        
        if (!hashMap.has(fullHash)) {
          hashMap.set(fullHash, []);
        }
        hashMap.get(fullHash).push(file);
      } catch (e) {
        console.error(`Failed to full hash ${file.name}:`, e);
      }
    }
  }

  // Step 3: Extract duplicate groups
  for (const [hash, group] of hashMap) {
    if (group.length > 1) {
      duplicateGroups.push({
        hash,
        count: group.length,
        files: group,
        size: group[0].size,
        wastedSpace: group[0].size * (group.length - 1)
      });
    }
  }

  // Sort by wasted space (descending)
  duplicateGroups.sort((a, b) => b.wastedSpace - a.wastedSpace);

  return duplicateGroups;
}

/**
 * Find duplicates among remote files (from API)
 * Fetches file content from server
 */
export async function findRemoteDuplicates(fileList, apiClient, onProgress = null) {
  const hashMap = new Map();
  const duplicateGroups = [];

  for (let i = 0; i < fileList.length; i++) {
    const file = fileList[i];
    
    if (file.is_dir) continue; // Skip directories

    if (onProgress) {
      onProgress({ phase: 'scanning', current: i + 1, total: fileList.length });
    }

    try {
      // Fetch file content
      const response = await apiClient.downloadFile(file.name);
      const blob = await response.blob();
      
      // Calculate hash
      const buffer = await blob.arrayBuffer();
      const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      const hash = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
      
      if (!hashMap.has(hash)) {
        hashMap.set(hash, []);
      }
      hashMap.get(hash).push(file);
    } catch (e) {
      console.error(`Failed to hash remote file ${file.name}:`, e);
    }
  }

  // Extract duplicate groups
  for (const [hash, group] of hashMap) {
    if (group.length > 1) {
      duplicateGroups.push({
        hash,
        count: group.length,
        files: group,
        size: group[0].size,
        wastedSpace: group[0].size * (group.length - 1)
      });
    }
  }

  duplicateGroups.sort((a, b) => b.wastedSpace - a.wastedSpace);

  return duplicateGroups;
}

/**
 * Format bytes to human-readable size
 */
export function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/**
 * Check if two files are identical (same size and name)
 */
export function areFilesIdentical(file1, file2) {
  return file1.name === file2.name && file1.size === file2.size;
}
