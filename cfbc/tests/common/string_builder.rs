#![allow(unused_imports)]

use cfb_runtime::types::{SOffset, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET};
use cfb_runtime::{Builder, PushReferenceInto, PushScalarInto};

#[derive(Debug)]
pub struct AuthorBuilder<'a> {
    pub name: Option<&'a str>,
}

impl<'a> AuthorBuilder<'a> {
    const VT_NAME: usize = 4;
    const SIZE_NAME: usize = 4;
    const ALIGNMENT_NAME: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'a> Default for AuthorBuilder<'a> {
    fn default() -> Self {
        AuthorBuilder { name: None }
    }
}

impl<'a> PushReferenceInto for AuthorBuilder<'a> {
    fn push_into(self, builder: &mut Builder) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.name.map(str::is_empty) == Some(false) {
                vtable.add_field(
                    AuthorBuilder::VT_NAME,
                    AuthorBuilder::SIZE_NAME,
                    AuthorBuilder::ALIGNMENT_NAME,
                );
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, AuthorBuilder::ALIGNMENT);

        let table_start = builder.len();
        builder.push_scalar((table_start - vtable_start) as SOffset);

        let name_pos = if self.name.map(str::is_empty) == Some(false) {
            builder.align(AuthorBuilder::ALIGNMENT_NAME);
            let pos = builder.len();
            builder.pad(SIZE_OF_UOFFSET);
            pos
        } else {
            0
        };

        if let Some(name) = self.name {
            if name_pos > 0 {
                builder.push_reference(name_pos, name);
            }
        }

        table_start
    }
}
