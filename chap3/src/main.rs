fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let s_ref = &s;
    let s_ref2 = &s;
    myprint(s_ref);    // リファレンスによって関数に渡している
    myprint(s_ref);    // s が所有権を失わないので 2 回実行できる
    myprint(s_ref2);    // リファレンスによって関数に渡している
    myprint(s_ref2);    // s が所有権を失わないので 2 回実行できる
}
