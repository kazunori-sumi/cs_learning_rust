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

**双方向リスト**
各ノードが前後のノードへの参照を持つ線形データ構造  
単方向の連結リストと比較して、要素の追加や削除が扱いやすい点がメリットだが、
前後の参照を持つためポインタが2倍必要であることや実装の複雑であることがデメリット

┌──────┐    ┌──────┐    ┌──────┐    ┌──────┐
│ HEAD │───→│ Node │───→│ Node │───→│ TAIL │
│      │    │  10  │    │  20  │    │      │
│      │←───│ prev │←───│ prev │←───│      │
└──────┘    │ next │    │ next │    └──────┘
            └──────┘    └──────┘

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

#### **ハッシュ関数:**
異なる入力値を、同じ長さの値に変換する関数のこと  
```rust
  fn simple_hash(s: &str) -> usize {
      let mut hash = 0;
      for (i, ch) in s.chars().enumerate() {
          hash += (ch as usize) * (31_usize.pow(i as u32));
      }
      return hash;
  }

  fn main() {
    println!("{}", simple_hash("test_string")); // 87422868327719646
  }
```

同じ入力に対しては同じハッシュ値となる  
異なる入力に対して同じハッシュ値（＝ハッシュの衝突）

```
内部では key がハッシュ化されて配列のインデックスとなる
"apple" → 12345 → 12345 % capacity → bucket_index
```

**衝突解決法:**
1. **チェイニング**: 各バケットに連結リストを持つ
2. **オープンアドレッシング**: 別の空きスロットを探す

**計算量:**
- 挿入: 平均 O(1)、最悪 O(n)
- 探索: 平均 O(1)、最悪 O(n)
- 削除: 平均 O(1)、最悪 O(n)

#### **暗号学的ハッシュ**
セキュリティ要件を満たすハッシュ関数  

```rust
use sha2::{Sha256, Digest};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();

    println!("{:x}", result); // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
}
```
出力はアルゴリズムにより可変
SHA-256 -> 256bit -> 32byte -> 64文字の16進数  

** ハッシュ値から入力値を復元できない理由**
- データ圧縮により情報が欠損している
- 複数の入力値が同じ出力値になりうる
- 数学的な制約（逆関数が存在しない）

**対衝突性**
異なる入力値が同じ出力値になるようなペアを見つけることが困難  
※ただしアルゴリズムによる(MD5は衝突可能性あり)  

**雪崩効果**
入力1ビットの変化で、出力50%のビットが変化する  

**SHA-2５６アルゴリズムの内部構造**

1.前処理（padding）  
元のメッセージ: "hello" (長さ 40 ビット = 5byte)

1.1. 元のメッセージ（bit） 01101000 01100101 01101100 01101100 01101111
1.2. "1" を追加 10000000
1.3. ゼロで埋める 00000000 00000000 ...
1.4. 長さを追加 00000000 00000000 00000000 00101000 (40 ビット = 0x28)

最終的に 512 ビット（64バイト）の倍数にする. 
これにより、、、
- メッセージを固定長のブロックに分割可能
- 長さ情報を含めることで、異なる長さのメッセージの衝突を防ぐ  

2.初期ハッシュ値  
SHA-256は8つの32ビット定数から始まる  

```rust
  let H0: u32 = 0x6a09e667;  // sqrt(2) の小数部分
  let H1: u32 = 0xbb67ae85;  // sqrt(3)
  let H2: u32 = 0x3c6ef372;  // sqrt(5)
  let H3: u32 = 0xa54ff53a;  // sqrt(7)
  let H4: u32 = 0x510e527f;  // sqrt(11)
  let H5: u32 = 0x9b05688c;  // sqrt(13)
  let H6: u32 = 0x1f83d9ab;  // sqrt(17)
  let H7: u32 = 0x5be0cd19;  // sqrt(19)
```

**素数の平方根の小数部分を使う理由**
- 恣意性の排除
- バックドアがないことの証明

3.ラウンド定数
64個の定数（最初の64個の素数の立方根）
```rust
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    // ... 56 個続く
  ];
```

4.メッセージスケジュール
512ビットのブロックを64個の32ビットワードに展開

W[0..15]  = メッセージブロックをそのまま使用
W[16..63] = 前のワードから計算で生成

W[t] = σ1(W[t-2]) + W[t-7] + σ0(W[t-15]) + W[t-16];
＊σ0,σ1はビット操作する関数

