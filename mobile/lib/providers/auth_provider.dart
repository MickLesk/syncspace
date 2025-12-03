import 'package:flutter/material.dart';
import '../models/user.dart';
import '../services/api_service.dart';

/// Auth Provider - Manages authentication state
class AuthProvider with ChangeNotifier {
  final ApiService _apiService;
  User? _user;
  bool _isAuthenticated = false;
  bool _isLoading = false;
  String? _error;

  AuthProvider(this._apiService);

  User? get user => _user;
  bool get isAuthenticated => _isAuthenticated;
  bool get isLoading => _isLoading;
  String? get error => _error;

  /// Check if user is authenticated (has valid token)
  Future<void> checkAuth() async {
    _isLoading = true;
    notifyListeners();

    try {
      final token = await _apiService.getAuthToken();
      if (token != null) {
        // Try to get user profile to validate token
        final profileData = await _apiService.getUserProfile();
        _user = User.fromJson(profileData);
        _isAuthenticated = true;
        _error = null;
      } else {
        _isAuthenticated = false;
        _user = null;
      }
    } catch (e) {
      _error = e.toString();
      _isAuthenticated = false;
      _user = null;
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }

  /// Login
  Future<bool> login(String username, String password) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final response = await _apiService.login(username, password);
      _user = User.fromJson(response['user'] as Map<String, dynamic>);
      _isAuthenticated = true;
      _error = null;
      _isLoading = false;
      notifyListeners();
      return true;
    } catch (e) {
      _error = e.toString();
      _isAuthenticated = false;
      _user = null;
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  /// Logout
  Future<void> logout() async {
    await _apiService.logout();
    _user = null;
    _isAuthenticated = false;
    _error = null;
    notifyListeners();
  }

  /// Set server URL
  Future<void> setServerUrl(String url) async {
    await _apiService.setServerUrl(url);
  }

  /// Get server URL
  Future<String> getServerUrl() async {
    return await _apiService.getServerUrl();
  }
}
