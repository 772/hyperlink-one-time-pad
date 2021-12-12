use error_chain::error_chain;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::num::Wrapping;
use std::path::Path;
use std::process;

extern crate reqwest;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn download_file(target: String) -> Result<()> {
    let response = reqwest::get(&target).await?;
    let path = Path::new("./tempfile_hyperlink-one-time-pad");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let param_amount = args.len();
    if param_amount < 4 {
        println!("\nExample:\nhyperlink-one-time-pad \"secret_stuff.zip\" add http://example.com/vid.mp4 http://example.com/data.rar\n\nDescription:\nThe above example uses two files from the internet (both should have a bigger file size than the file to encrypt) that are both downloaded automatically and \"layed over\" the file to encrypt. Decrypting works the same way using the parameter sub instead of add. You only need to memorize the files that are online available and don't need to store or exchange huge keys, which is a negative point with the normal one-time-pad. It is also possible to use local files as keys instead of hyperlinks.\n\nNotes:\n- Remember that the internet providers may safe the files you download. Use this on top of normal encryption methods.\n- The order of the key parameters does not matter.\n- Hyperlinks must start with http:// or https://.");
        process::exit(0);
    }

    if !Path::new(&args[1]).exists() {
        println!("Error: File {} does not exist.", args[1]);
        process::exit(0);
    }

    let mut encrypt = false;
    if args[2] == "add" {
        encrypt = true;
    } else if args[2] != "sub" {
        println!("Error: Either use add for encrypting or sub for decrypting.");
        process::exit(0);
    }

    let mut file = File::open(&args[1]).unwrap();
    let filesize = file.metadata().unwrap().len();
    let length: usize = filesize as usize;
    let mut vec = vec![0u8; length];
    let _count = file.read(vec.as_mut_slice()).unwrap();

    for x in 3..param_amount {
        let mut online = false;
        if args[x].starts_with("http://") ^ args[x].starts_with("https://") {
            print!("({}) Online key ", x - 2);
            let _a = download_file(args[x].clone());
            online = true;
        } else {
            print!("({}) Local key ", x - 2);
        }

        let mut file2;
        if online {
            file2 = File::open(&"tempfile_hyperlink-one-time-pad").unwrap();
        } else {
            file2 = File::open(&args[x]).unwrap();
        }
        let filesize2 = file2.metadata().unwrap().len();
        let length2: usize = filesize2 as usize;
        let mut vec2 = vec![0u8; length2];
        let count2 = file2.read(vec2.as_mut_slice()).unwrap();
        let times: f32 = (length2 as f32) / (length as f32);
        println!("with {} bytes -> {} times layed over.", count2, times);

        let mut j = 0;
        for i in 0..length2 {
            let foo: Wrapping<u8>;
            if encrypt {
                foo = Wrapping(vec[j]) + Wrapping(vec2[i]);
            } else {
                foo = Wrapping(vec[j]) - Wrapping(vec2[i]);
            }
            vec[j] = foo.0;
            j = j + 1;
            if j > length - 1 {
                j = 0;
            }
        }
    }
    let mut file = File::create(&args[1])?;
    file.write_all(&vec)?;
    Ok(())
}