```rust
fn sigma0(x: u32) -> u32 { 
    x.rotate_right(7) ^ x.rotate_right(8) ^ (x >> 3)
}

fn sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}
```
5.圧縮関数
64ラウンドの処理を繰り返す. 
各ラウンドt(0 <= t < 64). 
    T1 = h + Σ1(e) + ch(e,f,g) + k[t] + W[t]
    T2 = Σ0(a) + Maj(a,b,c)

    h = g
    g = f
    f = e
    e = d + T1
    d = c
    c = b
    b = a
    a = T1 + T2

```rust
// choose: e が 1 なら f を、0 なら g を選択
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

// majority: 多数決
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

// Σ0,1 ビット開店と XOR の組み合わせ
fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

```

6.最終出力
64ラウンド後、初期ハッシュ値に加算

H0 = H0 + a
H1 = H1 + b
H2 = H2 + c
H3 = H3 + d
H4 = H4 + e
H5 = H5 + f
H6 = H6 + g
H7 = H7 + h

// 最終的なハッシュ値
hash = H0 || H1 || H2 || H3 || H4 || H5 || H6 || H7

視覚化：SHA-256 の処理フロー

  入力メッセージ: "hello world"
       ↓
  ┌────────────────────────┐
  │  1. パディング           │  → 512ビットの倍数に
  └────────────────────────┘
       ↓
  ┌────────────────────────┐
  │  2. ブロック分割         │  → 512ビットずつ
  └────────────────────────┘
       ↓
  ┌────────────────────────┐
  │  3. 初期値設定           │  → H0..H7 (8つの定数)
  └────────────────────────┘
       ↓
  ┌────────────────────────┐
  │  各ブロックに対して:      │
  │  - メッセージスケジュール  │  → W[0..63] 生成
  │  - 64ラウンドの圧縮       │  → ビット演算の繰り返し
  │  - ハッシュ値更新         │  → H0..H7 を更新
  └────────────────────────┘
       ↓
  最終ハッシュ値: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9

#### 負荷率(load factor)
ハッシュマップのパフォーマンスの指標  

負荷率 = 要素数 / バケット数

```rust
fn load_factor(&self) -> f64 {
    self.len as f64 / self.capacity as f64
}
```

ハッシュマップの容量に対して要素数が増える  
-> 空きが少ないので衝突しやすくなる　　

## 💻 実装課題

### 課題1: 動的配列（Vec）の簡易実装

`MyVec<T>`を実装してください。以下の機能を含む:
- `new()`: 新しい空のベクターを作成
- `push(item)`: 要素を追加
- `pop()`: 最後の要素を削除して返す
- `get(index)`: インデックスで要素を取得
- `len()`: 要素数を返す
- `drop()`: ベクターを削除

**ポイント:**
- 容量が足りなくなったら2倍に拡張
- `Box<[T]>`や生ポインタを使う
- ドロップ時のメモリ解放を忘れない

### 課題2: 単方向連結リスト

`LinkedList<T>`を実装してください:
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

## 🎓 理解度確認問題

次のステップに進む前に、以下の問題に答えて理解度を確認してください。

### カテゴリ1: メモリと計算量の理解

#### 問題1.1: 配列とVecのメモリレイアウト
以下の質問に答えてください：
1. 配列 `[i32; 5]` とベクター `Vec<i32>` のメモリ配置の違いを説明してください
2. `Vec<i32>` のメタデータ（ptr, len, cap）はそれぞれ何バイトですか？（64ビットシステムの場合）

<details>
<summary>解答例</summary>

**1. メモリ配置の違い**:

```
配列 [i32; 5]:
┌─────────────────────────┐
│ スタック                 │
│  [0][1][2][3][4]        │  20バイト (i32 × 5)
│  連続配置               │
└─────────────────────────┘

Vec<i32>:
┌─────────────────────────┐
│ スタック                 │
│  ptr: *mut i32          │  8バイト
│  len: usize             │  8バイト
│  cap: usize             │  8バイト
└────────┬────────────────┘
         │
         ↓
┌─────────────────────────┐
│ ヒープ                   │
│  [0][1][2][3][4]        │  可変サイズ
└─────────────────────────┘
```

**2. メタデータのサイズ**（64ビットシステム）:
- ptr: 8バイト（ポインタ）
- len: 8バイト（usize）
- cap: 8バイト（usize）
- 合計: 24バイト

**重要な違い**:
- 配列: コンパイル時にサイズ確定、スタック配置
- Vec: 実行時にサイズ可変、ヒープ配置
- Vecはメタデータのオーバーヘッドがある
</details>

#### 問題1.2: データ構造の操作の計算量
以下の操作の計算量を答えてください：

