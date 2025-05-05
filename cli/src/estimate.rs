//! Bare bones upper bound estimator that uses the rv32im
//! emulation utils for fast lookups in the opcode list
//! to extract the cycle count from an elf.

use anyhow::Result;
use risc0_binfmt::{MemoryImage, Program, ProgramBinary};
use risc0_zkvm::{ExecutorEnv, ExecutorImpl, Session, GUEST_MAX_MEM};

pub fn estimate<E: MkImage>(elf: E, env: ExecutorEnv) -> Result<()> {
    let session = get_session(elf, env)?;
    println!(
        "User cycles: {}\nTotal cycles: {}\nSegments: {}",
        session.user_cycles,
        session.total_cycles,
        session.segments.len()
    );

    Ok(())
}

/// Get the total number of cycles by stepping through the ELF using emulation
/// tools from the risc0_circuit_rv32im module.
pub fn get_session<E: MkImage>(elf: E, env: ExecutorEnv) -> Result<Session> {
    let x = ExecutorImpl::new(env, elf.mk_image()?)?.run();
    println!("{:?}", x.is_err());
    x
}

/// Helper trait for loading an image from an elf.
pub trait MkImage {
    fn mk_image(self) -> Result<MemoryImage>;
}
impl<'a> MkImage for &'a [u8] {
    fn mk_image(self) -> Result<MemoryImage> {
        let program = ProgramBinary::decode(self)?;
        Ok(program.to_image()?)
    }
}

#[cfg(test)]
mod estimate_tests {
    use anyhow::Result;
    use risc0_binfmt::MemoryImage;
    use risc0_circuit_rv32im::execute::DEFAULT_SEGMENT_LIMIT_PO2;
    use risc0_circuit_rv32im::execute::testutil::{DEFAULT_SESSION_LIMIT, user};
    use risc0_zkvm::{ExecutorEnv};

    use super::MkImage;
    use crate::estimate;

    impl MkImage for MemoryImage {
        fn mk_image(self) -> Result<MemoryImage> {
            Ok(self)
        }
    }

    #[test]
    fn estimate_basic() {
        let program = user::basic();
        let mut env = &mut ExecutorEnv::builder();
        env = env
            .segment_limit_po2(DEFAULT_SEGMENT_LIMIT_PO2 as u32)
            .session_limit(DEFAULT_SESSION_LIMIT);
        let image = MemoryImage::new_kernel(program);
        let res = estimate::get_session(image, env.build().unwrap());

        assert_eq!(res.ok().map(|session| session.total_cycles), Some(8192));
    }
}
