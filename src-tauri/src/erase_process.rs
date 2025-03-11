/// # 消去プロセスモジュール
/// 
/// このモジュールは、選択されたディスクの消去プロセスを管理します。
/// ディスクの種類（HDD/SSD）に応じて適切な消去方法を選択します。

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

    let errors: Vec<String> = selected_disks
        .par_iter()
        .map(|disk| {
            log_message(
                &format!("{}の消去開始", disk),
                "進行中",
                "ディスク消去プロセスを開始します。",
            );

            let result = if disk.contains("HDD") {
                hdd_erase::erase_hdd_with_dod5220(disk)
            } else if disk.contains("SSD") {
                ssd_erase::secure_erase_ssd(disk)
            } else {
                Err(format!("不明なディスクタイプ: {}。スキップします...", disk))
            };

            match result {
                Ok(_) => {
                    log_message(
                        &format!("{}の消去完了", disk),
                        "成功",
                        "ディスクは正常に消去されました。",
                    );
                    String::new() // エラーなし
                }
                Err(e) => {
                    log_message(
                        &format!("{}の消去失敗", disk),
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