| 操作 | Vec | LinkedList | HashMap |
|-----|-----|-----------|---------|
| インデックスアクセス | ? | ? | N/A |
| 末尾への追加 | ? | ? | ? |
| 先頭への追加 | ? | ? | N/A |
| 中間への挿入 | ? | ? | N/A |
| 探索 | ? | ? | ? |

<details>
<summary>解答例</summary>

| 操作 | Vec | LinkedList | HashMap |
|-----|-----|-----------|---------|
| **インデックスアクセス** | O(1) | O(n) | N/A |
| **末尾への追加** | O(1)* | O(n)** | O(1)*** |
| **先頭への追加** | O(n) | O(1) | N/A |
| **中間への挿入** | O(n) | O(1)**** | N/A |
| **探索** | O(n) | O(n) | O(1)*** |

\* 償却計算量。容量拡張時は O(n)
\*\* 単方向リストで末尾ポインタなしの場合
\*\*\* 平均ケース。最悪ケースは O(n)（衝突時）
\*\*\*\* 挿入位置が既知の場合

**重要な考察**:
- **Vec**: 連続メモリでキャッシュ効率が良い。末尾操作が高速。
- **LinkedList**: ノード間が飛び飛びでキャッシュミス多発。現代では非推奨。
- **HashMap**: キー based アクセスに最適。順序は保証されない。
</details>

#### 問題1.3: Vecの容量拡張
`Vec::push()` を10回呼んだ場合、メモリの再アロケーションは何回発生しますか？また、最終的な容量は？

<details>
<summary>解答例</summary>

**前提**: 初期容量0、拡張戦略は「容量を2倍にする」

```
push 1回目: cap=0 → 1  (再アロケーション) len=1, cap=1
push 2回目: cap=1 → 2  (再アロケーション) len=2, cap=2
push 3回目: cap=2 → 4  (再アロケーション) len=3, cap=4
push 4回目: len=4, cap=4（拡張不要）
push 5回目: cap=4 → 8  (再アロケーション) len=5, cap=8
push 6-8回目: 拡張不要
push 9回目: cap=8 → 16 (再アロケーション) len=9, cap=16
push 10回目: len=10, cap=16（拡張不要）
```

**答え**:
- 再アロケーション回数: **5回**
- 最終容量: **16**

**償却計算量**:
- 個々の push は最悪 O(n) だが、償却すると O(1)
- n 回の push で、再アロケーションは log n 回のみ

**最適化**:
事前に要素数がわかっている場合は `Vec::with_capacity(n)` を使う
</details>

#### 問題1.4: キャッシュ効率
なぜ連結リストは現代のプログラミングで推奨されないのか、キャッシュの観点から説明してください。

<details>
<summary>解答例</summary>

**連結リストの問題点**:

1. **空間的局所性の欠如**
```
Vec:  [0][1][2][3][4]  ← 連続配置
      ↑ 1回のキャッシュラインで複数要素を取得

LinkedList:
[Node0]     [Node2]     [Node1]     [Node3]
  ↓           ↓           ↓           ↓
ヒープのランダムな位置に分散
→ 各ノードアクセスでキャッシュミス
```

2. **メモリオーバーヘッド**
```
Vec<i32>:
  要素1個あたり: 4バイト

LinkedList<i32>:
  要素1個あたり: 4バイト (data) + 8バイト (next) = 12バイト
  → 3倍のメモリ使用
```

3. **現代CPUの最適化**
- CPU: プリフェッチ、投機実行が連続メモリで効果的
- キャッシュライン: 通常64バイト。Vecなら連続する要素をまとめて取得

**実測データ**（n=1,000,000の反復アクセス）:
- Vec: ~5ms
- LinkedList: ~150ms（30倍遅い）

**結論**:
挿入・削除が O(1) でも、実用上は Vec の O(n) 挿入の方が高速なことが多い。
</details>

---

### カテゴリ2: データ構造の特性理解

#### 問題2.1: スタックとキューの応用
以下のアプリケーションで、スタック（LIFO）とキュー（FIFO）のどちらが適切か答えてください：

1. ブラウザの「戻る」ボタン
2. プリンタのジョブ管理
3. 関数の呼び出し履歴
4. 幅優先探索（BFS）
5. 深さ優先探索（DFS）
6. Undo/Redo機能

<details>
<summary>解答例</summary>

