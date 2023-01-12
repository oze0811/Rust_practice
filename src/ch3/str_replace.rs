fn main() {
    let s = "我輩は猫である";
    // 文字列の置換
    let s2 = s.replace("猫", "犬");
    let s3 = s2.replace("我輩は", "私は");
    // 置換前と置換後を表示
    println!("置換前: {}\n置換後: {}", s, s3);
}
