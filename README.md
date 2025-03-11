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

### GitHub Actionsによるリリース

このプロジェクトはGitHub Actionsを使用して自動的にリリースビルドを作成できます。

1. **バージョンタグの作成:**
   ```bash
   git tag -a v0.1.0 -m "リリースバージョン0.1.0"
   git push origin v0.1.0
   ```

2. **自動ビルドとリリース:**
   タグをプッシュすると、GitHub Actionsが自動的に以下のプラットフォーム用のビルドを作成します：
   - Windows (.msi)
   - macOS (.dmg)
   - Linux (.AppImage)

3. **リリースの公開:**
   ビルドが完了すると、GitHubリリースページにドラフトリリースが作成されます。
   内容を確認し、必要に応じてリリースノートを編集してから公開してください。

## 実行ファイルの利用方法

### Windows
- `.msi`ファイルをダウンロードして実行するだけで、アプリケーションがインストールされます。
- 追加のソフトウェアは必要ありません。

### macOS
- `.dmg`ファイルをダウンロードして開き、アプリケーションをApplicationsフォルダにドラッグします。
- 初回起動時にセキュリティ警告が表示される場合があります。

### Linux
- `.AppImage`ファイルをダウンロードして実行権限を付与します：
  ```bash
  chmod +x ディスク消去ユーティリティ*.AppImage
  ```
- 実行ファイルをダブルクリックするか、ターミナルから実行します。
- 以下のパッケージが必要です：
  - `lsblk`
  - `hdparm`（SATA SSD/HDD用）
  - `nvme-cli`（NVMe SSD用）
  - `dd`（HDD消去用）
  - `sudo`

## 注意事項

⚠️ **警告**

- **データの完全消去:** このプログラムは、選択したディスク上のすべてのデータを**復元不可能**な形で消去します。
- **バックアップ:** 重要なデータは、必ず事前にバックアップしてください。
- **ディスクの選択:** 間違ったディスクを選択しないように、十分に注意してください。
- **自己責任:** このプログラムの使用によって生じたいかなる損害についても、作者は責任を負いません。

## ライセンス

このプロジェクトはMITライセンスのもとで公開されています。 