| アプリケーション | データ構造 | 理由 |
|---------------|----------|------|
| **1. ブラウザの「戻る」ボタン** | Stack | 最後に訪れたページから戻る（LIFO） |
| **2. プリンタのジョブ管理** | Queue | 先に送られたジョブから処理（FIFO） |
| **3. 関数の呼び出し履歴** | Stack | 最後に呼ばれた関数から戻る（LIFO） |
| **4. 幅優先探索（BFS）** | Queue | 同じ深さのノードを先に処理（FIFO） |
| **5. 深さ優先探索（DFS）** | Stack | 深く潜ってから戻る（LIFO） |
| **6. Undo/Redo機能** | 2つのStack | Undo stack + Redo stack |

**詳細: Undo/Redo**
```
操作履歴: A → B → C
          ↑ 現在

Undo実行:
  Undo Stack: [A, B, C] → [A, B]
  Redo Stack: [] → [C]

Redo実行:
  Undo Stack: [A, B] → [A, B, C]
  Redo Stack: [C] → []
```
</details>

#### 問題2.2: ハッシュマップの衝突解決
チェイニング方式とオープンアドレッシング方式の違いを説明し、それぞれの長所・短所を述べてください。

<details>
<summary>解答例</summary>

**1. チェイニング方式**

```
buckets:
[0] → [("apple", 1)] → [("banana", 2)]
[1] → None
[2] → [("cherry", 3)]
[3] → [("date", 4)] → [("elderberry", 5)]
```

**仕組み**: 各バケットに連結リストを持ち、衝突した要素を同じバケットに追加

**長所**:
- 実装がシンプル
- 負荷率が1を超えても動作する
- 削除が簡単

**短所**:
- ポインタのオーバーヘッド
- キャッシュ効率が悪い（連結リスト）
- メモリの断片化

---

**2. オープンアドレッシング方式**

```
buckets:
[0] → ("apple", 1)
[1] → ("banana", 2)    ← "banana" の本来の位置は 0 だが衝突
[2] → ("cherry", 3)
[3] → None
```

**仕組み**: 衝突時、別の空きバケットを探す（線形探査、二次探査、二重ハッシュ）

**長所**:
- メモリ効率が良い（ポインタ不要）
- キャッシュ効率が良い（連続配置）
- 小さいデータに最適

**短所**:
- 負荷率が高いと性能劣化
- 削除が複雑（tombstone必要）
- クラスタリング問題

---

**実用的な選択**:
- **Rustの `std::collections::HashMap`**: オープンアドレッシング（Robin Hood hashing）
- **Pythonの `dict`**: オープンアドレッシング
- **Javaの `HashMap`**: チェイニング

**負荷率の推奨値**:
- チェイニング: 0.75以下
- オープンアドレッシング: 0.5以下
</details>

#### 問題2.3: 負荷率（Load Factor）
ハッシュマップの負荷率が 0.9 の場合と 0.5 の場合で、操作の性能にどのような違いがありますか？

<details>
<summary>解答例</summary>

**負荷率の定義**: `負荷率 = 要素数 / バケット数`

**負荷率 0.5 の場合**:
- 要素数: 50、バケット数: 100
- 衝突確率: 低い
- 探索: ほぼ O(1)
- メモリ使用量: 多い

**負荷率 0.9 の場合**:
- 要素数: 90、バケット数: 100
- 衝突確率: 高い
- 探索: O(1) に近いが劣化
- メモリ使用量: 少ない

**具体例**（チェイニング方式）:

```
負荷率 0.5:
[0] → ("a", 1)
[1] → None
[2] → ("b", 2)
[3] → None
...
平均チェーン長: 0.5

負荷率 0.9:
[0] → ("a", 1) → ("b", 2)
[1] → ("c", 3)
[2] → ("d", 4) → ("e", 5) → ("f", 6)
[3] → ("g", 7)
...
平均チェーン長: 0.9
```

**パフォーマンス**:
- 探索の期待値: 負荷率に比例
- 負荷率 0.9: 平均0.9個のノードを辿る
- 負荷率 0.5: 平均0.5個のノードを辿る

**トレードオフ**:
- 低負荷率（0.5）: 速度↑ メモリ使用量↑
- 高負荷率（0.9）: 速度↓ メモリ使用量↓

**実用的な設定**:
- デフォルト: 0.75（速度とメモリのバランス）
- メモリ重視: 0.9
- 速度重視: 0.5
</details>

---

### カテゴリ3: データ構造の選択問題

#### 問題3.1: 実践シナリオ1
Webサーバーで、最近アクセスされた10,000件のURLを記録し、重複チェックを高速に行いたい。どのデータ構造を使うべきか？

