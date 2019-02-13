fn main() {
    let s = String::from("字符串a");
    let s_bor = &s; // 一个不可变变量(s)的不可变引用(&s)赋值给不可变变量(s_bor)
    // 此时s的类型是String, s_bor的类型是&String
    /*
    let s_mut_ref = &mut s; // 不能得到一个不可变变量(s)的可变引用(&mut s)
    */
    println!("{}|{}", s, s_bor);
    assert_eq!(&s, s_bor);
    assert_eq!(s, *s_bor); // (*)操作符解引用

    let mut s = String::from("字符串b");
    s.push_str("other");
    {
        let s_mut_bor = &mut s; // 一个可变变量(s)的可变引用(&mut s)赋值给不可变变量(s_mut_bor)
        // 进行可变引用后，在s_mut_bor的作用域内，不能有第二个的可变引用
        // s.push_str("other"); // 对原来的变量(s)使用可变方法会出现一个可变引用(传参时)，因此不能使用可变方法
        println!("{}", s_mut_bor);
        println!("{}", s.to_uppercase()); // 会隐式地出现一个不可变引用(传参时)，因此只能在可变引用之后使用不可变方法
        // 只能在所有用到可变引用之后使用不可变引用，上面2行互换会error
        // 官方文档说要么只能有一个可变引用，要么只能有多个不可变引用，但实际上可变引用和不可变引用是可以共存的，只要保证用到可变引用的地方始终在用到不可变引用的地方之前
        let s_bor = &s; // 可以继续使用不可变引用
        println!("{}", s_bor);
    }
    let s_mut_bor2 = &mut s; // 离开了上面那个s_mut_bor的作用域，可以用可变引用了
    println!("{}", s_mut_bor2);
}
