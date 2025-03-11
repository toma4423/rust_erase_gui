/// # ログ記録モジュール
/// 
/// このモジュールは、アプリケーションの動作ログを記録する機能を提供します。
/// ログはファイルに保存され、後で参照することができます。

use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// ログファイルへメッセージを記録する関数
/// 
/// # 引数
/// 
/// * `action` - 実行されたアクション（例: "ディスク消去開始"）
/// * `result` - アクションの結果（例: "成功"、"失敗"）
/// * `details` - 詳細情報
pub fn log_message(action: &str, result: &str, details: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("時間が逆行しています");
    let timestamp = format!("{:?}", since_the_epoch);

    let log_entry = format!(
        "[{}] アクション: {}\n結果: {}\n詳細: {}\n",
        timestamp, action, result, details
    );

    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("erasure_log.txt") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("ログファイルを開けませんでした: {}", e);
                return;
            }
        };
    
    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("ログファイルへの書き込みに失敗しました: {}", e);
    }
} 