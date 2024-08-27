use std::fs::File;
use std::io::Read;

fn count_bytes<R: Read>(mut reader: R) -> usize {
    let mut contents = Vec::new();
    reader
        .read_to_end(&mut contents)
        .expect("Unable to read input");
    contents.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 && args[1] == "-c" {
        let filename = &args[2];
        let file = File::open(filename).expect("Unable to open file");
        let byte_count = count_bytes(file);
        println!("{:>8} {}", byte_count, filename);
    } else {
        eprintln!("Usage: {} -c <filename>", args[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_count() {
        let test_data = b"Hello, world!";
        let byte_count = count_bytes(&test_data[..]);
        assert_eq!(byte_count, test_data.len());
    }
}
