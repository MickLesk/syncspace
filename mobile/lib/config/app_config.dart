/// App Configuration
class AppConfig {
  // App Info
  static const String appName = 'SyncSpace';
  static const String appVersion = '1.0.0';

  // API Configuration
  static const String defaultBaseUrl = 'http://localhost:8080';
  static const String apiPrefix = '/api';

  // Storage Keys
  static const String keyAuthToken = 'auth_token';
  static const String keyRefreshToken = 'refresh_token';
  static const String keyUserId = 'user_id';
  static const String keyUsername = 'username';
  static const String keyServerUrl = 'server_url';
  static const String keyLanguage = 'language';
  static const String keyThemeMode = 'theme_mode';

  // Timeouts
  static const Duration connectTimeout = Duration(seconds: 30);
  static const Duration receiveTimeout = Duration(seconds: 30);

  // Pagination
  static const int defaultPageSize = 50;
  static const int maxPageSize = 100;

  // Upload/Download
  static const int chunkSize = 1024 * 1024; // 1MB chunks
  static const int maxUploadSize = 1024 * 1024 * 100; // 100MB max

  // Cache
  static const Duration cacheExpiry = Duration(hours: 24);
  static const int maxCacheSize = 100;
}
