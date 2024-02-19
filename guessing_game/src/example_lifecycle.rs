// 生命周期存在的主要作用是用于向rust描述各变量之间的存续关系
// 在返回参数的过程中，不能明确返回的是x还是y，故而需要明确一个生命周期，用于确认函数返回
// 在下述函数中，'a表示 x/y/返回值 三者必须活相同的时间（生命周期）
// ’a 所表示的生命周期就是指（x/y）生命周期的交集
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod test {
    use crate::example_lifecycle::longest;

    #[test]
    fn lifecycle() {
        let r;
        // let b = r;
        let x = 5;
        r = &x;
        println!("r: {}", r);

        let s1 = String::from("abcd");
        let s2 = "xyz";
        let result = longest(s1.as_str(), s2);
    }
}