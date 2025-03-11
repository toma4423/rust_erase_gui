# ディスク消去ユーティリティ

このアプリケーションは、HDDとSSDのデータを安全かつ完全に消去するためのGUIツールです。DoD 5220.22-M方式（HDD）およびSecure Erase（SSD）を使用して、ディスクのデータを完全に消去します。

## 機能

- **ディスクの自動検出:** システムに接続されているディスクを自動的に検出します。
- **ディスク情報の表示:** 各ディスクのデバイス名、モデル、タイプ（HDD/SSD）、接続タイプ（SATA/NVMe/USB）を表示します。
- **消去方式の選択:**
  - **HDD:** DoD 5220.22-M方式（3回のランダムデータ書き込み + 1回のゼロ書き込み）
  - **SATA SSD:** ATA Secure Erase（可能な場合はEnhanced Secure Erase）
  - **NVMe SSD:** NVMe Secure Erase
- **消去処理のログ:** `erasure_log.txt`ファイルに、消去処理の詳細なログを記録します。

## 動作環境

- **OS:** Windows, macOS, Linux
- **必要なコマンド（Linux環境のみ）:**
  - `lsblk`
  - `hdparm`（SATA SSD/HDD用）
  - `nvme-cli`（NVMe SSD用）
  - `dd`（HDD消去用）
  - `sudo`

## 開発環境

- [Tauri](https://tauri.app/) - Rustバックエンド + Webフロントエンドのデスクトップアプリケーションフレームワーク
- [React](https://reactjs.org/) - UIライブラリ
- [TypeScript](https://www.typescriptlang.org/) - 型付きJavaScript
- [Rust](https://www.rust-lang.org/) - システムプログラミング言語

## インストール方法

### 開発環境のセットアップ

1. **前提条件:**
   - [Node.js](https://nodejs.org/)（16以上）
   - [Rust](https://www.rust-lang.org/tools/install)（1.60以上）
   - [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

2. **リポジトリのクローン:**
   ```bash
   git clone https://github.com/yourusername/disk-eraser.git
   cd disk-eraser
   ```

3. **依存関係のインストール:**
   ```bash
   npm install
   ```

4. **開発モードで実行:**
   ```bash
   npm run tauri dev
   ```

### ビルド方法

```bash
npm run tauri build
```

ビルドされたアプリケーションは`src-tauri/target/release`ディレクトリに生成されます。

## 使い方

1. アプリケーションを起動します。
2. 消去するディスクを選択します。
3. 「消去」ボタンをクリックします。
4. 確認画面で「消去を実行」ボタンをクリックします。
5. 消去プロセスが完了するまで待ちます。

## 注意事項

⚠️ **警告**

- **データの完全消去:** このプログラムは、選択したディスク上のすべてのデータを**復元不可能**な形で消去します。
- **バックアップ:** 重要なデータは、必ず事前にバックアップしてください。
- **ディスクの選択:** 間違ったディスクを選択しないように、十分に注意してください。
- **自己責任:** このプログラムの使用によって生じたいかなる損害についても、作者は責任を負いません。

## ライセンス

このプロジェクトはMITライセンスのもとで公開されています。 