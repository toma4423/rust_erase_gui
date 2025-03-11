/// # ディスク選択モジュール
/// 
/// このモジュールは、システム上の利用可能なディスクを検出し、
/// ユーザーがディスクを選択できるようにする機能を提供します。

use crate::DiskInfo;
use crate::logger::log_message;
use std::process::Command;

/// 利用可能なディスク情報を取得する関数
pub fn get_available_disks() -> Vec<DiskInfo> {
    log_message(
        "ディスク検出",
        "開始",
        "システム上の利用可能なディスクを検出しています...",
    );

    // lsblk コマンドでディスクのデバイス名を取得
    let output = match Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("NAME,TRAN")
        .output() {
            Ok(output) => output,
            Err(e) => {
                log_message(
                    "ディスク検出エラー",
                    "エラー",
                    &format!("lsblkコマンドの実行に失敗しました: {}", e),
                );
                // エラーの場合はダミーデータを返す（開発用）
                return get_dummy_disks();
            }
        };

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut disks = vec![];

    // 取得したデバイス名に対してhdparmを実行して詳細情報を取得
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let disk_name = parts[0].trim();
        let transport = if parts.len() >= 2 { parts[1].trim() } else { "unknown" };

        // NVMeディスクの場合
        if disk_name.starts_with("nvme") {
            if let Some(disk_info) = get_nvme_disk_info(disk_name) {
                disks.push(disk_info);
                continue;
            }
        }

        // SATA/USBディスクの場合
        if let Some(disk_info) = get_sata_disk_info(disk_name, transport) {
            disks.push(disk_info);
        }
    }

    if disks.is_empty() {
        log_message(
            "ディスク検出",
            "警告",
            "利用可能なディスクが見つかりませんでした。ダミーデータを使用します。",
        );
        // ディスクが見つからない場合はダミーデータを返す（開発用）
        return get_dummy_disks();
    }

    log_message(
        "ディスク検出",
        "完了",
        &format!("{}台のディスクが検出されました。", disks.len()),
    );

    disks
}

/// NVMeディスクの情報を取得する関数
fn get_nvme_disk_info(disk_name: &str) -> Option<DiskInfo> {
    log_message(
        "NVMeディスク検出",
        "情報",
        &format!("NVMeディスク {} の情報を取得しています...", disk_name),
    );

    // nvme id-ctrl コマンドを実行
    let output = Command::new("sudo")
        .arg("nvme")
        .arg("id-ctrl")
        .arg(format!("/dev/{}", disk_name))
        .arg("-H")
        .output();

    let output_str = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            log_message(
                "NVMeディスク検出エラー",
                "警告",
                &format!("nvmeコマンドの実行に失敗しました: {}", e),
            );
            return Some(DiskInfo {
                device_name: format!("/dev/{}", disk_name),
                model: format!("NVMe Drive {}", disk_name),
                device_type: "SSD".to_string(), // NVMeはSSDとして扱う
                transport: "NVMe".to_string(),
            });
        }
    };

    let mut model = format!("NVMe Drive {}", disk_name);

    // モデル名を抽出
    for line in output_str.lines() {
        if line.contains("mn") || line.contains("Model Number") {
            if let Some(model_part) = line.split(":").nth(1) {
                model = model_part.trim().to_string();
                break;
            }
        }
    }

    Some(DiskInfo {
        device_name: format!("/dev/{}", disk_name),
        model,
        device_type: "SSD".to_string(), // NVMeはSSDとして扱う
        transport: "NVMe".to_string(),
    })
}

/// SATA/USBディスクの情報を取得する関数
fn get_sata_disk_info(disk_name: &str, transport: &str) -> Option<DiskInfo> {
    log_message(
        "SATAディスク検出",
        "情報",
        &format!("SATAディスク {} の情報を取得しています...", disk_name),
    );

    // hdparm -I コマンドを実行
    let output = Command::new("sudo")
        .arg("hdparm")
        .arg("-I")
        .arg(format!("/dev/{}", disk_name))
        .output();

    let output_str = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            log_message(
                "SATAディスク検出エラー",
                "警告",
                &format!("hdparmコマンドの実行に失敗しました: {}", e),
            );
            return Some(DiskInfo {
                device_name: format!("/dev/{}", disk_name),
                model: format!("Disk {}", disk_name),
                device_type: "Unknown".to_string(),
                transport: transport.to_uppercase(),
            });
        }
    };

    let mut model = format!("Disk {}", disk_name);
    let mut device_type = "Unknown".to_string();

    // hdparmの出力からモデル名と回転速度を取得
    for line in output_str.lines() {
        if line.contains("Model Number") {
            // モデル名の抽出
            if let Some(model_number) = line.split(":").nth(1) {
                model = model_number.trim().to_string();
            }
        }

        if line.contains("Rotation Rate") {
            // 回転速度からディスクタイプを判別
            if let Some(rotation_rate) = line.split(":").nth(1) {
                let rotation_rate = rotation_rate.trim();
                if rotation_rate == "Solid State Device" {
                    device_type = "SSD".to_string();
                } else if let Ok(rate) = rotation_rate.parse::<u32>() {
                    if rate > 0 {
                        device_type = "HDD".to_string();
                    }
                }
            }
        }

        // SSDの特徴的な文字列を検索
        if device_type == "Unknown" && (
            line.contains("Solid State") || 
            line.contains("SSD") || 
            line.contains("Flash")
        ) {
            device_type = "SSD".to_string();
        }
    }

    // ディスクタイプが不明な場合、モデル名からの推測を試みる
    if device_type == "Unknown" {
        let model_lower = model.to_lowercase();
        if model_lower.contains("ssd") || 
           model_lower.contains("solid") || 
           model_lower.contains("flash") || 
           model_lower.contains("nvme") {
            device_type = "SSD".to_string();
        } else if model_lower.contains("hdd") || 
                model_lower.contains("hard drive") || 
                model_lower.contains("harddisk") {
            device_type = "HDD".to_string();
        }
    }

    Some(DiskInfo {
        device_name: format!("/dev/{}", disk_name),
        model,
        device_type,
        transport: transport.to_uppercase(),
    })
}

/// 開発用のダミーディスクデータを生成する関数
fn get_dummy_disks() -> Vec<DiskInfo> {
    vec![
        DiskInfo {
            device_name: "/dev/sda".to_string(),
            model: "Samsung SSD 970 EVO Plus 1TB".to_string(),
            device_type: "SSD".to_string(),
            transport: "SATA".to_string(),
        },
        DiskInfo {
            device_name: "/dev/sdb".to_string(),
            model: "WD Blue 2TB".to_string(),
            device_type: "HDD".to_string(),
            transport: "SATA".to_string(),
        },
        DiskInfo {
            device_name: "/dev/nvme0n1".to_string(),
            model: "Samsung PM9A1 NVMe 512GB".to_string(),
            device_type: "SSD".to_string(),
            transport: "NVME".to_string(),
        },
    ]
} 