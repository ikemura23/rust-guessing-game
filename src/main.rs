use std::io; // ユーザ入力を受け付け、結果を出力するライブラリ

fn main() {
    println!("数を当ててごらん");
    println!("ほら、予想を入力してね");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("次のように予想しました: {}", guess);
}
