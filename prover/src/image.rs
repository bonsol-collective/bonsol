use std::path::PathBuf;

use anyhow::Result;
use bytes::Bytes;
use risc0_binfmt::{MemoryImage, Program};
use risc0_zkvm::{GUEST_MAX_MEM, PAGE_SIZE};
use tokio::fs::read;

pub struct Image {
    pub id: String,
    bytes: Option<Bytes>,
    pub size: u64,
    pub path: PathBuf,
    pub last_used: u64,
    pub image: MemoryImage,
}

impl Image {
    fn load_elf(elf: &[u8]) -> Result<Program> {
        let program = Program::load_elf(elf, GUEST_MAX_MEM as u32)?;
        Ok(program)
    }

    pub fn bytes(&self) -> Option<&Bytes> {
        self.bytes.as_ref()
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Image> {
        let program = Image::load_elf(&bytes)?;
        let mut img = MemoryImage::new_kernel(program);
        Ok(Image {
            id: img.image_id().to_string(),
            bytes: Some(bytes),
            size: img.get_page_indexes().len() as u64 * PAGE_SIZE as u64,
            path: PathBuf::new(),
            last_used: 0,
            image: img,
        })
    }

    pub async fn new(path: PathBuf) -> Result<Image> {
        let data = read(&path).await?;
        let program = Image::load_elf(&data)?;
        let mut img = MemoryImage::new_kernel(program);

        Ok(Image {
            id: img.image_id().to_string(),
            bytes: Some(Bytes::from(data)),
            size: img.get_page_indexes().len() as u64 * PAGE_SIZE as u64,
            path,
            last_used: 0,
            image: img,
        })
    }

    pub fn compress(&mut self) {
        self.bytes = None;
    }

    pub async fn load(&mut self) -> Result<()> {
        if self.bytes.is_some() {
            return Ok(());
        }
        let data = read(&self.path).await?;
        let program = Image::load_elf(&data)?;
        self.image = MemoryImage::new_kernel(program);
        self.bytes = Some(Bytes::from(data));
        Ok(())
    }

    pub fn get_memory_image(&self) -> Result<MemoryImage> {
        Ok(self.image.clone())
    }
}
