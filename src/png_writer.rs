// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub const IMAGE_WIDTH : u32 = 400;
pub const IMAGE_HEIGHT : u32 = 400;
pub const IMAGE_DATA_SIZE : u32 = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;

// Writes out the given pixel data to the given path.
//
// Pixel data is expected to be a single flattened array in RGB format such that the
// R value for the pixel in row i and column j would be at index i * j, the G value
// would be at index i * j + 1, and the B value would be at index i * j + 2.
pub fn write_png(path_name: &str, data: &[u8]) {
    let path = Path::new(path_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save
}
