
enum TradeDirection {
    buy,
    sell,
}

struct NewOrderSingle {
    account_id: [char;10],
    client_ord_id: u64,
    security_id: [char;8],
    direction: char,
    price: i64,
    count: i64,
}

fn main() {
    println!("Hello, world!");

    let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];


}
