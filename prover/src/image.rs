use std::path::PathBuf;

use anyhow::Result;
use bytes::Bytes;
use risc0_binfmt::{MemoryImage, ProgramBinary};
use risc0_zkvm::PAGE_SIZE;
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
    fn load_bin(blob: &[u8]) -> Result<ProgramBinary> {
        let program = ProgramBinary::decode(blob)?;
        Ok(program)
    }

    pub fn bytes(&self) -> Option<&Bytes> {
        self.bytes.as_ref()
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Image> {
        let program = Image::load_bin(&bytes)?;
        let img_id = program.compute_image_id()?.to_string();
        let img = program.to_image()?;
        Ok(Image {
            id: img_id,
            bytes: Some(bytes),
            size: img.get_page_indexes().len() as u64 * PAGE_SIZE as u64,
            path: PathBuf::new(),
            last_used: 0,
            image: img,
        })
    }

    pub async fn new(path: PathBuf) -> Result<Image> {
        let data = read(&path).await?;
        let program = Image::load_bin(&data)?;
        let img_id = program.compute_image_id()?.to_string();
        let img = program.to_image()?;

        Ok(Image {
            id: img_id,
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
        let program = Image::load_bin(&data)?;
        self.image = program.to_image()?;
        self.bytes = Some(Bytes::from(data));
        Ok(())
    }

    pub fn get_memory_image(&self) -> Result<MemoryImage> {
        Ok(self.image.clone())
    }
}
