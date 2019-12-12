// #[derive(PartialEq, Eq, Clone)]
// pub enum Address<AddressId, Secp256k1, Actor, Bls> {
// /// Id represents the address ID protocol.
// Id(AddressId),
// /// Secp256k1 represents the address Secp256k1 protocol.
// Secp256k1(Secp256k1),
// /// Actor represents the address Actor protocl.
// Actor(Actor),
// /// BLS represents the address BLS protocol.
// Bls(Bls),
// Unknown,
// }

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    // Unknown address network
    UnknownNetwork,
    // Unknown address protocol
    UnknownProtocol,
    // Invalid address payload
    InvalidPayload,
    // Invalid address length
    InvalidLength,
    // Invalid address checksum
    InvalidChecksum,
}
// UndefAddressString is the string used to represent an empty address when encoded to a string.
// const UndefAddressString: '&static str = "<empty>";

// PayloadHashLength defines the hash length taken over addresses using the Actor and SECP256K1 protocols.
const PayloadHashLength: u8 = 20;

// ChecksumHashLength defines the hash length used for calculating address checksums.
const ChecksumHashLength: u8 = 4;

// MaxAddressStringLength is the max length of an address encoded as a string
// it include the network prefx, protocol, and bls publickey
const MaxAddressStringLength: u8 = 2 + 84;

const encodeStd: &'static str = "abcdefghijklmnopqrstuvwxyz234567";

// AddressEncoding defines the base32 config used for address encoding and decoding.
// var AddressEncoding = base32.NewEncoding(encodeStd)

use blake2_rfc::blake2b::{blake2b, Blake2b};

pub type NetworkPrefix = String;
#[derive(PartialEq, Eq, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
}

// #[derive(PartialEq, Eq, Clone)]
// pub enum Protocol {
// /// Id represents the address ID protocol.
// Id(&[u8]),
// /// Secp256k1 represents the address Secp256k1 protocol.
// Secp256k1(&[u8]),
// /// Actor represents the address Actor protocl.
// Actor(&[u8]),
// /// BLS represents the address BLS protocol.
// Bls(&[u8]),
// }

#[derive(Default)]
pub struct Address(pub Vec<u8>);

impl From<[u8; 32]> for Address {
    fn from(_addr: [u8; 32]) -> Self {
        Address("".into())
    }
}

// switch protocol {
// case ID:
// _, n, err := varint.FromUvarint(payload)
// if err != nil {
// return Undef, xerrors.Errorf("could not decode: %v: %w", err, ErrInvalidPayload)
// }
// if n != len(payload) {
// return Undef, xerrors.Errorf("different varint length (v:%d != p:%d): %w",
// n, len(payload), ErrInvalidPayload)
// }
// case SECP256K1, Actor:
// if len(payload) != PayloadHashLength {
// return Undef, ErrInvalidPayload
// }
// case BLS:
// if len(payload) != bls.PublicKeyBytes {
// return Undef, ErrInvalidPayload
// }
// default:
// return Undef, ErrUnknownProtocol
// }
// explen := 1 + len(payload)
// buf := make([]byte, explen)

// buf[0] = protocol
// copy(buf[1:], payload)

// return Address{string(buf)}, nil

// pub fn new_address(protocol: Protocol, payload &[u8]) -> std::result::Result<Address, Error> {
// match protocol {
// Protocol::Id => Ok(Default::default()),
// Protocol::Secp256k1 | Protocol::Actor => Ok(Default::default()),
// Protocol::Bls => Ok(Default::default()),
// Protocol::Unknown => Ok(Default::default()),
// }
// Ok(Default::default())
// }

// impl Address {
// pub fn new(addr_ty: AddressType) -> Self {
// match addr_ty {
// AddressType::Id => Address("id".into()),
// AddressType::Secp256k1 => Address("secp256k1".into()),
// AddressType::Actor => Address("actor".into()),
// AddressType::Bls => Address("bls".into()),
// AddressType::Unknown => Address("uknown".into()),
// }
// }
// pub fn payload(&self) {}
// }

pub const PAYLOAD_HASH_LENGTH: usize = 20;
pub const CHECKSUM_HASH_LENGTH: usize = 4;

pub fn new_secp256k1(pubkey: &[u8]) -> Vec<u8> {
    let hash = address_hash(pubkey);
    let mut v = Vec::new();
    v.push(1u8);
    v.extend_from_slice(&hash);
    v
    // println!("hash: {:?}", hash.clone());
    // println!("hash bytes: {:?}", hash.as_bytes());
}

