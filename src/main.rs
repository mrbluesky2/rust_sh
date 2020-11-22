use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Child;
use std::process::Stdio;

fn main() {
    loop {
        // use HOME + ':' char as prompt
        let home = env::current_dir().unwrap().into_os_string().into_string().unwrap();
        let dir: Vec<&str> = home.rsplit('/').collect();
        print!("[{}]$ ", dir[0]);
        let _f = stdout().flush();

        // read input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // get all commands by searching for the pipe "|" char
        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command = None;

        // do each command
        while let Some(command) = commands.next() {
            // find the args
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // change directories
                    let dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(&dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    prev_command = None;
                },
                "exit" => return,
                command => {
                    // get the stdin
                    let stdin = prev_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    // get the stdout
                    let stdout = if commands.peek().is_some() {
                        // send out to next command
                        Stdio::piped()
                    } else {
                        // no more behind, send to stdout
                        Stdio::inherit()
                    };

                    // do command
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // check for errors
                    match output {
                        Ok(output) => { prev_command = Some(output); },
                        Err(e) => {
                            prev_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = prev_command {
            // block until command is finished
            let _w = final_command.wait();
        }
    }
}