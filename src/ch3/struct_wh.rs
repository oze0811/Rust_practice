// 身長と体重のデータをもつ構造体
struct Body {
    weigh: f64,
    height: f64,
}

fn main() {
    // 構造体を初期化
    let ichi = Body{weigh: 80.0, height: 165.0};
    let jiro = Body{weigh: 80.0, height: 170.0};
    // 関数を呼び出す
    println!("Ichi={:.1}", calc_bmi(&ichi));
    println!("Jiro={:.1}", calc_bmi(&jiro));
}

fn calc_bmi(body: &Body) -> f64 {
    let h = body.height/100.0;
    body.weigh / h.powf(2.0)
}
