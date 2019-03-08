pub mod example {
    #![allow(unused_imports)]

    use cfb::builder::{
        Builder, Component, DesignatedComponent, ReferenceVectorComponent, ScalarVectorComponent,
        StringComponent,
    };
    use cfb::scalar::Scalar;
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};
    #[cfg(not(target_endian = "little"))]
    use std::mem::transmute;

    #[derive(Clone, PartialEq, Debug)]
    pub enum Role {
        Hero(Hero),
        Monster(Monster),
    }

    impl Role {
        pub fn union_type(&self) -> u8 {
            match self {
                Role::Hero(_) => 1,
                Role::Monster(_) => 2,
            }
        }
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Hero {
        pub hp: u32,
    }

    impl Hero {
        const VT_HP: usize = 4;
        const SIZE_HP: usize = 4;
        const ALIGNMENT_HP: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Hero {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.hp != 0u32 {
                    vtable.add_field(Self::VT_HP, Self::SIZE_HP, Self::ALIGNMENT_HP);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.hp != 0u32 {
                builder.align(Self::ALIGNMENT_HP);
                builder.push_scalar(self.hp);
            }

            table_start
        }
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Monster {
        pub hp: u32,
    }

    impl Monster {
        const VT_HP: usize = 4;
        const SIZE_HP: usize = 4;
        const ALIGNMENT_HP: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Monster {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.hp != 0u32 {
                    vtable.add_field(Self::VT_HP, Self::SIZE_HP, Self::ALIGNMENT_HP);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.hp != 0u32 {
                builder.align(Self::ALIGNMENT_HP);
                builder.push_scalar(self.hp);
            }

            table_start
        }
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Player {
        pub role: Option<Role>,
    }

    impl Player {
        const VT_ROLE_TYPE: usize = 4;
        const SIZE_ROLE_TYPE: usize = 1;
        const ALIGNMENT_ROLE_TYPE: usize = 1;
        const VT_ROLE: usize = 6;
        const SIZE_ROLE: usize = 4;
        const ALIGNMENT_ROLE: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Player {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.role.is_some() {
                    vtable.add_field(Self::VT_ROLE, Self::SIZE_ROLE, Self::ALIGNMENT_ROLE);
                }
                if self.role.is_some() {
                    vtable.add_field(Self::VT_ROLE_TYPE, Self::SIZE_ROLE_TYPE, Self::ALIGNMENT_ROLE_TYPE);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            let role_type = self.role.as_ref().map(|v| v.union_type());
            if let Some(f) = self.role {
                builder.align(Self::ALIGNMENT_ROLE);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_ROLE);
                let component: Box<dyn Component<'c> + 'c> = match f {
                    Role::Hero(v) => Box::new(v),
                    Role::Monster(v) => Box::new(v),
                };
                builder.push_component(DesignatedComponent::new(offset_position, component));
            }
            if let Some(f) = role_type {
                builder.align(Self::ALIGNMENT_ROLE_TYPE);
                builder.push_scalar(f);
            }

            table_start
        }
    }
}
