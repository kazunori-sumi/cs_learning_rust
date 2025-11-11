// Week 1-2: 基本データ構造の実装

// ---------------------------------------------------------
// 課題1: 動的配列（MyVec）の実装
// ---------------------------------------------------------

pub mod my_vec {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::{self, NonNull};

    /// 動的配列の簡易実装
    ///
    /// # Examples
    /// ```
    /// let mut vec = MyVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.pop(), Some(2));
    /// ```
    pub struct MyVec<T> {
        ptr: NonNull<T>, // データへのポインタ（null でないことが保証される）
        len: usize,      // 現在の要素数
        capacity: usize, // 確保済みの容量
    }

    impl<T> MyVec<T> {
        /// 新しい空のベクターを作成
        pub fn new() -> Self {
            MyVec {
                ptr: NonNull::dangling(), // 容量0の場合はダミーポインタ
                len: 0,
                capacity: 0,
            }
        }

        /// 指定した容量で新しいベクターを作成
        pub fn with_capacity(capacity: usize) -> Self {
            if capacity == 0 {
                return Self::new();
            }

            // 1. メモリレイアウト作成
            let layout = Layout::array::<T>(capacity).expect("Failed to create layout");

            // 2. メモリ確保
            let ptr = unsafe { alloc(layout) };

            // 3. null チェック
            if ptr.is_null() {
                panic!("Memory allocation failed");
            }

            // 4. 型付きポインタに変換（NonNull で wrap）
            let ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };

            MyVec {
                ptr,
                len: 0,
                capacity,
            }
        }

        /// 要素を末尾に追加
        pub fn push(&mut self, value: T) {
            // 1. 容量が足りない場合は grow() を呼ぶ
            if self.len == self.capacity {
                self.grow();
            }

            // 2. ptr.add(len) に値を書き込む
            unsafe {
                ptr::write(self.ptr.as_ptr().add(self.len), value);
            }

            // 3. len をインクリメント
            self.len += 1;
        }

        /// 末尾の要素を削除して返す
        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                unsafe {
                    let raw_ptr = self.ptr.as_ptr();
                    let element_ptr = raw_ptr.add(self.len);
                    Some(std::ptr::read(element_ptr))
                }
            }
        }

        /// インデックスで要素を取得
        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                None
            } else {
                unsafe {
                    let raw_ptr = self.ptr.as_ptr();
                    let element_ptr = raw_ptr.add(index);
                    Some(&*element_ptr)
                }
            }
        }

        /// 現在の要素数
        pub fn len(&self) -> usize {
            self.len
        }

        /// 現在の容量
        pub fn capacity(&self) -> usize {
            self.capacity
        }

        /// 空かどうか
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// 容量を拡張（内部関数）
        fn grow(&mut self) {
            // 1. 新しい容量 = max(capacity * 2, 1)
            let new_capacity = if self.capacity == 0 {
                1
            } else {
                self.capacity * 2
            };
            // 2. 新しいメモリを確保
            let layout = Layout::array::<T>(new_capacity).expect("Failed to Create layout");
            let new_ptr = unsafe { alloc(layout) };
            if new_ptr.is_null() {
                panic!("Could not allocate");
            }
            let new_ptr = unsafe { NonNull::new_unchecked(new_ptr as *mut T) };
            // 3. 古いデータをコピー
            unsafe { ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len) };

            // 4. 古いメモリを解放
            if self.capacity > 0 {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
                }
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            if self.capacity == 0 {
                return;
            }
            // 1. 各要素に対して drop を呼ぶ
            for i in 0..self.len {
                unsafe {
                    let idx = self.ptr.as_ptr().add(i);
                    ptr::drop_in_place(idx);
                }
            }
            // 2. メモリを解放
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let vec: MyVec<i32> = MyVec::new();
            assert_eq!(vec.len(), 0);
            assert_eq!(vec.capacity(), 0);
        }

        #[test]
        fn test_with_capacity() {
            let vec: MyVec<usize> = MyVec::with_capacity(8);
            assert_eq!(vec.capacity, 8);
            assert_eq!(vec.capacity(), 8);
        }

        #[test]
        fn test_push_and_pop() {
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
        fn test_capacity_growth() {
            let mut vec = MyVec::new();

            for i in 0..10 {
                vec.push(i);
            }

            assert!(vec.capacity() >= 10);
            assert_eq!(vec.len(), 10);
        }
        #[test]
        fn test_get() {
            let mut vec = MyVec::new();
            for i in 0..10 {
                vec.push(i + 1);
            }

            assert_eq!(vec.get(0), Some(&1));
            assert_eq!(vec.get(9), Some(&10));
        }

        #[test]
        fn test_drop_with_many_elements() {
            // 大量の要素でメモリリークがないか確認
            for _ in 0..100 {
                let mut vec = MyVec::new();
                for i in 0..100 {
                    vec.push(format!("String {}", i));
                }
                // スコープを抜けるときに全要素がdropされる
            }
        }

        #[test]
        fn test_drop_count() {
            use std::sync::atomic::{AtomicUsize, Ordering};
            use std::sync::Arc;

            // Dropが何回呼ばれたかカウントする型
            struct DropCounter {
                count: Arc<AtomicUsize>,
            }

            impl Drop for DropCounter {
                fn drop(&mut self) {
                    self.count.fetch_add(1, Ordering::SeqCst);
                }
            }

            let drop_count = Arc::new(AtomicUsize::new(0));

            {
                let mut vec = MyVec::new();
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
            } // ここで5つのDropCounterがdropされるはず

            // 5回dropが呼ばれたか確認
            assert_eq!(drop_count.load(Ordering::SeqCst), 5);
        }

        #[test]
        fn test_drop_after_pop() {
            use std::sync::atomic::{AtomicUsize, Ordering};
            use std::sync::Arc;

            struct DropCounter {
                count: Arc<AtomicUsize>,
            }

            impl Drop for DropCounter {
                fn drop(&mut self) {
                    self.count.fetch_add(1, Ordering::SeqCst);
                }
            }

            let drop_count = Arc::new(AtomicUsize::new(0));

            {
                let mut vec = MyVec::new();
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });
                vec.push(DropCounter {
                    count: drop_count.clone(),
                });

                // 1つpopする（この時点で1回dropされる）
                let _popped = vec.pop();
                drop(_popped); // 明示的にdrop

                assert_eq!(drop_count.load(Ordering::SeqCst), 1);

                // 残り2つはMyVecのdropで処理される
            }

            // 合計3回dropが呼ばれたか確認
            assert_eq!(drop_count.load(Ordering::SeqCst), 3);
        }

        #[test]
        fn test_drop_empty_vec() {
            // 空のベクタのdropでパニックしないか確認
            let vec: MyVec<i32> = MyVec::new();
            drop(vec); // 明示的にdrop
        }

        #[test]
        fn test_drop_with_capacity_but_no_elements() {
            // 容量はあるが要素がない場合
            let vec: MyVec<String> = MyVec::with_capacity(10);
            drop(vec); // パニックしないはず
        }
    }
}

