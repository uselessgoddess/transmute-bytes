use std::{
    error,
    fmt::{Display, Formatter},
    mem::size_of,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    LengthMismatch { slice_size: usize, type_size: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Error::LengthMismatch {
                slice_size,
                type_size,
            } => {
                format!(
                    "cannot cast byte slice with size: {} to type with size: {}",
                    slice_size, type_size
                )
            }
            #[allow(unreachable_patterns)]
            _ => todo!(),
        };
        write!(f, "{}", err)
    }
}

impl error::Error for Error {}

pub unsafe trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>;
}

macro_rules! from_bytes_impl {
    ($($ty:ty)*) => {
        $(unsafe impl FromBytes for $ty {
            fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
                if bytes.len() > size_of::<$ty>() {
                    Err(Error::LengthMismatch {
                        slice_size: bytes.len(),
                        type_size: size_of::<$ty>(),
                    })
                } else {
                    let mut place: [u8; size_of::<$ty>()] = [0; size_of::<$ty>()];
                    place[..bytes.len()].copy_from_slice(bytes);
                    Ok(<$ty>::from_ne_bytes(place))
                }
            }
        })*
    };
}

from_bytes_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
