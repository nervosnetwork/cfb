// automatically generated by the FlatBuffers compiler, do not modify


pub mod example {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum AuthorOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Author<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Author<'a> {
    type Inner = Author<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Author<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Author {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args AuthorArgs<'args>) -> flatbuffers::WIPOffset<Author<'bldr>> {
      let mut builder = AuthorBuilder::new(_fbb);
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Author::VT_NAME, None)
  }
}

pub struct AuthorArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for AuthorArgs<'a> {
    #[inline]
    fn default() -> Self {
        AuthorArgs {
            name: None,
        }
    }
}
pub struct AuthorBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AuthorBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Author::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AuthorBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AuthorBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Author<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod example