// ---------------------------------------------------------
// 課題2: 単方向連結リスト
// ---------------------------------------------------------

pub mod linked_list {
    use std::fmt::Display;

    /// ノード構造
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    /// 単方向連結リスト
    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
        len: usize,
    }

    impl<T> LinkedList<T> {
        /// 新しい空のリストを作成
        pub fn new() -> Self {
            LinkedList { head: None, len: 0 }
        }

        /// 先頭に要素を追加
        pub fn push_front(&mut self, data: T) {
            // 1. 新しいノードを作成
            // 2. 新しいノードの next に現在の head を設定
            let new_node = Node {
                data,
                next: self.head.take(),
            };
            // 3. head を新しいノードに更新
            self.head = Some(Box::new(new_node));
            // 4. len をインクリメント
            self.len += 1;
        }

        /// 先頭の要素を削除して返す
        pub fn pop_front(&mut self) -> Option<T> {
            // match で先頭ノードを評価
            match self.head.take() {
                Some(node) => {
                    // 新たな head に取り出した Node.next をセット
                    self.head = node.next;
                    self.len -= 1;
                    Some(node.data)
                }
                None => None,
            }
        }

        /// 末尾に要素を追加
        pub fn push_back(&mut self, data: T) {
            // TODO: 実装してください（チャレンジ課題）
            // ヒント: 末尾まで辿る必要がある
            let new_node = Box::new(Node { data, next: None });

            match self.head {
                None => {
                    self.head = Some(new_node);
                }
                Some(ref mut head) => {
                    let mut current = head;
                    while current.next.is_some() {
                        current = current.next.as_mut().unwrap();
                    }

                    current.next = Some(new_node);
                }
            }
            self.len += 1;
        }

        /// 先頭の要素を参照
        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.data)
        }

        /// 要素数
        pub fn len(&self) -> usize {
            self.len
        }

        /// 空かどうか
        pub fn is_empty(&self) -> bool {
            self.head.is_none()
        }

        pub fn iter(&self) -> Iter<T> {
            Iter {
                current: self.head.as_ref().map(|node| &**node),
            }
        }
    }

    pub struct Iter<'a, T> {
        current: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.current.map(|node| {
                self.current = node.next.as_ref().map(|n| &**n);
                &node.data
            })
        }
    }

    impl<T: Display> LinkedList<T> {
        /// リストを表示（デバッグ用）
        pub fn display(&self) {
            let mut current = &self.head;
            print!("LinkedList: ");
            while let Some(node) = current {
                print!("{} -> ", node.data);
                current = &node.next;
            }
            println!("None");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let list: LinkedList<i32> = LinkedList::new();
            assert_eq!(list.len(), 0);
            assert!(list.is_empty());
        }

        #[test]
        fn test_push_front() {
            let mut list = LinkedList::new();
            list.push_front(1);
            list.push_front(2);
            list.push_front(3);

            assert_eq!(list.len(), 3);
            assert_eq!(list.peek(), Some(&3));
        }

        #[test]
        fn test_pop_front() {
            let mut list = LinkedList::new();
            list.push_front(1);
            list.push_front(2);

            assert_eq!(list.pop_front(), Some(2));
            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_front(), None);
        }
    }
}

