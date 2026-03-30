use std::env;

enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn validate_unit(unit: &str, size: f64) -> Option<FileSize> {
        match unit {
            "b" => Some(FileSize::Bytes(size)),
            "kb" => Some(FileSize::Kilobytes(size)),
            "mb" => Some(FileSize::Megabytes(size)),
            "gb" => Some(FileSize::Gigabytes(size)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct FileDisplay {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl FileDisplay {
    fn new(bytes: String,
           kilobytes: String,
           megabytes: String,
           gigabytes: String,) -> Self {
        Self {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        }
    }

    fn format_size(input: FileSize) -> FileDisplay {
        match input {
            FileSize::Bytes(size) => { 
                    let bytes = format!("{:?} b", size);
                    let kilobytes = format!("{:.8?} KB", size / 1024.0);
                    let megabytes = format!("{:.8?} MB", size / 1024_f64.powf(2.0));
                    let gigabytes = format!("{:.8?} GB", size / 1024_f64.powf(3.0));
                    let filedisplay = FileDisplay::new(bytes, kilobytes, megabytes, gigabytes);

                    filedisplay
                    },
            FileSize::Kilobytes(size)=> { 
                    let bytes = format!("{:?} b", size * 1024.0);
                    let kilobytes = format!("{:?} KB", size);
                    let megabytes = format!("{:.4?} MB", size / 1024.0);
                    let gigabytes = format!("{:.8?} GB", size / 1024.0_f64.powf(2.0));
                    let filedisplay = FileDisplay::new(bytes, kilobytes, megabytes, gigabytes);

                    filedisplay
                    }
            FileSize::Megabytes(size) => { 
                    let bytes = format!("{:?} b", size * 1024.0_f64.powf(2.0));
                    let kilobytes = format!("{:?} KB", size * 1024.0);
                    let megabytes = format!("{:?} MB", size);
                    let gigabytes = format!("{:.4?} GB", size / 1024.0);
                    let filedisplay = FileDisplay::new(bytes, kilobytes, megabytes, gigabytes);

                    filedisplay
                    }
            FileSize::Gigabytes(size) => { 
                    let bytes = format!("{:?} b", size * 1024.0_f64.powf(3.0));
                    let kilobytes = format!("{:?} KB", size * 1024.0_f64.powf(2.0));
                    let megabytes = format !("{:?} MB", size * 1024.0);
                    let gigabytes = format!("{:?} GB", size);
                    let filedisplay = FileDisplay::new(bytes, kilobytes, megabytes, gigabytes);

                    filedisplay
                    },
        }
        
    }

}

fn process_args(fs: &String) -> FileSize {
    let fs_vec: Vec<&str> = fs.split_whitespace().collect();
    let filesize = match fs_vec[0].parse::<f64>() {
        Ok(size) => size,
        Err(e) => panic!("ERROR: Wrong number format: {}", e),
    };

    let input = match FileSize::validate_unit(fs_vec[1].to_lowercase().as_str(), filesize) {
        Some(unit) => unit,
        None => panic!("Invalid unit: {}", fs_vec[1])
    };
    
    input
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: {} <file-size>", &args[0]);
        println!("Example: {} \"24 mb\"", &args[0]);
        std::process::exit(1);
    }
    
    let input = process_args(&args[1]);
    
    let result = FileDisplay::format_size(input);
    println!("{:#?}", result); 

}
