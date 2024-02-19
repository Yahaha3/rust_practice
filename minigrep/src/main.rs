use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::build1(env::args()).unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let _ = minigrep::run(config).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        process::exit(1);
    });

    let a = 3.1 as i8;
   let b = 127_i8 as i32;
   let c = 'a' as u8; // 将字符'a'转换为整数，97

   println!("{},{},{}",a,b,c)
}
