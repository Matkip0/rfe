use serde::Serialize;
use std::{fs, path::PathBuf, process::Command};
use sysinfo::{DiskKind, Disks};

#[derive(Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub progress: u8,
    pub color: String,
    pub max_space: String,
    pub used_space: String,
}

#[cfg(target_os = "linux")]
fn get_volume_label(device: &str) -> Option<String> {
    if let Ok(entries) = fs::read_dir("/dev/disk/by-label") {
        for entry in entries.filter_map(Result::ok) {
            let label_name = entry.file_name().to_string_lossy().into_owned();
            if let Ok(target) = fs::read_link(entry.path()) {
                let dev = PathBuf::from("/dev").join(target.file_name()?);
                if dev.to_string_lossy() == device {
                    return Some(label_name);
                }
            }
        }
    }
    None
}
#[cfg(not(target_os = "linux"))]
fn get_volume_label(_device: &str) -> Option<String> {
    None
}

#[tauri::command]
pub fn get_devices() -> Result<Vec<DeviceInfo>, String> {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = mount_usb_devices() {
            eprintln!("Failed to mount USB devices: {}", e);
        }
    }

    let disks = Disks::new_with_refreshed_list();
    let mut devices = Vec::new();

    for disk in disks.iter() {
        let mount_point = disk.mount_point().to_string_lossy().to_string();

        #[cfg(target_os = "linux")]
        let include = mount_point.starts_with("/media/")
            || mount_point.starts_with("/run/media/")
            || mount_point.starts_with("/mnt/")
            || mount_point == "/";

        #[cfg(target_os = "macos")]
        let include = mount_point.starts_with("/Volumes/") || mount_point == "/";

        #[cfg(target_os = "windows")]
        let include = mount_point.ends_with(":\\");

        if !include {
            continue;
        }

        let total = disk.total_space();
        let available = disk.available_space();
        if total == 0 {
            eprintln!("Disk {} has total space of 0, skipping", mount_point);
            continue;
        }

        let used = total - available;
        let used_percentage = ((used as f64 / total as f64) * 100.0).round() as u8;

        let size_label = format_size(total);
        let used_label = format_size(used);

        let kind = disk.kind();
        let device_path = disk.name().to_string_lossy().to_string();

        let label = get_volume_label(&device_path)
            .or_else(|| {
                if mount_point != "/"
                    && (mount_point.starts_with("/media/")
                        || mount_point.starts_with("/run/media/"))
                {
                    mount_point.split('/').next_back().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| match kind {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                DiskKind::Unknown(_) => "Removable".to_string(),
            });

        let name = label;

        devices.push(DeviceInfo {
            name,
            progress: used_percentage,
            color: if used_percentage >= 90 {
                "bg-rfe-red".to_string()
            } else {
                "bg-rfe-blue".to_string()
            },
            max_space: size_label,
            used_space: used_label,
        });
    }

    Ok(devices)
}

#[cfg(target_os = "linux")]
fn mount_usb_devices() -> Result<(), std::io::Error> {
    let usb_devices = fs::read_dir("/dev/disk/by-id")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().into_owned();

            if file_name.contains("usb") && !file_name.contains("part") {
                let path = fs::read_link(entry.path()).ok()?;
                Some(format!("/dev/{}", path.file_name()?.to_string_lossy()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for device in usb_devices {
        match Command::new("udisksctl")
            .args(["mount", "-b", &device])
            .status()
        {
            Ok(status) => {
                if status.success() {
                    println!("Mounted USB device {}", device);
                } else {
                    eprintln!("udisksctl failed for {} with non-zero exit", device);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute udisksctl for {}: {}", device, e);
            }
        }
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const GIB: u64 = 1 << 30;
    const TIB: u64 = 1 << 40;

    if bytes >= TIB {
        format!("{:.1} TB", bytes as f64 / TIB as f64)
    } else {
        format!("{:.0} GiB", bytes as f64 / GIB as f64)
    }
}
