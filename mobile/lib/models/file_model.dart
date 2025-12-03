/// File Model
class FileModel {
  final String id;
  final String filename;
  final String filePath;
  final int sizeBytes;
  final String? mimeType;
  final String ownerId;
  final String? ownerUsername;
  final DateTime createdAt;
  final DateTime updatedAt;
  final bool isDeleted;
  final bool isFavorite;
  final List<String>? tags;

  FileModel({
    required this.id,
    required this.filename,
    required this.filePath,
    required this.sizeBytes,
    this.mimeType,
    required this.ownerId,
    this.ownerUsername,
    required this.createdAt,
    required this.updatedAt,
    this.isDeleted = false,
    this.isFavorite = false,
    this.tags,
  });

  factory FileModel.fromJson(Map<String, dynamic> json) {
    return FileModel(
      id: json['id'] as String,
      filename: json['filename'] as String,
      filePath: json['file_path'] as String? ?? json['path'] as String,
      sizeBytes: json['size_bytes'] as int? ?? json['size'] as int? ?? 0,
      mimeType: json['mime_type'] as String?,
      ownerId: json['owner_id'] as String,
      ownerUsername: json['owner_username'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
      isDeleted: json['is_deleted'] as bool? ?? false,
      isFavorite: json['is_favorite'] as bool? ?? false,
      tags: json['tags'] != null
          ? List<String>.from(json['tags'] as List)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'filename': filename,
      'file_path': filePath,
      'size_bytes': sizeBytes,
      'mime_type': mimeType,
      'owner_id': ownerId,
      'owner_username': ownerUsername,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
      'is_deleted': isDeleted,
      'is_favorite': isFavorite,
      'tags': tags,
    };
  }

  bool get isDirectory =>
      mimeType == 'directory' || mimeType == 'inode/directory';

  String get formattedSize {
    if (sizeBytes == 0) return '0 B';
    const suffixes = ['B', 'KB', 'MB', 'GB', 'TB'];
    var i = 0;
    var size = sizeBytes.toDouble();
    while (size >= 1024 && i < suffixes.length - 1) {
      size /= 1024;
      i++;
    }
    return '${size.toStringAsFixed(2)} ${suffixes[i]}';
  }

  String get extension {
    final parts = filename.split('.');
    return parts.length > 1 ? parts.last.toLowerCase() : '';
  }

  bool get isImage =>
      ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp'].contains(extension);
  bool get isVideo =>
      ['mp4', 'webm', 'ogg', 'mov', 'avi', 'mkv'].contains(extension);
  bool get isAudio =>
      ['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].contains(extension);
  bool get isDocument => [
    'pdf',
    'doc',
    'docx',
    'txt',
    'md',
    'xls',
    'xlsx',
    'ppt',
    'pptx',
  ].contains(extension);
}
