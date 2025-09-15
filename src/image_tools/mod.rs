pub fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let y = ((66 * r as i32 + 129 * g as i32 + 25 * b as i32 + 128) >> 8) + 16;
    let cb = ((-38 * r as i32 - 74 * g as i32 + 112 * b as i32 + 128) >> 8) + 128;
    let cr = ((112 * r as i32 - 94 * g as i32 - 18 * b as i32 + 128) >> 8) + 128;
    (
        y.clamp(0, 255) as u8,
        cb.clamp(0, 255) as u8,
        cr.clamp(0, 255) as u8,
    )
}

pub fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let y = y as i32 - 16;
    let cb = cb as i32 - 128;
    let cr = cr as i32 - 128;
    let r = (298 * y + 409 * cr + 128) >> 8;
    let g = (298 * y - 100 * cb - 208 * cr + 128) >> 8;
    let b = (298 * y + 516 * cb + 128) >> 8;
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}