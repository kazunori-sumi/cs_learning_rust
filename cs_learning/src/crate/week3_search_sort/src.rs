// Week 3-4: 探索とソートアルゴリズム

// ---------------------------------------------------------
// 課題1: 探索アルゴリズム
// ---------------------------------------------------------

pub mod search {
    /// 線形探索
    ///
    /// # 計算量
    /// - 時間: O(n)
    /// - 空間: O(1)
    ///
    /// # Examples
    /// ```
    /// let arr = vec![5, 2, 8, 1, 9];
    /// assert_eq!(linear_search(&arr, &8), Some(2));
    /// ```
    pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        for i in 0..arr.len() {
            if &arr[i] == target {
                return Some(i);
            }
        }
        None
    }

    /// 二分探索（反復版）
    ///
    /// # 前提条件
    /// 配列は昇順にソートされている必要があります
    ///
    /// # 計算量
    /// - 時間: O(log n)
    /// - 空間: O(1)
    ///
    /// # Examples
    /// ```
    /// let arr = vec![1, 2, 5, 8, 9];
    /// assert_eq!(binary_search(&arr, &5), Some(2));
    /// ```
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        // 1. left = 0, right = arr.len() - 1
        // 2. while left <= right
        // 3.   mid = (left + right) / 2
        // 4.   arr[mid] と target を比較
        // 5.   target が小さければ右半分を捨てる（right = mid - 1）
        // 6.   target が大きければ左半分を捨てる（left = mid + 1）
        let mut left = 0;
        let mut right = arr.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if &arr[mid] < target {
                left = mid + 1;
            } else if &arr[mid] > target {
                right = mid - 1;
            } else {
                return Some(mid);
            }
        }

        None
    }

    /// 二分探索（再帰版）
    ///
    /// # 計算量
    /// - 時間: O(log n)
    /// - 空間: O(log n)（再帰スタック）
    pub fn binary_search_recursive<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        fn search_helper<T: Ord>(
            arr: &[T],
            target: &T,
            left: usize,
            right: usize,
        ) -> Option<usize> {
            if left > right {
                // 忘れないようにすること
                return None;
            }
            let mid = (left + right) / 2;
            if &arr[mid] == target {
                return Some(mid);
            }
            if &arr[mid] < target {
                return search_helper(arr, target, mid + 1, right);
            } else if &arr[mid] > target {
                // mid - 1 によるアンダーフロー対策
                if let Some(new_right) = mid.checked_sub(1) {
                    return search_helper(arr, target, left, new_right);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        if arr.is_empty() {
            return None;
        }
        search_helper(arr, target, 0, arr.len() - 1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_linear_search() {
            let arr = vec![5, 2, 8, 1, 9];
            assert_eq!(linear_search(&arr, &8), Some(2));
            assert_eq!(linear_search(&arr, &1), Some(3));
            assert_eq!(linear_search(&arr, &10), None);
        }

        #[test]
        fn test_linear_search_empty() {
            let arr: Vec<i32> = vec![];
            assert_eq!(linear_search(&arr, &5), None);
        }

        #[test]
        fn test_binary_search() {
            let arr = vec![1, 2, 5, 8, 9];
            assert_eq!(binary_search(&arr, &5), Some(2));
            assert_eq!(binary_search(&arr, &1), Some(0));
            assert_eq!(binary_search(&arr, &9), Some(4));
            assert_eq!(binary_search(&arr, &10), None);
        }

        #[test]
        fn test_binary_search_recursive() {
            let arr = vec![1, 2, 5, 8, 9];
            assert_eq!(binary_search_recursive(&arr, &5), Some(2));
            assert_eq!(binary_search_recursive(&arr, &1), Some(0));
            assert_eq!(binary_search_recursive(&arr, &9), Some(4));
            assert_eq!(binary_search_recursive(&arr, &10), None);
        }
    }
}

// ---------------------------------------------------------
// 課題2: 基本ソートアルゴリズム（O(n²)）
// ---------------------------------------------------------

pub mod basic_sorts {
    /// バブルソート
    ///
    /// # 計算量
    /// - 時間: O(n²)
    /// - 空間: O(1)
    /// - 安定: Yes
    ///
    /// # アルゴリズム
    /// 隣接する要素を比較し、順番が逆なら交換する。
    /// 最大値が配列の最後まで「浮かび上がる」様子からバブルソートと呼ばれる。
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        // 1. 外側のループ: n-1 回
        // 2. 内側のループ: 隣接要素を比較
        // 3. 順番が逆なら swap
        let last_idx = arr.len() - 1;
        for i in 0..last_idx {
            for j in 0..last_idx - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1); // 一時変数を使った場合はTがCopyトレイトを実装するような制限が必要
                }
            }
        }
    }

    /// バブルソート（最適化版）
    ///
    /// 交換が発生しなかった場合、既にソート済みなので終了する
    pub fn bubble_sort_optimized<T: Ord>(arr: &mut [T]) {
        bubble_sort_optimized_with_count(arr);
    }

    /// 選択ソート
    ///
    /// # 計算量
    /// - 時間: O(n²)
    /// - 空間: O(1)
    /// - 安定: No
    ///
    /// # アルゴリズム
    /// 未ソート部分から最小値を見つけて、先頭に移動する。
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        // 1. 外側のループ: i = 0 to n-1
        // 2. 未ソート部分 (i..n) から最小値のインデックスを見つける
        // 3. arr[i] と arr[min_index] を交換
        for i in 0..arr.len() - 1 {
            let mut min_idx = i;
            for j in (i + 1)..arr.len() {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            if min_idx != i {
                arr.swap(i, min_idx);
            }
        }
    }

    /// 挿入ソート
    ///
    /// # 計算量
    /// - 時間: O(n²)（最悪）、O(n)（最良：ソート済み）
    /// - 空間: O(1)
    /// - 安定: Yes
    ///
    /// # アルゴリズム
    /// 要素を1つずつ取り出し、ソート済み部分の適切な位置に挿入する。
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        // 1. i = 1 から開始（arr[0] はソート済みとみなす）
        // 2. arr[i] を取り出す
        // 3. ソート済み部分で適切な位置を見つける
        // 4. 要素をシフトして挿入

        for i in 1..arr.len() {
            let mut j = i;
            // 後ろから前に比較し、arr[j]が適切な位置に来るまでswap
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // テスト用：比較回数とswap回数をカウントするバージョン
    fn bubble_sort_optimized_with_count<T: Ord>(arr: &mut [T]) -> (usize, usize) {
        let last_idx = arr.len() - 1;
        let mut comparisons = 0;
        let mut swaps = 0;

        for i in 0..last_idx {
            let mut swapped = false;
            for j in 0..last_idx - i {
                comparisons += 1;
                if arr[j] > arr[j + 1] {
                    swapped = true;
                    arr.swap(j, j + 1);
                    swaps += 1;
                }
            }
            if !swapped {
                break;
            }
        }
        (comparisons, swaps)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = vec![5, 2, 8, 1, 9];
            bubble_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);
        }

        #[test]
        fn test_bubble_sort_optimized() {
            let mut arr = vec![5, 2, 8, 1, 9];
            bubble_sort_optimized(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);

            // ほぼソート済みのケース
            let mut arr = vec![1, 2, 3, 5, 4];
            bubble_sort_optimized(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        }

        #[test]
        fn test_bubble_sort_optimized_early_termination() {
            // 既にソート済みの配列
            let mut arr = vec![1, 2, 3, 4, 5];
            let (comparisons, swaps) = bubble_sort_optimized_with_count(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5]);

            // 最適化が機能していれば、1パス（n-1回の比較）で終了するはず
            // 期待値: 4回の比較、0回のswap
            println!(
                "ソート済み配列: comparisons={}, swaps={}",
                comparisons, swaps
            );
            assert_eq!(swaps, 0, "ソート済み配列なのでswapは0回のはず");
            assert_eq!(
                comparisons, 4,
                "最適化が機能していれば1パス(4回)で終了するはず"
            );

            // ほぼソート済み（1箇所だけ乱れている）
            let mut arr = vec![1, 2, 3, 5, 4];
            let (comparisons, swaps) = bubble_sort_optimized_with_count(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5]);

            // 最適化が機能していれば、2パスで終了するはず
            // 1パス目: 4回比較、1回swap → [1,2,3,4,5]
            // 2パス目: 3回比較、0回swap → 早期終了
            // 期待値: 7回の比較、1回のswap
            println!(
                "ほぼソート済み配列: comparisons={}, swaps={}",
                comparisons, swaps
            );
            assert_eq!(swaps, 1, "1箇所の乱れなので1回のswapのはず");
            // このテストが失敗したら、最適化が機能していない証拠
            assert!(
                comparisons <= 7,
                "最適化が機能していれば7回以下の比較で終了するはず。実際: {}回",
                comparisons
            );
        }

        #[test]
        fn test_bubble_sort_optimized_reverse_array() {
            // 逆順配列（最悪ケース）- ループ回数不足のバグを検出
            let mut arr = vec![5, 4, 3, 2, 1];
            bubble_sort_optimized(&mut arr);
            assert_eq!(
                arr,
                vec![1, 2, 3, 4, 5],
                "逆順配列が正しくソートされていない"
            );
        }

        #[test]
        fn test_selection_sort() {
            let mut arr = vec![5, 2, 8, 1, 9];
            selection_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);
        }

        #[test]
        fn test_insertion_sort() {
            let mut arr = vec![5, 2, 8, 1, 9];
            insertion_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);
        }

        #[test]
        #[ignore]
        fn test_sort_empty_array() {
            let mut arr: Vec<i32> = vec![];
            bubble_sort(&mut arr);
            assert_eq!(arr, vec![]);
        }

        #[test]
        fn test_sort_single_element() {
            let mut arr = vec![42];
            selection_sort(&mut arr);
            assert_eq!(arr, vec![42]);
        }

        #[test]
        fn test_sort_already_sorted() {
            let mut arr = vec![1, 2, 3, 4, 5];
            insertion_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        }
    }
}

