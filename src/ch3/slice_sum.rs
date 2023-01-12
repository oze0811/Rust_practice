// スライスの各要素を加算する関数
fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for i in items {
        total += i;
    }
    total
}

fn main() {
    // 配列を初期化
    let a = [1, 2, 3, 4, 5];
    println!("a={}", sum_slice(&a[..]));
    // ベクターを初期化
    let b = vec![1, 2, 3, 4, 5];
    println!("b={}", sum_slice(&b[..]));
}
