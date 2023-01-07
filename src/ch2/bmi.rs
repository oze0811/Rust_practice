// RustでBMI肥満度判定ツール
fn main() {
    // 身長と体重の入力
    let height_cm = input("身長(cm)は?");
    let weight = input("体重(kg)は?");
    // BMIの計算
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
}

fn input(prompt: &str) -> f64 {
    // メッセージを表示
    println!("{}", prompt);
    // 入力を得る
    let mut s = String::new();
    std::io::stdin() .read_line (&mut s).expect("入力エラー");
    // 空白を除去して数値に変換
    return s.trim().parse().expect("数値変換エラー");
}
