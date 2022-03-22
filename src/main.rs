extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use std::env;
use std::process;

fn main() {
    let mut best = 0;
    for i in 1..100000000 {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            println!("Wrong number of arguments");
            process::exit(1);
        }
        let function_name = &args[1]; 
        let hex_num = format!("{:X}", i);
        let candidate = function_name.replace("?", &hex_num);
        let mut hasher = Sha3::keccak256();

        hasher.input_str(&candidate);
        let hash_result = hasher.result_str();
        let num_zeros = hash_result[..8].matches("0").count();
        if num_zeros > best {
            best = num_zeros;
            println!("{} {} {}", candidate, &hash_result[..8], &num_zeros);
        }

        // 8 zeros will collide with the fallback function
        if num_zeros > 6 {
            break;
        }
    }
}