<details>
<summary>解答例</summary>

**推奨**: **HashSet**（または HashMap<URL, ()>）

**理由**:
1. **重複チェックが O(1)**
   - Vec での線形探索: O(n) = 10,000回の比較
   - HashSet: O(1) = 数回のハッシュ計算のみ

2. **挿入が O(1)**
   - 新しいURLを高速に追加

3. **メモリ効率**
   - 10,000件程度なら問題なし

**実装例**:
```rust
use std::collections::HashSet;

let mut visited: HashSet<String> = HashSet::new();

fn is_visited(url: &str) -> bool {
    visited.contains(url)  // O(1)
}

fn mark_visited(url: String) {
    visited.insert(url);  // O(1)
}
```

**代替案**:
- **BloomFilter**: メモリ使用量を極限まで減らしたい場合（偽陽性を許容）
- **LRU Cache**: 固定サイズで古いものを削除したい場合

**避けるべき**:
- Vec: 重複チェックが O(n) で遅い
- LinkedList: すべての操作が遅い
</details>

#### 問題3.2: 実践シナリオ2
大量のログメッセージ（数百万件）を時系列順に保存し、末尾への追加を頻繁に行う。どのデータ構造が最適か？

<details>
<summary>解答例</summary>

**推奨**: **Vec<LogMessage>**

**理由**:
1. **末尾への追加が O(1)**（償却）
   - push() が高速

2. **キャッシュ効率が良い**
   - 連続メモリ配置
   - 時系列走査が高速

3. **メモリ効率**
   - ポインタのオーバーヘッドなし

4. **ファイル書き込みに最適**
   - 連続データをそのまま書き出せる

**最適化**:
```rust
// 事前に容量を確保
let mut logs = Vec::with_capacity(1_000_000);

// バッチ書き込み
logs.extend(new_batch);  // 個別pushより高速
```

**代替案**:
- **VecDeque**: 先頭からの削除も必要な場合（循環バッファ）
- **ファイル直接書き込み**: メモリ節約したい場合

**避けるべき**:
- LinkedList: キャッシュ効率が最悪
- HashMap: 時系列順序が保証されない
</details>

#### 問題3.3: 実践シナリオ3
コマンドライン履歴機能を実装する。上下キーで履歴を辿り、新しいコマンドを末尾に追加する。どのデータ構造を使うべきか？

<details>
<summary>解答例</summary>

**推奨**: **Vec<String>** + カーソル位置管理

**理由**:
1. **履歴の走査が O(1)**
   - インデックスアクセスで高速

2. **末尾への追加が O(1)**
   - 新コマンドを push

3. **メモリ効率**
   - 連続配置

**実装例**:
```rust
struct CommandHistory {
    history: Vec<String>,
    cursor: usize,  // 現在表示中の履歴位置
}

impl CommandHistory {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            cursor: 0,
        }
    }

    fn add(&mut self, cmd: String) {
        self.history.push(cmd);
        self.cursor = self.history.len();  // 最新位置にリセット
    }

    fn previous(&mut self) -> Option<&str> {
        if self.cursor > 0 {
            self.cursor -= 1;
            Some(&self.history[self.cursor])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<&str> {
        if self.cursor < self.history.len() - 1 {
            self.cursor += 1;
            Some(&self.history[self.cursor])
        } else {
            self.cursor = self.history.len();
            None  // 最新を超えたら空
        }
    }
}
```

**最適化**:
- 履歴数制限（例：1000件）
- 重複削除
- 永続化（ファイル保存）

**避けるべき**:
- LinkedList: ランダムアクセスが O(n)
- VecDeque: この用途では利点なし
</details>

#### 問題3.4: 実践シナリオ4
タスクスケジューラーで、優先度付きタスクを管理する。高優先度のタスクから処理したい。どのデータ構造を使うべきか？

<details>
<summary>解答例</summary>

**推奨**: **BinaryHeap<Task>**（優先度キュー）

**理由**:
1. **最高優先度の取得が O(1)**
   - peek() で即座にアクセス

2. **挿入が O(log n)**
   - ヒープ構造で効率的

3. **削除が O(log n)**
   - pop() で最高優先度を取得・削除

**実装例**:
```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

let mut scheduler = BinaryHeap::new();

// タスク追加
scheduler.push(Task { priority: 5, name: "Low".into() });
scheduler.push(Task { priority: 10, name: "High".into() });
scheduler.push(Task { priority: 7, name: "Med".into() });

// 処理（優先度順）
while let Some(task) = scheduler.pop() {
    println!("Processing: {}", task.name);  // High, Med, Low
}
```

