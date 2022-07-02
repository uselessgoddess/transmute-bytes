## Simple. Safe. From bytes without loss

### Usage

```rust
use transmute_bytes::transmute_bytes;

fn main() {
    // Your byte data
    let bytes = [1_u8, 2, 3, 4, 5, 6, 7, 8, 0, 1];
    // You will receive:
    // - `Cow::Borrow(slice)` if data is aligned to `u64`
    //    and  length is a multiple of the length of u64
    //    ----------------------------------------------
    // - `Cow::Owned(vec)` if data is not aligned to `u64`
    //    or length is not a multiple of the length of u64
    let cow = transmute_bytes::<u64>(&bytes);

    // It depends on your endian
    let le = cow.as_ref() == [578437695752307201, 256];
    let be = cow.as_ref() == [72623859790382856, 281474976710656];
    assert!(le || be);
}
```