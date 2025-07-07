use std::io::{self, Write};

const VERSION:&str=env!("CARGO_PKG_VERSION");

fn main() {
    println!(
        "+-----------------+\n|Jaba 张浩扬与你同在\n|jvav 和 jaba作者:张浩扬\n|jvav改造者:Reiz\n|当前版本:{}\n|jaba改造者:xiaole6324\n|输入help来获取帮助\n+-----------------+
",VERSION
    );
    
    loop{
        print!(">>> ");
        io::stdout().flush().expect("F");
        let mut command=String::new();
        std::io::stdin().read_line(&mut command).expect("F");
        let command=command.trim();
        
    }

}
