use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数を当ててごらん");

    // 下限値は含むが上限値は含まない、1..100の範囲になる
    // 同等で 1..=100 という範囲も渡せる
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("秘密の数値は次の通り: {}", secret_number);
    loop {
        println!("ほら、予想を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // シャドーイング
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("次のように予想しました: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("同じだ、やったね！");
                break;
            }
        }
    }
}
