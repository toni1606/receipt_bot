use rqrr::{PreparedImage, DeQRError};

pub fn read_url_from_qr(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let img = image::open(path)?.to_luma8();

    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();

    match grids.len() {
        1 => {
            let (_, content) = grids[0].decode()?;
            Ok(content)
        },
        _ => Err(Box::new(DeQRError::DataUnderflow))
    }
}
