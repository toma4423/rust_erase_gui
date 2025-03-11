// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// # ディスク消去ユーティリティのメインエントリーポイント
/// 
/// このアプリケーションは、HDDとSSDのデータを安全に消去するためのGUIツールです。
/// Tauriフレームワークを使用して、RustバックエンドとReactフロントエンドを組み合わせています。

fn main() {
    tauri_app_lib::run()
}