**代替案**:
- **Vec + sort**: 頻繁な挿入がない場合
- **BTreeMap<Priority, VecDeque<Task>>**: 同じ優先度をFIFO処理

**避けるべき**:
- Vec のみ: 最高優先度の検索が O(n)
- LinkedList: すべてが遅い

**計算量比較**:

| 操作 | Vec（ソート） | BinaryHeap |
|-----|------------|-----------|
| 挿入 | O(n) | O(log n) |
| 最大値取得 | O(1) | O(1) |
| 最大値削除 | O(n) | O(log n) |

</details>

---

### カテゴリ4: 実装の理解（バグ発見）

#### 問題4.1: MyVecのバグ
以下の `MyVec` 実装にはバグがあります。どこが間違っているか指摘してください。

```rust
pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
        self.grow();
    }

    unsafe {
        ptr::write(self.ptr.as_ptr().add(self.capacity), value);  // ← バグ
    }

    self.len += 1;
}
```

<details>
<summary>解答</summary>

**バグ**: `self.capacity` ではなく `self.len` を使うべき

**問題点**:
```
capacity = 4, len = 2 の場合:

正しい挿入位置:
[0][1][?][?]
      ↑ len=2 の位置に書き込むべき

間違った挿入位置:
[0][1][?][?]
            ↑ capacity=4 の位置（範囲外）
```

**修正**:
```rust
unsafe {
    ptr::write(self.ptr.as_ptr().add(self.len), value);
}
```

**理由**:
- `len` は現在の要素数（次の挿入位置）
- `capacity` は確保済みの容量（最大インデックス + 1）
</details>

#### 問題4.2: LinkedListのバグ
以下の `push_back` 実装にはバグがあります。どこが間違っているか指摘してください。

```rust
pub fn push_back(&mut self, data: T) {
    let new_node = Box::new(Node { data, next: None });

    let mut current = &mut self.head;
    while current.is_some() {
        current = &mut current.as_mut().unwrap().next;  // ← バグ
    }

    *current = Some(new_node);
    self.len += 1;
}
```

<details>
<summary>解答</summary>

**バグ**: 空のリストの場合を処理していない

**問題点**:
```rust
// head が None の場合
let mut current = &mut self.head;  // current = &mut None
while current.is_some() {          // false なのでループしない
    // ...
}
*current = Some(new_node);  // head に直接セット（これは正しい）
```

実はこのコードは**動作する**が、以下の点で非効率：

1. **空のリストでもループに入る**
2. **最後のノードまで辿る必要がある**（O(n)）

**改善版**:
```rust
pub fn push_back(&mut self, data: T) {
    let new_node = Box::new(Node { data, next: None });

    match self.head {
        None => {
            // 空のリストの場合は head にセット
            self.head = Some(new_node);
        }
        Some(ref mut head) => {
            // 末尾まで辿る
            let mut current = head;
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }
    }

    self.len += 1;
}
```

**さらなる最適化**:
末尾ポインタ（tail）を持つ
```rust
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,  // 末尾への生ポインタ
    len: usize,
}

// push_back が O(1) になる
```
</details>

#### 問題4.3: HashMapのバグ
以下の `insert` 実装にはバグがあります。どこが間違っているか指摘してください。

```rust
pub fn insert(&mut self, key: K, value: V) {
    let index = self.hash(&key);
    let bucket = &mut self.buckets[index];

    // 既存のキーを探す
    for entry in bucket.iter_mut() {
        if entry.key == key {
            entry.value = value;
            self.len += 1;  // ← バグ
            return;
        }
    }

    // 新しいエントリを追加
    bucket.push_back(Entry { key, value });
    self.len += 1;
}
```

<details>
<summary>解答</summary>

**バグ**: 既存キーの値を更新する場合、`len` をインクリメントしてはいけない

**問題点**:
```rust
if entry.key == key {
    entry.value = value;  // 値を更新
    self.len += 1;        // ← 要素数は変わっていないのにインクリメント
    return;
}
```

**修正**:
```rust
pub fn insert(&mut self, key: K, value: V) {
    let index = self.hash(&key);
    let bucket = &mut self.buckets[index];

    // 既存のキーを探す
    for entry in bucket.iter_mut() {
        if entry.key == key {
            entry.value = value;
            return;  // len はインクリメントしない
        }
    }

    // 新しいエントリを追加
    bucket.push_back(Entry { key, value });
    self.len += 1;  // 新規追加時のみインクリメント

    // リサイズチェック
    if self.load_factor() > 0.75 {
        self.resize();
    }
}
```

