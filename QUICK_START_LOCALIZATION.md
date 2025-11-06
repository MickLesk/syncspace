# Quick Start Guide - Continue Localization

## 🎯 What's Done

- ✅ SetupWizard.svelte (100% - 845 lines)
- ✅ ModalPortal.svelte (100% - all modals)
- ✅ RecentFilesView.svelte (100%)
- ✅ DuplicatesView.svelte (100%)
- ✅ TrashView.svelte (100%)
- ✅ FilterBar.svelte (100%)
- ✅ i18n.js - 850+ keys (German + English)
- ✅ Full documentation & checklists created

**Progress: 7 files / 120 = 5.8% Complete**

---

## 🚀 Phase 1: CRITICAL (Start Here!)

### Task 1: Localize Login.svelte

**File:** `frontend/src/pages/auth/Login.svelte`  
**Time:** 2-3 hours  
**Priority:** 🔴 CRITICAL

**What needs tr():**

- Page title "SyncSpace" / "Login"
- Form labels: "Username", "Password"
- Placeholders: "Enter username", "Enter password"
- Button label: "Login"
- Links: "Forgot Password", "Sign up here"
- Demo credentials text
- Error messages: "Invalid credentials", "Login failed"
- Loading text: "Logging in..."

**Steps:**

1. Add imports at top of script:

```svelte
import { currentLang } from "../stores/ui.js";
import { t } from "../i18n.js";
```

2. Add tr derivation after imports:

```svelte
const tr = $derived((key, ...args) => t($currentLang, key, ...args));
```

3. Replace all hardcoded strings with `tr()` calls

**Needed Translation Keys:**

```javascript
// German
loginTitle: 'Anmelden',
username: 'Benutzername',
password: 'Passwort',
enterUsername: 'Benutzername eingeben',
enterPassword: 'Passwort eingeben',
login: 'Anmelden',
forgotPassword: 'Passwort vergessen',
demoCredentials: 'Demo-Anmeldedaten',
invalidCredentials: 'Ungültige Anmeldedaten',
loggingIn: 'Anmeldung läuft...',
noAccount: 'Kein Konto? Hier anmelden',
signUpHere: 'Hier anmelden',

// English
loginTitle: 'Sign In',
username: 'Username',
password: 'Password',
enterUsername: 'Enter username',
enterPassword: 'Enter password',
login: 'Login',
forgotPassword: 'Forgot Password',
demoCredentials: 'Demo Credentials',
invalidCredentials: 'Invalid credentials',
loggingIn: 'Logging in...',
noAccount: "Don't have an account? Sign up here",
signUpHere: 'Sign up here',
```

---

### Task 2: Localize Signup.svelte

**File:** `frontend/src/pages/auth/Signup.svelte`  
**Time:** 1.5-2 hours  
**Priority:** 🔴 CRITICAL

**What needs tr():**

- Page title
- Form labels: Username, Email, Password, Confirm Password
- Password requirement checkboxes
- Terms & conditions checkbox
- Buttons: Create Account, Cancel
- Validation messages
- Error states

**Similar to Login - use same pattern + new keys for sign-up specific terms**

---

### Task 3: Localize FilesView.svelte

**File:** `frontend/src/pages/files/FilesView.svelte`  
**Time:** 1.5-2 hours  
**Priority:** 🔴 CRITICAL

**What needs tr():**

- Toolbar buttons: Upload, New Folder, etc.
- Column headers: Name, Size, Modified, Owner
- View mode buttons: Grid, List
- Sort options
- Context menu items
- Empty state: "No files", "Drag & drop"
- Selection: "X files selected"
- Bulk actions: Delete, Move, Copy

---

## 📋 Phases 2-5 Quick Reference

### Phase 2: Admin & Security (3-4 hours)

1. UsersView.svelte
2. SecurityView.svelte
3. SettingsView.svelte + complete GeneralSettings.svelte

