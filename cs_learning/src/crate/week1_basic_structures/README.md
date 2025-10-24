# Week 1-2: 基本データ構造

## 🎯 学習目標

- 配列とベクターの違いとメモリレイアウトを理解する
- 連結リストの実装とポインタ操作を習得する
- スタックとキューのLIFO/FIFO特性を理解する
- ハッシュマップの内部動作を理解する

## 📚 重要な概念

### 1. 配列（Array）vs ベクター（Vec）

**配列の特徴:**
- コンパイル時にサイズが確定
- スタックに配置される（小さい場合）
- サイズ変更不可
- `[T; N]`型

**ベクターの特徴:**
- 実行時にサイズ変更可能
- ヒープに配置される
- スタック上にはメタデータが置かれる
    - ポインタ|ベクターのサイズ|cap（ヒープ上に確保済みのメモリ容量 capacity）
- データの参照フロー
    - スタック上のメタデータへのポインタを参照することでヒープに配置されたベクターデータが取得できる
    - （スタック上のメタデータの）ptr -> （ヒープ上の実データ）[0][1][2]
    - 計算量O(1)だが、厳密には スタックメモリ上のvectorのptrへのアクセス＋ヒープからの実データ読み取り = O(2)
        -  ※ただし、ループ内ではポインタはレジスタにキャッシュされるので、実質的には配列とほぼ同じ速度
- cap の必要性は？
    - 要素をpushする際にメモリの再アロケーションを実行するかどうかの判断に利用する
        - len <= cap であれば不要。そうでなければメモリ容量の追加確保が必要のため再アロケーションが必要
- 自動的に容量を拡張
- `Vec<T>`型

**メモリレイアウト:**
```
配列:    [1][2][3][4][5]  ← スタック上の連続メモリ領域

Vector:
  スタック: [ptr|len|cap] (24 bytes)
               ↓
  ヒープ:   [1][2][3][4][5]  ← ヒープ上の連続メモリ領域

どちらもインデックスアクセス: O(1)
（連続配置なのでキャッシュ効率も良い）
```

### 2. 連結リスト（Linked List）

**単方向リスト:**
```
[data|next] -> [data|next] -> [data|next] -> None
```

**特徴:**
- 挿入・削除: O(1)（位置が既知の場合）
- アクセス: O(n)
- メモリ: 各ノードにポインタ分のオーバーヘッド
- 現代のプログラミングではあまり使われない。
    - メモリへの配置が飛び飛びになるため、アクセス速度が遅い(キャッシュ効率の悪さ)
    - リンク用のポインタ（next）が必要でメモリの無駄

**Rustでの実装の難しさ:**
- 所有権システムのため、複数の参照を持つことが難しい
- `Box<T>`、`Rc<T>`、`RefCell<T>`を理解する必要がある

### 3. スタック（Stack）- LIFO

```
    push(3)
    push(2)
    push(1)
    ┌───┐
    │ 1 │ ← top
    ├───┤
    │ 2 │
    ├───┤
    │ 3 │
    └───┘
    pop() -> 1
```

**操作:**
- `push(item)`: O(1)
- `pop()`: O(1)
- `peek()`: O(1)

**用途:**
- 関数呼び出しスタック
- 括弧のマッチング
- Undo機能
- 深さ優先探索（DFS）

### 4. キュー（Queue）- FIFO

```
    enqueue(1)
    enqueue(2)
    enqueue(3)

    [1] <- [2] <- [3]
    ↑front        ↑rear

    dequeue() -> 1
```

**操作:**
- `enqueue(item)`: O(1)
- `dequeue()`: O(1)
- `peek()`: O(1)

**用途:**
- タスクスケジューリング
- 幅優先探索（BFS）
- バッファ管理

### 5. ハッシュマップ（HashMap）

**ハッシュ関数:**
```
key → hash_function() → index
"apple" → 12345 → 12345 % capacity → bucket_index
```

**衝突解決法:**
1. **チェイニング**: 各バケットに連結リストを持つ
2. **オープンアドレッシング**: 別の空きスロットを探す