**正しい動作**:
```
初期: map.len() = 0
insert("a", 1): len = 1
insert("b", 2): len = 2
insert("a", 3): len = 2  ← 既存キー更新なので変わらない
```
</details>

---

### カテゴリ5: 応用問題

#### 問題5.1: Rustの所有権とデータ構造
なぜRustで双方向連結リストの実装が難しいのか、所有権システムの観点から説明してください。

<details>
<summary>解答例</summary>

**双方向連結リストの構造**:
```
┌──────┐    ┌──────┐    ┌──────┐
│ Node │←──→│ Node │←──→│ Node │
│  A   │    │  B   │    │  C   │
└──────┘    └──────┘    └──────┘
```

**Rustの所有権ルール**:
1. 各値は1つの所有者のみ
2. 所有者がスコープを抜けると値は破棄される
3. 同時に複数の可変参照は持てない

**問題1: 相互参照**
```rust
struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,  // ← Node C は Node B を所有
    next: Option<Box<Node<T>>>,  // ← Node B は Node C を所有
}
// → 循環所有で、どちらが真の所有者か？
```

**問題2: 複数の可変参照**
```rust
// Node B から見ると:
let b_prev = &mut node_a;  // A への可変参照
let b_next = &mut node_c;  // C への可変参照

// Node C から見ると:
let c_prev = &mut node_b;  // B への可変参照
// → B への可変参照が2つ存在（所有権ルール違反）
```

**解決策**:

**1. `Rc<RefCell<Node<T>>>`**
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,  // 共有所有権
    next: Option<Rc<RefCell<Node<T>>>>,  // 実行時借用チェック
}
```

- `Rc`: 参照カウント（複数の所有者）
- `RefCell`: 実行時の可変借用チェック

**トレードオフ**:
- ✅ 所有権ルールを満たす
- ❌ 参照カウントのオーバーヘッド
- ❌ 実行時チェックのコスト
- ❌ 循環参照によるメモリリーク可能性

**2. 生ポインタ `*mut Node<T>`**
```rust
struct Node<T> {
    data: T,
    prev: *mut Node<T>,  // 生ポインタ（unsafe）
    next: *mut Node<T>,
}
```

- unsafe ブロックが必要
- 手動でメモリ管理
- 高速だが危険

**結論**:
Rustの双方向連結リストは実装可能だが、複雑で実用性が低い。
**Vec や VecDeque を使うべき**。
</details>

#### 問題5.2: データ構造の最適化
`Vec` の `remove(index)` は O(n) です。どのように最適化できますか？

<details>
<summary>解答例</summary>

**問題: `Vec::remove(index)` は O(n)**

```rust
let mut vec = vec![1, 2, 3, 4, 5];
vec.remove(1);  // [1, _, 3, 4, 5] → [1, 3, 4, 5]
                // 後続要素を前に詰める（O(n)）
```

**最適化1: `swap_remove(index)` - O(1)**

順序を保持する必要がない場合:
```rust
let mut vec = vec![1, 2, 3, 4, 5];
vec.swap_remove(1);  // [1, 5, 3, 4]
                      // ↑ 最後の要素と交換してpop
```

**実装**:
```rust
pub fn swap_remove(&mut self, index: usize) -> T {
    let last_index = self.len() - 1;
    self.swap(index, last_index);  // O(1)
    self.pop().unwrap()            // O(1)
}
```

**最適化2: 削除のバッチ処理**

複数要素を削除する場合:
```rust
// 悪い例: O(n²)
for index in indices_to_remove {
    vec.remove(index);  // 各削除で O(n)
}

// 良い例: O(n)
vec.retain(|&x| !should_remove(x));  // 1パスで削除
```

**最適化3: `VecDeque` を使う**

先頭削除が頻繁な場合:
```rust
use std::collections::VecDeque;

let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
deque.pop_front();  // O(1)
```

**最適化4: インデックスではなくマーカーを使う**

```rust
struct Item {
    data: i32,
    deleted: bool,  // 削除マーク
}

// 削除を O(1) に
item.deleted = true;

