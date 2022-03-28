extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use std::env;
use std::process;
use std::thread;

fn num_zeros(result: &String, leading: &bool) -> i32 {
    let mut counter = 0;
    for i in (0..8).step_by(2) {
        if &result[i..(i+2)] == "00" {
            counter += 1;
        }
        else {
            if *leading {
                break;
            }
        }
    }
    return counter;
}

fn search(start: usize, step: &usize, function_name: &str, leading: &bool) {
    let mut best = 0;
    for i in (start..1000000000).step_by(*step) {
        let hex_num = format!("{:X}", i);
        let candidate = function_name.replace("?", &hex_num);
        let mut hasher = Sha3::keccak256();

        hasher.input_str(&candidate);
        let hash_result = hasher.result_str();
        let num_zero_bytes = num_zeros(&hash_result, &leading);
        if num_zero_bytes == 4 {
            continue;
        }
        if num_zero_bytes > best {
            best = num_zero_bytes;
            println!("{} {} {}", candidate, &hash_result[..8], &num_zero_bytes);
        }

        if num_zero_bytes == 3 {
            return;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Wrong number of arguments");
        process::exit(1);
    }
    let funn =  &args[1];
    let nt = match args[2].parse::<usize>() {
        Ok(i) => i,
        Err(_e) => 1,
    };
    let num_threads = nt.to_owned();
    let leading_arg = &args[3];

    if !(leading_arg == "true" || leading_arg == "false") {
        println!("must specify if zero bytes are required to be leading bytes (true|false)");
        process::exit(1);
    }
    let leading = if leading_arg == "true" { true } else { false };

    let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 0..num_threads {
        let function_name = funn.to_owned();
        let handle = thread::spawn(move || {
            search(i, &num_threads, &function_name, &leading);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap()
    }
}
