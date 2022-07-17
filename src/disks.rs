use crate::color_encoding_systems::ColorEncodingSystem;
use crate::images::Image;
use crate::languages::Language;

struct Disk {
    pub color_encoding_system: ColorEncodingSystem,
    pub language: Language,
    pub other_identifiers: Vec<String>,
    pub images: Vec<DiskImage>,
}

pub struct DiskImage {
    pub image: Image,
    pub description: String,
}
