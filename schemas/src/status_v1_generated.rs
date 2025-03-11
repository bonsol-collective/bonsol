// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_STATUS_TYPES: u8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_STATUS_TYPES: u8 = 4;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_STATUS_TYPES: [StatusTypes; 5] = [
    StatusTypes::Unknown,
    StatusTypes::Queued,
    StatusTypes::Claimed,
    StatusTypes::Completed,
    StatusTypes::Failed,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StatusTypes(pub u8);
#[allow(non_upper_case_globals)]
impl StatusTypes {
    pub const Unknown: Self = Self(0);
    pub const Queued: Self = Self(1);
    pub const Claimed: Self = Self(2);
    pub const Completed: Self = Self(3);
    pub const Failed: Self = Self(4);

    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_MAX: u8 = 4;
    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Unknown,
        Self::Queued,
        Self::Claimed,
        Self::Completed,
        Self::Failed,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Unknown => Some("Unknown"),
            Self::Queued => Some("Queued"),
            Self::Claimed => Some("Claimed"),
            Self::Completed => Some("Completed"),
            Self::Failed => Some("Failed"),
            _ => None,
        }
    }
}
impl core::fmt::Debug for StatusTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for StatusTypes {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
        Self(b)
    }
}

impl flatbuffers::Push for StatusTypes {
    type Output = StatusTypes;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for StatusTypes {
    type Scalar = u8;
    #[inline]
    fn to_little_endian(self) -> u8 {
        self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: u8) -> Self {
        let b = u8::from_le(v);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for StatusTypes {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        u8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for StatusTypes {}
pub enum StatusV1Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StatusV1<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StatusV1<'a> {
    type Inner = StatusV1<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> StatusV1<'a> {
    pub const VT_EXECUTION_ID: flatbuffers::VOffsetT = 4;
    pub const VT_STATUS: flatbuffers::VOffsetT = 6;
    pub const VT_PROOF: flatbuffers::VOffsetT = 8;
    pub const VT_EXECUTION_DIGEST: flatbuffers::VOffsetT = 10;
    pub const VT_INPUT_DIGEST: flatbuffers::VOffsetT = 12;
    pub const VT_COMMITTED_OUTPUTS: flatbuffers::VOffsetT = 14;
    pub const VT_ASSUMPTION_DIGEST: flatbuffers::VOffsetT = 16;
    pub const VT_EXIT_CODE_SYSTEM: flatbuffers::VOffsetT = 18;
    pub const VT_EXIT_CODE_USER: flatbuffers::VOffsetT = 20;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        StatusV1 { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args StatusV1Args<'args>,
    ) -> flatbuffers::WIPOffset<StatusV1<'bldr>> {
        let mut builder = StatusV1Builder::new(_fbb);
        builder.add_exit_code_user(args.exit_code_user);
        builder.add_exit_code_system(args.exit_code_system);
        if let Some(x) = args.assumption_digest {
            builder.add_assumption_digest(x);
        }
        if let Some(x) = args.committed_outputs {
            builder.add_committed_outputs(x);
        }
        if let Some(x) = args.input_digest {
            builder.add_input_digest(x);
        }
        if let Some(x) = args.execution_digest {
            builder.add_execution_digest(x);
        }
        if let Some(x) = args.proof {
            builder.add_proof(x);
        }
        if let Some(x) = args.execution_id {
            builder.add_execution_id(x);
        }
        builder.add_status(args.status);
        builder.finish()
    }

    pub fn unpack(&self) -> StatusV1T {
        let execution_id = self.execution_id().map(|x| x.to_string());
        let status = self.status();
        let proof = self.proof().map(|x| x.into_iter().collect());
        let execution_digest = self.execution_digest().map(|x| x.into_iter().collect());
        let input_digest = self.input_digest().map(|x| x.into_iter().collect());
        let committed_outputs = self.committed_outputs().map(|x| x.into_iter().collect());
        let assumption_digest = self.assumption_digest().map(|x| x.into_iter().collect());
        let exit_code_system = self.exit_code_system();
        let exit_code_user = self.exit_code_user();
        StatusV1T {
            execution_id,
            status,
            proof,
            execution_digest,
            input_digest,
            committed_outputs,
            assumption_digest,
            exit_code_system,
            exit_code_user,
        }
    }

    #[inline]
    pub fn execution_id(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(StatusV1::VT_EXECUTION_ID, None)
        }
    }
    #[inline]
    pub fn status(&self) -> StatusTypes {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<StatusTypes>(StatusV1::VT_STATUS, Some(StatusTypes::Unknown))
                .unwrap()
        }
    }
    #[inline]
    pub fn proof(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    StatusV1::VT_PROOF,
                    None,
                )
        }
    }
    #[inline]
    pub fn execution_digest(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    StatusV1::VT_EXECUTION_DIGEST,
                    None,
                )
        }
    }
    #[inline]
    pub fn input_digest(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    StatusV1::VT_INPUT_DIGEST,
                    None,
                )
        }
    }
    #[inline]
    pub fn committed_outputs(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    StatusV1::VT_COMMITTED_OUTPUTS,
                    None,
                )
        }
    }
    #[inline]
    pub fn assumption_digest(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    StatusV1::VT_ASSUMPTION_DIGEST,
                    None,
                )
        }
    }
    #[inline]
    pub fn exit_code_system(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(StatusV1::VT_EXIT_CODE_SYSTEM, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn exit_code_user(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(StatusV1::VT_EXIT_CODE_USER, Some(0))
                .unwrap()
        }
    }
}

