use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let byte_array: Vec<u8> = fs::read(config.file_path)?;
    let byte_array_size: u32 = byte_array.len() as u32;

    let block_numbers = 0..(config.bytes_length/16);

    for i in block_numbers {
        let block_start_ind = i * 16;

        if block_start_ind >= byte_array_size {
            break;
        }

        let mut formed_line = format!("{block_start_ind:08x}");

        for j in 0..16 {
            let curr_ind = block_start_ind+j;

            if curr_ind >= byte_array_size {
                break;
            }

            let ind_byte = byte_array[curr_ind as usize];
            let byte_hex = format!("{ind_byte:02x}");

            if j % 2 == 0 {
                formed_line.push_str(" ");
                formed_line.push_str(&byte_hex);
            } else {
                formed_line.push_str(&byte_hex);
            }
        }

        println!("{}", formed_line);
    }

    Ok(())
}

struct Config {
    bytes_length: u32,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let arg_name = args[1].clone();
    let bytes_length: u32 = args[2].clone().parse().unwrap();
    let file_path = args[3].clone();

    assert_eq!(arg_name, "-n");
    assert_eq!(bytes_length % 16, 0);

    Config { bytes_length, file_path }
}