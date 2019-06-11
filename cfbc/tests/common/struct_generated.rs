// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

// struct Vec3, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x_: u64,
    y_: u8,
    padding0__: u8,
    padding1__: u16,
    padding2__: u32,
    z_: u64,
} // pub struct Vec3
impl flatbuffers::SafeSliceAccess for Vec3 {}
impl<'a> flatbuffers::Follow<'a> for Vec3 {
    type Inner = &'a Vec3;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a Vec3>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
    type Inner = &'a Vec3;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<Vec3>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for Vec3 {
    type Output = Vec3;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src =
            unsafe { ::std::slice::from_raw_parts(self as *const Vec3 as *const u8, Self::size()) };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Vec3 {
    type Output = Vec3;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Vec3 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl Vec3 {
    pub fn new<'a>(_x: u64, _y: u8, _z: u64) -> Self {
        Vec3 {
            x_: _x.to_little_endian(),
            y_: _y.to_little_endian(),
            z_: _z.to_little_endian(),

            padding0__: 0,
            padding1__: 0,
            padding2__: 0,
        }
    }
    pub fn x<'a>(&'a self) -> u64 {
        self.x_.from_little_endian()
    }
    pub fn y<'a>(&'a self) -> u8 {
        self.y_.from_little_endian()
    }
    pub fn z<'a>(&'a self) -> u64 {
        self.z_.from_little_endian()
    }
}

pub enum PointOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Point<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Point<'a> {
    type Inner = Point<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Point<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Point { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PointArgs<'args>,
    ) -> flatbuffers::WIPOffset<Point<'bldr>> {
        let mut builder = PointBuilder::new(_fbb);
        if let Some(x) = args.position {
            builder.add_position(x);
        }
        builder.finish()
    }

    pub const VT_POSITION: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn position(&self) -> Option<&'a Vec3> {
        self._tab.get::<Vec3>(Point::VT_POSITION, None)
    }
}

pub struct PointArgs<'a> {
    pub position: Option<&'a Vec3>,
}
impl<'a> Default for PointArgs<'a> {
    #[inline]
    fn default() -> Self {
        PointArgs { position: None }
    }
}
pub struct PointBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PointBuilder<'a, 'b> {
    #[inline]
    pub fn add_position(&mut self, position: &'b Vec3) {
        self.fbb_
            .push_slot_always::<&Vec3>(Point::VT_POSITION, position);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PointBuilder<'a, 'b> {
        let start = _fbb.start_table();
        PointBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Point<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}