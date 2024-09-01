use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn read_permission(path: &str) -> u32 {
    let metadata = fs::metadata(path).expect("Unable to read metadata.");
    let permissions = metadata.permissions();
    permissions.mode()
}

pub fn format_permissions(path: &str, mode: u32) -> String {
    let metadata = fs::metadata(path).expect("Unable to read metadata.");
    let mut permissions = String::new();

    permissions.push(if metadata.is_dir() { 'd' } else { '-' });

    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    permissions
}