// ---------------------------------------------------------
// 課題3: 高度なソートアルゴリズム（O(n log n)）
// ---------------------------------------------------------

pub mod advanced_sorts {
    /// マージソート
    ///
    /// # 計算量
    /// - 時間: O(n log n)（すべてのケース）
    /// - 空間: O(n)
    /// - 安定: Yes
    ///
    /// # アルゴリズム
    /// 分割統治法。配列を再帰的に半分に分割し、ソートしてマージする。
    pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        // ヒント:
        // 1. 配列が1要素以下なら終了
        if arr.len() <= 1 {
            return;
        }
        // 2. 配列を半分に分割（mid = arr.len() / 2）
        let mid = arr.len() / 2;
        // 3. 左半分を再帰的にソート（&mut arr[..mid]）
        merge_sort(&mut arr[..mid]);
        // 4. 右半分を再帰的にソート（&mut arr[mid..]）
        merge_sort(&mut arr[mid..]);
        // 5. 2つのソート済み配列をマージ（merge関数使用）
        let merged = merge(&arr[..mid], &arr[mid..]);
        // 6. マージ結果を元の配列にコピーバック（arr.copy_from_slice）
        arr.clone_from_slice(&merged);
    }

    /// 2つのソート済み配列をマージする補助関数
    fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(left.len() + right.len());
        let mut left_iter = left.iter().peekable();
        let mut right_iter = right.iter().peekable();

        while let (Some(&l), Some(&r)) = (left_iter.peek(), right_iter.peek()) {
            if l <= r {
                result.push(left_iter.next().unwrap().clone());
            } else {
                result.push(right_iter.next().unwrap().clone());
            }
        }

        result.extend(left_iter.cloned());
        result.extend(right_iter.cloned());

        return result;
    }

    /// クイックソート
    ///
    /// # 計算量
    /// - 時間: O(n log n)（平均）、O(n²)（最悪）
    /// - 空間: O(log n)（再帰スタック）
    /// - 安定: No
    ///
    /// # アルゴリズム
    /// ピボットを選び、それより小さい要素と大きい要素に分割して再帰的にソート。
    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        // 1. 配列が1要素以下なら終了（arr.len() <= 1）
        if arr.len() <= 1 {
            return;
        }
        // 2. partition関数を呼び出してピボット位置を取得
        let pivot_posi = partition(arr);
        // 3. 左側を再帰的にソート（&mut arr[..pivot_pos]）
        quick_sort(&mut arr[..pivot_posi]);
        // 4. 右側を再帰的にソート（&mut arr[pivot_pos + 1..]）
        quick_sort(&mut arr[pivot_posi + 1..]);
    }

    /// パーティション操作（クイックソートの補助関数）
    ///
    /// 配列の最後の要素をピボットとして、ピボットより小さい要素を左、
    /// 大きい要素を右に分割し、ピボットの最終位置を返す。
    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        // ヒント（Lomuto partition scheme）:
        // 1. ピボット = 最後の要素（arr[arr.len() - 1]）
        let pivot_index = arr.len() - 1;
        // 2. i = -1（小さい要素の境界）
        let mut i: isize = -1;
        for j in 0..arr.len() - 1 {
            if arr[j] <= arr[pivot_index] {
                i += 1;
                arr.swap(i as usize, j);
            }
        }
        let pivot_pos = (i + 1) as usize;
        arr.swap(pivot_pos, pivot_index);

        pivot_pos
    }

    /// ヒープソート（チャレンジ課題）
    ///
    /// # 計算量
    /// - 時間: O(n log n)
    /// - 空間: O(1)
    /// - 安定: No
    pub fn heap_sort<T: Ord>(arr: &mut [T]) {
        // TODO: チャレンジ課題
        // ヒント:
        // 1. 配列を最大ヒープに変換（i = n/2 - 1 から 0 まで heapify を呼ぶ）
        // 2. ヒープサイズ n から 1 まで繰り返す:
        //    a. 最大値（arr[0]）を配列の最後（arr[n-1]）と交換
        //    b. ヒープサイズを1減らす
        //    c. ルート（0）に対して heapify を実行
        todo!()
    }

    /// ヒープの性質を維持する補助関数（sift_down操作）
    ///
    /// インデックス i を根とする部分木を最大ヒープにする。
    /// n はヒープのサイズ（配列全体のサイズではない場合がある）。
    fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
        // TODO: チャレンジ課題
        // ヒント:
        // 1. largest = i（現在の最大値）
        // 2. left = 2 * i + 1（左の子）
        // 3. right = 2 * i + 2（右の子）
        // 4. left < n かつ arr[left] > arr[largest] なら largest = left
        // 5. right < n かつ arr[right] > arr[largest] なら largest = right
        // 6. largest != i なら swap(arr[i], arr[largest]) して heapify(arr, n, largest) を再帰呼び出し
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_merge_sort() {
            let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 7, 8, 9]);
        }

        #[test]
        fn test_quick_sort() {
            let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
            quick_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 7, 8, 9]);
        }

        #[test]
        #[ignore]
        fn test_heap_sort() {
            let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
            heap_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 7, 8, 9]);
        }

        #[test]
        fn test_merge_sort_reverse() {
            let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }

        #[test]
        fn test_merge_sort_empty() {
            let mut arr: Vec<i32> = vec![];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![]);
        }

        #[test]
        fn test_quick_sort_duplicates() {
            let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
            quick_sort(&mut arr);
            assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
        }

        #[test]
        fn test_quick_sort_empty() {
            let mut arr: Vec<i32> = vec![];
            quick_sort(&mut arr);
            assert_eq!(arr, vec![]);
        }

        #[test]
        #[ignore]
        fn test_heap_sort_empty() {
            let mut arr: Vec<i32> = vec![];
            heap_sort(&mut arr);
            assert_eq!(arr, vec![]);
        }

        #[test]
        fn test_merge_sort_single() {
            let mut arr = vec![42];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![42]);
        }

        #[test]
        #[ignore]
        fn test_quick_sort_single() {
            let mut arr = vec![42];
            quick_sort(&mut arr);
            assert_eq!(arr, vec![42]);
        }

        #[test]
        #[ignore]
        fn test_heap_sort_single() {
            let mut arr = vec![42];
            heap_sort(&mut arr);
            assert_eq!(arr, vec![42]);
        }
    }
}

