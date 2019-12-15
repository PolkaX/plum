use blake2_rfc::blake2b::{blake2b, Blake2b};
use data_encoding::{Specification, BASE32};
use std::convert::TryInto;

use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Secp256k1Public(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct ActorPublic(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct BlsPublic(pub Vec<u8>);

// SignatureBytes is the length of a BLS signature
const BlsSignatureBytes: u8 = 96;

// PrivateKeyBytes is the length of a BLS private key
const BlsPrivateKeyBytes: u8 = 32;

// PublicKeyBytes is the length of a BLS public key
const BlsPublicKeyBytes: u8 = 48;

// DigestBytes is the length of a BLS message hash/digest
const BlsDigestBytes: u8 = 96;

#[derive(Debug, Clone)]
pub struct Id(u64);

impl TryInto<Address> for Id {
    type Error = ();
    fn try_into(self) -> Result<Address, Self::Error> {
        // TODO check

        let mut v = Vec::new();
        v.push(0u8);
        v.extend_from_slice(&to_varint(self.0));

        Ok(Address::Id(v))
    }
}

impl TryInto<Address> for BlsPublic {
    type Error = ();

    fn try_into(self) -> Result<Address, Self::Error> {
        if self.0.len() != BlsPublicKeyBytes as usize {
            return Err(());
        }

        let mut v = Vec::new();
        v.push(3u8);
        v.extend_from_slice(&self.0);

        Ok(Address::Bls(v))
    }
}

impl TryInto<Address> for ActorPublic {
    type Error = ();
    fn try_into(self) -> Result<Address, Self::Error> {
        let hash = address_hash(&self.0);

        if hash.len() != PAYLOAD_HASH_LENGTH {
            return Err(());
        }

        let mut v = Vec::new();
        v.push(2u8);
        v.extend_from_slice(&hash);

        let mut x = [0u8; PAYLOAD_HASH_LENGTH + 1];
        x.copy_from_slice(&v);

        Ok(Address::Actor(x))
    }
}

impl TryInto<Address> for Secp256k1Public {
    type Error = ();
    fn try_into(self) -> Result<Address, Self::Error> {
        let hash = address_hash(&self.0);

        if hash.len() != PAYLOAD_HASH_LENGTH {
            return Err(());
        }

        let mut v = Vec::new();
        v.push(1u8);
        v.extend_from_slice(&hash);

        let mut x = [0u8; PAYLOAD_HASH_LENGTH + 1];
        x.copy_from_slice(&v);

        Ok(Address::Secp256k1(x))
    }
}

fn to_varint(x: u64) -> Vec<u8> {
    let mut buf = varint::encode::u64_buffer();
    let bytes = varint::encode::u64(x, &mut buf);
    bytes.to_vec()
}

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

// MaxAddressStringLength is the max length of an address encoded as a string
// it include the network prefx, protocol, and bls publickey
const MaxAddressStringLength: u8 = 2 + 84;

const encodeStd: &'static str = "abcdefghijklmnopqrstuvwxyz234567";

// AddressEncoding defines the base32 config used for address encoding and decoding.
// var AddressEncoding = base32.NewEncoding(encodeStd)

#[derive(PartialEq, Eq, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
}

// PayloadHashLength defines the hash length taken over addresses using the Actor and SECP256K1 protocols.
pub const PAYLOAD_HASH_LENGTH: usize = 20;

// ChecksumHashLength defines the hash length used for calculating address checksums.
pub const CHECKSUM_HASH_LENGTH: usize = 4;

pub const BLS_PUBLICKEY_BYTES: usize = 48;

#[derive(Debug, Clone)]
pub enum Address {
    Id(Vec<u8>),
    Secp256k1([u8; PAYLOAD_HASH_LENGTH + 1]),
    Actor([u8; PAYLOAD_HASH_LENGTH + 1]),
    Bls(Vec<u8>),
}

impl Address {
    pub fn protocol(&self) -> u8 {
        match *self {
            Self::Id(_) => 0u8,
            Self::Secp256k1(_) => 1u8,
            Self::Actor(_) => 2u8,
            Self::Bls(_) => 3u8,
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        let c = self.clone();
        match c {
            Self::Id(x) => x[1..].to_vec(),
            Self::Secp256k1(x) => x[1..].to_vec(),
            Self::Actor(x) => x[1..].to_vec(),
            Self::Bls(x) => x[1..].to_vec(),
        }
    }

    pub fn checksum(&self) -> Vec<u8> {
        match self.clone() {
            Self::Secp256k1(addr) => checksum(&addr[..]),
            Self::Actor(addr) => checksum(&addr[..]),
            Self::Bls(addr) => checksum(&addr[..]),
            _ => Vec::new(),
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        let network = "t";
        match self {
            Self::Secp256k1(_) | Self::Actor(_) | Self::Bls(_) => {
                let payload = self.payload();
                let chsm = self.checksum();

                let mut t = Vec::new();
                t.extend_from_slice(&payload);
                t.extend_from_slice(&chsm);

                let protocol = self.protocol();

                format!("{}{}{}", network, protocol, base32_encode(&t))
            }
            Self::Id(_) => {
                let (i, _) = varint::decode::u64(&self.payload()).unwrap();
                format!("{}{}{}", network, self.protocol(), i)
            }
        }
    }
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

const ENCODE_STD: &str = "abcdefghijklmnopqrstuvwxyz234567";

pub fn base32_encode(input: &[u8]) -> String {
    let mut spec = Specification::new();
    spec.symbols.push_str(ENCODE_STD);
    spec.padding = None;
    let encoder = spec.encoding().unwrap();

    encoder.encode(&input)
}

// cbor decode

// cbor encode

#[cfg(test)]
mod tests {
    use super::*;

    use data_encoding::HEXUPPER;
    use std::convert::TryInto;

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
    fn base32_encoding_should_work() {
        let input = [
            253, 29, 15, 77, 252, 215, 233, 154, 252, 185, 154, 131, 38, 183, 220, 69, 157, 50,
            198, 40, 148, 236, 248, 227,
        ];
        let base32_encoded = "7uoq6tp427uzv7fztkbsnn64iwotfrristwpryy";
        assert_eq!(base32_encoded.to_string(), base32_encode(&input));
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
                "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
            ),
            (
                [
                    4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                    125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127,
                    92, 118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57,
                    53, 118, 238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
                ],
                "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
            ),
            (
                [
                    4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58,
                    20, 129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95,
                    41, 133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28,
                    194, 255, 53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
                ],
                "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
            ),
            (
                [
                    4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178,
                    209, 128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4,
                    121, 216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119,
                    186, 118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
                ],
                "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
            ),
            (
                [
                    4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214,
                    2, 37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65,
                    97, 169, 30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251,
                    241, 251, 242, 113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
                ],
                "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
            ),
            (
                [
                    4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149,
                    133, 156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30,
                    186, 188, 95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142,
                    183, 90, 82, 104, 120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
                ],
                "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
            ),
        ];

        for (b, s) in test_cases.into_iter() {
            let addr: crate::address::Address = crate::address::Secp256k1Public(b.to_vec())
                .try_into()
                .unwrap();
            assert_eq!(s.to_string(), addr.to_string());
        }
    }

    #[test]
    fn actor_address_should_work() {
        let test_cases = [
            (
                [
                    118, 18, 129, 144, 205, 240, 104, 209, 65, 128, 68, 172, 192, 62, 11, 103, 129,
                    151, 13, 96,
                ],
                "t24vg6ut43yw2h2jqydgbg2xq7x6f4kub3bg6as6i",
            ),
            (
                [
                    44, 175, 184, 226, 224, 107, 186, 152, 234, 101, 124, 92, 245, 244, 32, 35,
                    170, 35, 232, 142,
                ],
                "t25nml2cfbljvn4goqtclhifepvfnicv6g7mfmmvq",
            ),
            (
                [
                    2, 44, 158, 14, 162, 157, 143, 64, 197, 106, 190, 195, 92, 141, 88, 125, 160,
                    166, 76, 24,
                ],
                "t2nuqrg7vuysaue2pistjjnt3fadsdzvyuatqtfei",
            ),
            (
                [
                    223, 236, 3, 14, 32, 79, 15, 89, 216, 15, 29, 94, 233, 29, 253, 6, 109, 127,
                    99, 189,
                ],
                "t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a",
            ),
            (
                [
                    61, 58, 137, 232, 221, 171, 84, 120, 50, 113, 108, 109, 70, 140, 53, 96, 201,
                    244, 127, 216,
                ],
                "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            ),
        ];

        for (b, s) in test_cases.into_iter() {
            let addr: crate::address::Address =
                crate::address::ActorPublic(b.to_vec()).try_into().unwrap();
            assert_eq!(s.to_string(), addr.to_string());
        }
    }

    #[test]
    fn bls_address() {
        let test_cases = [
		([173, 88, 223, 105, 110, 45, 78, 145, 234, 134, 200, 129, 233, 56,
			186, 78, 168, 27, 57, 94, 18, 121, 123, 132, 185, 207, 49, 75, 149, 70,
			112, 94, 131, 156, 122, 153, 214, 6, 178, 71, 221, 180, 249, 172, 122,
			52, 20, 221],
			"t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a"),
		([179, 41, 79, 10, 46, 41, 224, 198, 110, 188, 35, 93, 47, 237,
			202, 86, 151, 191, 120, 74, 246, 5, 199, 90, 246, 8, 230, 166, 61, 92,
			211, 142, 168, 92, 168, 152, 158, 14, 253, 233, 24, 139, 56, 47,
			147, 114, 70, 13],
			"t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa"),
		([150, 161, 163, 228, 234, 122, 20, 212, 153, 133, 230, 97, 178,
			36, 1, 212, 79, 237, 64, 45, 29, 9, 37, 178, 67, 201, 35, 88, 156,
			15, 188, 126, 50, 205, 4, 226, 158, 215, 141, 21, 211, 125, 58, 170,
			63, 230, 218, 51],
			"t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a"),
		([134, 180, 84, 37, 140, 88, 148, 117, 247, 209, 111, 90, 172, 1,
			138, 121, 246, 193, 22, 157, 32, 252, 51, 146, 29, 216, 181, 206, 28,
			172, 108, 52, 143, 144, 163, 96, 54, 36, 246, 174, 185, 27, 100, 81,
			140, 46, 128, 149],
			"t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a"),
		([167, 114, 107, 3, 128, 34, 247, 90, 56, 70, 23, 88, 83, 96, 206,
			230, 41, 7, 10, 45, 157, 40, 113, 41, 101, 229, 242, 110, 204, 64,
			133, 131, 130, 128, 55, 36, 237, 52, 242, 114, 3, 54, 240, 157, 182,
			49, 240, 116],
			"t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa")];

        for (b, s) in test_cases.into_iter() {
            let addr: crate::address::Address =
                crate::address::BlsPublic(b.to_vec()).try_into().unwrap();
            assert_eq!(s.to_string(), addr.to_string());
        }
    }

    #[test]
    fn id_address() {
        let test_cases = [
            (0, "t00"),
            (1, "t01"),
            (10, "t010"),
            (150, "t0150"),
            (499, "t0499"),
            (1024, "t01024"),
            (1729, "t01729"),
            (999999, "t0999999"),
        ];
        // {math.MaxUint64, fmt.Sprintf("t0%s", strconv.FormatUint(math.MaxUint64, 10))},
        for (b, s) in test_cases.into_iter() {
            let addr: crate::address::Address = crate::address::Id(*b).try_into().unwrap();
            assert_eq!(s.to_string(), addr.to_string());
        }
    }
}
