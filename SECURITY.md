# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x     | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in SyncSpace, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

### How to Report

1. Email: Create a private security advisory via GitHub (Settings → Security → Advisories)
2. Or contact the maintainer directly

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Resolution**: Depends on severity, typically 30-90 days

### Disclosure Policy

- We follow coordinated disclosure
- Credit will be given to reporters (unless anonymity is requested)
- Public disclosure after patch is released

## Security Best Practices

When deploying SyncSpace:

1. **Change default credentials** immediately after installation
2. **Enable 2FA** for all admin accounts
3. **Use HTTPS** in production (reverse proxy recommended)
4. **Keep software updated** to receive security patches
5. **Restrict network access** to trusted networks if possible
6. **Regular backups** of database and files

## Known Security Features

- JWT authentication with 24h expiration
- Argon2 password hashing
- TOTP-based two-factor authentication
- Rate limiting on authentication endpoints
- Input validation and path traversal protection
