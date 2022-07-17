use crate::images::Image;
use crate::languages::Language;

pub struct Manual {
    pub language: Language,
    pub other_identifiers: Vec<String>,
    pub images: Vec<ManualImage>,
}

pub enum ManualViewpoint {
    Single { page_number: usize },
    Double { left_page_number: usize, right_page_number: usize },
}

pub struct ManualImage {
    pub image: Image,
    pub viewpoint: ManualViewpoint,
    pub description: String,
}
