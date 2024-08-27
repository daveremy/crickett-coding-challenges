use std::fs::File;
use std::io::{self, Read};

fn count_bytes<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut buffer = [0; 8192]; // 8KB buffer
    let mut byte_count = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // end of file
        }
        byte_count += bytes_read;
    }

    Ok(byte_count)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 && args[1] == "-c" {
        let filename = &args[2];
        match File::open(filename) {
            Ok(file) => match count_bytes(file) {
                Ok(byte_count) => println!("{:>8} {}", byte_count, filename),
                Err(e) => eprintln!("Error reading file {}: {}", filename, e),
            },
            Err(e) => eprintln!("Error reading file {}: {}", filename, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_count() {
        let test_data = b"Hello, world!";
        let byte_count = count_bytes(&test_data[..]).unwrap();
        assert_eq!(byte_count, test_data.len());
    }
}
