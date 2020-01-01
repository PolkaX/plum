// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::*;

#[test]
fn test_random_id_address() {
    let id = rand::random::<u64>();
    let addr = Address::new_id_addr(id).unwrap();
    assert_eq!(addr.protocol(), Protocol::ID);

    let encoded = addr.encode(Network::Test).unwrap();
    let decoded = Address::decode(&encoded).unwrap();
    assert_eq!(decoded, addr);
}

#[test]
fn test_id_address() {
    let test_cases = vec![
        (0, "t00"),
        (1, "t01"),
        (10, "t010"),
        (150, "t0150"),
        (499, "t0499"),
        (1024, "t01024"),
        (1729, "t01729"),
        (999_999, "t0999999"),
        (u64::max_value(), "t018446744073709551615"),
    ];

    for (id, expect) in test_cases {
        let addr = Address::new_id_addr(id).unwrap();
        assert_eq!(addr.protocol(), Protocol::ID);
        assert_eq!(addr.encode(Network::Test).unwrap(), expect);
    }
}

#[test]
fn test_random_secp256k1_address() {
    let seckey = secp256k1::SecretKey::random(&mut rand::rngs::OsRng);
    let pubkey = secp256k1::PublicKey::from_secret_key(&seckey);
    let addr = Address::new_secp256k1_addr(&pubkey.serialize()).unwrap();
    assert_eq!(addr.protocol(), Protocol::SECP256K1);

    let encoded = addr.encode(Network::Test).unwrap();
    let decoded = Address::decode(&encoded).unwrap();
    assert_eq!(decoded, addr);
}

#[test]
fn test_secp256k1_address() {
    let test_cases = vec![
        (
            [
                4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90,
                179, 178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37, 17,
                224, 210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169, 108, 59,
                193, 57, 32, 211, 255, 35, 63, 208, 188, 5,
            ],
            "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
        ),
        (
            [
                4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127, 92,
                118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57, 53, 118,
                238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
            ],
            "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
        ),
        (
            [
                4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58, 20,
                129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95, 41,
                133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28, 194, 255,
                53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
            ],
            "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
        ),
        (
            [
                4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178, 209,
                128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4, 121,
                216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119, 186,
                118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
            ],
            "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
        ),
        (
            [
                4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214, 2,
                37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65, 97, 169,
                30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251, 241, 251, 242,
                113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
            ],
            "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
        ),
        (
            [
                4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149, 133,
                156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30, 186, 188,
                95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142, 183, 90, 82, 104,
                120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
            ],
            "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
        ),
    ];

    for (pubkey, expect) in test_cases {
        let addr = Address::new_secp256k1_addr(&pubkey).unwrap();
        assert_eq!(addr.protocol(), Protocol::SECP256K1);
        assert_eq!(addr.encode(Network::Test).unwrap(), expect);
    }
}

#[test]
fn test_random_actor_address() {
    let data = rand::random::<[u8; 20]>();
    let addr = Address::new_actor_addr(&data).unwrap();
    assert_eq!(addr.protocol(), Protocol::Actor);

    let encoded = addr.encode(Network::Test).unwrap();
    let decoded = Address::decode(&encoded).unwrap();
    assert_eq!(decoded, addr);
}

#[test]
fn test_actor_address() {
    let test_cases = vec![
        (
            [
                118, 18, 129, 144, 205, 240, 104, 209, 65, 128, 68, 172, 192, 62, 11, 103, 129,
                151, 13, 96,
            ],
            "t24vg6ut43yw2h2jqydgbg2xq7x6f4kub3bg6as6i",
        ),
        (
            [
                44, 175, 184, 226, 224, 107, 186, 152, 234, 101, 124, 92, 245, 244, 32, 35, 170,
                35, 232, 142,
            ],
            "t25nml2cfbljvn4goqtclhifepvfnicv6g7mfmmvq",
        ),
        (
            [
                2, 44, 158, 14, 162, 157, 143, 64, 197, 106, 190, 195, 92, 141, 88, 125, 160, 166,
                76, 24,
            ],
            "t2nuqrg7vuysaue2pistjjnt3fadsdzvyuatqtfei",
        ),
        (
            [
                223, 236, 3, 14, 32, 79, 15, 89, 216, 15, 29, 94, 233, 29, 253, 6, 109, 127, 99,
                189,
            ],
            "t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a",
        ),
        (
            [
                61, 58, 137, 232, 221, 171, 84, 120, 50, 113, 108, 109, 70, 140, 53, 96, 201, 244,
                127, 216,
            ],
            "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
        ),
    ];

    for (data, expect) in test_cases {
        let addr = Address::new_actor_addr(&data).unwrap();
        assert_eq!(addr.protocol(), Protocol::Actor);
        assert_eq!(addr.encode(Network::Test).unwrap(), expect);
    }
}

#[test]
fn test_random_bls_address() {
    use bls::Serialize;
    let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
    let pubkey = privkey.public_key();
    let addr = Address::new_bls_addr(&pubkey.as_bytes()).unwrap();
    assert_eq!(addr.protocol(), Protocol::BLS);

    let encoded = addr.encode(Network::Test).unwrap();
    let decoded = Address::decode(&encoded).unwrap();
    assert_eq!(decoded, addr);
}

