use embedded_msgpack::decode::Value;
use embedded_msgpack::encode::{Binary, Serializable};

// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

fn print_slice(data: &[u8]) {
    print!("[");
    for i in 0..data.len() {
        print!("{}0x{:02x}", if i > 0 { ", " } else { "" }, &data[i]);
    }
    println!("]");
}

fn test_encode<T: Serializable>(data: T, expected: &[u8]) {
    let mut buf = [0u8; 100];
    let len = data.write_into(&mut buf[..]).unwrap();
    print_slice(&buf[..len]);
    assert_eq!(expected.len(), len);
    assert_eq!(expected, &buf[..len]);
}

#[test]
fn encode_timestamp() {
    test_encode(
        embedded_msgpack::timestamp::Timestamp::new(1514862245, 0).unwrap(),
        &[0xd6, 0xff, 0x5a, 0x4a, 0xf6, 0xa5],
    );
    test_encode(
        embedded_msgpack::timestamp::Timestamp::new(1514862245, 678901234).unwrap(),
        &[0xd7, 0xff, 0xa1, 0xdc, 0xd7, 0xc8, 0x5a, 0x4a, 0xf6, 0xa5],
    );
    test_encode(
        embedded_msgpack::timestamp::Timestamp::new(2147483647, 999999999).unwrap(),
        &[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xfc, 0x7f, 0xff, 0xff, 0xff],
    );
    test_encode(
        embedded_msgpack::timestamp::Timestamp::new(2147483648, 0).unwrap(),
        &[0xd6, 0xff, 0x80, 0x00, 0x00, 0x00],
    );
}

#[test]
fn encode_int() {
    test_encode(true, &[0xc3]);
    test_encode(false, &[0xc2]);
    test_encode(4u32, &[4]);
    test_encode(0x99u32, &[0xcc, 0x99]);
    test_encode(0x1234u32, &[0xcd, 0x12, 0x34]);
    test_encode(0x12345678u32, &[0xce, 0x12, 0x34, 0x56, 0x78]);
    #[cfg(feature = "u64")]
    test_encode(0x12345678u64, &[0xce, 0x12, 0x34, 0x56, 0x78]);
    #[cfg(feature = "u64")]
    test_encode(
        0x1234567890abcdefu64,
        &[0xcf, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef],
    );
}
#[test]
fn encode_map() {
    let map: &[(&str, u32)] = &[("abc", 34), ("def", 128)];
    test_encode(
        map,
        &[
            0x82, 0xA3, 0x61, 0x62, 0x63, 0x22, 0xA3, 0x64, 0x65, 0x66, 0xCC, 0x80,
        ],
    );
}
#[test]
fn encode_slice() {
    test_encode(
        &["abc", "def"][..],
        &[0x92, 0xA3, 0x61, 0x62, 0x63, 0xA3, 0x64, 0x65, 0x66],
    );
    test_encode(&[1u32, 2, 3][..], &[0x93, 1, 2, 3]);
}
#[test]
fn encode_bin() {
    let s: &[u8] = &[1u8, 2, 3, 4, 5, 6, 7];
    // let bin: Binary = s.into();
    test_encode(Binary::new(s), &[0xc4, 7, 1, 2, 3, 4, 5, 6, 7]);
}

// #[test]
// fn fixint() {
//     let buf = [0x10, 0xcc, 0x20, 0xcd, 0x12, 0x34];
//     let mut pos = 0;
//     let b = &buf[pos..];
//     let x = Value::read(&b).unwrap();
//     println!("{:?}", x);
//     assert_eq!(x.0, Value::U8(0x10));

//     pos += x.1;
//     let b = &buf[pos..];
//     let x = Value::read(&b).unwrap();
//     println!("{:?}", x);
//     assert_eq!(x.0, Value::U8(0x20));

//     pos += x.1;
//     let b = &buf[pos..];
//     let x = Value::read(&b).unwrap();
//     println!("{:?}", x);
//     assert_eq!(x.0, Value::U16(&[0x12, 0x34]));
//     if let Value::U16(a) = x.0 {
//         let ptr = a.as_ptr();
//         println!("{:?}", ptr);
//         println!("{:?}", buf.as_ptr());
//         println!("{}", core::mem::size_of::<Value>());
//         assert_eq!(ptr, unsafe { b.as_ptr().offset(1) });
//     }
// }
