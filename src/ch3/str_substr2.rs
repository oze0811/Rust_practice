fn main() {
    let pr = "我輩は猫である";

    // 先頭2文字の部分文字列を得る
    let sub3: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub3);

    // 4-5文字目を取り出す
    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars = &pr_chars[3..=4];
    // スライスを文字列に変換
    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5文字目: {}", sub4)
}