**計算量:**
- 挿入: 平均 O(1)、最悪 O(n)
- 探索: 平均 O(1)、最悪 O(n)
- 削除: 平均 O(1)、最悪 O(n)

## 💻 実装課題

### 課題1: 動的配列（Vec）の簡易実装

`MyVec<T>`を実装してください。以下の機能を含む:
- `new()`: 新しい空のベクターを作成
- `push(item)`: 要素を追加
- `pop()`: 最後の要素を削除して返す
- `get(index)`: インデックスで要素を取得
- `len()`: 要素数を返す
- `capacity()`: 現在の容量を返す

**ポイント:**
- 容量が足りなくなったら2倍に拡張
- `Box<[T]>`や生ポインタを使う
- ドロップ時のメモリ解放を忘れない

### 課題2: 単方向連結リスト

`LinkedList<T>`を実装してください:
- `new()`: 新しいリストを作成
- `push_front(item)`: 先頭に追加
- `push_back(item)`: 末尾に追加
- `pop_front()`: 先頭から削除
- `len()`: 要素数を返す
- `iter()`: イテレータを返す

**ポイント:**
- `Box<Node<T>>`を使う
- 再帰的なデータ構造に注意

### 課題3: スタックの実装

`Stack<T>`を実装してください:
- `new()`: 新しいスタックを作成
- `push(item)`: 要素を追加
- `pop()`: 要素を削除して返す
- `peek()`: トップの要素を参照
- `is_empty()`: 空かどうか判定

**応用問題:**
- 括弧のバランスチェッカーを実装
- 逆ポーランド記法（RPN）計算機を実装

### 課題4: キューの実装

`Queue<T>`を実装してください:
- `new()`: 新しいキューを作成
- `enqueue(item)`: 要素を追加
- `dequeue()`: 要素を削除して返す
- `peek()`: 先頭の要素を参照
- `is_empty()`: 空かどうか判定

**ポイント:**
- `VecDeque`を使わず、自分で実装
- リングバッファの実装に挑戦

### 課題5: シンプルなハッシュマップ

`SimpleHashMap<K, V>`を実装してください:
- `new()`: 新しいハッシュマップを作成
- `insert(key, value)`: キーと値を挿入
- `get(key)`: キーで値を取得
- `remove(key)`: キーを削除
- `contains_key(key)`: キーの存在確認

**ポイント:**
- チェイニングで衝突を解決
- 負荷率が0.75を超えたらリサイズ
- 簡単なハッシュ関数を実装（例: 文字コードの合計）

## 🧪 テストケース例

各実装には以下のようなテストを書いてください:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_push_and_pop() {
        let mut vec = MyVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_vec_capacity_growth() {
        let mut vec = MyVec::new();
        let initial_capacity = vec.capacity();

        for i in 0..initial_capacity + 1 {
            vec.push(i);
        }

        assert!(vec.capacity() > initial_capacity);
    }
}
```

## 🔍 デバッグのヒント

1. **所有権エラー**: 借用チェッカーと戦う場合は、`Rc`や`RefCell`の使用を検討
2. **メモリリーク**: `cargo test`の後に`valgrind`（またはRustの`miri`）で確認
3. **パフォーマンス**: `cargo bench`でベンチマークを取る

## 📖 追加学習リソース

- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rustについて
- [std::collections のドキュメント](https://doc.rust-lang.org/std/collections/)

## ✅ チェックリスト

- [ ] MyVec の基本機能を実装
- [ ] MyVec のテストをすべてパス
- [ ] LinkedList を実装
- [ ] LinkedList のイテレータを実装
- [ ] Stack を実装し、括弧チェッカーを作成
- [ ] Queue を実装
- [ ] SimpleHashMap を実装
- [ ] すべてのテストが通る
- [ ] 各データ構造の計算量を理解した
- [ ] どの状況でどのデータ構造を使うべきか説明できる

## 🚀 次のステップ

Week 1-2が完了したら、Week 3-4の探索とソートアルゴリズムに進みましょう！
