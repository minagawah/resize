use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use image::{GenericImageView, ImageFormat};
use std::fs::{read, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};

const DEFAULT_WIDTH: u32 = 760;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    #[arg(long, value_name = "WIDTH")]
    width: Option<u32>,

    #[arg(long, value_name = "HEIGHT")]
    height: Option<u32>,
}

fn exit(message: &str) {
    let mut cmd = Cli::command();
    cmd.error(ErrorKind::ArgumentConflict, message).exit();
}

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    // let image_path = Path::new(&args[1]);

    let args = Cli::parse();

    if args.path.is_none() {
        exit("You need to specify \"--path\"");
    }

    if args.width.is_some() && args.height.is_some() {
        exit("You can only specify either \"--width\" or \"--height\"");
    }

    let image_path = Path::new(args.path.as_deref().unwrap());

    let img_format = match image_path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        _ => panic!("Unsupported image format"),
    };

    let file_content = read(image_path)?;
    let cursor = Cursor::new(file_content);
    let img = image::load(cursor, img_format)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let (width, height) = img.dimensions();

    // let new_width = args
    //     .get(2)
    //     .and_then(|s| s.parse().ok())
    //     .unwrap_or(DEFAULT_WIDTH);
    //
    // let new_height = new_width * height / width;

    let new_width;
    let new_height;

    if let Some(h) = args.height {
        new_height = h;
        new_width = new_height * width / height;
    } else {
        if let Some(w) = args.width {
            new_width = w;
        } else {
            new_width = DEFAULT_WIDTH;
        }
        new_height = new_width * height / width;
    }

    println!("(old) {} x {}", width, height);
    println!("(new) {} x {}", new_width, new_height);

    let resized_img = img.resize_exact(
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3,
    );

    let mut new_file_path = PathBuf::new();
    new_file_path.push(image_path.parent().unwrap());
    new_file_path.push(format!(
        "{}.new.{}",
        image_path.file_stem().unwrap().to_str().unwrap(),
        image_path.extension().unwrap().to_str().unwrap()
    ));

    let mut new_file = File::create(new_file_path.as_path())?;
    resized_img
        .write_to(&mut new_file, img_format)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}
