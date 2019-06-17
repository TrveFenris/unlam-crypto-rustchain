use serde::{Deserialize, Serialize};
extern crate image;

use image::{ImageBuffer, Rgba};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: i32,
}

pub struct ImageTransaction {
    pub sender: String,
    pub recipient: String,
    pub img: Option<image::ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl ImageTransaction {
    pub fn new(s: String, r: String, image_path: String) -> ImageTransaction {
        ImageTransaction {
            sender: s,
            recipient: r,
            img: Some(image::open(image_path).unwrap().to_rgba()),
        }
    }

    fn save_image_to_file(&self, path: String) {
        match &self.img {
            None => None,
            Some(i) => Some(i.save(path).ok().expect("Saving image failed")),
        };
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_transaction_with_image() {
        // TODO Why should it be mutable?
        let mut it = ImageTransaction::new(
            "User1".to_string(),
            "Recipient1".to_string(),
            "src/blockdata/bitcoin.png".to_string(),
        );
        it.save_image_to_file("src/blockdata/out.png".to_string());
    }

    #[test]
    fn test_load_flip_and_save_img() {
        let img = image::open("src/blockdata/bitcoin.png")
            .ok()
            .expect("Opening image failed");
        let filtered = img.fliph();
        let _ = filtered
            .save("src/blockdata/out.png")
            .ok()
            .expect("Saving image failed");
    }
}
