//     Copyright 2019 Haoran Wang
//
//     Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.
extern crate linenoise;

mod core;

const PROJ: &'static str = env!("CARGO_PKG_NAME");
const NAME: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VER: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &'static str = "Apache License 2.0";

fn print_info() {
    println!("{}(v {}): {}", PROJ, VER, NAME);
    println!("Released under {}", LICENSE);
    println!("Copyright 2019 {}", AUTHORS);
}

fn is_prefix(s: &str, of: &str) -> bool {
    if s.len() > of.len() {
        return false;
    }

    return of.starts_with(s);
}

fn callback(input: &str) -> Vec<String> {
    let mut ret: Vec<&str> = Vec::new();
    let cmd_all = vec![
        "exit",
        "read",
        "load",
        "execute",
        "run",
        "step",
        "continue",
        "print",
        "print_all_regs",
        "print_sym",
        "reinitialize",
        "breakpoint",
        "list",
        "?",
        "help",
    ];

    // iterate through cmd_all, and try to match the first character to find proper
    // command tips.
    for str_ in cmd_all.iter() {
        if is_prefix(input, str_) {
            ret.push(&str_);
        }
    }

    if ret.is_empty() {
        ret = cmd_all;
    }

    return ret.iter().map(|s| s.to_string()).collect();
}

fn print_cmd_help_info() {
    println!("exit  -- Exit the simulator");
    println!("quit  -- Exit the simulator");
    println!("[unimplemented] read \"FILE\" -- Read FILE containing assembly code into memory");
    println!("[unimplemented] load \"FILE\" -- Same as read");
    println!("[unimplemented] run <ADDR> -- Start the program at (optional) ADDRESS");
    println!("[unimplemented] step <N> -- Step the program for N instructions (default 1)");
    println!("[unimplemented] continue -- Continue program execution without stepping");
    println!("[unimplemented] print $N -- Print register N");
    println!("[unimplemented] print $fN -- Print floating point register N");
    println!("[unimplemented] print ADDR -- Print contents of memory at ADDRESS");
    println!("[unimplemented] print_symbols -- Print all global symbols");
    println!("[unimplemented] print_all_regs -- Print all MIPS registers");
    println!("[unimplemented] print_all_regs hex -- Print all MIPS registers in hex");
    println!("[unimplemented] reinitialize -- Clear the memory and registers");
    println!("[unimplemented] breakpoint <ADDR> -- Set a breakpoint at address ADDR");
    println!("[unimplemented] delete <ADDR> -- Delete breakpoint at address ADDR");
    println!("[unimplemented] list -- List all breakpoints");
    println!("[unimplemented] dump [ \"FILE\" ] -- Dump binary code to spim.dump or FILE in network byte order");
    println!("[unimplemented] dumpnative [ \"FILE\" ] -- Dump binary code to spim.dump or FILE in host byte order");
    println!("[unimplemented] . -- Rest of line is assembly instruction to execute");
    println!("[unimplemented] <cr> -- Newline reexecutes previous command");
    println!("?, help -- Print this message");
    println!("\nMost commands can be abbreviated to their unique prefix");
    println!("e.g., ex(it), re(ad), l(oad), ru(n), s(tep), p(rint)\n");
}

fn handle_cmd(line: &String) -> Result<bool, String> {
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.is_empty() {
        return Ok(true);
    }

    let cmd = args[0];
    if is_prefix(cmd, "quit") || is_prefix(cmd, "exit") {
        return Ok(false);
    } else if is_prefix(cmd, "?") || is_prefix(cmd, "help") {
        print_cmd_help_info();
        return Ok(true);
    } else {
        return Err(format!(
            "Unknown command `{}` or command not implemented",
            cmd
        ));
    }
}

use structopt::StructOpt;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The path to the assembly file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

use exitfailure::ExitFailure;
use failure::ResultExt;
fn main() -> Result<(), ExitFailure> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let args = Cli::from_args();
    match args.path {
        Some(file_path) => {
            // TODO: now just print the assembly file.
            print_info();
            println!("MIPS assembly file: {:?}\n", file_path);
            let _continuable: bool = false;
            // TODO: should be able to pass the arguments for the assembly program itself
            //       to the program itself, now just set it to empty.
            let program_args: Vec<String> = Vec::new();
            core::utils::initialize_run_stack(&program_args);
            let tmp_path = file_path.clone();
            let file = File::open(file_path)
                .with_context(|_| format!("could not read file `{:#?}`", tmp_path))?;

            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .with_context(|_| format!("could not read file `{:#?}`", tmp_path))?;

            println!("file content: {}", contents);
        }
        None => {
            print_info();
            println!();

            // run the loop to accept command that simulator can accept
            linenoise::set_callback(callback);
            loop {
                let line = linenoise::input("(mips_sim)> ");
                match line {
                    None => break,
                    Some(input) => {
                        match handle_cmd(&input) {
                            Ok(t) => {
                                if t == false {
                                    break;
                                }
                            }
                            Err(msg) => {
                                println!("{}", msg);
                                continue;
                            }
                        }
                        linenoise::history_add(&input);
                    }
                }
            }
        }
    }

    return Ok(());
}
