/// # SSD消去モジュール
/// 
/// このモジュールは、SSDのデータを安全に消去するための機能を提供します。
/// ATA Secure Eraseコマンドを使用して、SSDのデータを完全に消去します。

use crate::logger::log_message;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// SSDをSecure Eraseで消去する関数
/// 
/// # 引数
/// 
/// * `device` - 消去対象のデバイス名（例: /dev/sda）
/// 
/// # 戻り値
/// 
/// * `Result<(), String>` - 成功時は`Ok(())`、失敗時はエラーメッセージを含む`Err`
pub fn secure_erase_ssd(device: &str) -> Result<(), String> {
    let device_name = device.split_whitespace().next().unwrap_or(device);

    log_message(
        &format!("Secure Erase開始: {}", device_name),
        "進行中",
        "デバイスタイプに基づいてSecure Eraseを実行します。",
    );

    // Windowsでの開発環境では実際の消去処理をシミュレーションする
    #[cfg(not(target_os = "linux"))]
    {
        // 開発環境では消去をシミュレーション
        log_message(
            &format!("{}のSecure Eraseをシミュレーション中", device_name),
            "進行中",
            "シミュレーションモード",
        );
        
        // 進行状況のシミュレーション
        thread::sleep(Duration::from_secs(5));
        
        log_message(
            &format!("Secure Erase完了: {}", device_name),
            "成功",
            "シミュレーションモードでの消去が完了しました。",
        );
        
        return Ok(());
    }

    // 実際のLinux環境での消去処理
    #[cfg(target_os = "linux")]
    {
        if device.contains("SATA") {
            // パスワードを設定（ATA Secure Erase）
            let output = Command::new("hdparm")
                .arg("--user-master")
                .arg("u")
                .arg("--security-set-pass")
                .arg("0000")
                .arg(device_name)
                .output()
                .map_err(|e| format!("パスワード設定に失敗しました: {}", e))?;

            if !output.status.success() {
                let error_message = format!(
                    "{}のパスワード設定に失敗しました: {}",
                    device_name,
                    String::from_utf8_lossy(&output.stderr)
                );
                log_message("ATA Secure Erase", "失敗", &error_message);
                return Err(error_message);
            }

            // Enhanced Secure Eraseを試みる
            let output = Command::new("hdparm")
                .arg("--user-master")
                .arg("u")
                .arg("--security-erase-enhanced")
                .arg("0000")
                .arg(device_name)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        log_message(
                            "ATA Secure Erase",
                            "成功",
                            "Enhanced Secure Eraseが正常に完了しました。",
                        );
                        return Ok(());
                    }
                }
                Err(_) => {}
            }

            // 通常のSecure Eraseを試みる
            let output = Command::new("hdparm")
                .arg("--user-master")
                .arg("u")
                .arg("--security-erase")
                .arg("0000")
                .arg(device_name)
                .output()
                .map_err(|e| format!("Secure Eraseの実行に失敗しました: {}", e))?;

            if !output.status.success() {
                let error_message = format!(
                    "{}のSecure Eraseに失敗しました: {}",
                    device_name,
                    String::from_utf8_lossy(&output.stderr)
                );
                log_message("ATA Secure Erase", "失敗", &error_message);
                return Err(error_message);
            }

            log_message(
                "ATA Secure Erase",
                "成功",
                "Secure Eraseが正常に完了しました。",
            );
            Ok(())
        } else if device.contains("NVMe") {
            // NVMe用のSecure Erase
            let output = Command::new("nvme")
                .arg("format")
                .arg("--ses=1")
                .arg(device_name)
                .output()
                .map_err(|e| format!("NVMe formatの実行に失敗しました: {}", e))?;

            if !output.status.success() {
                let error_message = format!(
                    "{}のNVMe formatに失敗しました: {}",
                    device_name,
                    String::from_utf8_lossy(&output.stderr)
                );
                log_message("NVMe Secure Erase", "失敗", &error_message);
                return Err(error_message);
            }

            log_message(
                "NVMe Secure Erase",
                "成功",
                "NVMe formatが正常に完了しました。",
            );
            Ok(())
        } else {
            // その他のSSD（USB接続など）
            log_message(
                &format!("不明なSSDタイプ: {}", device),
                "警告",
                "標準的なブロックワイプを実行します。",
            );
            
            // ブロックデバイスにゼロを書き込む
            let output = Command::new("dd")
                .arg("if=/dev/zero")
                .arg(format!("of={}", device_name))
                .arg("bs=4M")
                .arg("status=progress")
                .output()
                .map_err(|e| format!("ddコマンドの実行に失敗しました: {}", e))?;

            if !output.status.success() {
                let error_message = format!(
                    "{}のブロックワイプに失敗しました",
                    device_name
                );
                log_message("ブロックワイプ", "失敗", &error_message);
                return Err(error_message);
            }

            log_message(
                "ブロックワイプ",
                "成功",
                "ブロックワイプが正常に完了しました。",
            );
            Ok(())
        }
    }
} 