// ---------------------------------------------------------
// 課題3: スタック（LIFO）
// ---------------------------------------------------------

pub mod stack {
    /// スタックの実装
    /// Vec を内部で使用する簡易版
    pub struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        /// 新しい空のスタックを作成
        pub fn new() -> Self {
            Stack { items: Vec::new() }
        }

        /// 要素をプッシュ
        pub fn push(&mut self, item: T) {
            self.items.push(item);
        }

        /// 要素をポップ
        pub fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        /// トップの要素を参照
        pub fn peek(&self) -> Option<&T> {
            self.items.last()
        }

        /// 空かどうか
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// 要素数
        pub fn len(&self) -> usize {
            self.items.len()
        }
    }

    /// 括弧のバランスをチェックする関数（応用問題）
    ///
    /// # Examples
    /// ```
    /// assert_eq!(check_balanced_parentheses("()"), true);
    /// assert_eq!(check_balanced_parentheses("(()"), false);
    /// assert_eq!(check_balanced_parentheses("{[()]}"), true);
    /// ```
    pub fn check_balanced_parentheses(s: &str) -> bool {
        // TODO: 実装してください
        // ヒント:
        // 1. 開き括弧が来たらスタックにプッシュ
        // 2. 閉じ括弧が来たらスタックからポップして対応を確認
        // 3. 最後にスタックが空なら true
        let mut stack = Stack::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_stack_operations() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);
            stack.push(3);

            assert_eq!(stack.peek(), Some(&3));
            assert_eq!(stack.pop(), Some(3));
            assert_eq!(stack.pop(), Some(2));
            assert_eq!(stack.len(), 1);
        }

        #[test]
        fn test_balanced_parentheses() {
            assert!(check_balanced_parentheses("()"));
            assert!(check_balanced_parentheses("()[]{}"));
            assert!(check_balanced_parentheses("{[()]}"));
            assert!(!check_balanced_parentheses("(()"));
            assert!(!check_balanced_parentheses("({)}"));
        }
    }
}

// ---------------------------------------------------------
// 課題4: キュー（FIFO）
// ---------------------------------------------------------

pub mod queue {
    /// キューの実装
    pub struct Queue<T> {
        items: Vec<T>,
    }

    impl<T> Queue<T> {
        /// 新しい空のキューを作成
        pub fn new() -> Self {
            Self { items: Vec::new() }
        }

        /// 要素をエンキュー（末尾に追加）
        pub fn enqueue(&mut self, item: T) {
            self.items.push(item)
        }

        /// 要素をデキュー（先頭から削除）
        pub fn dequeue(&mut self) -> Option<T> {
            // 注意: Vec::remove(0) は O(n) なので、後でリングバッファ版も実装してみよう
            if self.len() > 0 {
                Some(self.items.remove(0))
            } else {
                None
            }
        }

        /// 先頭の要素を参照
        pub fn peek(&self) -> Option<&T> {
            if self.len() > 0 {
                Some(&self.items[0])
            } else {
                None
            }
        }

        /// 空かどうか
        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        /// 要素数
        pub fn len(&self) -> usize {
            self.items.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_queue_operations() {
            let mut queue = Queue::new();
            queue.enqueue(1);
            queue.enqueue(2);
            queue.enqueue(3);

            assert_eq!(queue.dequeue(), Some(1));
            assert_eq!(queue.dequeue(), Some(2));
            assert_eq!(queue.len(), 1);
            assert_eq!(queue.peek(), Some(&3));
        }
    }
}

// ---------------------------------------------------------
// 課題5: シンプルなハッシュマップ
// ---------------------------------------------------------

pub mod hash_map {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::LinkedList;
    use std::hash::{Hash, Hasher};

