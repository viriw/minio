use std::{io, mem::size_of};

macro_rules! read_impl {
    ($t: ty, $name: literal, $le: ident, $be: ident, $ne: ident) => {
        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (little-endian) from the underlying reader."]
        fn $le(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_le_bytes(read_impl_body!(self, $t)))
        }

        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (big-endian) from the underlying reader."]
        fn $be(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_be_bytes(read_impl_body!(self, $t)))
        }

        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` (native-endian) from the underlying reader."]
        fn $ne(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_ne_bytes(read_impl_body!(self, $t)))
        }
    };

    // This one is used for i8/u8 alternate functions because endianness does not matter.
    ($t: ty, $name: literal, $fn: ident) => {
        #[doc = "Reads a `"]
        #[doc = $name]
        #[doc = "` from the underlying reader."]
        fn $fn(&mut self) -> io::Result<$t> {
            Ok(<$t>::from_ne_bytes(read_impl_body!(self, $t)))
        }
    };
}

macro_rules! read_impl_body {
    ($context: expr, $t: ty) => {{
        let mut buf = [0u8; size_of::<$t>()];
        $context.read_exact(&mut buf)?;
        buf
    }};
}

pub trait ReadPrimitives: io::Read {
    read_impl!(i8, "i8", read_i8);
    read_impl!(u8, "u8", read_u8);
    read_impl!(i8, "i8", read_i8_le, read_i8_be, read_i8_ne);
    read_impl!(u8, "u8", read_u8_le, read_u8_be, read_u8_ne);
    read_impl!(i16, "i16", read_i16_le, read_i16_be, read_i16_ne);
    read_impl!(u16, "u16", read_u16_le, read_u16_be, read_u16_ne);
    read_impl!(i32, "i32", read_i32_le, read_i32_be, read_i32_ne);
    read_impl!(u32, "u32", read_u32_le, read_u32_be, read_u32_ne);
    read_impl!(i64, "i64", read_i64_le, read_i64_be, read_i64_ne);
    read_impl!(u64, "u64", read_u64_le, read_u64_be, read_u64_ne);
    read_impl!(i128, "i128", read_i128_le, read_i128_be, read_i128_ne);
    read_impl!(u128, "u128", read_u128_le, read_u128_be, read_u128_ne);
}