impl flatbuffers::Verifiable for StatusV1<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                "execution_id",
                Self::VT_EXECUTION_ID,
                false,
            )?
            .visit_field::<StatusTypes>("status", Self::VT_STATUS, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "proof",
                Self::VT_PROOF,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "execution_digest",
                Self::VT_EXECUTION_DIGEST,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "input_digest",
                Self::VT_INPUT_DIGEST,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "committed_outputs",
                Self::VT_COMMITTED_OUTPUTS,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "assumption_digest",
                Self::VT_ASSUMPTION_DIGEST,
                false,
            )?
            .visit_field::<u32>("exit_code_system", Self::VT_EXIT_CODE_SYSTEM, false)?
            .visit_field::<u32>("exit_code_user", Self::VT_EXIT_CODE_USER, false)?
            .finish();
        Ok(())
    }
}
pub struct StatusV1Args<'a> {
    pub execution_id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub status: StatusTypes,
    pub proof: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub execution_digest: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub input_digest: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub committed_outputs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub assumption_digest: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub exit_code_system: u32,
    pub exit_code_user: u32,
}
impl<'a> Default for StatusV1Args<'a> {
    #[inline]
    fn default() -> Self {
        StatusV1Args {
            execution_id: None,
            status: StatusTypes::Unknown,
            proof: None,
            execution_digest: None,
            input_digest: None,
            committed_outputs: None,
            assumption_digest: None,
            exit_code_system: 0,
            exit_code_user: 0,
        }
    }
}

