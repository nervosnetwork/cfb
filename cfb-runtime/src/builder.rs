use std::collections::HashMap;

pub struct VTableBuilder {}

#[derive(Default, Debug)]
pub struct Builder {
    buffer: Vec<u8>,
    vtables: HashMap<u64, usize>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }
}

#[cfg(test)]
mod poc {
    #![allow(dead_code, clippy::blacklisted_name)]

    use super::*;

    // Ensure when drop:
    // - finish is called
    struct FooVTableBuilder<'a> {
        builder: &'a mut Builder,
    }

    impl<'a> FooVTableBuilder<'a> {
        fn hp(&mut self) -> &mut Self {
            self.builder.buffer.push(0);
            self
        }
        fn mp(&mut self) {
            self.builder.buffer.push(1);
        }
        fn bar_a(&mut self) {
            self.builder.buffer.push(2);
        }
        fn bar_b(&mut self) {
            self.builder.buffer.push(3);
        }
        fn finish(&mut self) -> FooBuilder {
            self.builder.buffer.push(4);
            FooBuilder {
                builder: self.builder,
            }
        }
    }

    // Ensure when drop:
    // - all references are built
    struct FooBuilder<'a> {
        builder: &'a mut Builder,
    }

    impl<'a> FooBuilder<'a> {
        fn begin_vtable(builder: &'a mut Builder) -> FooVTableBuilder<'a> {
            FooVTableBuilder { builder }
        }

        fn hp(&mut self, v: u32) {
            self.builder.buffer.push(v as u8);
        }
        fn mp(&mut self, v: u32) {
            self.builder.buffer.push(v as u8);
        }

        fn bar_a(&mut self) -> BarVTableBuilder {
            // check that:
            // - bar_a is not built yet
            BarVTableBuilder {
                builder: self.builder,
            }
        }
        fn bar_b(&mut self) -> BarVTableBuilder {
            // check that:
            // - bar_a is not built yet
            // - bar_b is not built yet
            BarVTableBuilder {
                builder: self.builder,
            }
        }
    }

    struct BarVTableBuilder<'a> {
        builder: &'a mut Builder,
    }

    impl<'a> BarVTableBuilder<'a> {
        fn hp(&mut self) {
            self.builder.buffer.push(0);
        }
        fn mp(&mut self) {
            self.builder.buffer.push(1);
        }
        fn finish(&mut self) -> BarBuilder {
            self.builder.buffer.push(2);
            BarBuilder {
                builder: self.builder,
            }
        }
    }

    struct BarBuilder<'a> {
        builder: &'a mut Builder,
    }

    impl<'a> BarBuilder<'a> {
        fn begin_vtable(builder: &'a mut Builder) -> BarVTableBuilder<'a> {
            BarVTableBuilder { builder }
        }

        fn hp(&mut self, v: u32) {
            self.builder.buffer.push(v as u8);
        }
        fn mp(&mut self, v: u32) {
            self.builder.buffer.push(v as u8);
        }
    }

    #[test]
    fn poc_test() {
        let mut b = Builder::new();

        let mut foo_v = FooBuilder::begin_vtable(&mut b);
        foo_v.hp();
        foo_v.bar_a();
        foo_v.bar_b();
        let mut foo = foo_v.finish();
        foo.hp(10);

        let mut bar_a_v = foo.bar_a();
        bar_a_v.hp();
        let mut bar_a = bar_a_v.finish();
        bar_a.hp(20);

        let mut bar_b_v = foo.bar_b();
        bar_b_v.hp();
        let mut bar_b = bar_b_v.finish();
        bar_b.hp(30);

        dbg!(b);
    }
}