// ---------------------------------------------------------
// 課題4: 応用問題
// ---------------------------------------------------------

pub mod applications {
    use std::cmp::Ordering;

    /// カスタム比較関数を使ったソート
    ///
    /// # Examples
    /// ```
    /// let mut arr = vec![5, 2, 8, 1, 9];
    /// sort_by(&mut arr, |a, b| b.cmp(a)); // 降順
    /// assert_eq!(arr, vec![9, 8, 5, 2, 1]);
    /// ```
    pub fn sort_by<T, F>(arr: &mut [T], compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        // TODO: 実装してください
        // ヒント: 挿入ソートやクイックソートを改造して compare を使う
        todo!()
    }

    /// 安定性をテストするための構造体
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Item {
        pub key: i32,
        pub id: usize,
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            self.key.cmp(&other.key)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::week3_search_sort::basic_sorts::insertion_sort;

        #[test]
        #[ignore]
        fn test_sort_by_descending() {
            let mut arr = vec![5, 2, 8, 1, 9];
            sort_by(&mut arr, |a, b| b.cmp(a));
            assert_eq!(arr, vec![9, 8, 5, 2, 1]);
        }

        #[test]
        #[ignore]
        fn test_stability() {
            let mut arr = vec![
                Item { key: 3, id: 1 },
                Item { key: 1, id: 2 },
                Item { key: 3, id: 3 },
                Item { key: 2, id: 4 },
            ];

            insertion_sort(&mut arr);

            // 安定ソートの場合、同じkeyのidの順序が保たれる
            assert_eq!(arr[0], Item { key: 1, id: 2 });
            assert_eq!(arr[1], Item { key: 2, id: 4 });
            assert_eq!(arr[2], Item { key: 3, id: 1 }); // 最初の3
            assert_eq!(arr[3], Item { key: 3, id: 3 }); // 2番目の3
        }
    }
}

