use std::collections::HashMap;
use std::env;
use regex::Regex;
use std::process;
use std::process::exit;
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

fn help() {
    println!("usage:
--help
    Print this help message.
--env
    Show environment variables and their values.
--debug
    Show debug message (echo the final command arguments generated by this wrapper).
-V
    Show version message.

Available environment variables:
    ${{WRAPPED_CMD}}: the wrapped main command.
    ${{WRAPPED_REMOVE_DUP_ARGS}} : the duplicated arguments to be removed (only keep the first one).
    ${{WRAPPED_REPLACE_ARGS}}: the arguments to be replace.
    ${{WRAPPED_REMOVE_ARGS}}: the arguments to be removed.
    ${{WRAPPED_PREPEND_ARGS}} : the arguments to be appended in frontend of the arguments list.
    ${{WRAPPED_PREPEND_IF}} : regex for arguments prepending. Only the regex matched, will prepending be performed.
");
}

fn version(app: String) {
    println!("{}", app);
    println!("version: {}", GIT_VERSION);
}

// cat two Vec<T>
fn cat<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut v = Vec::with_capacity(a.len() + b.len());
    v.extend_from_slice(a);
    v.extend_from_slice(b);
    v
}

fn vec_of_str(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|&x| x.into()).collect()
}

fn load_env(name: &str, fallback: &str) -> String {
    let env_val = match env::var(name) {
        Ok(v) => { v }
        Err(_e) => {
            String::from(fallback)
        }
    };
    return env_val;
}

fn remove_dup_args(args: &Vec<String>) -> Vec<String> {
    let remove_dup_args_env = load_env("WRAPPED_REMOVE_DUP_ARGS", "");
    let remove_dup_args: Vec<String> = vec_of_str(remove_dup_args_env.split(':').collect());

    let mut rm_map = HashMap::new();
    for rm_arg in &remove_dup_args {
        rm_map.insert(rm_arg, false);
    }

    let mut new_arg_list: Vec<String> = Vec::new();
    for arg in args {
        match rm_map.get(&arg) {
            None => {
                new_arg_list.push(arg.to_string());
            }
            Some(dup) => {
                if !dup { // first time appear
                    new_arg_list.push(arg.to_string());
                    rm_map.insert(&arg, true);
                }
            }
        }
    }
    return new_arg_list;
}

fn check_prepend_if(origin_args: &Vec<String>) -> Result<bool, regex::Error> {
    // check prepend condition
    let prepend_if_env = load_env("WRAPPED_PREPEND_IF", "");
    if prepend_if_env == "" {
        return Ok(true);
    }

    let formatted = format!(r"{}", prepend_if_env);
    match Regex::new(formatted.as_str()) {
        Ok(r) => {
            for arg in origin_args {
                if r.is_match(arg) {
                    return Ok(true);
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    };
    return Ok(false);
}

fn parse_prepend_args_env(origin_args: &Vec<String>) -> Vec<String> {
    let prepend_args_env = load_env("WRAPPED_PREPEND_ARGS", "");
    if prepend_args_env != "" {
        match check_prepend_if(origin_args) {
            Ok(ok) => {
                if ok {
                    let prepend_args: Vec<&str> = prepend_args_env.split(':').collect();
                    return vec_of_str(prepend_args);
                }
            }
            Err(e) => {
                println!("match error of env `WRAPPED_PREPEND_IF`: {:?}", e);
                println!("now we will skip arguments prepending.");
                return Vec::new();
            }
        }
    }
    return Vec::new();
}

fn pass_by(debug: bool, args: Vec<String>) -> i32 {
    let removed_args_in_vec: Vec<String> = remove_dup_args(&args);
    let prepend_args_in_vec: Vec<String> = parse_prepend_args_env(&args);
    let new_args: Vec<String> = if prepend_args_in_vec.len() != 0 {
        // if prepend is set, use it.
        cat(&*prepend_args_in_vec, &*removed_args_in_vec)
    } else {
        removed_args_in_vec
    };

    if debug {
        println!("full arguments: {:?}", new_args);
    }

    // read main program from env.
    let wrapper_cmd: String = match env::var("WRAPPED_CMD") {
        Ok(lang) => lang,
        Err(e) => {
            println!("Couldn't use environment variable `WRAPPED_CMD` ({})", e);
            return 1;
        }
    };

    let mut child = process::Command::new(wrapper_cmd)
        .args(new_args)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()
        .unwrap();
    let status = child.wait().unwrap();

    return match status.code() {
        Some(code) => { code } // Exited with status code: {}"
        None => { -1 } // Process terminated by signal"
    };
}

fn env() {
    println!("Env:");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //   debug_to_file(format!("{:?}", args));

    match args.len() {
        // no arguments passed
        1 => {
            help();
        }
        // other cases
        _ => {
            let cmd = &args[1];
            // parse the command
            match &cmd[..] {
                "--help" => help(),
                "--env" => env(),
                "--debug" => {
                    exit(pass_by(true, args[2..].to_vec()));
                }
                "-V" => version(args[0].clone()),
                _ => {
                    exit(pass_by(false, args[1..].to_vec()));
                }
            }
        }
    }
}
