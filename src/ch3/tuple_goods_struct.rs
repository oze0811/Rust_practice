struct Item(String, i64);
fn main() {
    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("りんご".to_string(), 200);
    // Itemをベクターに追加
    let items = vec![banana, apple];
    // 合計金額を求める
    let total = print_and_sum_items(&items);
    println!("合計{}円", total)
}

// タプルを表示する関数
fn print_tuple(item: &Item) {
    println!("{}を{}円で購入", item.0, item.1);
}

// アイテムを順に表示し合計金額を求める
fn print_and_sum_items(items: & Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}
