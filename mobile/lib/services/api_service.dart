import 'package:dio/dio.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../config/app_config.dart';
import '../models/file_model.dart';

/// API Service - Handles all HTTP requests to the SyncSpace backend
class ApiService {
  late final Dio _dio;
  final FlutterSecureStorage _secureStorage = const FlutterSecureStorage();
  String? _authToken;
  String _baseUrl = AppConfig.defaultBaseUrl;

  ApiService() {
    _dio = Dio(
      BaseOptions(
        baseUrl: _baseUrl + AppConfig.apiPrefix,
        connectTimeout: AppConfig.connectTimeout,
        receiveTimeout: AppConfig.receiveTimeout,
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
      ),
    );

    // Add interceptor for auth token
    _dio.interceptors.add(
      InterceptorsWrapper(
        onRequest: (options, handler) async {
          if (_authToken != null) {
            options.headers['Authorization'] = 'Bearer $_authToken';
          }
          return handler.next(options);
        },
        onError: (error, handler) async {
          if (error.response?.statusCode == 401) {
            // Token expired, logout
            await logout();
          }
          return handler.next(error);
        },
      ),
    );
  }

  /// Set server URL
  Future<void> setServerUrl(String url) async {
    _baseUrl = url;
    _dio.options.baseUrl = _baseUrl + AppConfig.apiPrefix;
    await _secureStorage.write(key: AppConfig.keyServerUrl, value: url);
  }

  /// Get current server URL
  Future<String> getServerUrl() async {
    final url = await _secureStorage.read(key: AppConfig.keyServerUrl);
    return url ?? AppConfig.defaultBaseUrl;
  }

  /// Set auth token
  Future<void> setAuthToken(String token) async {
    _authToken = token;
    await _secureStorage.write(key: AppConfig.keyAuthToken, value: token);
  }

  /// Get auth token
  Future<String?> getAuthToken() async {
    if (_authToken != null) return _authToken;
    _authToken = await _secureStorage.read(key: AppConfig.keyAuthToken);
    return _authToken;
  }

  /// Clear auth token (logout)
  Future<void> logout() async {
    _authToken = null;
    await _secureStorage.delete(key: AppConfig.keyAuthToken);
    await _secureStorage.delete(key: AppConfig.keyRefreshToken);
    await _secureStorage.delete(key: AppConfig.keyUserId);
    await _secureStorage.delete(key: AppConfig.keyUsername);
  }

  /// Login
  Future<Map<String, dynamic>> login(String username, String password) async {
    try {
      final response = await _dio.post(
        '/auth/login',
        data: {'username': username, 'password': password},
      );

      if (response.statusCode == 200) {
        final token = response.data['token'] as String;
        await setAuthToken(token);

        // Save user info
        await _secureStorage.write(
          key: AppConfig.keyUserId,
          value: response.data['user']['id'] as String,
        );
        await _secureStorage.write(key: AppConfig.keyUsername, value: username);

        return response.data as Map<String, dynamic>;
      }
      throw Exception('Login failed');
    } catch (e) {
      throw Exception('Login error: $e');
    }
  }

  /// Get current user profile
  Future<Map<String, dynamic>> getUserProfile() async {
    try {
      final response = await _dio.get('/users/profile');
      return response.data as Map<String, dynamic>;
    } catch (e) {
      throw Exception('Failed to get user profile: $e');
    }
  }

  /// List files in directory
  Future<List<FileModel>> listFiles(String path) async {
    try {
      final response = await _dio.get('/files/$path');
      final List<dynamic> filesData = response.data['files'] as List<dynamic>;
      return filesData
          .map((json) => FileModel.fromJson(json as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to list files: $e');
    }
  }

  /// Upload file
  Future<Map<String, dynamic>> uploadFile(
    String filePath,
    String uploadPath, {
    Function(double)? onProgress,
  }) async {
    try {
      final formData = FormData.fromMap({
        'file': await MultipartFile.fromFile(filePath),
      });

      final response = await _dio.post(
        '/upload/$uploadPath',
        data: formData,
        onSendProgress: (sent, total) {
          if (onProgress != null && total > 0) {
            onProgress(sent / total);
          }
        },
      );

      return response.data as Map<String, dynamic>;
    } catch (e) {
      throw Exception('Failed to upload file: $e');
    }
  }

  /// Download file
  Future<void> downloadFile(
    String filePath,
    String savePath, {
    Function(double)? onProgress,
  }) async {
    try {
      await _dio.download(
        '/files/$filePath',
        savePath,
        onReceiveProgress: (received, total) {
          if (onProgress != null && total > 0) {
            onProgress(received / total);
          }
        },
      );
    } catch (e) {
      throw Exception('Failed to download file: $e');
    }
  }

  /// Delete file
  Future<void> deleteFile(String filePath) async {
    try {
      await _dio.delete('/files/$filePath');
    } catch (e) {
      throw Exception('Failed to delete file: $e');
    }
  }

  /// Create directory
  Future<void> createDirectory(String path) async {
    try {
      await _dio.post('/dirs/$path');
    } catch (e) {
      throw Exception('Failed to create directory: $e');
    }
  }

  /// Search files
  Future<Map<String, dynamic>> searchFiles(String query) async {
    try {
      final response = await _dio.get('/search', queryParameters: {'q': query});
      return response.data as Map<String, dynamic>;
    } catch (e) {
      throw Exception('Failed to search files: $e');
    }
  }

  /// Get favorites
  Future<List<dynamic>> getFavorites() async {
    try {
      final response = await _dio.get('/favorites');
      return response.data['favorites'] as List<dynamic>;
    } catch (e) {
      throw Exception('Failed to get favorites: $e');
    }
  }

  /// Toggle favorite
  Future<void> toggleFavorite(String fileId) async {
    try {
      await _dio.post('/favorites/$fileId');
    } catch (e) {
      throw Exception('Failed to toggle favorite: $e');
    }
  }
}
