# SyncSpace Mobile

Flutter mobile app for SyncSpace - self-hosted file synchronization and collaboration.

## Features

- ✅ **Authentication** - Secure login with JWT tokens
- ✅ **File Browser** - Browse files and folders with native UI
- ✅ **File Operations** - Upload, download, delete files
- ✅ **Folder Management** - Create and navigate folders
- ✅ **Progress Tracking** - Real-time upload/download progress
- ✅ **Search** - Search files across your library
- ✅ **Favorites** - Star and access favorite files quickly
- ✅ **Profile** - User profile management
- 🚧 **Offline Mode** - Coming soon
- 🚧 **File Sharing** - Coming soon
- 🚧 **Notifications** - Coming soon

## Tech Stack

- **Framework**: Flutter 3.35.2 / Dart 3.9.0
- **State Management**: Provider pattern
- **HTTP Client**: Dio 5.9.0 with interceptors
- **Secure Storage**: flutter_secure_storage for tokens
- **File Handling**: file_picker, image_picker
- **Architecture**: Clean architecture with services, providers, models

## Project Structure

```
mobile/
├── lib/
│   ├── config/
│   │   └── app_config.dart        # App configuration constants
│   ├── models/
│   │   ├── user.dart               # User model
│   │   └── file_model.dart         # File/directory model
│   ├── services/
│   │   └── api_service.dart        # HTTP API client with Dio
│   ├── providers/
│   │   └── auth_provider.dart      # Authentication state management
│   ├── screens/
│   │   ├── login_screen.dart       # Login UI
│   │   ├── home_screen.dart        # Main navigation
│   │   └── files_screen.dart       # File browser
│   ├── widgets/                    # Reusable widgets
│   └── main.dart                   # App entry point
├── android/                        # Android platform code
├── ios/                            # iOS platform code
├── pubspec.yaml                    # Dependencies
└── README.md                       # This file
```

## Setup & Running

### Prerequisites

- Flutter 3.35.2 or higher
- Dart 3.9.0 or higher
- Android Studio (for Android) or Xcode (for iOS)
- SyncSpace backend running (default: http://localhost:8080)

### Installation

1. **Install dependencies**:
   ```bash
   cd mobile
   flutter pub get
   ```

2. **Configure backend URL**:
   Edit `lib/config/app_config.dart` to set your backend URL:
   ```dart
   static const String defaultBaseUrl = 'http://your-server:8080';
   ```

3. **Run on emulator/device**:
   ```bash
   # List available devices
   flutter devices
   
   # Run on specific device
   flutter run -d <device_id>
   
   # Run in debug mode
   flutter run
   
   # Run in release mode
   flutter run --release
   ```

### Android Specific Setup

1. Open `android/` folder in Android Studio
2. Sync Gradle files
3. Update `android/app/src/main/AndroidManifest.xml` for internet permissions (already configured)
4. Run: `flutter run -d android`

### iOS Specific Setup

1. Open `ios/Runner.xcworkspace` in Xcode
2. Select development team
3. Update `ios/Runner/Info.plist` for network permissions (already configured)
4. Run: `flutter run -d ios`

## Configuration

### Backend Connection

The app connects to the SyncSpace backend via REST API. Default settings in `app_config.dart`:

- **Base URL**: `http://localhost:8080`
- **API Prefix**: `/api`
- **Timeout**: 30 seconds
- **Max Upload Size**: 100 MB (chunked)

### Secure Storage Keys

Authentication tokens are stored securely using `flutter_secure_storage`:

- `auth_token` - JWT access token
- `refresh_token` - JWT refresh token
- `user_id` - Current user ID
- `username` - Current username

## API Integration

The `ApiService` class provides complete backend integration:

### Authentication
- `login(username, password)` - Login and get JWT token
- `logout()` - Clear tokens and logout
- `getUserProfile()` - Get current user profile

### File Operations
- `listFiles(path)` - List files in directory
- `uploadFile(filePath, uploadPath, onProgress)` - Upload file with progress
- `downloadFile(filePath, savePath, onProgress)` - Download file with progress
- `deleteFile(filePath)` - Delete file or directory
- `createDirectory(path)` - Create new directory

### Search & Favorites
- `searchFiles(query)` - Full-text search
- `getFavorites()` - Get user's favorite files
- `toggleFavorite(fileId)` - Star/unstar file

## State Management

The app uses the Provider pattern for state management:

### AuthProvider

Manages authentication state:

```dart
// Check if authenticated
if (authProvider.isAuthenticated) {
  // User is logged in
}

// Login
await authProvider.login('username', 'password');

// Logout
await authProvider.logout();

// Access current user
User? user = authProvider.user;
```

## Development

### Code Style

```bash
# Format code
flutter format .

# Analyze code
flutter analyze

# Run tests
flutter test
```

### Adding New Screens

1. Create screen in `lib/screens/`
2. Add route in `main.dart`
3. Add navigation in `home_screen.dart`

### Adding New API Endpoints

1. Add method in `api_service.dart`
2. Use Dio for HTTP requests
3. Handle errors and progress callbacks

## Building for Production

### Android APK

```bash
flutter build apk --release
# Output: build/app/outputs/flutter-apk/app-release.apk
```

### Android App Bundle

```bash
flutter build appbundle --release
# Output: build/app/outputs/bundle/release/app-release.aab
```

### iOS

```bash
flutter build ios --release
# Then archive in Xcode
```

## Troubleshooting

### Backend Connection Issues

- Check that backend is running on correct port
- Verify network permissions in platform manifests
- For Android emulator: Use `http://10.0.2.2:8080` instead of `localhost`
- For iOS simulator: Use your machine's IP address

### Build Errors

```bash
# Clean build cache
flutter clean
flutter pub get
flutter pub upgrade

# Rebuild
flutter build apk
```

### Dependency Issues

```bash
# Update dependencies
flutter pub upgrade --major-versions

# Check outdated packages
flutter pub outdated
```

## Default Credentials

For testing with default backend setup:

- **Username**: `admin`
- **Password**: `admin`

**⚠️ Change these in production!**

## Contributing

1. Create feature branch
2. Make changes
3. Run `flutter analyze` and `flutter test`
4. Submit pull request

## License

Same as SyncSpace backend - check root LICENSE file.

## Support

- Backend issues: See backend/README.md
- Mobile app issues: File issue on GitHub
- Feature requests: Open discussion on GitHub

## Roadmap

- [x] Core authentication
- [x] File browsing and navigation
- [x] Upload/download with progress
- [x] File operations (delete, create folder)
- [ ] Offline file access
- [ ] File sharing and permissions
- [ ] Real-time notifications
- [ ] Dark mode toggle
- [ ] Settings screen
- [ ] Multi-language support
- [ ] Biometric authentication
- [ ] Auto-upload camera photos
- [ ] Document viewer (PDF, images, videos)
