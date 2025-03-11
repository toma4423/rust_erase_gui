/**
 * ディスク消去ユーティリティのメインコンポーネント
 * 
 * このコンポーネントは、アプリケーションのメインUIを提供します。
 * ディスクの選択、消去の確認、消去プロセスの表示を行います。
 */

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// ディスク情報の型定義
interface DiskInfo {
  device_name: string;
  model: string;
  device_type: string;
  transport: string;
}

function App() {
  // 状態管理
  const [disks, setDisks] = useState<DiskInfo[]>([]);
  const [selectedDisk, setSelectedDisk] = useState<string | null>(null);
  const [isConfirming, setIsConfirming] = useState(false);
  const [isErasing, setIsErasing] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  // コンポーネントマウント時にディスク情報を取得
  useEffect(() => {
    fetchDisks();
  }, []);

  // ディスク情報を取得する関数
  async function fetchDisks() {
    try {
      const availableDisks = await invoke<DiskInfo[]>("get_available_disks");
      setDisks(availableDisks);
      setError("");
    } catch (e) {
      console.error("ディスク情報の取得に失敗しました:", e);
      setError("ディスク情報の取得に失敗しました。");
    }
  }

  // ディスク選択ハンドラ
  function handleDiskSelect(deviceName: string) {
    setSelectedDisk(deviceName);
    setMessage("");
    setError("");
  }

  // 消去確認ハンドラ
  function handleConfirmErase() {
    if (!selectedDisk) {
      setError("ディスクが選択されていません。");
      return;
    }
    setIsConfirming(true);
    setMessage("");
    setError("");
  }

  // 消去キャンセルハンドラ
  function handleCancelErase() {
    setIsConfirming(false);
    setMessage("");
    setError("");
  }

  // 消去実行ハンドラ
  async function handleErase() {
    if (!selectedDisk) return;

    setIsErasing(true);
    setIsConfirming(false);
    setMessage("消去中...");
    setError("");

    try {
      const result = await invoke<string>("erase_disks", {
        disks: [selectedDisk],
      });
      setMessage(result);
    } catch (e: any) {
      console.error("消去中にエラーが発生しました:", e);
      setError(`消去中にエラーが発生しました: ${e.toString()}`);
    } finally {
      setIsErasing(false);
    }
  }

  return (
    <div className="container">
      <h1>ディスク消去ユーティリティ</h1>
      <p className="description">
        このアプリケーションは、HDDとSSDのデータを安全に消去するためのツールです。
        <br />
        HDDにはDoD 5220.22-M方式、SSDにはSecure Erase方式を使用します。
      </p>

      {error && <div className="error">{error}</div>}
      {message && <div className="message">{message}</div>}

      {!isConfirming && !isErasing && (
        <div className="disk-selection">
          <h2>ディスク選択</h2>
          <p>消去するディスクを選択してください：</p>

          <div className="disk-list">
            {disks.length === 0 ? (
              <p>利用可能なディスクがありません。</p>
            ) : (
              disks.map((disk) => (
                <div
                  key={disk.device_name}
                  className={`disk-item ${
                    selectedDisk === disk.device_name ? "selected" : ""
                  }`}
                  onClick={() => handleDiskSelect(disk.device_name)}
                >
                  <div className="disk-name">{disk.device_name}</div>
                  <div className="disk-model">{disk.model}</div>
                  <div className="disk-type">
                    {disk.device_type} - {disk.transport}
                  </div>
                </div>
              ))
            )}
          </div>

          <div className="actions">
            <button onClick={fetchDisks} className="refresh-button">
              更新
            </button>
            <button
              onClick={handleConfirmErase}
              disabled={!selectedDisk}
              className="erase-button"
            >
              消去
            </button>
          </div>
        </div>
      )}

      {isConfirming && (
        <div className="confirm-erase">
          <h2>消去確認</h2>
          <p className="warning">
            警告: 選択したディスクのすべてのデータが完全に消去されます。
            <br />
            この操作は元に戻すことができません。
          </p>
          <p>
            消去するディスク: <strong>{selectedDisk}</strong>
          </p>
          <div className="actions">
            <button onClick={handleCancelErase} className="cancel-button">
              キャンセル
            </button>
            <button onClick={handleErase} className="confirm-button">
              消去を実行
            </button>
          </div>
        </div>
      )}

      {isErasing && (
        <div className="erasing">
          <h2>消去中...</h2>
          <div className="progress-bar">
            <div className="progress-indicator"></div>
          </div>
          <p>ディスクの消去中です。このプロセスには時間がかかる場合があります。</p>
        </div>
      )}
    </div>
  );
}

export default App;
