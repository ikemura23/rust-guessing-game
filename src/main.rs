// ユーザ入力を受け付け、結果を出力するライブラリ
use std::io;
// 乱数生成
use rand::Rng;

fn main() {
    println!("数を当ててごらん");

    // 下限値は含むが上限値は含まない、1..100の範囲になる
    // 同等で 1..=100 という範囲も渡せる
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数値は次の通り: {}", secret_number);
    println!("ほら、予想を入力してね");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("次のように予想しました: {}", guess);
}
