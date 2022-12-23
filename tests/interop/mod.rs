use serde_json;

#[cfg(feature = "timestamp")]
#[test]
fn test_json_timestamp() {
    use embedded_msgpack::timestamp::Timestamp;
    let ts = Timestamp::new(1, 2).unwrap();
    let j = serde_json::to_string(&ts).unwrap();
    assert_eq!(&ts, &serde_json::from_str::<Timestamp>(&j).unwrap());
}

#[cfg(feature = "ext")]
#[test]
fn test_json_ext_ser() {
    use embedded_msgpack::Ext;
    let ext = Ext::new(10, &[1, 2, 3, 4]);
    let j = serde_json::to_string(&ext).unwrap();
    assert_eq!(&j, "{\"type\":10,\"data\":[1,2,3,4]}")
}

#[cfg(feature = "ext")]
#[test]
fn test_json_ext_de() {
    use embedded_msgpack::Ext;
    let ext = Ext::new(10, &[b'a', b'b', b'c', b'd']);
    let j = "{\"type\":10,\"data\":\"abcd\"}";
    assert_eq!(&ext, &serde_json::from_str::<Ext>(&j).unwrap());
}

// This test needs at least `alloc`, because serde_json serializes byte arrays as sequences,
// but sequences can only be deserialized into a `Binary` object with allocations
#[cfg(all(feature = "ext", any(feature = "std", feature = "alloc")))]
#[test]
fn test_json_ext_roundtrip() {
    use embedded_msgpack::Ext;
    let ext = Ext::new(10, &[1, 2, 3, 4]);
    let j = serde_json::to_string(&ext).unwrap();
    assert_eq!(&ext, &serde_json::from_str::<Ext>(&j).unwrap());
}
