use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

// Function to count bytes in a streaming fashion with error handling
fn count_bytes<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut buffer = [0; 8192]; // 8 KB buffer
    let mut byte_count = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        byte_count += bytes_read;
    }

    Ok(byte_count)
}

// Function to count lines in a streaming fashion with error handling
fn count_lines<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut line_count = 0;

    for line in reader.lines() {
        line_count += 1;
        line?; // Propagate any errors encountered while reading lines
    }

    Ok(line_count)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -c <filename> | -l <filename>", args[0]);
        return;
    }

    let option = &args[1];
    let filename = &args[2];

    // Open the file and handle any errors
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return;
        }
    };

    // Dispatch based on the option
    let result = match option.as_str() {
        "-c" => count_bytes(file), // Use the regular file for byte counting
        "-l" => {
            let reader = BufReader::new(file); // Use BufReader for line counting
            count_lines(reader)
        }
        _ => {
            eprintln!("Invalid option: {}. Use -c or -l.", option);
            return;
        }
    };

    // Handle the result from the counting function
    match result {
        Ok(count) => println!("{:>8} {}", count, filename),
        Err(e) => eprintln!("Error reading file {}: {}", filename, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_byte_count() {
        let test_data = b"Hello, world";
        let byte_count = count_bytes(&test_data[..]).unwrap(); // Unwrap here since we're sure it won't fail
        assert_eq!(byte_count, test_data.len());
    }

    #[test]
    fn test_line_count() {
        let test_data = b"Hello, world\nThis is a test\nAnother line";
        let cursor = Cursor::new(&test_data[..]);
        let line_count = count_lines(cursor).unwrap(); // Unwrap here since we're sure it won't fail
        assert_eq!(line_count, 3);
    }
}
