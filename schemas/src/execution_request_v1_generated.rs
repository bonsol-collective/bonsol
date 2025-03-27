// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::input_type_generated::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_PROVER_VERSION: u16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_PROVER_VERSION: u16 = 9;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_PROVER_VERSION: [ProverVersion; 3] = [
  ProverVersion::DEFAULT,
  ProverVersion::V1_0_1,
  ProverVersion::V1_2_1,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ProverVersion(pub u16);
#[allow(non_upper_case_globals)]
impl ProverVersion {
  pub const DEFAULT: Self = Self(0);
  pub const V1_0_1: Self = Self(1);
  pub const V1_2_1: Self = Self(9);

  pub const ENUM_MIN: u16 = 0;
  pub const ENUM_MAX: u16 = 9;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::DEFAULT,
    Self::V1_0_1,
    Self::V1_2_1,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::DEFAULT => Some("DEFAULT"),
      Self::V1_0_1 => Some("V1_0_1"),
      Self::V1_2_1 => Some("V1_2_1"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ProverVersion {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ProverVersion {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ProverVersion {
    type Output = ProverVersion;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ProverVersion {
  type Scalar = u16;
  #[inline]
  fn to_little_endian(self) -> u16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u16) -> Self {
    let b = u16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ProverVersion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ProverVersion {}
// struct Account, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Account(pub [u8; 40]);
impl Default for Account { 
  fn default() -> Self { 
    Self([0; 40])
  }
}
impl core::fmt::Debug for Account {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Account")
      .field("writable", &self.writable())
      .field("pubkey", &self.pubkey())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Account {}
impl<'a> flatbuffers::Follow<'a> for Account {
  type Inner = &'a Account;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Account>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Account {
  type Inner = &'a Account;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Account>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Account {
    type Output = Account;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Account as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Account {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Account {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    writable: u8,
    pubkey: &[u8; 32],
  ) -> Self {
    let mut s = Self([0; 40]);
    s.set_writable(writable);
    s.set_pubkey(pubkey);
    s
  }

  pub fn writable(&self) -> u8 {
    let mut mem = core::mem::MaybeUninit::<<u8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_writable(&mut self, x: u8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn pubkey(&'a self) -> flatbuffers::Array<'a, u8, 32> {
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid array in this slot
    unsafe { flatbuffers::Array::follow(&self.0, 1) }
  }

  pub fn set_pubkey(&mut self, items: &[u8; 32]) {
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid array in this slot
    unsafe { flatbuffers::emplace_scalar_array(&mut self.0, 1, items) };
  }

  pub fn unpack(&self) -> AccountT {
    AccountT {
      writable: self.writable(),
      pubkey: self.pubkey().into(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccountT {
  pub writable: u8,
  pub pubkey: [u8; 32],
}
impl AccountT {
  pub fn pack(&self) -> Account {
    Account::new(
      self.writable,
      &self.pubkey,
    )
  }
}

pub enum ExecutionRequestV1Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ExecutionRequestV1<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ExecutionRequestV1<'a> {
  type Inner = ExecutionRequestV1<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ExecutionRequestV1<'a> {
  pub const VT_TIP: flatbuffers::VOffsetT = 4;
  pub const VT_EXECUTION_ID: flatbuffers::VOffsetT = 6;
  pub const VT_IMAGE_ID: flatbuffers::VOffsetT = 8;
  pub const VT_CALLBACK_PROGRAM_ID: flatbuffers::VOffsetT = 10;
  pub const VT_CALLBACK_INSTRUCTION_PREFIX: flatbuffers::VOffsetT = 12;
  pub const VT_FORWARD_OUTPUT: flatbuffers::VOffsetT = 14;
  pub const VT_VERIFY_INPUT_HASH: flatbuffers::VOffsetT = 16;
  pub const VT_INPUT: flatbuffers::VOffsetT = 18;
  pub const VT_INPUT_DIGEST: flatbuffers::VOffsetT = 20;
  pub const VT_MAX_BLOCK_HEIGHT: flatbuffers::VOffsetT = 22;
  pub const VT_CALLBACK_EXTRA_ACCOUNTS: flatbuffers::VOffsetT = 24;
  pub const VT_PROVER_VERSION: flatbuffers::VOffsetT = 26;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ExecutionRequestV1 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ExecutionRequestV1Args<'args>
  ) -> flatbuffers::WIPOffset<ExecutionRequestV1<'bldr>> {
    let mut builder = ExecutionRequestV1Builder::new(_fbb);
    builder.add_max_block_height(args.max_block_height);
    builder.add_tip(args.tip);
    if let Some(x) = args.callback_extra_accounts { builder.add_callback_extra_accounts(x); }
    if let Some(x) = args.input_digest { builder.add_input_digest(x); }
    if let Some(x) = args.input { builder.add_input(x); }
    if let Some(x) = args.callback_instruction_prefix { builder.add_callback_instruction_prefix(x); }
    if let Some(x) = args.callback_program_id { builder.add_callback_program_id(x); }
    if let Some(x) = args.image_id { builder.add_image_id(x); }
    if let Some(x) = args.execution_id { builder.add_execution_id(x); }
    builder.add_prover_version(args.prover_version);
    builder.add_verify_input_hash(args.verify_input_hash);
    builder.add_forward_output(args.forward_output);
    builder.finish()
  }

  pub fn unpack(&self) -> ExecutionRequestV1T {
    let tip = self.tip();
    let execution_id = self.execution_id().map(|x| {
      x.to_string()
    });
    let image_id = self.image_id().map(|x| {
      x.to_string()
    });
    let callback_program_id = self.callback_program_id().map(|x| {
      x.into_iter().collect()
    });
    let callback_instruction_prefix = self.callback_instruction_prefix().map(|x| {
      x.into_iter().collect()
    });
    let forward_output = self.forward_output();
    let verify_input_hash = self.verify_input_hash();
    let input = self.input().map(|x| {
      x.iter().map(|t| t.unpack()).collect()
    });
    let input_digest = self.input_digest().map(|x| {
      x.into_iter().collect()
    });
    let max_block_height = self.max_block_height();
    let callback_extra_accounts = self.callback_extra_accounts().map(|x| {
      x.iter().map(|t| t.unpack()).collect()
    });
    let prover_version = self.prover_version();
    ExecutionRequestV1T {
      tip,
      execution_id,
      image_id,
      callback_program_id,
      callback_instruction_prefix,
      forward_output,
      verify_input_hash,
      input,
      input_digest,
      max_block_height,
      callback_extra_accounts,
      prover_version,
    }
  }

  #[inline]
  pub fn tip(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ExecutionRequestV1::VT_TIP, Some(0)).unwrap()}
  }
  #[inline]
  pub fn execution_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ExecutionRequestV1::VT_EXECUTION_ID, None)}
  }
  #[inline]
  pub fn image_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ExecutionRequestV1::VT_IMAGE_ID, None)}
  }
  #[inline]
  pub fn callback_program_id(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(ExecutionRequestV1::VT_CALLBACK_PROGRAM_ID, None)}
  }
  #[inline]
  pub fn callback_instruction_prefix(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(ExecutionRequestV1::VT_CALLBACK_INSTRUCTION_PREFIX, None)}
  }
  #[inline]
  pub fn forward_output(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(ExecutionRequestV1::VT_FORWARD_OUTPUT, Some(false)).unwrap()}
  }
  #[inline]
  pub fn verify_input_hash(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(ExecutionRequestV1::VT_VERIFY_INPUT_HASH, Some(true)).unwrap()}
  }
  #[inline]
  pub fn input(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Input<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Input>>>>(ExecutionRequestV1::VT_INPUT, None)}
  }
  #[inline]
  pub fn input_digest(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(ExecutionRequestV1::VT_INPUT_DIGEST, None)}
  }
  #[inline]
  pub fn max_block_height(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ExecutionRequestV1::VT_MAX_BLOCK_HEIGHT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn callback_extra_accounts(&self) -> Option<flatbuffers::Vector<'a, Account>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Account>>>(ExecutionRequestV1::VT_CALLBACK_EXTRA_ACCOUNTS, None)}
  }
  #[inline]
  pub fn prover_version(&self) -> ProverVersion {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ProverVersion>(ExecutionRequestV1::VT_PROVER_VERSION, Some(ProverVersion::DEFAULT)).unwrap()}
  }
}

