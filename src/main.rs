use clap::{Arg, Command};
use std::process::Command as os_cmd;

fn main() {
    let matches = Command::new("multi_exec")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen126@126.com>")
        .about("Execute command with multi threads")
        .args(&[Arg::new("exec_cmd")
            .value_name("exec_cmd")
            .required(true)
            .index(1)
            .help("exeucte command")])
        .get_matches();

    let cmd = matches.get_one::<String>("exec_cmd").unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    pool.scope(|s| {
        for _ in 0..num_cpus::get() {
            s.spawn(move |_| {
                let command = cmd.to_string();
                execute_shell_command(&command)
            });
        }
    });
    // pool.scope(|_| {
    //     execute_shell_command(cmd);
    // });
}

fn execute_shell_command(cmd_str: &str) {
    let array = cmd_str.split(' ').collect::<Vec<&str>>();

    let mut exec_cmd = os_cmd::new(array[0]);

    for idx in 1..array.len() {
        if let Some(arg) = array.get(idx) {
            exec_cmd.arg(arg);
        };
    }
    println!("array:{:?}", exec_cmd.output());
    // let cmd = Command::from("ls -lart");

    // println!("{:?}", cmd.args)
}
