fn test_decode<'a, T: serde::de::Deserialize<'a> + PartialEq + std::fmt::Debug>(expected: T, variants: &'a [&'a [u8]]) {
    for &x in variants.iter() {
        let v: T = embedded_msgpack::decode::from_slice(x).unwrap();
        assert_eq!(expected, v);
    }
}

#[test]
fn decode_nil() {
    let mut o = Some(true);
    test_decode(o, &[&[0xc3]]);
    o = None;
    test_decode(o, &[&[0xc0]]);
}
#[test]
fn decode_bool() {
    test_decode(true, &[&[0xc3]]);
    test_decode(false, &[&[0xc2]]);
}
#[test]
fn decode_uint() {
    test_decode(1u8, &[&[0x1], &[0xcc, 0x01], &[0xcd, 0x00, 0x01], &[0xce, 0x00, 0x00, 0x00, 0x01]]);
    test_decode(
        127u32,
        &[&[0x7f], &[0xcc, 0x7f], &[0xcd, 0x00, 0x7f], &[0xce, 0x00, 0x00, 0x00, 0x7f]],
    );
    test_decode(256u16, &[&[0xcd, 0x01, 0x00], &[0xce, 0x00, 0x00, 0x01, 0x00]]);
}
#[test]
fn decode_bin() {
    test_decode(
        embedded_msgpack::Bytes::new(&[
            0x31u8, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
        ]),
        &[
            &[
                0xbf, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
                0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
            &[
                0xc4, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
                0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
            &[
                0xd9, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
                0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
            &[
                0xc5, 0, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
            &[
                0xda, 0, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
        ],
    );
}
#[cfg(feature = "bin32")]
#[test]
fn decode_bin32() {
    test_decode(
        embedded_msgpack::Bytes::new(&[
            0x31u8, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
        ]),
        &[
            &[
                0xc6, 0, 0, 0, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
            &[
                0xdb, 0, 0, 0, 31, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
            ],
        ],
    );
}

#[test]
fn decode_enum() {
    use serde::Deserialize;
    use serde_repr::Deserialize_repr;
    #[allow(clippy::enum_variant_names)]
    #[derive(Deserialize, PartialEq, Eq, Debug)]
    enum Test {
        UnitVariant,
        NewTypeVariant(i32),
        TupleVariant(i32, u8),
        StructVariant { a: i32, b: u8 },
    }
    test_decode(
        Test::UnitVariant,
        &[&[0xAB, 0x55, 0x6E, 0x69, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74]],
    );
    test_decode(
        Test::NewTypeVariant(1),
        &[&[
            0xAE, 0x4E, 0x65, 0x77, 0x54, 0x79, 0x70, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74, 0x91, 0x01,
        ]],
    );
    test_decode(
        Test::TupleVariant(1, 2),
        &[&[
            0xAC, 0x54, 0x75, 0x70, 0x6C, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74, 0x92, 0x01, 0x02,
        ]],
    );
    test_decode(
        Test::StructVariant { a: 1, b: 2 },
        &[&[
            0xAD, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74, 0x82, 0xA1, 0x61, 0x01, 0xA1, 0x62, 0x02,
        ]],
    );
    #[derive(Deserialize_repr, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Test2 {
        Variant1 = 1,
        Variant2 = 2,
    }
    test_decode(Test2::Variant1, &[&[0x01]]);
}

#[cfg(feature = "timestamp")]
#[test]
fn decode_timestamp() {
    use embedded_msgpack::timestamp::Timestamp;
    test_decode(Timestamp::new(1514862245, 0).unwrap(), &[&[0xd6, 0xff, 0x5a, 0x4a, 0xf6, 0xa5]]);
    test_decode(
        Timestamp::new(1514862245, 678901234).unwrap(),
        &[&[0xd7, 0xff, 0xa1, 0xdc, 0xd7, 0xc8, 0x5a, 0x4a, 0xf6, 0xa5]],
    );
    test_decode(
        Timestamp::new(2147483647, 999999999).unwrap(),
        &[&[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xfc, 0x7f, 0xff, 0xff, 0xff]],
    );
    test_decode(Timestamp::new(2147483648, 0).unwrap(), &[&[0xd6, 0xff, 0x80, 0x00, 0x00, 0x00]]);
    test_decode(
        Timestamp::new(2147483648, 1).unwrap(),
        &[&[0xd7, 0xff, 0x00, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x00]],
    );
    test_decode(Timestamp::new(4294967295, 0).unwrap(), &[&[0xd6, 0xff, 0xff, 0xff, 0xff, 0xff]]);
    test_decode(
        Timestamp::new(4294967295, 999999999).unwrap(),
        &[&[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xfc, 0xff, 0xff, 0xff, 0xff]],
    );
    test_decode(
        Timestamp::new(4294967296, 0).unwrap(),
        &[&[0xd7, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]],
    );
    test_decode(
        Timestamp::new(17179869183, 999999999).unwrap(),
        &[&[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xff, 0xff, 0xff, 0xff, 0xff]],
    );
    test_decode(Timestamp::new(0, 0).unwrap(), &[&[0xd6, 0xff, 0x00, 0x00, 0x00, 0x00]]);
    test_decode(
        Timestamp::new(0, 1).unwrap(),
        &[&[0xd7, 0xff, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00]],
    );
    test_decode(Timestamp::new(1, 0).unwrap(), &[&[0xd6, 0xff, 0x00, 0x00, 0x00, 0x01]]);
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(17179869184, 0).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(-1, 0).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(-1, 999999999).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(-2208988801, 999999999).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7c, 0x55, 0x81, 0x7f,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(-2208988800, 0).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x7c, 0x55, 0x81, 0x80,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(-62167219200, 0).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xf1, 0x86, 0x8b, 0x84, 0x00,
        ]],
    );
    #[cfg(feature = "timestamp96")]
    test_decode(
        Timestamp::new(253402300799, 999999999).unwrap(),
        &[&[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0x00, 0x00, 0x00, 0x3a, 0xff, 0xf4, 0x41, 0x7f,
        ]],
    );
}