#[test]
fn test_bls_address() {
    let test_cases = vec![
        (
            [
                173, 88, 223, 105, 110, 45, 78, 145, 234, 134, 200, 129, 233, 56,
                186, 78, 168, 27, 57, 94, 18, 121, 123, 132, 185, 207, 49, 75, 149, 70,
                112, 94, 131, 156, 122, 153, 214, 6, 178, 71, 221, 180, 249, 172, 122,
                52, 20, 221
            ],
            "t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a",
        ),
        (
            [
                179, 41, 79, 10, 46, 41, 224, 198, 110, 188, 35, 93, 47, 237,
                202, 86, 151, 191, 120, 74, 246, 5, 199, 90, 246, 8, 230, 166, 61, 92,
                211, 142, 168, 92, 168, 152, 158, 14, 253, 233, 24, 139, 56, 47,
                147, 114, 70, 13
            ],
            "t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa"
        ),
        (
            [
                150, 161, 163, 228, 234, 122, 20, 212, 153, 133, 230, 97, 178,
                36, 1, 212, 79, 237, 64, 45, 29, 9, 37, 178, 67, 201, 35, 88, 156,
                15, 188, 126, 50, 205, 4, 226, 158, 215, 141, 21, 211, 125, 58, 170,
                63, 230, 218, 51
            ],
            "t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a"
        ),
        (
            [
                134, 180, 84, 37, 140, 88, 148, 117, 247, 209, 111, 90, 172, 1,
                138, 121, 246, 193, 22, 157, 32, 252, 51, 146, 29, 216, 181, 206, 28,
                172, 108, 52, 143, 144, 163, 96, 54, 36, 246, 174, 185, 27, 100, 81,
                140, 46, 128, 149
            ],
            "t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a"
        ),
        (
            [
                167, 114, 107, 3, 128, 34, 247, 90, 56, 70, 23, 88, 83, 96, 206,
                230, 41, 7, 10, 45, 157, 40, 113, 41, 101, 229, 242, 110, 204, 64,
                133, 131, 130, 128, 55, 36, 237, 52, 242, 114, 3, 54, 240, 157, 182,
                49, 240, 116
            ],
            "t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa"
        )
    ];

    for (pubkey, expect) in test_cases {
        let addr = Address::new_bls_addr(&pubkey).unwrap();
        assert_eq!(addr.protocol(), Protocol::BLS);
        assert_eq!(addr.encode(Network::Test).unwrap(), expect);
    }
}

#[test]
fn test_invalid_string_address() {
    let test_cases = vec![
        (
            "Q2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            AddressError::UnknownNetwork,
        ),
        (
            "t4gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            AddressError::UnknownProtocol,
        ),
        (
            "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr24y",
            AddressError::InvalidChecksum,
        ),
        ("t0banananananannnnnnnnn", AddressError::InvalidLength),
        ("t0banananananannnnnnnn", AddressError::InvalidPayload),
        (
            "t2gfvuyh7v2sx3patm1k23wdzmhyhtmqctasbr24y",
            AddressError::Base32Decode(data_encoding::DecodeError {
                position: 16,
                kind: data_encoding::DecodeKind::Symbol,
            }),
        ),
        (
            "t2gfvuyh7v2sx3paTm1k23wdzmhyhtmqctasbr24y",
            AddressError::Base32Decode(data_encoding::DecodeError {
                position: 16,
                kind: data_encoding::DecodeKind::Symbol,
            }),
        ),
        ("t2", AddressError::InvalidLength),
    ];

    for (addr, expect) in test_cases {
        let error = Address::decode(addr).unwrap_err();
        assert_eq!(error, expect);
    }
}

#[test]
fn test_invalid_bytes_address() {
    let test_cases = vec![
        (vec![4, 4, 4], AddressError::UnknownProtocol),
        (vec![0], AddressError::InvalidLength),
        /*
        (
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            AddressError::InvalidPayload,
        ),
        (
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            AddressError::InvalidPayload,
        ),
        (
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            AddressError::InvalidPayload,
        ),
        (
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            AddressError::InvalidPayload,
        ),
        */
        (
            vec![
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            AddressError::InvalidPayload,
        ),
        (
            vec![
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            AddressError::InvalidPayload,
        ),
    ];

    for (addr, expect) in test_cases {
        let error = Address::new_from_bytes(&addr).unwrap_err();
        assert_eq!(error, expect);
    }
}

#[test]
fn test_checksum() {
    let data = "helloworld";
    let checksum = checksum(data.as_bytes());
    assert_eq!(checksum.len(), constant::CHECKSUM_HASH_LEN);
    assert!(validate_checksum(data.as_bytes(), checksum.as_slice()));
    assert!(!validate_checksum(
        b"kittinmittins",
        checksum.as_slice()
    ));
}

/*
const ALL_TEST_ADDRESS: [&str; 24] = [
    "t00",
    "t01",
    "t010",
    "t0150",
    "t0499",
    "t01024",
    "t01729",
    "t0999999",
    "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
    "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
    "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
    "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
    "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
    "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
    "t24vg6ut43yw2h2jqydgbg2xq7x6f4kub3bg6as6i",
    "t25nml2cfbljvn4goqtclhifepvfnicv6g7mfmmvq",
    "t2nuqrg7vuysaue2pistjjnt3fadsdzvyuatqtfei",
    "t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a",
    "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
    "t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a",
    "t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa",
    "t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a",
    "t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a",
    "t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa",
];
*/
