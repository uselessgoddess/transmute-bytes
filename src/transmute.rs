use crate::FromBytes;
use std::{
    borrow::Cow,
    mem::{self, size_of},
    slice,
};

fn is_aligned_slice<T>(slice: &[u8]) -> bool {
    slice.as_ptr() as usize % mem::align_of::<T>() == 0
}

unsafe fn transmute_borrowed<T: FromBytes>(bytes: &[u8]) -> &[T] {
    slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len() / size_of::<T>())
}

fn transmute_owned<T: FromBytes>(bytes: &[u8]) -> Vec<T> {
    (0..bytes.len())
        .step_by(size_of::<T>())
        .map(|i| {
            let size = size_of::<T>();
            let bytes = if i + size >= bytes.len() {
                &bytes[i..]
            } else {
                &bytes[i..size]
            };
            // SAFETY: `bytes.len()` is less than or equal to `size_of::<T>()`
            unsafe { T::from_bytes(bytes).unwrap_unchecked() }
        })
        .collect()
}

pub fn transmute_bytes<T: Clone + FromBytes>(bytes: &impl AsRef<[u8]>) -> Cow<'_, [T]> {
    let bytes = bytes.as_ref();
    if is_aligned_slice::<T>(bytes) && bytes.len() % size_of::<T>() == 0 {
        // SAFETY: `bytes` is aligned and size is a multiple of `size_of::<T>()`
        unsafe { transmute_borrowed(bytes).into() }
    } else {
        transmute_owned(bytes).into()
    }
}