    /// キーと値のペア
    #[derive(Clone)]
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    /// シンプルなハッシュマップ（チェイニング方式）
    pub struct SimpleHashMap<K, V> {
        buckets: Vec<LinkedList<Entry<K, V>>>,
        len: usize,
        capacity: usize,
    }

    impl<K: Hash + Eq, V> SimpleHashMap<K, V> {
        /// 新しいハッシュマップを作成
        pub fn new() -> Self {
            Self::with_capacity(16)
        }

        /// 指定した容量でハッシュマップを作成
        pub fn with_capacity(capacity: usize) -> Self {
            let mut buckets = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buckets.push(LinkedList::new());
            }

            SimpleHashMap {
                buckets,
                len: 0,
                capacity,
            }
        }

        /// ハッシュ値を計算
        fn hash(&self, key: &K) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() as usize) % self.capacity
        }

        /// キーと値を挿入
        pub fn insert(&mut self, key: K, value: V) {
            // 1. hash(key) でバケットインデックスを取得
            let index = self.hash(&key);
            // 2. そのバケットの中に同じキーがあるか確認
            let bucket = &mut self.buckets[index];
            // 3. あれば値を更新、なければ新しいエントリを追加
            for entry in bucket.iter_mut() {
                if entry.key == key {
                    entry.value = value;
                    return;
                }
            }
            // 4. len をインクリメント
            bucket.push_back(Entry { key, value });
            self.len += 1;
            // 5. 負荷率をチェックして、必要なら resize
            if self.load_factor() > 0.75 {
                self.resize();
            }
        }

        /// キーで値を取得
        pub fn get(&self, key: &K) -> Option<&V> {
            let index = self.hash(key);
            let bucket = &self.buckets[index];
            for entry in bucket.iter() {
                if &entry.key == key {
                    return Some(&entry.value);
                }
            }
            None
        }

        /// キーを削除
        pub fn remove(&mut self, key: &K) -> Option<V> {
            let index = self.hash(key);
            let bucket = &mut self.buckets[index];
            // entry を探して削除
            // key を持つ entry の位置を把握
            let position = bucket.iter().position(|entry| &entry.key == key)?;
            // LinkedList をその位置で分割
            let mut after = bucket.split_off(position);
            // pop_front で取り出し
            let removed = after.pop_front().unwrap();
            // 分割して削除した後のLinkedListをバケットに追加
            bucket.append(&mut after);

            self.len -= 1;
            Some(removed.value)
        }

        /// キーが存在するか確認
        pub fn contains_key(&self, key: &K) -> bool {
            self.get(key).is_some()
        }

        /// 要素数
        pub fn len(&self) -> usize {
            self.len
        }

        /// 負荷率（load factor）
        fn load_factor(&self) -> f64 {
            self.len as f64 / self.capacity as f64
        }

        /// リサイズ（チャレンジ課題）
        fn resize(&mut self) {
            // 1. 新しい容量を計算
            let new_capacity = self.capacity * 2;

            // 2. 新しいバケット配列を作成
            let mut new_buckets = Vec::new();
            for _ in 0..new_capacity {
                new_buckets.push(LinkedList::new());
            }

            // 3. 古いバケットから全要素を取り出す
            for bucket in self.buckets.iter_mut() {
                while let Some(entry) = bucket.pop_front() {
                    // 4. 新しい容量でハッシュ値を再計算
                    let mut hasher = DefaultHasher::new();
                    entry.key.hash(&mut hasher);
                    let new_index = (hasher.finish() as usize) % new_capacity;

                    // 5. 新しいバケットに挿入
                    new_buckets[new_index].push_back(entry);
                }
            }

            // 6. 古いバケットを新しいバケットで置き換え
            self.buckets = new_buckets;
            self.capacity = new_capacity;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insert_and_get() {
            let mut map = SimpleHashMap::new();
            map.insert("apple", 100);
            map.insert("banana", 200);

            assert_eq!(map.get(&"apple"), Some(&100));
            assert_eq!(map.get(&"banana"), Some(&200));
            assert_eq!(map.get(&"cherry"), None);
        }

        #[test]
        fn test_update_value() {
            let mut map = SimpleHashMap::new();
            map.insert("key", 1);
            map.insert("key", 2);

            assert_eq!(map.get(&"key"), Some(&2));
            assert_eq!(map.len(), 1);
        }
    }
}
