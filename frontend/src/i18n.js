// i18n translations for SyncSpace
export const translations = {
  de: {
    // Navigation
    files: 'Dateien',
    shared: 'Geteilt',
    favorites: 'Favoriten',
    trash: 'Papierkorb',
    users: 'Benutzer',
    settings: 'Einstellungen',
    profile: 'Profil',
    
    // Common actions
    upload: 'Hochladen',
    download: 'Herunterladen',
    delete: 'Löschen',
    rename: 'Umbenennen',
    newFolder: 'Neuer Ordner',
    search: 'Suchen',
    logout: 'Abmelden',
    login: 'Anmelden',
    cancel: 'Abbrechen',
    save: 'Speichern',
    close: 'Schließen',
    
    // Login page
    loginTitle: 'SyncSpace',
    loginSubtitle: 'Moderne Datei-Synchronisation',
    username: 'Benutzername',
    password: 'Passwort',
    loggingIn: 'Anmeldung läuft...',
    demoCredentials: 'Demo-Zugangsdaten',
    enterUsername: 'Benutzername eingeben',
    enterPassword: 'Passwort eingeben',
    
    // Files view
    noFiles: 'Noch keine Dateien',
    dragAndDropHere: 'Dateien hier ablegen oder klicken zum Hochladen',
    uploadFiles: 'Dateien hochladen',
    
    // Shared view
    noSharedFiles: 'Noch keine geteilten Dateien',
    shareWithOthers: 'Teilen Sie Dateien mit anderen Benutzern',
    
    // Favorites view
    noFavorites: 'Noch keine Favoriten',
    markFilesAsFavorite: 'Markieren Sie Dateien als Favoriten',
    
    // Trash view
    trashIsEmpty: 'Papierkorb ist leer',
    deletedFilesStored: 'Gelöschte Dateien werden 30 Tage hier gespeichert',
    emptyTrash: 'Papierkorb leeren',
    
    // Users view
    addUser: 'Benutzer hinzufügen',
    created: 'Erstellt',
    twoFactorAuth: '2FA',
    actions: 'Aktionen',
    enabled: 'Aktiviert',
    disabled: 'Deaktiviert',
    
    // Settings
    appearance: 'Erscheinungsbild',
    theme: 'Theme',
    themeDescription: 'Wählen Sie Ihr bevorzugtes Farbschema',
    light: 'Hell',
    dark: 'Dunkel',
    language: 'Sprache',
    interfaceLanguage: 'Oberflächensprache',
    languageDescription: 'Wählen Sie Ihre bevorzugte Sprache',
    storage: 'Speicher',
    storageLocation: 'Speicherort',
    filesStoredIn: 'Dateien werden gespeichert in:',
    cache: 'Cache',
    cacheDescription: 'Anwendungs-Cache leeren',
    clearCache: 'Cache leeren',
    about: 'Über',
    version: 'Version',
    frontend: 'Frontend',
    backend: 'Backend',
    design: 'Design',
    
    // Errors
    errorOccurred: 'Ein Fehler ist aufgetreten',
    loginFailed: 'Anmeldung fehlgeschlagen',
    uploadFailed: 'Upload fehlgeschlagen',
    deleteFailed: 'Löschen fehlgeschlagen',
    renameFailed: 'Umbenennen fehlgeschlagen',
    
    // Success messages
    uploadSuccess: 'Erfolgreich hochgeladen',
    deleteSuccess: 'Erfolgreich gelöscht',
    renameSuccess: 'Erfolgreich umbenannt',
  },
  
  en: {
    // Navigation
    files: 'Files',
    shared: 'Shared',
    favorites: 'Favorites',
    trash: 'Trash',
    profile: 'Profile',
    users: 'Users',
    settings: 'Settings',
    
    // Common actions
    upload: 'Upload',
    download: 'Download',
    delete: 'Delete',
    rename: 'Rename',
    newFolder: 'New Folder',
    search: 'Search',
    logout: 'Logout',
    login: 'Login',
    cancel: 'Cancel',
    save: 'Save',
    close: 'Close',
    
    // Login page
    loginTitle: 'SyncSpace',
    loginSubtitle: 'Modern File Synchronization',
    username: 'Username',
    password: 'Password',
    loggingIn: 'Logging in...',
    demoCredentials: 'Demo credentials',
    enterUsername: 'Enter username',
    enterPassword: 'Enter password',
    
    // Files view
    noFiles: 'No files yet',
    dragAndDropHere: 'Drag and drop files here or click to upload',
    uploadFiles: 'Upload Files',
    
    // Shared view
    noSharedFiles: 'No shared files yet',
    shareWithOthers: 'Share files with other users',
    
    // Favorites view
    noFavorites: 'No favorites yet',
    markFilesAsFavorite: 'Mark files as favorites',
    
    // Trash view
    trashIsEmpty: 'Trash is empty',
    deletedFilesStored: 'Deleted files will be stored here for 30 days',
    emptyTrash: 'Empty Trash',
    
    // Users view
    addUser: 'Add User',
    created: 'Created',
    twoFactorAuth: '2FA',
    actions: 'Actions',
    enabled: 'Enabled',
    disabled: 'Disabled',
    
    // Settings
    appearance: 'Appearance',
    theme: 'Theme',
    themeDescription: 'Choose your preferred color scheme',
    light: 'Light',
    dark: 'Dark',
    language: 'Language',
    interfaceLanguage: 'Interface Language',
    languageDescription: 'Choose your preferred language',
    storage: 'Storage',
    storageLocation: 'Storage Location',
    filesStoredIn: 'Files are stored in:',
    cache: 'Cache',
    cacheDescription: 'Clear application cache',
    clearCache: 'Clear Cache',
    about: 'About',
    version: 'Version',
    frontend: 'Frontend',
    backend: 'Backend',
    design: 'Design',
    
    // Errors
    errorOccurred: 'An error occurred',
    loginFailed: 'Login failed',
    uploadFailed: 'Upload failed',
    deleteFailed: 'Delete failed',
    renameFailed: 'Rename failed',
    
    // Success messages
    uploadSuccess: 'Successfully uploaded',
    deleteSuccess: 'Successfully deleted',
    renameSuccess: 'Successfully renamed',
  }
};

// Translation helper function
export function t(lang, key) {
  return translations[lang]?.[key] || translations['en'][key] || key;
}