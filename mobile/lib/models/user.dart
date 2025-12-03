/// User Model
class User {
  final String id;
  final String username;
  final String? email;
  final String? displayName;
  final String? bio;
  final String? avatarBase64;
  final String role;
  final String theme;
  final String language;
  final String defaultView;
  final DateTime createdAt;
  final DateTime? lastLogin;

  User({
    required this.id,
    required this.username,
    this.email,
    this.displayName,
    this.bio,
    this.avatarBase64,
    required this.role,
    this.theme = 'dark',
    this.language = 'en',
    this.defaultView = 'files',
    required this.createdAt,
    this.lastLogin,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'] as String,
      username: json['username'] as String,
      email: json['email'] as String?,
      displayName: json['display_name'] as String?,
      bio: json['bio'] as String?,
      avatarBase64: json['avatar_base64'] as String?,
      role: json['role'] as String? ?? 'user',
      theme: json['theme'] as String? ?? 'dark',
      language: json['language'] as String? ?? 'en',
      defaultView: json['default_view'] as String? ?? 'files',
      createdAt: DateTime.parse(json['created_at'] as String),
      lastLogin: json['last_login'] != null
          ? DateTime.parse(json['last_login'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'username': username,
      'email': email,
      'display_name': displayName,
      'bio': bio,
      'avatar_base64': avatarBase64,
      'role': role,
      'theme': theme,
      'language': language,
      'default_view': defaultView,
      'created_at': createdAt.toIso8601String(),
      'last_login': lastLogin?.toIso8601String(),
    };
  }

  User copyWith({
    String? id,
    String? username,
    String? email,
    String? displayName,
    String? bio,
    String? avatarBase64,
    String? role,
    String? theme,
    String? language,
    String? defaultView,
    DateTime? createdAt,
    DateTime? lastLogin,
  }) {
    return User(
      id: id ?? this.id,
      username: username ?? this.username,
      email: email ?? this.email,
      displayName: displayName ?? this.displayName,
      bio: bio ?? this.bio,
      avatarBase64: avatarBase64 ?? this.avatarBase64,
      role: role ?? this.role,
      theme: theme ?? this.theme,
      language: language ?? this.language,
      defaultView: defaultView ?? this.defaultView,
      createdAt: createdAt ?? this.createdAt,
      lastLogin: lastLogin ?? this.lastLogin,
    );
  }
}
