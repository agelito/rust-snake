use std::fs::File;
use std::io::prelude::*;

pub fn extract_resources(resources: &[(&str, &[u8])]) {
    println!("unpacking: {} resources...", resources.len());

    for (res, bytes) in resources {
        println!("--> unpacking: {}", res);
        let mut buffer = File::create(res).unwrap();

        buffer.write_all(bytes).unwrap();
    }

    println!("finished unpacking");
}
