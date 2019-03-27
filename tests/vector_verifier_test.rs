pub mod common;

mod scalar_vector_test {
    use super::common::scalar_vector_generated::example::Sensor;
    use super::common::scalar_vector_generated_verifier::{get_root, Error};

    #[test]
    fn test_scalar_vector_uoffset_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // sensor
            le!(6i32),
            le!(4u32),
        ]
        .concat();
        let root = get_root::<Sensor>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_scalar_vector_len_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // sensor
            le!(6i32),
            le!(4u32),
            // readings
            le!(0u16),
        ]
        .concat();
        let root = get_root::<Sensor>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_scalar_vector_content_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // sensor
            le!(6i32),
            le!(4u32),
            // readings
            le!(2u32),
            le!(1u32),
        ]
        .concat();
        let root = get_root::<Sensor>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }
}

mod string_vector_test {
    use super::common::string_vector_generated::example::Text;
    use super::common::string_vector_generated_verifier::{get_root, Error};

    #[test]
    fn test_string_vector_uoffset_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // text
            le!(6i32),
            le!(4u32),
        ]
        .concat();
        let root = get_root::<Text>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_string_vector_len_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // text
            le!(6i32),
            le!(4u32),
            // lines
            le!(0u16),
        ]
        .concat();
        let root = get_root::<Text>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_string_vector_content_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // text
            le!(6i32),
            le!(4u32),
            // lines
            le!(2u32),
            le!(1u32),
        ]
        .concat();
        let root = get_root::<Text>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_string_vector_element_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // text
            le!(6i32),
            le!(4u32),
            // lines
            le!(1u32),
            le!(4u32),
            // lines[0]
            le!(1u32),
            b"ab",
        ]
        .concat();
        let root = get_root::<Text>(&buf);
        assert_eq!(root, Err(Error::NonNullTerminatedString));
    }
}

mod table_vector_test {
    use super::common::table_vector_generated::example::Hero;
    use super::common::table_vector_generated_verifier::{get_root, Error};

    #[test]
    fn test_table_vector_uoffset_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // hero
            le!(6i32),
            le!(4u32),
        ]
        .concat();
        let root = get_root::<Hero>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_table_vector_len_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // hero
            le!(6i32),
            le!(4u32),
            // stats
            le!(0u16),
        ]
        .concat();
        let root = get_root::<Hero>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_table_vector_content_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // hero
            le!(6i32),
            le!(4u32),
            // stats
            le!(2u32),
            le!(4u32),
        ]
        .concat();
        let root = get_root::<Hero>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_table_vector_element_out_of_bounds() {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(8u16),
            le!(4u16),
            // hero
            le!(6i32),
            le!(4u32),
            // stats
            le!(1u32),
            le!(4u32),
            // stats[0]
            le!(22u32),
            le!(4u16),
        ]
        .concat();
        let root = get_root::<Hero>(&buf);
        assert_eq!(root, Err(Error::OutOfBounds));
    }
}
