use std::path::PathBuf;

use anyhow::Result;
use bytes::Bytes;
use risc0_binfmt::{MemoryImage, Program};
use risc0_zkvm::GUEST_MAX_MEM;
use tokio::fs::read;

pub struct Image {
    pub id: String,
    bytes: Option<Bytes>,
    pub path: PathBuf,
    pub last_used: u64,
}

impl Image {
    fn load_elf(elf: &[u8]) -> Result<Program> {
        let program = Program::load_elf(elf, GUEST_MAX_MEM as u32)?;
        Ok(program)
    }

    fn mem_img(program: Program) -> Result<MemoryImage> {
        let image = MemoryImage::new_user(program);
        Ok(image)
    }

    pub fn bytes(&self) -> Option<&Bytes> {
        self.bytes.as_ref()
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self> {
        let program = Image::load_elf(&bytes)?;
        let mut img = Image::mem_img(program)?;
        Ok(Self {
            id: img.image_id().to_string(),
            bytes: Some(bytes),
            path: PathBuf::new(),
            last_used: 0,
        })
    }

    pub async fn new(path: PathBuf) -> Result<Image> {
        let data = read(&path).await?;
        let program = Image::load_elf(&data)?;
        let mut img = Image::mem_img(program)?;

        Ok(Image {
            id: img.image_id().to_string(),
            bytes: Some(Bytes::from(data)),
            path,
            last_used: 0,
        })
    }

    pub fn compress(&mut self) {
        self.bytes = None;
    }

    pub async fn load(&mut self) -> Result<()> {
        let data = read(&self.path).await?;
        self.bytes = Some(Bytes::from(data));
        Ok(())
    }

    pub fn get_memory_image(&self) -> Result<MemoryImage> {
        let program = Image::load_elf(&self.bytes.as_ref().unwrap())?;
        let image = Image::mem_img(program)?;
        Ok(image)
    }
}
