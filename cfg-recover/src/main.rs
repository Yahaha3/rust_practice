mod rust_common;

fn main() {
    let result = rust_common::read_json("D:/python/home.json".to_string());
    println!("1, {:?}", result);
    let result = rust_common::copy_file("D:/python/home.json".to_string(), "D:/测试文件/home1.json".to_string());
    println!("2, {:?}", result);
}
