/// # 消去プロセスモジュール
/// 
/// このモジュールは、選択されたディスクの消去プロセスを管理します。
/// ディスクの種類（HDD/SSD）に応じて適切な消去方法を選択します。

use crate::disk_selection;
use crate::hdd_erase;
use crate::logger::log_message;
use crate::ssd_erase;
use rayon::prelude::*;

/// 消去プロセスのエントリーポイント
/// 
/// # 引数
/// 
/// * `selected_disks` - 消去対象のディスクのリスト
/// 
/// # 戻り値
/// 
/// * `Result<(), String>` - 成功時は`Ok(())`、失敗時はエラーメッセージを含む`Err`
pub fn start(selected_disks: &Vec<String>) -> Result<(), String> {
    log_message(
        "消去プロセス開始",
        "進行中",
        &format!("選択されたディスク: {:?}", selected_disks),
    );

    // 利用可能なディスク情報を取得
    let available_disks = disk_selection::get_available_disks();

    let errors: Vec<String> = selected_disks
        .par_iter()
        .map(|disk_path| {
            log_message(
                &format!("{}の消去開始", disk_path),
                "進行中",
                "ディスク消去プロセスを開始します。",
            );

            // 選択されたディスクパスに対応するディスク情報を検索
            let disk_info = available_disks.iter().find(|d| d.device_name == *disk_path);

            let result = match disk_info {
                Some(info) => {
                    if info.device_type == "HDD" {
                        log_message(
                            &format!("{}はHDDとして検出されました", disk_path),
                            "情報",
                            "DoD 5220.22-M方式で消去します。",
                        );
                        hdd_erase::erase_hdd_with_dod5220(disk_path)
                    } else if info.device_type == "SSD" {
                        log_message(
                            &format!("{}はSSDとして検出されました", disk_path),
                            "情報",
                            "Secure Erase方式で消去します。",
                        );
                        ssd_erase::secure_erase_ssd(disk_path)
                    } else {
                        log_message(
                            &format!("{}は不明なディスクタイプです: {}", disk_path, info.device_type),
                            "警告",
                            "ディスクタイプが不明なため、消去をスキップします。",
                        );
                        Err(format!("不明なディスクタイプ: {}。スキップします...", info.device_type))
                    }
                },
                None => {
                    log_message(
                        &format!("{}の情報が見つかりません", disk_path),
                        "エラー",
                        "ディスク情報が取得できないため、消去をスキップします。",
                    );
                    Err(format!("ディスク情報が見つかりません: {}。スキップします...", disk_path))
                }
            };

            match result {
                Ok(_) => {
                    log_message(
                        &format!("{}の消去完了", disk_path),
                        "成功",
                        "ディスクは正常に消去されました。",
                    );
                    String::new() // エラーなし
                }
                Err(e) => {
                    log_message(
                        &format!("{}の消去失敗", disk_path),
                        "エラー",
                        &e,
                    );
                    e.to_string() // エラーメッセージを返す
                }
            }
        })
        .filter(|error| !error.is_empty())
        .collect(); // エラーのあるものだけをフィルタリング

    log_message(
        "消去プロセス",
        "完了",
        "すべての選択されたディスクの消去プロセスが完了しました。",
    );

    if !errors.is_empty() {
        log_message(
            "消去中にエラーが発生しました",
            "エラー",
            &format!("{:?}", errors),
        );
        return Err(format!("消去中にエラーが発生しました: {:?}", errors));
    }

    Ok(())
} 