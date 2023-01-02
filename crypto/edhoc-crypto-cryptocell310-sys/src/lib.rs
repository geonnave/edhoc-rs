#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use edhoc_consts::*;
use hacspec_lib::*;

fn convert_array(input: &[u32]) -> [u8; SHA256_DIGEST_LEN] {
    assert!(input.len() == SHA256_DIGEST_LEN / 4);

    let mut output = [0x00u8; SHA256_DIGEST_LEN];
    for i in 0..SHA256_DIGEST_LEN / 4 {
        output[4 * i..4 * i + 4].copy_from_slice(&input[i].to_le_bytes());
    }
    output
}

pub fn sha256_digest(message: &BytesMaxBuffer, message_len: usize) -> BytesHashLen {
    let mut buffer: [u32; 64 / 4] = [0x00; 64 / 4];

    unsafe {
        CRYS_HASH(
            CRYS_HASH_OperationMode_t_CRYS_HASH_SHA256_mode,
            message.to_public_array().as_mut_ptr(),
            message_len,
            buffer.as_mut_ptr(),
        );
    }

    BytesHashLen::from_public_slice(&convert_array(&buffer[0..SHA256_DIGEST_LEN / 4]))
}

pub fn hkdf_expand(
    prk: &BytesHashLen,
    info: &BytesMaxInfoBuffer,
    info_len: usize,
    length: usize,
) -> BytesMaxBuffer {
    BytesMaxBuffer::new()
}

pub fn hkdf_extract(salt: &BytesHashLen, ikm: &BytesP256ElemLen) -> BytesHashLen {
    // Implementation of HKDF-Extract as per RFC 5869

    // TODO generalize if salt is not provided
    let output = hmac_sha256(&mut ikm.to_public_array(), salt.to_public_array());

    output
}

pub fn aes_ccm_encrypt_tag_8(
    key: &BytesCcmKeyLen,
    iv: &BytesCcmIvLen,
    ad: &BytesEncStructureLen,
    plaintext: &BytesPlaintext3,
) -> BytesCiphertext3 {
    let mut output = [0x0u8; CIPHERTEXT_3_LEN];
    let mut tag: CRYS_AESCCM_Mac_Res_t = Default::default();
    let mut aesccm_key: CRYS_AESCCM_Key_t = Default::default();

    aesccm_key[0..AES_CCM_KEY_LEN].copy_from_slice(&key.to_public_array());

    let err = unsafe {
        CC_AESCCM(
            SaSiAesEncryptMode_t_SASI_AES_ENCRYPT,
            aesccm_key.as_mut_ptr(),
            CRYS_AESCCM_KeySize_t_CRYS_AES_Key128BitSize,
            iv.to_public_array().as_mut_ptr(),
            iv.len() as u8,
            ad.to_public_array().as_mut_ptr(),
            ad.len() as u32,
            plaintext.to_public_array().as_mut_ptr(),
            plaintext.len() as u32,
            output.as_mut_ptr(),
            AES_CCM_TAG_LEN as u8, // authentication tag length
            tag.as_mut_ptr(),
            0 as u32, // CCM
        )
    };

    output[CIPHERTEXT_3_LEN - AES_CCM_TAG_LEN..].copy_from_slice(&tag[..AES_CCM_TAG_LEN]);

    BytesCiphertext3::from_public_slice(&output)
}

pub fn aes_ccm_decrypt_tag_8(
    key: &BytesCcmKeyLen,
    iv: &BytesCcmIvLen,
    ad: &BytesEncStructureLen,
    ciphertext: &BytesCiphertext3,
) -> (EDHOCError, BytesPlaintext3) {
    let mut output = [0x0u8; PLAINTEXT_3_LEN];
    let mut aesccm_key: CRYS_AESCCM_Key_t = Default::default();

    aesccm_key[0..AES_CCM_KEY_LEN].copy_from_slice(&key.to_public_array());

    let mut err = EDHOCError::MacVerificationFailed;
    let mut plaintext = BytesPlaintext3::new();

    unsafe {
        (err, plaintext) = match CC_AESCCM(
            SaSiAesEncryptMode_t_SASI_AES_DECRYPT,
            aesccm_key.as_mut_ptr(),
            CRYS_AESCCM_KeySize_t_CRYS_AES_Key128BitSize,
            iv.to_public_array().as_mut_ptr(),
            iv.len() as u8,
            ad.to_public_array().as_mut_ptr(),
            ad.len() as u32,
            ciphertext.to_public_array().as_mut_ptr(),
            (ciphertext.len() - AES_CCM_TAG_LEN) as u32,
            output.as_mut_ptr(),
            AES_CCM_TAG_LEN as u8, // authentication tag length
            ciphertext.to_public_array()[CIPHERTEXT_3_LEN - AES_CCM_TAG_LEN..].as_mut_ptr(),
            0 as u32, // CCM
        ) {
            CRYS_OK => (
                EDHOCError::Success,
                BytesPlaintext3::from_public_slice(&output[..]),
            ),
            _ => (EDHOCError::MacVerificationFailed, BytesPlaintext3::new()),
        };
    }

    (err, plaintext)
}
pub fn p256_ecdh(
    private_key: &BytesP256ElemLen,
    public_key: &BytesP256ElemLen,
) -> BytesP256ElemLen {
    BytesP256ElemLen::new()
}

fn hmac_sha256(message: &mut [u8], mut key: [u8; SHA256_DIGEST_LEN]) -> BytesHashLen {
    let mut buffer: [u32; 64 / 4] = [0x00; 64 / 4];

    unsafe {
        CRYS_HMAC(
            CRYS_HASH_OperationMode_t_CRYS_HASH_SHA256_mode,
            key.as_mut_ptr(),
            key.len() as u16,
            message.as_mut_ptr(),
            message.len(),
            buffer.as_mut_ptr(),
        );
    }

    BytesHashLen::from_public_slice(&convert_array(&buffer[..SHA256_DIGEST_LEN / 4]))
}

pub fn test_hmac_sha256() {
    let mut KEY: [u8; 32] = [0x0b; 32];
    let mut MESSAGE_1: [u8; 0] = [];
    const RESULT_1_TV: [u8; 32] = [
        0x51, 0x77, 0xe6, 0x37, 0xaa, 0xac, 0x0b, 0x50, 0xe5, 0xdc, 0xa8, 0xbb, 0x05, 0xb0, 0xb5,
        0x71, 0x44, 0x4b, 0xd5, 0x9b, 0x9b, 0x0d, 0x83, 0x4d, 0x50, 0x68, 0x1a, 0xf2, 0x1f, 0xc1,
        0x4b, 0x1e,
    ];
    let mut MESSAGE_2: [u8; 1] = [0x0a];
    const RESULT_2_TV: [u8; 32] = [
        0x30, 0x50, 0x86, 0x79, 0x39, 0x85, 0x02, 0xd9, 0xdd, 0x70, 0x7e, 0xff, 0x6c, 0x84, 0x08,
        0x9d, 0x83, 0x12, 0xcc, 0xea, 0x25, 0x36, 0x4d, 0x9c, 0xb8, 0xb0, 0xbd, 0x94, 0xd0, 0xe6,
        0x55, 0xa3,
    ];

    let result_1 = hmac_sha256(&mut MESSAGE_1, KEY).to_public_array();
    assert_eq!(result_1, RESULT_1_TV);

    let result_2 = hmac_sha256(&mut MESSAGE_2, KEY).to_public_array();
    assert_eq!(result_2, RESULT_2_TV);
}
