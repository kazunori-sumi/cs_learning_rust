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
        // TODO: 実装してください
        // ヒント:
        // 1. 外側のループ: n-1 回
        // 2. 内側のループ: 隣接要素を比較
        // 3. 順番が逆なら swap
        todo!()
    }

    /// バブルソート（最適化版）
    ///
    /// 交換が発生しなかった場合、既にソート済みなので終了する
    pub fn bubble_sort_optimized<T: Ord>(arr: &mut [T]) {
        // TODO: チャレンジ課題
        // ヒント: swapped フラグを使う
        todo!()
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
        // TODO: 実装してください
        // ヒント:
        // 1. 外側のループ: i = 0 to n-1
        // 2. 未ソート部分 (i..n) から最小値のインデックスを見つける
        // 3. arr[i] と arr[min_index] を交換
        todo!()
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
        // TODO: 実装してください
        // ヒント:
        // 1. i = 1 から開始（arr[0] はソート済みとみなす）
        // 2. arr[i] を取り出す
        // 3. ソート済み部分で適切な位置を見つける
        // 4. 要素をシフトして挿入
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[ignore]
        fn test_bubble_sort() {
            let mut arr = vec![5, 2, 8, 1, 9];
            bubble_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);
        }

        #[test]
        #[ignore]
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
        #[ignore]
        fn test_selection_sort() {
            let mut arr = vec![5, 2, 8, 1, 9];
            selection_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 5, 8, 9]);
        }

        #[test]
        #[ignore]
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
        #[ignore]
        fn test_sort_single_element() {
            let mut arr = vec![42];
            selection_sort(&mut arr);
            assert_eq!(arr, vec![42]);
        }

        #[test]
        #[ignore]
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
        // TODO: 実装してください
        // ヒント:
        // 1. 配列が1要素以下なら終了
        // 2. 配列を半分に分割
        // 3. 左半分を再帰的にソート
        // 4. 右半分を再帰的にソート
        // 5. 2つのソート済み配列をマージ
        todo!()
    }

    /// 2つのソート済み配列をマージする補助関数
    fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
        // TODO: 実装してください
        // ヒント:
        // 1. 結果用の配列を作成
        // 2. leftとrightの先頭から順に小さい方を結果に追加
        // 3. どちらかが終わったら、残りをすべて追加
        todo!()
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
        // TODO: 実装してください
        // ヒント:
        // 1. 配列が1要素以下なら終了
        // 2. ピボットを選ぶ（最後の要素など）
        // 3. partition: ピボットより小さい要素を左、大きい要素を右に
        // 4. 左側を再帰的にソート
        // 5. 右側を再帰的にソート
        todo!()
    }

    /// パーティション操作（クイックソートの補助関数）
    fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
        // TODO: 実装してください
        // ヒント: Lomuto partition scheme または Hoare partition scheme
        todo!()
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
        // 1. 配列を最大ヒープに変換（heapify）
        // 2. 最大値（ルート）を配列の最後と交換
        // 3. ヒープサイズを1減らす
        // 4. ルートから再度heapify
        // 5. ヒープサイズが1になるまで繰り返す
        todo!()
    }

    /// ヒープの性質を維持する補助関数
    fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
        // TODO: チャレンジ課題
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[ignore]
        fn test_merge_sort() {
            let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 7, 8, 9]);
        }

        #[test]
        #[ignore]
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
        #[ignore]
        fn test_merge_sort_reverse() {
            let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
            merge_sort(&mut arr);
            assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }

        #[test]
        #[ignore]
        fn test_quick_sort_duplicates() {
            let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
            quick_sort(&mut arr);
            assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
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
