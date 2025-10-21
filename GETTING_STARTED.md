# 学習の始め方

## 🎉 ようこそ！

このリポジトリは、Rustを使ってコンピュータサイエンスの基礎から応用までを体系的に学ぶためのプロジェクトです。

## 📖 学習リソース

1. **CS_CURRICULUM.md** - 全体のカリキュラムと学習ロードマップ
2. **各週のREADME.md** - 詳細な学習目標と課題説明
3. **src.rs ファイル** - 実装テンプレートとテストコード

## 🚀 クイックスタート

### 1. プロジェクトの確認

```bash
# プロジェクトのディレクトリに移動
cd cs_learning

# ビルドが通るか確認
cargo check

# プログラムを実行
cargo run
```

### 2. Week 1-2 の課題を開始

```bash
# READMEを読む
cat src/crate/week1_basic_structures/README.md

# ソースコードを開く
# エディタで cs_learning/src/crate/week1_basic_structures/src.rs を開く
```

### 3. 課題を実装

各関数に `unimplemented!()` マクロが配置されています。これを実際の実装に置き換えていきます。

例:
```rust
pub fn push(&mut self, value: T) {
    // TODO: 実装してください
    unimplemented!("push を実装してください")
}
```

↓ 実装後

```rust
pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
        self.grow();
    }
    unsafe {
        ptr::write(self.ptr.add(self.len), value);
    }
    self.len += 1;
}
```

### 4. テストを実行

```bash
# すべてのテストを実行（最初は #[ignore] のため多くがスキップされます）
cargo test

# 特定のモジュールのテストを実行
cargo test my_vec

# ignore されているテストも含めてすべて実行
cargo test -- --ignored

# すべてのテスト（通常 + ignored）を実行
cargo test -- --include-ignored
```

### 5. テストを有効化

実装が完了したら、テストの `#[ignore]` 属性を削除してテストを有効にします:

```rust
#[test]
#[ignore]  // ← この行を削除
fn test_push_and_pop() {
    // ...
}
```

↓

```rust
#[test]
fn test_push_and_pop() {
    // ...
}
```

## 📝 学習の流れ

### ステップ1: 理論を学ぶ
- README.md の「重要な概念」セクションを読む
- データ構造の図やアルゴリズムの説明を理解する

### ステップ2: コードを読む
- 提供されたテンプレートコードを読む
- 型定義、関数シグネチャを確認
- TODO コメントとヒントを読む

### ステップ3: 実装する
- `unimplemented!()` を実際のコードに置き換える
- コンパイルエラーを修正
- まずは動くコードを書く（最適化は後で）

### ステップ4: テストする
- テストを実行して正しく動作するか確認
- エッジケースを考える
- 必要に応じて追加のテストを書く

### ステップ5: 最適化と理解を深める
- 計算量（Big O）を分析
- より効率的な実装を考える
- 他のデータ構造と比較

## 🎯 課題の難易度

各課題には難易度の目安があります:

- **基本**: `MyVec`, `Stack`, `Queue` - 比較的シンプル
- **中級**: `LinkedList` - Rustの所有権システムとの戦い
- **上級**: `SimpleHashMap` - 複雑なロジックとリサイズ処理

**推奨順序:**
1. Stack → Queue（Vecを使った簡単な実装）
2. MyVec（メモリ管理の基礎）
3. LinkedList（所有権システムの理解）
4. SimpleHashMap（総合演習）

## 🛠️ 便利なコマンド

```bash
# フォーマットをチェック
cargo fmt -- --check

# コードをフォーマット
cargo fmt

# Lintを実行（より厳しいチェック）
cargo clippy

# ドキュメントを生成
cargo doc --open

# ベンチマークを実行（後で追加）
cargo bench
```

## 💡 学習のヒント

### Rustの所有権で困ったら

1. **エラーメッセージを読む** - Rustのエラーメッセージは非常に親切です
2. **所有権のルールを復習** - [The Rust Book](https://doc.rust-lang.org/book/) を参照
3. **Rc/RefCell を検討** - 複雑な参照関係には `Rc<RefCell<T>>` が役立つ
4. **シンプルに始める** - まずは動くコードを書き、後でリファクタリング

### デバッグのコツ

1. **println! デバッグ** - 変数の値を出力して確認
2. **dbg! マクロ** - より詳細なデバッグ情報
3. **テストを細かく書く** - 小さなユニットテストで問題を特定
4. **Rust Playground** - ブラウザで試せる https://play.rust-lang.org/

### 学習リソース

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Too Many Linked Lists**: https://rust-unofficial.github.io/too-many-lists/
- **VisuAlgo**: https://visualgo.net/ - アルゴリズムの可視化

## 📊 進捗管理

各週のREADMEの最後に「チェックリスト」があります。完了した項目にチェックを入れましょう:

```markdown
## ✅ チェックリスト

- [x] MyVec の基本機能を実装
- [x] MyVec のテストをすべてパス
- [ ] LinkedList を実装
- [ ] LinkedList のイテレータを実装
...
```

## 🤔 質問がある場合

1. まず README.md の「重要な概念」セクションを読み直す
2. コメントやヒントを再確認
3. 関連するRustドキュメントを読む
4. 実装例を探す（ただし、まずは自分で考えることが大切！）

## 🎓 次のステップ

Week 1-2 が完了したら:

1. CS_CURRICULUM.md で次の週の内容を確認
2. 新しい課題のディレクトリとREADMEが用意されているか確認
3. まだなければ、同じパターンで新しい週のモジュールを作成

---

**重要**: この学習は競争ではありません。自分のペースで、しっかり理解しながら進めてください。
各データ構造やアルゴリズムの「なぜ」を理解することが、コンピュータサイエンスの真の力になります。

頑張ってください！ 🚀
