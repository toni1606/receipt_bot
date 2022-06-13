use rqrr::{DeQRError, PreparedImage};

pub fn read_url_from_qr(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let img = image::open(path)?.to_luma8();

    log::debug!("Opened image and converted to luma8");

    let mut img = PreparedImage::prepare(img);

    log::debug!("Prepared image for conversion");

    let grids = img.detect_grids();

    log::debug!("Found grids: {}", grids.len());

    match grids.len() {
        1 => {
            let (_, content) = grids[0].decode()?;
            Ok(content)
        }
        _ => Err(Box::new(DeQRError::DataUnderflow)),
    }
}
