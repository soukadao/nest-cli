# nest-cli

Nest VCS の CLI クライアント。`nest` コマンドとしてインストールされます。

## インストール

```bash
cargo install --path .
```

## コマンド一覧

### リポジトリ

```bash
nest init [path] --user <name>   # リポジトリを初期化
nest status                      # 現在のブランチとユーザーを表示
nest log                         # スナップショット履歴を表示
nest record                      # ファイル変更を検出し CRDT 操作として記録
```

### スナップショット

```bash
nest snapshot create -m "メッセージ"   # スナップショットを作成
nest snapshot list                     # スナップショット一覧
```

### 差分

```bash
nest diff                  # 直前のスナップショットと最新の差分
nest diff <from>           # 指定スナップショットと最新の差分
nest diff <from> <to>      # 2つのスナップショット間の差分
```

### ブランチ

```bash
nest branch list              # ブランチ一覧
nest branch create <name>     # ブランチを作成
nest branch switch <name>     # ブランチを切り替え
```

### Issue

```bash
nest issue create "<title>" [-b "<body>"]   # Issue を作成
nest issue list                              # Issue 一覧
nest issue show <id>                         # Issue 詳細
nest issue close <id>                        # Issue をクローズ
nest issue comment <id> "<body>"             # Issue にコメント
```

### Review

```bash
nest review create "<title>" --source <branch> --target <branch>   # Review を作成
nest review list                             # Review 一覧
nest review show <id>                        # Review 詳細
nest review approve <id>                     # Review を承認
nest review close <id>                       # Review をクローズ
nest review comment <id> "<body>"            # Review にコメント
nest review merge <id>                       # Review をマージ
```

### Document

```bash
nest doc create "<title>" [-b "<body>"]   # ドキュメントを作成（-b 省略時は $EDITOR）
nest doc list                              # ドキュメント一覧
nest doc show <id>                         # ドキュメントを表示
nest doc edit <id>                         # $EDITOR でドキュメントを編集
```

### リモート / 同期

```bash
nest remote add <name> <url>     # リモートを追加
nest remote list                 # リモート一覧
nest remote remove <name>        # リモートを削除
nest sync [remote]               # リモートと WebSocket 経由で同期
```

## 内部構成

```
src/
├── main.rs          エントリーポイント・Clap コマンド定義
├── watcher.rs       ファイルシステム変更検出・CRDT 操作生成
├── sync_client.rs   WebSocket 同期クライアント
└── cmd/
    ├── init.rs      nest init
    ├── branch.rs    nest branch
    ├── snapshot.rs  nest snapshot
    ├── log.rs       nest log
    ├── diff.rs      nest diff
    ├── issue.rs     nest issue
    ├── review.rs    nest review
    ├── doc.rs       nest doc
    ├── remote.rs    nest remote
    └── sync.rs      nest sync
```

## リポジトリの自動検出

`nest` コマンドはカレントディレクトリから親方向に `.nest/` ディレクトリを探索します。Git の `.git/` 検出と同様の動作です。
