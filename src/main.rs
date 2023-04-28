use image::{GenericImageView, ImageFormat};
use std::fs::{read, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};

const DEFAULT_WIDTH: u32 = 760;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let image_path = Path::new(&args[1]);

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

    let new_width = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);

    let new_height = new_width * height / width;

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
