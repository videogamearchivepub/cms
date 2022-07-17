use crate::color_encoding_systems::ColorEncodingSystem;
use crate::images::Image;
use crate::languages::Language;

pub struct Cover {
    pub color_encoding_system: ColorEncodingSystem,
    pub language: Language,
    pub barcode: String,
    pub other_identifiers: Vec<String>,
    pub images: Vec<CoverImage>,
}

pub enum CoverViewpoint {
    Flat,
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}

pub struct CoverImage {
    pub image: Image,
    pub viewpoint: CoverViewpoint,
    pub description: String,
}
