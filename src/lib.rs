pub mod image_tools;
pub mod luma_lsb;

pub fn luma_lsb_hide(img_path:&str,bin_path:&str)->Result<(), &'static str>{
    luma_lsb::hide(img_path, bin_path)
}

pub fn luma_lsb_read(output_path:&str)->Result<Vec<u8>,String>{
    luma_lsb::read(output_path)
}