// 定期的にクリーンアップ
vec.retain(|item| !item.deleted);
```

**計算量比較**:

| 操作 | Vec::remove | swap_remove | VecDeque |
|-----|------------|-------------|----------|
| 先頭削除 | O(n) | O(1)* | O(1) |
| 末尾削除 | O(1) | O(1) | O(1) |
| 中間削除 | O(n) | O(1)* | O(n) |

\* 順序が変わる
</details>

#### 問題5.3: メモリレイアウトの詳細
以下のコードのメモリ使用量を計算してください（64ビットシステム）：

```rust
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
```

<details>
<summary>解答例</summary>

**スタック上のメモリ**:
```
Vec のメタデータ:
- ptr: 8バイト（ポインタ）
- len: 8バイト（usize）
- cap: 8バイト（usize）
合計: 24バイト
```

**ヒープ上のメモリ**:
```
要素: i32 × 5 = 4 × 5 = 20バイト

実際の確保量（capacity）:
Vec の拡張戦略によるが、len=5 なら cap=8 の可能性
→ 4 × 8 = 32バイト確保
```

**合計メモリ使用量**:
```
24バイト（スタック）+ 32バイト（ヒープ） = 56バイト
```

**詳細**:
```
スタック:
┌─────────────────┐
│ Vec<i32>        │
│ ptr: 0x7fff... │  8バイト
│ len: 5          │  8バイト
│ cap: 8          │  8バイト
└────────┬────────┘
         │
         ↓
ヒープ:
┌─────────────────────────────────┐
│ [1][2][3][4][5][?][?][?]        │  32バイト
│  使用中 5個    |  未使用 3個     │
└─────────────────────────────────┘
```

**実測方法**:
```rust
use std::mem;

let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
println!("Stack size: {}", mem::size_of::<Vec<i32>>());  // 24
println!("Heap size: {}", vec.capacity() * mem::size_of::<i32>());  // 32
println!("Total: {}", mem::size_of_val(&vec) + vec.capacity() * mem::size_of::<i32>());  // 56
```

**比較: 配列の場合**:
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
// スタックのみ: 4 × 5 = 20バイト
// メタデータなし
```

**結論**:
- Vec: 56バイト（オーバーヘッド大）
- 配列: 20バイト（オーバーヘッドなし）
- 小さいデータ（<数百要素）では配列が有利
- 動的サイズが必要なら Vec
</details>

#### 問題5.4: キャッシュ最適化
以下のコードのパフォーマンスを改善する方法を提案してください：

```rust
let mut results = Vec::new();
for i in 0..1000 {
    results.push(expensive_computation(i));
}
```

<details>
<summary>解答例</summary>

**問題点**:
- `Vec::new()` は容量0から開始
- `push()` のたびに再アロケーション発生の可能性
- 1000回の push で log₂(1000) ≈ 10回の再アロケーション

**最適化1: 事前容量確保**
```rust
let mut results = Vec::with_capacity(1000);
for i in 0..1000 {
    results.push(expensive_computation(i));
}
// 再アロケーション: 0回
```

**最適化2: イテレータの collect**
```rust
let results: Vec<_> = (0..1000)
    .map(|i| expensive_computation(i))
    .collect();
// collect が自動的に容量を確保
```

**最適化3: 並列化**
```rust
use rayon::prelude::*;

let results: Vec<_> = (0..1000)
    .into_par_iter()
    .map(|i| expensive_computation(i))
    .collect();
// マルチコアで並列処理
```

**最適化4: プリフェッチ**
```rust
let mut results = Vec::with_capacity(1000);
for chunk in (0..1000).step_by(8) {
    // 8要素ずつ処理（キャッシュラインサイズ考慮）
    for i in chunk..chunk.min(1000, chunk + 8) {
        results.push(expensive_computation(i));
    }
}
```

**パフォーマンス比較**（1000要素）:

| 方法 | 再アロケーション | 相対速度 |
|-----|-------------|--------|
| `Vec::new()` | ~10回 | 1.0x |
| `with_capacity(1000)` | 0回 | 1.2x |
| `collect()` | 0-1回 | 1.2x |
| 並列化 | 0回 | 4-8x |

**推奨**:
- シンプル: `with_capacity()`
- 関数型: `collect()`
- 高速: 並列化（計算コストが高い場合）
</details>

---

### 理解度チェックの目安

以下の基準で次のステップへの準備ができているか判断してください：

| レベル | 基準 | 推奨アクション |
|-------|------|-------------|
| **🟢 準備完了** | カテゴリ1-3の80%以上正答 | 次のWeekへ進んで問題なし |
| **🟡 要復習** | カテゴリ1-3の50-80%正答 | 不正解の分野を復習後、次へ |
| **🔴 理解不足** | カテゴリ1-3の50%未満 | 実装とテストを再度実施 |

**カテゴリ4-5**: 発展的内容のため、次のWeekで学びながら理解を深めても問題ありません。

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
