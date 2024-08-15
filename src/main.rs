use image::{open, DynamicImage, GrayImage};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    let input_dir = "./images";
    let output_dir = "./output";

    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Get all image files in the input directory
    let image_paths: Vec<_> = fs::read_dir(input_dir)
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    // Process images concurrently
    image_paths.par_iter().for_each(|entry| {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        // Open the image file
        let img = open(&path).expect("Failed to open image");

        // Convert to grayscale
        let gray_img = to_grayscale(&img);

        // Save the grayscale image to the output directory
        let output_path = Path::new(output_dir).join(filename);
        gray_img.save(&output_path).expect("Failed to save image");

        println!("Processed: {}", filename);
    });

    println!("Image processing complete!");
}

fn to_grayscale(img: &DynamicImage) -> GrayImage {
    img.to_luma8()
}

