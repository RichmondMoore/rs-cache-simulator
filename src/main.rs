use std::env;
use std::fs::File;
use std::io::prelude::*;

// Trace format: 0x######## {w, r}
// Cache Parameters:
//  File path
//  Block Size
//  L1 Size
//  L1 Assoc
//  L2 Size
//  L2 Assoc

struct ParamsList {
    file_path: String,
    trace_file_path: String,
    blk_size: u32,
    l1_size: u32,
    l1_assoc: u32,
    l2_size: u32,
    l2_assoc: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    if args.len() == 2 {
        let mut params = ParamsList {
            file_path: String::from(&args[2]),
            trace_file_path: String::from(""),
            blk_size: 0,
            l1_size: 0,
            l1_assoc: 0,
            l2_size: 0,
            l2_assoc: 0
        };

        // Open file
        // Fill in parameters
    } else {
        // Fill in paramters from args
    }  

    // Init Cache
    // Open trace file
}