// ---------------------------------------------------------
// ベンチマーク（オプション）
// ---------------------------------------------------------

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn benchmark_sorts() {
        let sizes = vec![10, 100, 1000];

        println!("\n=== Sorting Algorithm Benchmarks ===\n");

        for size in sizes {
            println!("Array size: {}", size);

            // 逆順の配列を生成（最悪ケース）
            let original: Vec<i32> = (0..size).rev().collect();

            // バブルソート
            let mut arr = original.clone();
            let start = Instant::now();
            basic_sorts::bubble_sort(&mut arr);
            let duration = start.elapsed();
            println!("  Bubble Sort:     {:?}", duration);

            // 選択ソート
            let mut arr = original.clone();
            let start = Instant::now();
            basic_sorts::selection_sort(&mut arr);
            let duration = start.elapsed();
            println!("  Selection Sort:  {:?}", duration);

            // 挿入ソート
            let mut arr = original.clone();
            let start = Instant::now();
            basic_sorts::insertion_sort(&mut arr);
            let duration = start.elapsed();
            println!("  Insertion Sort:  {:?}", duration);

            // マージソート
            let mut arr = original.clone();
            let start = Instant::now();
            advanced_sorts::merge_sort(&mut arr);
            let duration = start.elapsed();
            println!("  Merge Sort:      {:?}", duration);

            // クイックソート
            let mut arr = original.clone();
            let start = Instant::now();
            advanced_sorts::quick_sort(&mut arr);
            let duration = start.elapsed();
            println!("  Quick Sort:      {:?}", duration);

            println!();
        }
    }
}