impl flatbuffers::Verifiable for ExecutionRequestV1<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("tip", Self::VT_TIP, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("execution_id", Self::VT_EXECUTION_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("image_id", Self::VT_IMAGE_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("callback_program_id", Self::VT_CALLBACK_PROGRAM_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("callback_instruction_prefix", Self::VT_CALLBACK_INSTRUCTION_PREFIX, false)?
     .visit_field::<bool>("forward_output", Self::VT_FORWARD_OUTPUT, false)?
     .visit_field::<bool>("verify_input_hash", Self::VT_VERIFY_INPUT_HASH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Input>>>>("input", Self::VT_INPUT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("input_digest", Self::VT_INPUT_DIGEST, false)?
     .visit_field::<u64>("max_block_height", Self::VT_MAX_BLOCK_HEIGHT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Account>>>("callback_extra_accounts", Self::VT_CALLBACK_EXTRA_ACCOUNTS, false)?
     .visit_field::<ProverVersion>("prover_version", Self::VT_PROVER_VERSION, false)?
     .finish();
    Ok(())
  }
}
pub struct ExecutionRequestV1Args<'a> {
    pub tip: u64,
    pub execution_id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub image_id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub callback_program_id: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub callback_instruction_prefix: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub forward_output: bool,
    pub verify_input_hash: bool,
    pub input: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Input<'a>>>>>,
    pub input_digest: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub max_block_height: u64,
    pub callback_extra_accounts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Account>>>,
    pub prover_version: ProverVersion,
}
impl<'a> Default for ExecutionRequestV1Args<'a> {
  #[inline]
  fn default() -> Self {
    ExecutionRequestV1Args {
      tip: 0,
      execution_id: None,
      image_id: None,
      callback_program_id: None,
      callback_instruction_prefix: None,
      forward_output: false,
      verify_input_hash: true,
      input: None,
      input_digest: None,
      max_block_height: 0,
      callback_extra_accounts: None,
      prover_version: ProverVersion::DEFAULT,
    }
  }
}

pub struct ExecutionRequestV1Builder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ExecutionRequestV1Builder<'a, 'b, A> {
  #[inline]
  pub fn add_tip(&mut self, tip: u64) {
    self.fbb_.push_slot::<u64>(ExecutionRequestV1::VT_TIP, tip, 0);
  }
  #[inline]
  pub fn add_execution_id(&mut self, execution_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_EXECUTION_ID, execution_id);
  }
  #[inline]
  pub fn add_image_id(&mut self, image_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_IMAGE_ID, image_id);
  }
  #[inline]
  pub fn add_callback_program_id(&mut self, callback_program_id: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_CALLBACK_PROGRAM_ID, callback_program_id);
  }
  #[inline]
  pub fn add_callback_instruction_prefix(&mut self, callback_instruction_prefix: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_CALLBACK_INSTRUCTION_PREFIX, callback_instruction_prefix);
  }
  #[inline]
  pub fn add_forward_output(&mut self, forward_output: bool) {
    self.fbb_.push_slot::<bool>(ExecutionRequestV1::VT_FORWARD_OUTPUT, forward_output, false);
  }
  #[inline]
  pub fn add_verify_input_hash(&mut self, verify_input_hash: bool) {
    self.fbb_.push_slot::<bool>(ExecutionRequestV1::VT_VERIFY_INPUT_HASH, verify_input_hash, true);
  }
  #[inline]
  pub fn add_input(&mut self, input: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Input<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_INPUT, input);
  }
  #[inline]
  pub fn add_input_digest(&mut self, input_digest: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_INPUT_DIGEST, input_digest);
  }
  #[inline]
  pub fn add_max_block_height(&mut self, max_block_height: u64) {
    self.fbb_.push_slot::<u64>(ExecutionRequestV1::VT_MAX_BLOCK_HEIGHT, max_block_height, 0);
  }
  #[inline]
  pub fn add_callback_extra_accounts(&mut self, callback_extra_accounts: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Account>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ExecutionRequestV1::VT_CALLBACK_EXTRA_ACCOUNTS, callback_extra_accounts);
  }
  #[inline]
  pub fn add_prover_version(&mut self, prover_version: ProverVersion) {
    self.fbb_.push_slot::<ProverVersion>(ExecutionRequestV1::VT_PROVER_VERSION, prover_version, ProverVersion::DEFAULT);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ExecutionRequestV1Builder<'a, 'b, A> {
    let start = _fbb.start_table();
    ExecutionRequestV1Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ExecutionRequestV1<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ExecutionRequestV1<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ExecutionRequestV1");
      ds.field("tip", &self.tip());
      ds.field("execution_id", &self.execution_id());
      ds.field("image_id", &self.image_id());
      ds.field("callback_program_id", &self.callback_program_id());
      ds.field("callback_instruction_prefix", &self.callback_instruction_prefix());
      ds.field("forward_output", &self.forward_output());
      ds.field("verify_input_hash", &self.verify_input_hash());
      ds.field("input", &self.input());
      ds.field("input_digest", &self.input_digest());
      ds.field("max_block_height", &self.max_block_height());
      ds.field("callback_extra_accounts", &self.callback_extra_accounts());
      ds.field("prover_version", &self.prover_version());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionRequestV1T {
  pub tip: u64,
  pub execution_id: Option<String>,
  pub image_id: Option<String>,
  pub callback_program_id: Option<Vec<u8>>,
  pub callback_instruction_prefix: Option<Vec<u8>>,
  pub forward_output: bool,
  pub verify_input_hash: bool,
  pub input: Option<Vec<InputT>>,
  pub input_digest: Option<Vec<u8>>,
  pub max_block_height: u64,
  pub callback_extra_accounts: Option<Vec<AccountT>>,
  pub prover_version: ProverVersion,
}
impl Default for ExecutionRequestV1T {
  fn default() -> Self {
    Self {
      tip: 0,
      execution_id: None,
      image_id: None,
      callback_program_id: None,
      callback_instruction_prefix: None,
      forward_output: false,
      verify_input_hash: true,
      input: None,
      input_digest: None,
      max_block_height: 0,
      callback_extra_accounts: None,
      prover_version: ProverVersion::DEFAULT,
    }
  }
}
impl ExecutionRequestV1T {
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>
  ) -> flatbuffers::WIPOffset<ExecutionRequestV1<'b>> {
    let tip = self.tip;
    let execution_id = self.execution_id.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let image_id = self.image_id.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let callback_program_id = self.callback_program_id.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    let callback_instruction_prefix = self.callback_instruction_prefix.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    let forward_output = self.forward_output;
    let verify_input_hash = self.verify_input_hash;
    let input = self.input.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| t.pack(_fbb)).collect();_fbb.create_vector(&w)
    });
    let input_digest = self.input_digest.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    let max_block_height = self.max_block_height;
    let callback_extra_accounts = self.callback_extra_accounts.as_ref().map(|x|{
      let w: Vec<_> = x.iter().map(|t| t.pack()).collect();_fbb.create_vector(&w)
    });
    let prover_version = self.prover_version;
    ExecutionRequestV1::create(_fbb, &ExecutionRequestV1Args{
      tip,
      execution_id,
      image_id,
      callback_program_id,
      callback_instruction_prefix,
      forward_output,
      verify_input_hash,
      input,
      input_digest,
      max_block_height,
      callback_extra_accounts,
      prover_version,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `ExecutionRequestV1`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_execution_request_v1_unchecked`.
pub fn root_as_execution_request_v1(buf: &[u8]) -> Result<ExecutionRequestV1, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ExecutionRequestV1>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ExecutionRequestV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_execution_request_v1_unchecked`.
pub fn size_prefixed_root_as_execution_request_v1(buf: &[u8]) -> Result<ExecutionRequestV1, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ExecutionRequestV1>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ExecutionRequestV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_execution_request_v1_unchecked`.
pub fn root_as_execution_request_v1_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ExecutionRequestV1<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ExecutionRequestV1<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ExecutionRequestV1` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_execution_request_v1_unchecked`.
pub fn size_prefixed_root_as_execution_request_v1_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ExecutionRequestV1<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ExecutionRequestV1<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ExecutionRequestV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ExecutionRequestV1`.
pub unsafe fn root_as_execution_request_v1_unchecked(buf: &[u8]) -> ExecutionRequestV1 {
  flatbuffers::root_unchecked::<ExecutionRequestV1>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ExecutionRequestV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ExecutionRequestV1`.
pub unsafe fn size_prefixed_root_as_execution_request_v1_unchecked(buf: &[u8]) -> ExecutionRequestV1 {
  flatbuffers::size_prefixed_root_unchecked::<ExecutionRequestV1>(buf)
}
#[inline]
pub fn finish_execution_request_v1_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<ExecutionRequestV1<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_execution_request_v1_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<ExecutionRequestV1<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