### Phase 3: Settings (3-4 hours)

1. SecuritySettings.svelte
2. StorageSettings.svelte
3. BackupSettings.svelte
4. PerformanceSettings.svelte
5. UsersSettings.svelte
6. AboutSettings.svelte

### Phase 4: Components (5-6 hours)

1. FileUploadZone.svelte
2. FileActionsMenu.svelte
3. FileToolbar.svelte
4. Key UI components

### Phase 5: Remaining (3-4 hours)

- All other pages and components
- Showcase pages (low priority)

---

## 🛠️ How to Add New Translation Keys

Edit `frontend/src/i18n.js`:

```javascript
// Find German section (starts ~line 1)
export const translations = {
  de: {
    // ... existing keys ...
    // ADD YOUR KEYS HERE
    myNewKey: "German text",
  },

  en: {
    // ... existing keys ...
    // ADD ENGLISH TRANSLATION HERE
    myNewKey: "English text",
  },
};
```

**Key Naming Convention:**

- Use camelCase: `userSettingsTitle`, `deleteConfirm`, `errorOccurred`
- Group related keys: `login`, `loginFailed`, `loginTitle`
- Use descriptive names: `usernameRequired` not `err1`

---

## ✅ Testing Checklist After Each File

1. Check for errors: `npm run dev` in frontend terminal
2. Verify tr() calls work - check browser console
3. Test language switch - confirm text changes
4. Check placeholder text (in inputs)
5. Verify tooltips/aria-labels if present
6. Test error states - confirm messages translate

---

## 📚 Reference Files

1. **LOCALIZATION_AUDIT.md** - Comprehensive overview
2. **COMPONENTS_TRANSLATION_CHECKLIST.md** - All 84 components listed
3. **PAGES_TRANSLATION_CHECKLIST.md** - All 36 pages listed
4. **LOCALIZATION_SESSION_COMPLETE.md** - Session report & status

---

## 🔑 Key Translation Categories

**Already in i18n.js:**

- Navigation (files, shared, favorites, trash, users, settings, profile)
- Common actions (upload, download, delete, rename, share, copy, move)
- Time labels (just now, minutes ago, hours ago, days ago)
- File types (image, video, audio, document, archive, code)
- Error messages (login failed, upload failed, delete failed, etc.)
- Setup wizard (all 6 steps + validation)
- Settings (theme, language, default view)
- Colors (blue, green, purple, orange, pink, red, yellow, cyan)

**Still needed:**

- Auth pages (login, signup, forgot password)
- File operations (new folder, upload, drag & drop)
- User management (add user, edit user, delete user, roles)
- Backup/storage (backup schedule, quota, retention)
- Notifications (mark as read, delete, notification types)

---

## ⚡ Tips for Efficiency

1. **Use Find & Replace**

   - Search for hardcoded strings
   - Replace with `{tr("keyName")}`
   - Verify it matches expected translation key

2. **Group Similar Files**

   - Do all Settings pages together
   - Do auth pages back-to-back
   - Share pattern between similar files

3. **Copy-Paste Keys**

   - When in doubt, copy key from working file
   - SetupWizard, ModalPortal, TrashView are good references
   - Use pattern: `{tr("keyName")}`

4. **Batch Operations**
   - Consider Python/Node script for bulk replacements
   - Would save ~20-30% of time on large files

---

## 🎯 Next Session Goal

**Recommended:** Complete Phase 1 (Login + Signup + FilesView)

- 3 critical files = 4-5 hours
- Unblocks deployment
- Establishes pattern for rest of project

**Then:** Continue with Phase 2 (Admin pages) next session

---

## 📞 Quick Commands

```bash
# Check for i18n errors
npm run lint

# Test current setup
npm run dev

# Search for untranslated strings (example)
grep -r "Loading..." frontend/src/components/ --include="*.svelte"
```

---

**Start with Task 1: Login.svelte** - You've got this! 🚀
