use crate::webp::{CHUNK_ALPH, CHUNK_ANIM, CHUNK_ANMF, CHUNK_XMP};

use super::{WebP, CHUNK_EXIF, CHUNK_ICCP};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct WebPFlags(pub(super) [u8; 4]);

impl WebPFlags {
    pub(super) fn from_webp(webp: &WebP) -> WebPFlags {
        let mut flags = WebPFlags::default();
        if webp.has_chunk(CHUNK_ICCP) {
            flags.0[0] |= 0b0010_0000;
        }
        if webp.has_chunk(CHUNK_ALPH) {
            flags.0[0] |= 0b0001_0000;
        }
        if webp.has_chunk(CHUNK_EXIF) {
            flags.0[0] |= 0b0000_1000;
        }
        if webp.has_chunk(CHUNK_XMP) {
            flags.0[0] |= 0b0000_0100;
        }
        if webp.has_chunk(CHUNK_ANIM) || webp.has_chunk(CHUNK_ANMF) {
            flags.0[0] |= 0b0000_0010;
        }
        println!("VP8X: {flags:?}");
        flags
    }
}
