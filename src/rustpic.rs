extern crate image;

use std::cmp::{max, min};
use std::io::Error;
use std::fs::File;
use std::io::Write;
use std::os::raw::c_float;
use image::{GenericImageView, Pixel};

// rustpic is a library for converting an image to ASCII charasters written on rust

// made by Mookie
// date -> 02.03.2023

/// # Value char - average value of the color shade
/// # Return char -  returned character based on the average value of the color shade
pub fn get_density(value: u8, density: &str) -> char {
    //! a public function that serves to determine the average value of the color and then select a symbol that depends on the shade
    let mut char_value: u8 = (density.len() as c_float / 255.0 * value as c_float) as u8;
    char_value = max(char_value, 0);

    char_value = min(char_value, density.len() as u8 - 1);
    let x = density.chars().nth(char_value as usize);
    return x.unwrap_or('F')
}

/// # Value  chars - a two-dimensional vector containing symbols for constructing an image
/// # Value path - the path where the file with the image of the characters will be saved
pub fn write_to_file(chars: Vec<Vec<char>>, path: &str) -> Result<(), Error>{
    //! public function for writing a finished image from symbols to a file , the image itself is also built in the write_to_file function
    //! if the file specified in path does not exist , a new one will be created along the same path !!!
    let mut output = File::create(path)?;

    for i in &chars{
        write!(output, "{:?}\n", String::from_iter(i).to_string())?;
    }
    Ok(())
}

/// # Value path - the path to the image to be read
/// # Value scale_height - these are the values ( an even number ) needed to crop the image n times
/// # Value scale_width - these are the values (an even number ) needed to crop the image n times
/// # Value density - symbols used to write the image
/// # Return the read_image function returns a two-dimensional vector with characters that will then be printed by another function ( write_to_file )
pub fn read_image(path: &str,  scale_height: u32,  scale_width: u32, density: &str) -> Vec<Vec<char>> {
    //! the public read_image function is the main one in the rustpic library,
    //! it is used to "read the image", that is, it serves to take each pixel of the image and return it
    let img = image::open(path).unwrap();
    let (width, height): (u32, u32) = (img.width() / scale_width, img.height() / scale_height);

    let _chars = [height, width];
    let mut a1: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];

    for a in 0..height {
        for b in 0..width {
            let mut color_rbg: u8 = 0;
            for c in 0..scale_height {
                for d in 0..scale_width {
                    color_rbg = img.get_pixel(b * scale_width + d, a * scale_height + c).channels()[2];
                }
            }
            a1[a as usize][b as usize] = get_density(color_rbg, density);
        }
    }
    return a1;
}
