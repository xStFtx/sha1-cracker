use std::{
    env,
    error::Error,
};

cosnt SHA1_HEDX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1: <wordlist.txt> <sha1_hash>");
        return;
    }
    
    
}