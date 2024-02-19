#[warn(unused_imports)]
// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

// mod second;
mod test_complex;
mod test_char_bool_unit;

// fn spawn_function() {
//     for i in 0..5 {
//         println!("spawned thread print {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn send_recv() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = 123;
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// fn main() {
    // send_recv();
    // second::penguin_data();
    // let (a,b,c,d,e);
    // (a,b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // e = 7;

    // let d;
    // d = 8;
    // let ii = 5i8;

    // let tt = 100_000;

    // println!("a={}, b={}",a,b);

        // 二进制为00000010
        // let a:i32 = 2;
        // 二进制为00000011
        // let b:i32 = 3;
    
        // println!("(a & b) value is {}", a & b);
    
        // println!("(a | b) value is {}", a | b);
    
        // println!("(a ^ b) value is {}", a ^ b);
    
        // println!("(!b) value is {} ", !b);
    
        // println!("(a << b) value is {}", a << b);
    
        // println!("(a >> b) value is {}", a >> b);
        // for i in 'A'..='z' {
        //     println!("{}",i);
        // }
    
        // let mut a = a;
        // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
        // a <<= b;
        // println!("(a << b) value is {}", a);

        // test_complex::test_complex();
        // test_char_bool_unit::test();

        // let x = "32".parse::<i32>().unwrap();
        // println!("{}", x);
        // let s1 = String::from("hello, world");
        // let s2 = take_ownership(s1);

        // println!("{}", s2);
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    // let s = String::from_raw_parts_();
    // s.ca
    // let x: Box<str> = "xxx";
    // let byte_escape = "I'm writing \x52\x75\x73\x74!";
    // println!("What are you doing\x3F (\x3F means ?) {}", byte_escape);

    // // \u 可以输出一个 unicode 字符
    // let unicode_codepoint = "\u{211D}";
    // let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    // println!(
    //     "Unicode character {} (U+211D) is called {}",
    //     unicode_codepoint, character_name
    // );

    // // 换行了也会保持之前的字符串格式
    // // 使用\忽略换行符
    // let long_string = "String literals
    //                     can span multiple lines.
    //                     The linebreak and indentation here ->\
    //                     <- can be escaped too!";
    // println!("{}", long_string);
// }

// fn take_ownership(s: String) -> String {
//     println!("{}", &s);
//     s
// }
// #[derive(Debug)]
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// mod test_list;
mod test_traits;

fn main() {
    // let age = Some(30);
    // println!("在匹配前，age是{:?}",age);
    // if let Some(age) = age {
    //     println!("匹配出来的age是{:?}",age);
    // }
 
    // println!("在匹配后，age是{:?}",age);
    // test_list::test();
    test_traits::test1();
}   

// fn check_color(p: Color, u: i32) {
//     let Color(x, y, z) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
//  }