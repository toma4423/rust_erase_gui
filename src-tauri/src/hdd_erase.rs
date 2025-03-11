/// # HDD消去モジュール
/// 
/// このモジュールは、HDDのデータを安全に消去するための機能を提供します。
/// DoD 5220.22-M方式（3パス）を使用して、データを完全に消去します。

use crate::logger::log_message;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// DoD 5220.22-M方式でHDDを消去する関数
/// 
/// # 引数
/// 
/// * `device` - 消去対象のデバイス名（例: /dev/sda）
/// 
/// # 戻り値
/// 
/// * `Result<(), String>` - 成功時は`Ok(())`、失敗時はエラーメッセージを含む`Err`
pub fn erase_hdd_with_dod5220(device: &str) -> Result<(), String> {
    let device_name = device.split_whitespace().next().unwrap_or(device);

    log_message(
        &format!("DoD5220.22-M消去開始: {}", device_name),
        "進行中",
        "ランダムデータとゼロが複数パスで書き込まれます。",
    );

    // Windowsでの開発環境では実際の消去処理をシミュレーションする
    #[cfg(not(target_os = "linux"))]
    {
        // 開発環境では消去をシミュレーション
        for i in 0..3 {
            log_message(
                &format!("パス {}/3: {}にランダムデータを書き込み中", i + 1, device_name),
                "進行中",
                "シミュレーションモード",
            );
            
            // 進行状況のシミュレーション
            thread::sleep(Duration::from_secs(2));
        }
        
        log_message(
            &format!("DoD5220.22-M消去完了: {}", device_name),
            "成功",
            "シミュレーションモードでの消去が完了しました。",
        );
        
        return Ok(());
    }

    // 実際のLinux環境での消去処理
    #[cfg(target_os = "linux")]
    {
        for i in 0..3 {
            let action = format!("パス {}/3: {}にランダムデータを書き込み中", i + 1, device_name);
            log_message(&action, "進行中", "");

            // デバイスサイズの取得
            let device_size_output = Command::new("blockdev")
                .arg("--getsize64")
                .arg(device_name)
                .output()
                .map_err(|e| format!("デバイスサイズの取得に失敗しました: {}", e))?;

            let device_size_str = String::from_utf8_lossy(&device_size_output.stdout);
            let device_size: u64 = device_size_str.trim().parse().unwrap_or(0);

            if device_size == 0 {
                return Err(format!("デバイスサイズの取得に失敗しました: {}", device_name));
            }

            let block_size = 4 * 1024 * 1024; // 4MBブロック
            let count = device_size / block_size as u64;

            // パスに応じて異なるパターンを書き込む
            let pattern = match i {
                0 => "random", // 1パス目: ランダムデータ
                1 => "0x00",   // 2パス目: オールゼロ
                _ => "random", // 3パス目: ランダムデータ
            };

            let status = if pattern == "random" {
                Command::new("dd")
                    .arg("if=/dev/urandom")
                    .arg(format!("of={}", device_name))
                    .arg(format!("bs={}", block_size))
                    .arg(format!("count={}", count))
                    .arg("status=progress")
                    .status()
            } else {
                Command::new("dd")
                    .arg("if=/dev/zero")
                    .arg(format!("of={}", device_name))
                    .arg(format!("bs={}", block_size))
                    .arg(format!("count={}", count))
                    .arg("status=progress")
                    .status()
            };

            match status {
                Ok(exit_status) if exit_status.success() => {
                    log_message(&format!("パス {}/3 完了", i + 1), "成功", "");
                }
                Ok(_) => {
                    return Err(format!("パス {}/3 の実行に失敗しました", i + 1));
                }
                Err(e) => {
                    return Err(format!("ddコマンドの実行に失敗しました: {}", e));
                }
            }
        }

        log_message(
            &format!("DoD5220.22-M消去完了: {}", device_name),
            "成功",
            "すべてのパスが正常に完了しました。",
        );

        Ok(())
    }
} 