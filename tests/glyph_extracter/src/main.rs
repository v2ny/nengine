use image::{RgbaImage, Rgba};
use rusttype::{Font, Scale, point};

fn main() {
    // Load a font (adjust the path as necessary)
    let font_data = include_bytes!("../../../resources/fonts/default.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Define the scale (size of the text) and the glyph to extract
    let scale = Scale::uniform(50.0);
    let glyph = font.glyph('A').scaled(scale).positioned(point(0.0, 0.0));

    // Get the bounding box of the glyph
    let bb = glyph.pixel_bounding_box().unwrap();

    // Create an empty image buffer to hold the glyph (RGBA format)
    let width = bb.width() as u32;
    let height = bb.height() as u32;
    let mut image = RgbaImage::new(width, height);

    // Rasterize the glyph into the image buffer
    glyph.draw(|x, y, v| {
        // x and y are offsets into the image
        let px = x as u32;
        let py = y as u32;

        // The 'v' is the coverage value (0.0 to 1.0), we map it to 255 for alpha
        let alpha = (v * 255.0) as u8;

        // Write the pixel into the image with white color and 'alpha' transparency
        image.put_pixel(px, py, Rgba([255, 255, 255, alpha]));
    });

    // Save the image (optional)
    image.save("glyph_A.png").expect("Failed to save image");
}