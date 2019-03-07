from cfb.reflection.BaseType import BaseType

SIZE_OF_UOFFSET = 4

BASE_TYPE_SIZE = dict([
    (BaseType.Bool, 1),
    (BaseType.Byte, 1),
    (BaseType.Short, 2),
    (BaseType.Int, 4),
    (BaseType.Long, 8),
    (BaseType.UByte, 1),
    (BaseType.UShort, 2),
    (BaseType.UInt, 4),
    (BaseType.ULong, 8),
    (BaseType.Float, 4),
    (BaseType.Double, 8),

    (BaseType.String, 4),
])

BASE_TYPE_RUST_TYPE = dict([
    (BaseType.Bool, 'bool'),
    (BaseType.Byte, 'i8'),
    (BaseType.Short, 'i16'),
    (BaseType.Int, 'i32'),
    (BaseType.Long, 'i64'),
    (BaseType.UByte, 'u8'),
    (BaseType.UShort, 'u16'),
    (BaseType.UInt, 'u32'),
    (BaseType.ULong, 'u64'),
    (BaseType.Float, 'f32'),
    (BaseType.Double, 'f64'),

    (BaseType.String, "&'c str"),
])

BASE_TYPE_DEFAULT = dict([
    (BaseType.Bool, 'false'),
    (BaseType.Byte, '9i8'),
    (BaseType.Short, '0i16'),
    (BaseType.Int, '0i32'),
    (BaseType.Long, '0i64'),
    (BaseType.UByte, '0u8'),
    (BaseType.UShort, '0u16'),
    (BaseType.UInt, '0u32'),
    (BaseType.ULong, '0u64'),
    (BaseType.Float, '0f32'),
    (BaseType.Double, '0f64'),
])
