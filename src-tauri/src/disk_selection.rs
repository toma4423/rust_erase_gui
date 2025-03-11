/// # ディスク選択モジュール
/// 
/// このモジュールは、システム上の利用可能なディスクを検出し、
/// ユーザーがディスクを選択できるようにする機能を提供します。

use crate::DiskInfo;
use std::process::Command;

/// 利用可能なディスク情報を取得する関数
pub fn get_available_disks() -> Vec<DiskInfo> {
    // lsblk コマンドでディスクのデバイス名を取得
    let output = match Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("NAME,TRAN")
        .output() {
            Ok(output) => output,
            Err(_) => {
                // Windowsの場合はダミーデータを返す（開発用）
                return get_dummy_disks();
            }
        };

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut disks = vec![];

    // 取得したデバイス名に対してhdparmを実行して詳細情報を取得
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let disk_name = parts[0].trim();
            let transport = parts[1].trim();

            // hdparmコマンドを使用してディスク情報を取得
            if let Some((model, device_type)) = get_disk_info(disk_name) {
                let disk_info = DiskInfo {
                    device_name: format!("/dev/{}", disk_name),
                    model,
                    device_type,
                    transport: transport.to_uppercase(),
                };
                disks.push(disk_info);
            }
        }
    }

    if disks.is_empty() {
        // ディスクが見つからない場合はダミーデータを返す（開発用）
        return get_dummy_disks();
    }

    disks
}

/// hdparmコマンドを使ってディスクのモデル名と回転速度（ディスクタイプ）を取得する関数
fn get_disk_info(disk_name: &str) -> Option<(String, String)> {
    // hdparm -I /dev/<disk_name> コマンドを実行
    let output = Command::new("sudo")
        .arg("hdparm")
        .arg("-I")
        .arg(format!("/dev/{}", disk_name))
        .output();

    let output_str = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => return Some((format!("Model {}", disk_name), "Unknown".to_string())),
    };

    let mut model = "Unknown Model".to_string();
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
    }

    // モデル名が見つかった場合、モデル名とディスクタイプを返す
    if model != "Unknown Model" {
        Some((model, device_type))
    } else {
        Some((format!("Model {}", disk_name), "Unknown".to_string()))
    }
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
            device_name: "/dev/sdc".to_string(),
            model: "SanDisk Ultra 32GB".to_string(),
            device_type: "SSD".to_string(),
            transport: "USB".to_string(),
        },
    ]
} 