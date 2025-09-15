pub mod image_tools;
pub mod luma_lsb;

pub fn luma_lsb_hide(img_path:&str,bin_path:&str)->Result<(), &'static str>{
    luma_lsb::hide(img_path, bin_path)
}