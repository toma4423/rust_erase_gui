// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
/// # ディスク消去ユーティリティのライブラリモジュール
/// 
/// このライブラリは、HDDとSSDのデータを安全に消去するための機能を提供します。
/// HDDにはDoD 5220.22-M方式、SSDにはSecure Erase方式を使用します。
/// 
/// ## 主な機能
/// 
/// - ディスク情報の取得と表示
/// - HDDの安全な消去（DoD 5220.22-M方式）
/// - SSDの安全な消去（Secure Erase方式）
/// - 消去プロセスのログ記録

mod disk_selection;
mod erase_process;
mod hdd_erase;
mod logger;
mod ssd_erase;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// アプリケーションの状態を管理する構造体
#[derive(Default)]
pub struct AppState {
    /// 選択されたディスクのリスト
    pub selected_disks: Vec<String>,
}

/// ディスク情報を表す構造体
#[derive(Serialize, Deserialize, Clone)]
pub struct DiskInfo {
    /// ディスクのデバイス名（例: /dev/sda）
    pub device_name: String,
    /// ディスクのモデル名
    pub model: String,
    /// ディスクの種類（HDD/SSD）
    pub device_type: String,
    /// ディスクの接続方式（SATA/USB等）
    pub transport: String,
}

/// 利用可能なディスクの一覧を取得するコマンド
#[tauri::command]
fn get_available_disks() -> Vec<DiskInfo> {
    disk_selection::get_available_disks()
}

/// 選択されたディスクを消去するコマンド
#[tauri::command]
fn erase_disks(app_handle: AppHandle, disks: Vec<String>) -> Result<String, String> {
    erase_process::start(&disks)?;
    Ok("消去が完了しました。".to_string())
}

/// 消去プロセスをキャンセルするコマンド
#[tauri::command]
fn cancel_erase() -> Result<String, String> {
    Ok("消去がキャンセルされました。".to_string())
}

/// Tauriアプリケーションを実行する関数
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_available_disks,
            erase_disks,
            cancel_erase
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
