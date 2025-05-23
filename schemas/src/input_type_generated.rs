// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_PROGRAM_INPUT_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_PROGRAM_INPUT_TYPE: u8 = 3;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_PROGRAM_INPUT_TYPE: [ProgramInputType; 4] = [
  ProgramInputType::Unknown,
  ProgramInputType::Public,
  ProgramInputType::Private,
  ProgramInputType::PublicProof,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ProgramInputType(pub u8);
#[allow(non_upper_case_globals)]
impl ProgramInputType {
  pub const Unknown: Self = Self(0);
  pub const Public: Self = Self(1);
  pub const Private: Self = Self(2);
  pub const PublicProof: Self = Self(3);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 3;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Unknown,
    Self::Public,
    Self::Private,
    Self::PublicProof,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Unknown => Some("Unknown"),
      Self::Public => Some("Public"),
      Self::Private => Some("Private"),
      Self::PublicProof => Some("PublicProof"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ProgramInputType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ProgramInputType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ProgramInputType {
    type Output = ProgramInputType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ProgramInputType {
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

impl<'a> flatbuffers::Verifiable for ProgramInputType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ProgramInputType {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INPUT_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INPUT_TYPE: u8 = 8;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INPUT_TYPE: [InputType; 7] = [
  InputType::Unknown,
  InputType::PublicData,
  InputType::PublicAccountData,
  InputType::PublicUrl,
  InputType::Private,
  InputType::PublicProof,
  InputType::PrivateLocal,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InputType(pub u8);
#[allow(non_upper_case_globals)]
impl InputType {
  pub const Unknown: Self = Self(0);
  pub const PublicData: Self = Self(1);
  pub const PublicAccountData: Self = Self(3);
  pub const PublicUrl: Self = Self(4);
  pub const Private: Self = Self(5);
  pub const PublicProof: Self = Self(7);
  pub const PrivateLocal: Self = Self(8);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 8;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Unknown,
    Self::PublicData,
    Self::PublicAccountData,
    Self::PublicUrl,
    Self::Private,
    Self::PublicProof,
    Self::PrivateLocal,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Unknown => Some("Unknown"),
      Self::PublicData => Some("PublicData"),
      Self::PublicAccountData => Some("PublicAccountData"),
      Self::PublicUrl => Some("PublicUrl"),
      Self::Private => Some("Private"),
      Self::PublicProof => Some("PublicProof"),
      Self::PrivateLocal => Some("PrivateLocal"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for InputType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InputType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for InputType {
    type Output = InputType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for InputType {
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

impl<'a> flatbuffers::Verifiable for InputType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InputType {}
pub enum InputOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Input<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Input<'a> {
  type Inner = Input<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Input<'a> {
  pub const VT_INPUT_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_DATA: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Input { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args InputArgs<'args>
  ) -> flatbuffers::WIPOffset<Input<'bldr>> {
    let mut builder = InputBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.add_input_type(args.input_type);
    builder.finish()
  }

  pub fn unpack(&self) -> InputT {
    let input_type = self.input_type();
    let data = self.data().map(|x| {
      x.into_iter().collect()
    });
    InputT {
      input_type,
      data,
    }
  }

  #[inline]
  pub fn input_type(&self) -> InputType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<InputType>(Input::VT_INPUT_TYPE, Some(InputType::PublicData)).unwrap()}
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Input::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for Input<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<InputType>("input_type", Self::VT_INPUT_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct InputArgs<'a> {
    pub input_type: InputType,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for InputArgs<'a> {
  #[inline]
  fn default() -> Self {
    InputArgs {
      input_type: InputType::PublicData,
      data: None,
    }
  }
}

pub struct InputBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> InputBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_input_type(&mut self, input_type: InputType) {
    self.fbb_.push_slot::<InputType>(Input::VT_INPUT_TYPE, input_type, InputType::PublicData);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Input::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> InputBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    InputBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Input<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Input<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Input");
      ds.field("input_type", &self.input_type());
      ds.field("data", &self.data());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct InputT {
  pub input_type: InputType,
  pub data: Option<Vec<u8>>,
}
impl Default for InputT {
  fn default() -> Self {
    Self {
      input_type: InputType::PublicData,
      data: None,
    }
  }
}
impl InputT {
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>
  ) -> flatbuffers::WIPOffset<Input<'b>> {
    let input_type = self.input_type;
    let data = self.data.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    Input::create(_fbb, &InputArgs{
      input_type,
      data,
    })
  }
}
