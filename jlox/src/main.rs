use std::fs;

fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();

    let args_count: usize = cmd_args.len();
    if  args_count > 2 {
        println!("Usage of Lox: {} [script]", cmd_args[0]);
        std::process::exit(0);
    } else if args_count == 1 {
        let lox_raw_program = std::fs::read(&cmd_args[1]);
    }


}
