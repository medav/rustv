use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: &String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}


pub fn parse_disasm(filename : &String) -> HashMap<u64, String> {
    let mut map = HashMap::<u64, String>::new();

    for maybe_line in read_lines(filename) {
        if let Ok(line) = maybe_line {
            let parts = line.split(" ").collect::<Vec<_>>();

            if parts.len() == 2 {
                if let Ok(addr) = u64::from_str_radix(parts[0], 16) {
                    let maybe_name = parts[1]
                        .strip_suffix(">:")
                        .unwrap_or("")
                        .strip_prefix("<");

                    if let Some(name) = maybe_name {
                        // println!("0x{:08x}: {}", addr, name);
                        map.insert(addr, name.to_string());
                    }
                }
            }
        }
    }


    map
}
