// Week 1のモジュールを宣言
// モジュールファイルの配置: src/crate/week1_basic_structures/src.rs
#[path = "crate/week1_basic_structures/src.rs"]
mod week1_basic_structures;

fn main() {
    println!("=== CS Learning with Rust ===");
    println!("コンピュータサイエンス学習プロジェクトへようこそ！");
    println!();
    println!("カリキュラムについては CS_CURRICULUM.md を参照してください。");
    println!("Week 1-2 の課題については cs_learning/src/crate/week1_basic_structures/README.md を参照してください。");
    println!();
    println!("テストを実行するには:");
    println!("  cd cs_learning && cargo test");
    println!();
    println!("特定のモジュールのテストを実行するには:");
    println!("  cd cs_learning && cargo test my_vec");
    println!("  cd cs_learning && cargo test linked_list");
    println!("  cd cs_learning && cargo test stack");
    println!("  cd cs_learning && cargo test queue");
    println!("  cd cs_learning && cargo test hash_map");
}