pub fn hash(ingest: &[u8], hash_config: usize) -> Vec<u8> {
    let hash = blake2b(hash_config, &[], ingest);
    hash.as_bytes().to_vec()
}

pub fn address_hash(ingest: &[u8]) -> Vec<u8> {
    hash(ingest, PAYLOAD_HASH_LENGTH)
}

pub fn checksum(ingest: &[u8]) -> Vec<u8> {
    hash(ingest, CHECKSUM_HASH_LENGTH)
}

pub fn encode(network: Network, addr: Address) -> String {
    let ntwk = match network {
        Network::Mainnet => "f",
        Network::Testnet => "t",
    };

    let raw_b: Vec<u8> = addr.0;
    let chsm = checksum(&raw_b);
    let protocol = raw_b[0];
    // let payload = raw_b[1:];
    let addr = format!("{}{}{}", ntwk, protocol, "hello");

    // SECP256K1

    // cksm := Checksum(append([]byte{addr.Protocol()}, addr.Payload()...));
    // strAddr = ntwk + fmt.Sprintf("%d", addr.Protocol()) + AddressEncoding.WithPadding(-1).EncodeToString(append(addr.Payload(), cksm[:]...));

    addr
}

// cbor decode

// cbor encode

#[cfg(test)]
mod tests {
    use super::*;

    use data_encoding::HEXUPPER;

    #[test]
    fn address_hash_should_work() {
        let ingest = [115, 97, 116, 111, 115, 104, 105];
        let hashed = [
            71, 22, 176, 35, 183, 254, 132, 182, 231, 220, 218, 48, 60, 61, 117, 75, 26, 143, 242,
            252,
        ];
        assert_eq!(address_hash(&ingest[..]), hashed.to_vec());
    }

    #[test]
    fn new_secp256k1_address_should_work() {
        let test_cases = [
            (
                [
                    4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90,
                    179, 178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37,
                    17, 224, 210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169,
                    108, 59, 193, 57, 32, 211, 255, 35, 63, 208, 188, 5,
                ],
                b"t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
            ),
            (
                [
                    4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                    125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127,
                    92, 118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57,
                    53, 118, 238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
                ],
                b"t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
            ),
            (
                [
                    4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58,
                    20, 129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95,
                    41, 133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28,
                    194, 255, 53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
                ],
                b"t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
            ),
            (
                [
                    4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178,
                    209, 128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4,
                    121, 216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119,
                    186, 118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
                ],
                b"t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
            ),
            (
                [
                    4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214,
                    2, 37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65,
                    97, 169, 30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251,
                    241, 251, 242, 113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
                ],
                b"t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
            ),
            (
                [
                    4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149,
                    133, 156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30,
                    186, 188, 95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142,
                    183, 90, 82, 104, 120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
                ],
                b"t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
            ),
        ];

        for (b, s) in test_cases.into_iter() {
            let addr = new_secp256k1(b);
            let decoded = HEXUPPER.decode(&s.to_vec());
            println!("decoded: {:?}", decoded);
            println!("b: {:?}, s: {:?}", &b[..], &s[..]);
        }

        // for _, tc := range testCases {
        // tc := tc
        // t.Run(fmt.Sprintf("testing secp256k1 address: %s", tc.expected), func(t *testing.T) {
        // assert := assert.New(t)

        // // Round trip encoding and decoding from string
        // addr, err := NewSecp256k1Address(tc.input)
        // assert.NoError(err)
        // assert.Equal(tc.expected, addr.String())

        // maybeAddr, err := NewFromString(tc.expected)
        // assert.NoError(err)
        // assert.Equal(SECP256K1, maybeAddr.Protocol())
        // assert.Equal(addressHash(tc.input), maybeAddr.Payload())

        // // Round trip to and from bytes
        // maybeAddrBytes, err := NewFromBytes(maybeAddr.Bytes())
        // assert.NoError(err)
        // assert.Equal(maybeAddr, maybeAddrBytes)

        // // Round trip encoding and decoding json
        // b, err := addr.MarshalJSON()
        // assert.NoError(err)

        // var newAddr Address
        // err = newAddr.UnmarshalJSON(b)
        // assert.NoError(err)
        // assert.Equal(addr, newAddr)
        // })
        // }
    }
}
