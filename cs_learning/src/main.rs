// Week 1のモジュールを宣言
// モジュールファイルの配置: src/crate/week1_basic_structures/src.rs
#[path = "crate/week1_basic_structures/src.rs"]
mod week1_basic_structures;

// Week 3-4のモジュールを宣言
// モジュールファイルの配置: src/crate/week3_search_sort/src.rs
#[path = "crate/week3_search_sort/src.rs"]
mod week3_search_sort;

fn main() {
    println!("hash: {}", simple_hash("test_string"));
    println!("=== CS Learning with Rust ===");
    println!("コンピュータサイエンス学習プロジェクトへようこそ！");
    println!();
    println!("カリキュラムについては CS_CURRICULUM.md を参照してください。");
    println!();
    println!("各週の課題:");
    println!("  Week 1-2: cs_learning/src/crate/week1_basic_structures/README.md");
    println!("  Week 3-4: cs_learning/src/crate/week3_search_sort/README.md");
    println!();
    println!("テストを実行するには:");
    println!("  cd cs_learning && cargo test");
    println!();
    println!("特定のモジュールのテストを実行するには:");
    println!("  Week 1-2:");
    println!("    cd cs_learning && cargo test week1_basic_structures");
    println!("  Week 3-4:");
    println!("    cd cs_learning && cargo test week3_search_sort");
}

fn simple_hash(s: &str) -> usize {
    let mut hash = 0;
    for (i, ch) in s.chars().enumerate() {
        hash += (ch as usize) * (31_usize.pow(i as u32));
    }
    return hash;
}
