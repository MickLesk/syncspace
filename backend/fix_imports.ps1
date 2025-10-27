# Fix service imports in API files

$apiFiles = Get-ChildItem -Path "src/api/*.rs" -File

foreach ($file in $apiFiles) {
    Write-Host "Processing: $($file.Name)"
    
    $content = Get-Content $file.FullName -Raw
    
    # Remove old service module imports
    $content = $content -replace 'use crate::services::\w+_service;', 'use crate::services;'
    
    # Fix service calls - file_service
    $content = $content -replace 'file_service::list_files', 'services::list_files'
    $content = $content -replace 'file_service::download_file', 'services::download_file'
    $content = $content -replace 'file_service::upload_file', 'services::upload_file'
    $content = $content -replace 'file_service::delete_file', 'services::delete_file'
    $content = $content -replace 'file_service::rename_file', 'services::rename_file'
    $content = $content -replace 'file_service::move_file', 'services::move_file'
    $content = $content -replace 'file_service::copy_file', 'services::copy_file'
    
    # Fix service calls - directory_service
    $content = $content -replace 'directory_service::create_directory', 'services::directory::create_directory'
    $content = $content -replace 'directory_service::delete_directory', 'services::directory::delete_directory'
    $content = $content -replace 'directory_service::move_directory', 'services::directory::move_directory'
    $content = $content -replace 'directory_service::rename_directory', 'services::directory::rename_directory'
    
    # Fix service calls - user_service
    $content = $content -replace 'user_service::get_profile', 'services::get_profile'
    $content = $content -replace 'user_service::update_profile', 'services::update_profile'
    $content = $content -replace 'user_service::get_settings', 'services::get_settings'
    $content = $content -replace 'user_service::update_settings', 'services::update_settings'
    $content = $content -replace 'user_service::get_preferences', 'services::get_preferences'
    $content = $content -replace 'user_service::update_preferences', 'services::update_preferences'
    
    # Fix service calls - search_service
    $content = $content -replace 'search_service::search', 'services::search'
    
    # Fix service calls - sharing_service
    $content = $content -replace 'sharing_service::create_share', 'services::sharing::create_share'
    $content = $content -replace 'sharing_service::list_shares', 'services::sharing::list_shares'
    $content = $content -replace 'sharing_service::delete_share', 'services::sharing::delete_share'
    
    # Fix service calls - activity_service
    $content = $content -replace 'activity_service::get_activity', 'services::activity::get_activity'
    
    # Fix service calls - tag_service
    $content = $content -replace 'tag_service::create_tag', 'services::tag::create_tag'
    $content = $content -replace 'tag_service::list_tags', 'services::tag::list_tags'
    
    # Fix service calls - favorites_service
    $content = $content -replace 'favorites_service::add_favorite', 'services::favorites::add_favorite'
    $content = $content -replace 'favorites_service::list_favorites', 'services::favorites::list_favorites'
    $content = $content -replace 'favorites_service::remove_favorite', 'services::favorites::remove_favorite'
    
    # Fix service calls - backup_service
    $content = $content -replace 'backup_service::create_backup', 'services::backup::create_backup'
    $content = $content -replace 'backup_service::list_backups', 'services::backup::list_backups'
    
    # Fix service calls - collaboration_service
    $content = $content -replace 'collaboration_service::acquire_lock', 'services::collaboration::acquire_lock'
    $content = $content -replace 'collaboration_service::release_lock', 'services::collaboration::release_lock'
    $content = $content -replace 'collaboration_service::update_presence', 'services::collaboration::update_presence'
    
    # Fix service calls - system_service
    $content = $content -replace 'system_service::get_stats', 'services::system::get_stats'
    $content = $content -replace 'system_service::get_storage_info', 'services::system::get_storage_info'
    
    # Fix service calls - auth_service (these stay the same)
    $content = $content -replace 'auth_service::register', 'services::register'
    $content = $content -replace 'auth_service::login', 'services::login'
    $content = $content -replace 'auth_service::change_password', 'services::change_password'
    $content = $content -replace 'auth_service::setup_2fa', 'services::setup_2fa'
    $content = $content -replace 'auth_service::enable_2fa', 'services::enable_2fa'
    $content = $content -replace 'auth_service::disable_2fa', 'services::disable_2fa'
    $content = $content -replace 'auth_service::refresh_token', 'services::refresh_token'
    
    Set-Content $file.FullName -Value $content -NoNewline
}

Write-Host "✅ Import fixes applied to all API files!"
