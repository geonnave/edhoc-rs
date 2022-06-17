use hacspec_edhoc::consts::*;
use hacspec_edhoc::*;
use hacspec_lib::*;

use hexlit::hex;

// test vectors (TV)
const X_TV: [u8; P256_ELEM_LEN] =
    hex!("368ec1f69aeb659ba37d5a8d45b21bdc0299dceaa8ef235f3ca42ce3530f9525");
const G_XY_TV: [u8; P256_ELEM_LEN] =
    hex!("2f0cb7e860ba538fbf5c8bded009f6259b4b628fe1eb7dbe9378e5ecf7a824ba");
const PRK_2E_TV: [u8; P256_ELEM_LEN] =
    hex!("fd9eef627487e40390cae922512db5a647c08dc90deb22b72ece6f156ff1c396");
const G_R_TV: [u8; P256_ELEM_LEN] =
    hex!("bbc34960526ea4d32e940cad2a234148ddc21791a12afbcbac93622046dd44f0");
const PRK_3E2M_TV: [u8; P256_ELEM_LEN] =
    hex!("af4b5918682adf4c96fd7305b69f8fb78efc9a230dd21f4c61be7d3c109446b3");
const C_R_TV: [i8; 1] = [-8];
const H_MESSAGE_1_TV: [u8; SHA256_DIGEST_LEN] =
    hex!("ca02cabda5a8902749b42f711050bb4dbd52153e87527594b39f50cdf019888c");
const TH_2_TV: [u8; SHA256_DIGEST_LEN] =
    hex!("9b99cfd7afdcbcc9950a6373507f2a81013319625697e4f9bf7a448fc8e633ca");
const ID_CRED_R_TV: [u8; 3] = hex!("a10432");
const CRED_R_TV : [u8; 94] = hex!("a2026b6578616d706c652e65647508a101a5010202322001215820bbc34960526ea4d32e940cad2a234148ddc21791a12afbcbac93622046dd44f02258204519e257236b2a0ce2023f0931f1f386ca7afda64fcde0108c224c51eabf6072");
const MAC_2_TV: [u8; MAC_LENGTH_2] = hex!("3324d5a4afcd4326");
const PLAINTEXT_2_TV: [u8; PLAINTEXT_2_LEN] = hex!("32483324d5a4afcd4326");
const KEYSTREAM_2_TV: [u8; PLAINTEXT_2_LEN] = hex!("7b86c04af73b50d31b6f");
const I_TV: [u8; P256_ELEM_LEN] =
    hex!("fb13adeb6518cee5f88417660841142e830a81fe334380a953406a1305e8706b");
const EAD_2_TV: [u8; 0] = hex!("");
const CONTEXT_INFO_MAC_2: [u8; 97] = hex!("A10432A2026B6578616D706C652E65647508A101A5010202322001215820BBC34960526EA4D32E940CAD2A234148DDC21791A12AFBCBAC93622046DD44F02258204519E257236B2A0CE2023F0931F1F386CA7AFDA64FCDE0108C224C51EABF6072");
const TH_3_TV: [u8; SHA256_DIGEST_LEN] =
    hex!("426f8f65c17f6210392e9a16d51fe07160a25ac6fda440cfb13ec196231f3624");
const PRK_4X3M_TV: [u8; P256_ELEM_LEN] =
    hex!("4a40f2aca7e1d9dbaf2b276bce75f0ce6d513f75a95af8905f2a14f2493b2477");
const ID_CRED_I_TV: [u8; 3] = hex!("a1042b");
const CRED_I_TV: [u8; 106] = hex!("a2027734322d35302d33312d46462d45462d33372d33322d333908a101a50102022b2001215820ac75e9ece3e50bfc8ed60399889522405c47bf16df96660a41298cb4307f7eb62258206e5de611388a4b8a8211334ac7d37ecb52a387d257e6db3c2a93df21ff3affc8");
const MAC_3_TV: [u8; MAC_LENGTH_3] = hex!("4cd53d74f0a6ed8b");
const CIPHERTEXT_3_TV: [u8; CIPHERTEXT_3_LEN] = hex!("885c63fd0b17f2c3f8f10bc8bf3f470ec8a1");
const MESSAGE_3_TV: [u8; MESSAGE_3_LEN] = hex!("52885c63fd0b17f2c3f8f10bc8bf3f470ec8a1");
const TH_4_TV: [u8; SHA256_DIGEST_LEN] =
    hex!("ba682e7165e9d484bd2ebb031c09da1ea5b82eb332439c4c7ec73c2c239e3450");

#[test]
fn test_encode_message_1() {
    let METHOD_TV = U8(0x03);
    let SUITES_I_TV = ByteSeq::from_hex("0602");
    let G_X_TV = BytesP256ElemLen::from_hex(
        "8af6f430ebe18d34184017a9a11bf511c8dff8f834730b96c1b7c8dbca2fc3b6",
    );
    let C_I_TV: i8 = -24i8;
    let MESSAGE_1_TV = ByteSeq::from_hex(
        "0382060258208af6f430ebe18d34184017a9a11bf511c8dff8f834730b96c1b7c8dbca2fc3b637",
    );

    let message_1 = encode_message_1(METHOD_TV, &SUITES_I_TV, &G_X_TV, C_I_TV);
    assert_eq!(message_1, MESSAGE_1_TV);
}

#[test]
fn test_parse_message_2() {
    let MESSAGE_2_TV = ByteSeq::from_hex("582a419701d7f00a26c2dc587a36dd752549f33763c893422c8ea0f955a13a4ff5d549cef36e229fff1e584927");
    let G_Y_TV = BytesP256ElemLen::from_hex(
        "419701d7f00a26c2dc587a36dd752549f33763c893422c8ea0f955a13a4ff5d5",
    );
    let CIPHERTEXT_2_TV = ByteSeq::from_hex("49cef36e229fff1e5849");

    let (g_y, ciphertext_2, c_r) = parse_message_2(&MESSAGE_2_TV);

    assert_eq!(ByteSeq::from_seq(&G_Y_TV), ByteSeq::from_seq(&g_y));
    assert_eq!(
        ByteSeq::from_seq(&CIPHERTEXT_2_TV),
        ByteSeq::from_seq(&ciphertext_2)
    );
}
