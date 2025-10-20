// ========================
// i18n Translations
// ========================

const translations = {
  en: {
    // Auth
    'app.title': 'SyncSpace',
    'app.subtitle': 'Secure File Synchronization',
    'login.title': 'Sign In',
    'login.username': 'Username',
    'login.password': 'Password',
    'login.2fa': '2FA Code',
    'login.button': 'Sign In',
    'login.success': 'Login successful!',
    'login.failed': 'Login failed',
    'login.2fa.required': 'Please enter your 2FA code',
    'logout.confirm': 'Are you sure you want to logout?',
    'logout.success': 'Logged out successfully',
    
    // Navigation
    'nav.files': 'Files',
    'nav.search': 'Search',
    'nav.peers': 'Peers',
    'nav.settings': 'Settings',
    'nav.logout': 'Logout',
    
    // Files
    'files.title': 'Files',
    'files.empty': 'No files in this directory',
    'files.upload': 'Upload File',
    'files.choose': 'Choose File',
    'files.dragdrop': 'Drag & Drop files here',
    'files.or': 'or use the button below',
    'files.newdir': 'New Folder',
    'files.newdir.placeholder': 'New directory name',
    'files.create': 'Create Directory',
    'files.rename': 'Rename',
    'files.delete': 'Delete',
    'files.download': 'Download',
    'files.preview': 'Preview',
    
    // Dialogs
    'dialog.rename.title': 'Rename Item',
    'dialog.rename.label': 'New name',
    'dialog.rename.cancel': 'Cancel',
    'dialog.rename.confirm': 'Rename',
    'dialog.delete.title': 'Delete Confirmation',
    'dialog.delete.message': 'Are you sure you want to delete',
    'dialog.delete.warning': 'This action cannot be undone!',
    'dialog.delete.cancel': 'Cancel',
    'dialog.delete.confirm': 'Delete',
    'dialog.mkdir.title': 'Create Directory',
    'dialog.mkdir.label': 'Directory name',
    'dialog.mkdir.cancel': 'Cancel',
    'dialog.mkdir.confirm': 'Create',
    
    // Search
    'search.title': 'Search Files',
    'search.placeholder': 'Search files and directories...',
    'search.button': 'Search',
    'search.results': 'results found',
    'search.empty': 'No results found',
    
    // Settings
    'settings.title': 'Settings',
    'settings.appearance': 'Appearance',
    'settings.security': 'Security',
    'settings.account': 'Account',
    'settings.language': 'Language',
    'settings.darkmode': 'Dark Mode',
    'settings.darkmode.desc': 'Switch between light and dark themes',
    'settings.2fa': 'Two-Factor Authentication',
    'settings.2fa.status': 'Not enabled',
    'settings.2fa.enabled': 'Enabled',
    'settings.2fa.setup': 'Setup 2FA',
    'settings.2fa.disable': 'Disable 2FA',
    'settings.password': 'Change Password',
    'settings.password.desc': 'Update your account password',
    'settings.password.change': 'Change',
    
    // Stats
    'stats.files': 'files',
    'stats.total': 'total',
    
    // Notifications
    'notify.uploaded': 'Uploaded',
    'notify.downloaded': 'Downloaded',
    'notify.deleted': 'Deleted',
    'notify.renamed': 'Renamed to',
    'notify.created': 'Created',
    'notify.error': 'Error',
    
    // Common
    'common.loading': 'Loading...',
    'common.save': 'Save',
    'common.cancel': 'Cancel',
    'common.confirm': 'Confirm',
    'common.close': 'Close',
  },
  de: {
    // Auth
    'app.title': 'SyncSpace',
    'app.subtitle': 'Sichere Dateisynchronisation',
    'login.title': 'Anmelden',
    'login.username': 'Benutzername',
    'login.password': 'Passwort',
    'login.2fa': '2FA Code',
    'login.button': 'Anmelden',
    'login.success': 'Anmeldung erfolgreich!',
    'login.failed': 'Anmeldung fehlgeschlagen',
    'login.2fa.required': 'Bitte 2FA Code eingeben',
    'logout.confirm': 'Möchten Sie sich wirklich abmelden?',
    'logout.success': 'Erfolgreich abgemeldet',
    
    // Navigation
    'nav.files': 'Dateien',
    'nav.search': 'Suche',
    'nav.peers': 'Peers',
    'nav.settings': 'Einstellungen',
    'nav.logout': 'Abmelden',
    
    // Files
    'files.title': 'Dateien',
    'files.empty': 'Keine Dateien in diesem Verzeichnis',
    'files.upload': 'Datei hochladen',
    'files.choose': 'Datei wählen',
    'files.dragdrop': 'Dateien hier ablegen',
    'files.or': 'oder Button verwenden',
    'files.newdir': 'Neuer Ordner',
    'files.newdir.placeholder': 'Name des neuen Ordners',
    'files.create': 'Ordner erstellen',
    'files.rename': 'Umbenennen',
    'files.delete': 'Löschen',
    'files.download': 'Herunterladen',
    'files.preview': 'Vorschau',
    
    // Dialogs
    'dialog.rename.title': 'Element umbenennen',
    'dialog.rename.label': 'Neuer Name',
    'dialog.rename.cancel': 'Abbrechen',
    'dialog.rename.confirm': 'Umbenennen',
    'dialog.delete.title': 'Löschbestätigung',
    'dialog.delete.message': 'Möchten Sie wirklich löschen',
    'dialog.delete.warning': 'Diese Aktion kann nicht rückgängig gemacht werden!',
    'dialog.delete.cancel': 'Abbrechen',
    'dialog.delete.confirm': 'Löschen',
    'dialog.mkdir.title': 'Ordner erstellen',
    'dialog.mkdir.label': 'Ordnername',
    'dialog.mkdir.cancel': 'Abbrechen',
    'dialog.mkdir.confirm': 'Erstellen',
    
    // Search
    'search.title': 'Dateien durchsuchen',
    'search.placeholder': 'Dateien und Ordner durchsuchen...',
    'search.button': 'Suchen',
    'search.results': 'Ergebnisse gefunden',
    'search.empty': 'Keine Ergebnisse gefunden',
    
    // Settings
    'settings.title': 'Einstellungen',
    'settings.appearance': 'Erscheinungsbild',
    'settings.security': 'Sicherheit',
    'settings.account': 'Konto',
    'settings.language': 'Sprache',
    'settings.darkmode': 'Dunkelmodus',
    'settings.darkmode.desc': 'Zwischen hellem und dunklem Design wechseln',
    'settings.2fa': 'Zwei-Faktor-Authentifizierung',
    'settings.2fa.status': 'Nicht aktiviert',
    'settings.2fa.enabled': 'Aktiviert',
    'settings.2fa.setup': '2FA einrichten',
    'settings.2fa.disable': '2FA deaktivieren',
    'settings.password': 'Passwort ändern',
    'settings.password.desc': 'Kontopasswort aktualisieren',
    'settings.password.change': 'Ändern',
    
    // Stats
    'stats.files': 'Dateien',
    'stats.total': 'gesamt',
    
    // Notifications
    'notify.uploaded': 'Hochgeladen',
    'notify.downloaded': 'Heruntergeladen',
    'notify.deleted': 'Gelöscht',
    'notify.renamed': 'Umbenannt zu',
    'notify.created': 'Erstellt',
    'notify.error': 'Fehler',
    
    // Common
    'common.loading': 'Wird geladen...',
    'common.save': 'Speichern',
    'common.cancel': 'Abbrechen',
    'common.confirm': 'Bestätigen',
    'common.close': 'Schließen',
  }
};

let currentLang = localStorage.getItem('language') || 'en';

function t(key) {
  return translations[currentLang][key] || key;
}

function setLanguage(lang) {
  currentLang = lang;
  localStorage.setItem('language', lang);
  updateAllTranslations();
}

function updateAllTranslations() {
  // Update all elements with data-i18n attribute
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    if (el.tagName === 'INPUT' && el.hasAttribute('placeholder')) {
      el.placeholder = t(key);
    } else {
      el.textContent = t(key);
    }
  });
}