pub struct StatusV1Builder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StatusV1Builder<'a, 'b, A> {
    #[inline]
    pub fn add_execution_id(&mut self, execution_id: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(StatusV1::VT_EXECUTION_ID, execution_id);
    }
    #[inline]
    pub fn add_status(&mut self, status: StatusTypes) {
        self.fbb_
            .push_slot::<StatusTypes>(StatusV1::VT_STATUS, status, StatusTypes::Unknown);
    }
    #[inline]
    pub fn add_proof(&mut self, proof: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(StatusV1::VT_PROOF, proof);
    }
    #[inline]
    pub fn add_execution_digest(
        &mut self,
        execution_digest: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StatusV1::VT_EXECUTION_DIGEST,
            execution_digest,
        );
    }
    #[inline]
    pub fn add_input_digest(
        &mut self,
        input_digest: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(StatusV1::VT_INPUT_DIGEST, input_digest);
    }
    #[inline]
    pub fn add_committed_outputs(
        &mut self,
        committed_outputs: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StatusV1::VT_COMMITTED_OUTPUTS,
            committed_outputs,
        );
    }
    #[inline]
    pub fn add_assumption_digest(
        &mut self,
        assumption_digest: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StatusV1::VT_ASSUMPTION_DIGEST,
            assumption_digest,
        );
    }
    #[inline]
    pub fn add_exit_code_system(&mut self, exit_code_system: u32) {
        self.fbb_
            .push_slot::<u32>(StatusV1::VT_EXIT_CODE_SYSTEM, exit_code_system, 0);
    }
    #[inline]
    pub fn add_exit_code_user(&mut self, exit_code_user: u32) {
        self.fbb_
            .push_slot::<u32>(StatusV1::VT_EXIT_CODE_USER, exit_code_user, 0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> StatusV1Builder<'a, 'b, A> {
        let start = _fbb.start_table();
        StatusV1Builder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<StatusV1<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for StatusV1<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("StatusV1");
        ds.field("execution_id", &self.execution_id());
        ds.field("status", &self.status());
        ds.field("proof", &self.proof());
        ds.field("execution_digest", &self.execution_digest());
        ds.field("input_digest", &self.input_digest());
        ds.field("committed_outputs", &self.committed_outputs());
        ds.field("assumption_digest", &self.assumption_digest());
        ds.field("exit_code_system", &self.exit_code_system());
        ds.field("exit_code_user", &self.exit_code_user());
        ds.finish()
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct StatusV1T {
    pub execution_id: Option<String>,
    pub status: StatusTypes,
    pub proof: Option<Vec<u8>>,
    pub execution_digest: Option<Vec<u8>>,
    pub input_digest: Option<Vec<u8>>,
    pub committed_outputs: Option<Vec<u8>>,
    pub assumption_digest: Option<Vec<u8>>,
    pub exit_code_system: u32,
    pub exit_code_user: u32,
}
impl Default for StatusV1T {
    fn default() -> Self {
        Self {
            execution_id: None,
            status: StatusTypes::Unknown,
            proof: None,
            execution_digest: None,
            input_digest: None,
            committed_outputs: None,
            assumption_digest: None,
            exit_code_system: 0,
            exit_code_user: 0,
        }
    }
}
impl StatusV1T {
    pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
        &self,
        _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>,
    ) -> flatbuffers::WIPOffset<StatusV1<'b>> {
        let execution_id = self.execution_id.as_ref().map(|x| _fbb.create_string(x));
        let status = self.status;
        let proof = self.proof.as_ref().map(|x| _fbb.create_vector(x));
        let execution_digest = self
            .execution_digest
            .as_ref()
            .map(|x| _fbb.create_vector(x));
        let input_digest = self.input_digest.as_ref().map(|x| _fbb.create_vector(x));
        let committed_outputs = self
            .committed_outputs
            .as_ref()
            .map(|x| _fbb.create_vector(x));
        let assumption_digest = self
            .assumption_digest
            .as_ref()
            .map(|x| _fbb.create_vector(x));
        let exit_code_system = self.exit_code_system;
        let exit_code_user = self.exit_code_user;
        StatusV1::create(
            _fbb,
            &StatusV1Args {
                execution_id,
                status,
                proof,
                execution_digest,
                input_digest,
                committed_outputs,
                assumption_digest,
                exit_code_system,
                exit_code_user,
            },
        )
    }
}
#[inline]
/// Verifies that a buffer of bytes contains a `StatusV1`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_status_v1_unchecked`.
pub fn root_as_status_v1(buf: &[u8]) -> Result<StatusV1, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<StatusV1>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `StatusV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_status_v1_unchecked`.
pub fn size_prefixed_root_as_status_v1(
    buf: &[u8],
) -> Result<StatusV1, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<StatusV1>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `StatusV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_status_v1_unchecked`.
pub fn root_as_status_v1_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StatusV1<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<StatusV1<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `StatusV1` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_status_v1_unchecked`.
pub fn size_prefixed_root_as_status_v1_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StatusV1<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<StatusV1<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a StatusV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `StatusV1`.
pub unsafe fn root_as_status_v1_unchecked(buf: &[u8]) -> StatusV1 {
    flatbuffers::root_unchecked::<StatusV1>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed StatusV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `StatusV1`.
pub unsafe fn size_prefixed_root_as_status_v1_unchecked(buf: &[u8]) -> StatusV1 {
    flatbuffers::size_prefixed_root_unchecked::<StatusV1>(buf)
}
#[inline]
pub fn finish_status_v1_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<StatusV1<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_status_v1_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<StatusV1<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
