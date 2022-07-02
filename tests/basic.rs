use std::borrow::Cow;
use transmute_bytes::transmute_bytes;

fn as_bytes(vec: Vec<u64>, size: usize) -> Vec<u8> {
    vec.into_iter()
        .flat_map(|x| {
            let bytes: [u8; 8] = unsafe { std::mem::transmute(x) };
            bytes
        })
        .take(size)
        .collect()
}

#[test]
fn basic() {
    let bytes = [1_u8, 2, 3, 4, 5, 6, 7, 8, 0, 1];
    let cow = transmute_bytes::<u64>(&bytes);

    assert_eq!(as_bytes(cow.to_vec(), bytes.len()), &bytes);
}

#[test]
fn owned() {
    let bytes = [0_u8];
    let cow = transmute_bytes::<u16>(&bytes);

    assert!(matches!(cow, Cow::Owned(slice) if slice == [0]));
}

#[test]
fn borrowed() {
    let bytes = [0_u8; 2];
    let cow = transmute_bytes::<i16>(&bytes);

    assert!(matches!(cow, Cow::Borrowed(slice) if slice == [0]));
}
