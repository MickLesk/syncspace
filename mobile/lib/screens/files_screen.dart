import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:file_picker/file_picker.dart';
import '../models/file_model.dart';
import '../services/api_service.dart';

/// Files Screen - Browse and manage files/folders
class FilesScreen extends StatefulWidget {
  const FilesScreen({super.key});

  @override
  State<FilesScreen> createState() => _FilesScreenState();
}

class _FilesScreenState extends State<FilesScreen> {
  List<FileModel> _files = [];
  String _currentPath = '/';
  bool _isLoading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadFiles();
  }

  Future<void> _loadFiles() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final apiService = context.read<ApiService>();
      final files = await apiService.listFiles(_currentPath);

      if (mounted) {
        setState(() {
          _files = files;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isLoading = false;
        });
      }
    }
  }

  Future<void> _navigateToFolder(String folderPath) async {
    setState(() {
      _currentPath = folderPath;
    });
    await _loadFiles();
  }

  Future<void> _navigateUp() async {
    if (_currentPath == '/') return;

    final parts = _currentPath.split('/').where((p) => p.isNotEmpty).toList();
    if (parts.isEmpty) {
      await _navigateToFolder('/');
    } else {
      parts.removeLast();
      final newPath = '/${parts.join('/')}';
      await _navigateToFolder(newPath);
    }
  }

  Future<void> _uploadFile() async {
    try {
      final result = await FilePicker.platform.pickFiles();

      if (result != null && result.files.single.path != null) {
        final filePath = result.files.single.path!;
        final fileName = result.files.single.name;

        if (!mounted) return;

        // Show upload progress dialog
        showDialog(
          context: context,
          barrierDismissible: false,
          builder: (context) => _UploadProgressDialog(
            fileName: fileName,
            filePath: filePath,
            uploadPath: _currentPath,
            onComplete: () {
              Navigator.of(context).pop();
              _loadFiles();
            },
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Upload failed: $e')));
      }
    }
  }

  Future<void> _createFolder() async {
    final controller = TextEditingController();

    final result = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Create Folder'),
        content: TextField(
          controller: controller,
          decoration: const InputDecoration(
            labelText: 'Folder Name',
            hintText: 'Enter folder name',
          ),
          autofocus: true,
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.of(context).pop(true),
            child: const Text('Create'),
          ),
        ],
      ),
    );

    if (result == true && controller.text.isNotEmpty) {
      try {
        final apiService = context.read<ApiService>();
        final folderPath = '$_currentPath/${controller.text}';
        await apiService.createDirectory(folderPath);

        if (mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(const SnackBar(content: Text('Folder created')));
          _loadFiles();
        }
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Failed to create folder: $e')),
          );
        }
      }
    }
  }

  Future<void> _deleteFile(FileModel file) async {
    final confirm = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete'),
        content: Text('Delete ${file.filename}?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.of(context).pop(true),
            style: FilledButton.styleFrom(backgroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );

    if (confirm == true) {
      try {
        final apiService = context.read<ApiService>();
        await apiService.deleteFile(file.filePath);

        if (mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(const SnackBar(content: Text('Deleted')));
          _loadFiles();
        }
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('Failed to delete: $e')));
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Files'),
        leading: _currentPath != '/'
            ? IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: _navigateUp,
              )
            : null,
        actions: [
          IconButton(icon: const Icon(Icons.refresh), onPressed: _loadFiles),
        ],
      ),
      body: Column(
        children: [
          // Current path breadcrumb
          if (_currentPath != '/')
            Container(
              width: double.infinity,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              color: Theme.of(context).colorScheme.surfaceContainerHighest,
              child: Text(
                _currentPath,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ),

          // File list
          Expanded(child: _buildBody()),
        ],
      ),
      floatingActionButton: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          FloatingActionButton.small(
            heroTag: 'create_folder',
            onPressed: _createFolder,
            child: const Icon(Icons.create_new_folder),
          ),
          const SizedBox(height: 8),
          FloatingActionButton(
            heroTag: 'upload_file',
            onPressed: _uploadFile,
            child: const Icon(Icons.upload_file),
          ),
        ],
      ),
    );
  }

  Widget _buildBody() {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.error_outline, size: 64, color: Colors.red[300]),
            const SizedBox(height: 16),
            Text('Error: $_error'),
            const SizedBox(height: 16),
            FilledButton.icon(
              onPressed: _loadFiles,
              icon: const Icon(Icons.refresh),
              label: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_files.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.folder_open, size: 64, color: Colors.grey[400]),
            const SizedBox(height: 16),
            Text(
              'No files',
              style: Theme.of(
                context,
              ).textTheme.titleMedium?.copyWith(color: Colors.grey[600]),
            ),
            const SizedBox(height: 8),
            Text(
              'Upload files or create folders',
              style: TextStyle(color: Colors.grey[600]),
            ),
          ],
        ),
      );
    }

    return RefreshIndicator(
      onRefresh: _loadFiles,
      child: ListView.builder(
        itemCount: _files.length,
        itemBuilder: (context, index) {
          final file = _files[index];
          return _FileListItem(
            file: file,
            onTap: () {
              if (file.isDirectory) {
                _navigateToFolder(file.filePath);
              } else {
                // TODO: Open file preview
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Open ${file.filename}')),
                );
              }
            },
            onDelete: () => _deleteFile(file),
          );
        },
      ),
    );
  }
}

