fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", *msg);
}

fn main() {
    let s = "Hello".to_string();
    myprint(&s);    // リファレンスによって関数に渡している
    myprint(&s);    // s が所有権を失わないので 2 回実行できる
}
