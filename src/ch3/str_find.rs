fn main() {
    let s = "我輩は猫である";

    // "猫"を検索
    match s.find('猫') {
        Some(i) => println!("猫={}B", i),
        None => println!("猫はなし"),
    };
}
