use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::num::Wrapping;
use std::path::Path;
use std::process;

#[tokio::main]
async fn download_file(target: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(&target).await?;
    let path = Path::new("./tempfile_hyperlink-one-time-pad");
    let mut file = File::create(path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "Example:\nhyperlink-one-time-pad \"secret_stuff.zip\" add http://example.com/vid.mp4 http://example.com/data.rar\n\nDescription:\nThe above example uses two files from the internet (both should have a bigger file size than the file to encrypt) that are both downloaded automatically and \"layed over\" the file to encrypt. Decrypting works the same way using the parameter sub instead of add. You only need to memorize the files that are online available and don't need to store or exchange huge keys, which is a negative point with the normal one-time-pad. It is also possible to use local files as keys instead of hyperlinks.\n\nNotes:\n- Remember that the internet providers may safe the files you download. Use this on top of normal encryption methods.\n- The order of the key parameters does not matter.\n- Hyperlinks must start with http:// or https://."
        );
        process::exit(0);
    }

    if !Path::new(&args[1]).exists() {
        eprintln!("Error: File {} does not exist.", args[1]);
        process::exit(1);
    }

    let encrypt = match args[2].as_str() {
        "add" => true,
        "sub" => false,
        _ => {
            eprintln!("Error: Use 'add' to encrypt or 'sub' to decrypt.");
            process::exit(1);
        }
    };

    let mut file = File::open(&args[1])?;
    let filesize = file.metadata()?.len();
    let mut data = vec![0u8; filesize as usize];
    file.read_exact(&mut data)?;

    for (i, arg) in args.iter().skip(3).enumerate() {
        let is_online = arg.starts_with("http://") || arg.starts_with("https://");
        print!(
            "({}) {} key ",
            i + 1,
            if is_online { "Online" } else { "Local" }
        );

        let mut key_data = Vec::new();
        if is_online {
            download_file(arg.clone())?;
            File::open("tempfile_hyperlink-one-time-pad")?.read_to_end(&mut key_data)?;
        } else {
            File::open(arg)?.read_to_end(&mut key_data)?;
        }

        let times = key_data.len() as f32 / data.len() as f32;
        println!(
            "with {} bytes -> {:.2} times layed over.",
            key_data.len(),
            times
        );

        for (i, &key_byte) in key_data.iter().enumerate() {
            let data_idx = i % data.len();
            data[data_idx] = if encrypt {
                (Wrapping(data[data_idx]) + Wrapping(key_byte)).0
            } else {
                (Wrapping(data[data_idx]) - Wrapping(key_byte)).0
            };
        }
    }

    File::create(&args[1])?.write_all(&data)?;
    Ok(())
}