/// File List Item Widget
class _FileListItem extends StatelessWidget {
  final FileModel file;
  final VoidCallback onTap;
  final VoidCallback onDelete;

  const _FileListItem({
    required this.file,
    required this.onTap,
    required this.onDelete,
  });

  IconData _getFileIcon() {
    if (file.isDirectory) return Icons.folder;
    if (file.isImage) return Icons.image;
    if (file.isVideo) return Icons.video_file;
    if (file.isAudio) return Icons.audio_file;
    if (file.isDocument) return Icons.description;
    return Icons.insert_drive_file;
  }

  Color _getFileColor(BuildContext context) {
    if (file.isDirectory) return Colors.amber[700]!;
    if (file.isImage) return Colors.blue[700]!;
    if (file.isVideo) return Colors.purple[700]!;
    if (file.isAudio) return Colors.green[700]!;
    if (file.isDocument) return Colors.red[700]!;
    return Colors.grey[600]!;
  }

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: Icon(_getFileIcon(), color: _getFileColor(context), size: 32),
      title: Text(file.filename),
      subtitle: file.isDirectory
          ? const Text('Folder')
          : Text(file.formattedSize),
      trailing: IconButton(
        icon: const Icon(Icons.delete_outline),
        onPressed: onDelete,
      ),
      onTap: onTap,
    );
  }
}

/// Upload Progress Dialog
class _UploadProgressDialog extends StatefulWidget {
  final String fileName;
  final String filePath;
  final String uploadPath;
  final VoidCallback onComplete;

  const _UploadProgressDialog({
    required this.fileName,
    required this.filePath,
    required this.uploadPath,
    required this.onComplete,
  });

  @override
  State<_UploadProgressDialog> createState() => _UploadProgressDialogState();
}

class _UploadProgressDialogState extends State<_UploadProgressDialog> {
  double _progress = 0.0;
  bool _isUploading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _upload();
  }

  Future<void> _upload() async {
    try {
      final apiService = context.read<ApiService>();

      await apiService.uploadFile(
        widget.filePath,
        widget.uploadPath,
        onProgress: (progress) {
          if (mounted) {
            setState(() {
              _progress = progress;
            });
          }
        },
      );

      if (mounted) {
        setState(() {
          _isUploading = false;
        });

        await Future.delayed(const Duration(milliseconds: 500));
        widget.onComplete();
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isUploading = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Uploading'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(widget.fileName),
          const SizedBox(height: 16),
          if (_isUploading) ...[
            LinearProgressIndicator(value: _progress),
            const SizedBox(height: 8),
            Text('${(_progress * 100).toStringAsFixed(0)}%'),
          ] else if (_error != null) ...[
            Text('Error: $_error', style: const TextStyle(color: Colors.red)),
          ] else ...[
            Row(
              children: [
                Icon(Icons.check_circle, color: Colors.green[700]),
                const SizedBox(width: 8),
                const Text('Upload complete'),
              ],
            ),
          ],
        ],
      ),
      actions: [
        if (!_isUploading)
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              if (_error == null) {
                widget.onComplete();
              }
            },
            child: const Text('Close'),
          ),
      ],
    );
  }
}
