/*
1から50までの数を画面に表示するが、
3の倍数と3のつく整数の場合にはAと画面に表示するコード
*/
fn main() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3{
            println!("A");
        } else if i >= 30 && i <= 39 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}
