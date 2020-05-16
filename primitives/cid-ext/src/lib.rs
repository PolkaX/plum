pub mod comm {
    use cid::{
        Cid, Codec, ExtCode, ExtMultihashRef, FilecoinMultihashCode, FilecoinSealedV1,
        FilecoinUnsealedV1,
    };
    use filecoin_proofs::types::Commitment;

    pub const FILECOIN_CODEC_TYPE: Codec = Codec::Raw;

    #[derive(thiserror::Error, Debug)]
    pub enum CommCidErr {
        #[error("not support for this code:{0:?}")]
        NotSupport(FilecoinMultihashCode),
        #[error("invalid multihash code:{0:?}")]
        InvalidHash(ExtCode),
        #[error("receive an unexpect multihash code|expect:{0:?}|get:{0:?}")]
        UnexpectHash(FilecoinMultihashCode, FilecoinMultihashCode),
    }

    pub fn replica_commitment_v1_to_cid(commitment: Commitment) -> Cid {
        commitment_to_cid(commitment, FilecoinMultihashCode::FcSealedV1)
            .expect("`commitment_to_cid` must receive `FcSealedV1`")
    }

    pub fn data_commitment_v1_to_cid(commitment: Commitment) -> Cid {
        commitment_to_cid(commitment, FilecoinMultihashCode::FcUnsealedV1)
            .expect("`commitment_to_cid` must receive `FcUnsealedV1`")
    }

    pub fn piece_commitment_v1_to_cid(commitment: Commitment) -> Cid {
        data_commitment_v1_to_cid(commitment)
    }

    pub fn commitment_to_cid(
        commitment: Commitment,
        code: FilecoinMultihashCode,
    ) -> Result<Cid, CommCidErr> {
        let hash = match code {
            FilecoinMultihashCode::FcUnsealedV1 => FilecoinUnsealedV1::digest(&commitment),
            FilecoinMultihashCode::FcSealedV1 => FilecoinSealedV1::digest(&commitment),
            _ => return Err(CommCidErr::NotSupport(code)),
        };
        Ok(Cid::new_v1(FILECOIN_CODEC_TYPE, hash))
    }

    pub fn cid_to_piece_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
        cid_to_data_commitment_v1(cid)
    }

    pub fn cid_to_data_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
        let hash = cid_to_commitment(cid, FilecoinMultihashCode::FcUnsealedV1)?;
        let mut r = Commitment::default();
        // hash.digest must be 32 bytes, if not panic here.
        r.copy_from_slice(hash.digest());
        Ok(r)
    }

    pub fn cid_to_replica_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
        let hash = cid_to_commitment(cid, FilecoinMultihashCode::FcSealedV1)?;
        let mut r = Commitment::default();
        // hash.digest must be 32 bytes, if not panic here.
        r.copy_from_slice(hash.digest());
        Ok(r)
    }

    fn cid_to_commitment(
        cid: &Cid,
        expect: FilecoinMultihashCode,
    ) -> Result<ExtMultihashRef, CommCidErr> {
        let hash = cid.hash();
        let code = hash.algorithm();
        match code {
            ExtCode::FL(fl_code) => {
                if fl_code != expect {
                    Err(CommCidErr::UnexpectHash(expect, fl_code))
                } else {
                    Ok(hash)
                }
            }
            _ => Err(CommCidErr::InvalidHash(code)),
        }
    }
}
