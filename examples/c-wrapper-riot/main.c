#include <stdio.h>
#include <string.h>
#include "od.h"
#include "edhoc_rs.h"

#ifdef RUST_PSA
extern void mbedtls_memory_buffer_alloc_init(uint8_t *buf, size_t len);
#endif

static const uint8_t ID_CRED_I[] = "a104412b";
static const uint8_t ID_CRED_R[] = "a104410a";
static const uint8_t CRED_I[] = "A2027734322D35302D33312D46462D45462D33372D33322D333908A101A5010202412B2001215820AC75E9ECE3E50BFC8ED60399889522405C47BF16DF96660A41298CB4307F7EB62258206E5DE611388A4B8A8211334AC7D37ECB52A387D257E6DB3C2A93DF21FF3AFFC8";
static const uint8_t G_I[] = "ac75e9ece3e50bfc8ed60399889522405c47bf16df96660a41298cb4307f7eb6";
static const uint8_t CRED_R[] = "A2026008A101A5010202410A2001215820BBC34960526EA4D32E940CAD2A234148DDC21791A12AFBCBAC93622046DD44F02258204519E257236B2A0CE2023F0931F1F386CA7AFDA64FCDE0108C224C51EABF6072";
static const uint8_t R[] = "72cc4761dbd4c78f758931aa589d348d1ef874a7e303ede2f140dcf3e6aa4aac";
static const uint8_t I[] = "fb13adeb6518cee5f88417660841142e830a81fe334380a953406a1305e8706b";
static const uint8_t G_R[] = "bbc34960526ea4d32e940cad2a234148ddc21791a12afbcbac93622046dd44f0";

int main(void)
 {
    puts("Calling edhoc-rs from C!");

#ifdef RUST_PSA
    // Memory buffer for mbedtls
    uint8_t buffer[4096 * 2] = {0};
    mbedtls_memory_buffer_alloc_init(buffer, 4096 * 2);
#endif

    edhoc_rs_crypto_init();
    puts("Crypto initialized.");

    puts("Begin test: generate key pair.");
    uint8_t out_private_key[32] = {0};
    uint8_t out_public_key[32] = {0};
    p256_generate_key_pair_from_c(out_private_key, out_public_key);
    puts("End test: generate key pair.");
    od_hex_dump(out_private_key, 32, OD_WIDTH_DEFAULT);
    od_hex_dump(out_public_key, 32, OD_WIDTH_DEFAULT);

    puts("Begin test: edhoc handshake.");
    EdhocInitiatorC initiator = initiator_new(I, 32*2, G_R, 32*2, ID_CRED_I, 4*2, CRED_I, 107*2, ID_CRED_R, 4*2, CRED_R, 84*2);
    EdhocResponderC responder = responder_new(R, 32*2, G_I, 32*2, ID_CRED_I, 4*2, CRED_I, 107*2, ID_CRED_R, 4*2, CRED_R, 84*2);

    EdhocMessageBuffer message_1;
    initiator_prepare_message_1(&initiator, &message_1);
    puts("message_1 prepared.");

    responder_process_message_1(&responder, &message_1);
    puts("message_1 processed.");
    EdhocMessageBuffer message_2;
    uint8_t c_r_sent;
    responder_prepare_message_2(&responder, &message_2, &c_r_sent);
    puts("message_2 prepared.");

    uint8_t c_r_received;
    initiator_process_message_2(&initiator, &message_2, &c_r_received);
    puts("message_2 processed.");
    EdhocMessageBuffer message_3;
    uint8_t prk_out_initiator[SHA256_DIGEST_LEN];
    initiator_prepare_message_3(&initiator, &message_3, &prk_out_initiator);
    puts("message_3 prepared.");

    uint8_t prk_out_responder[SHA256_DIGEST_LEN];
    responder_process_message_3(&responder, &message_3, &prk_out_responder);
    puts("message_3 processed.");

    printf("\nprk_out_initiator: \n");
    od_hex_dump(prk_out_initiator, SHA256_DIGEST_LEN, OD_WIDTH_DEFAULT);
    printf("\nprk_out_responder: \n");
    od_hex_dump(prk_out_responder, SHA256_DIGEST_LEN, OD_WIDTH_DEFAULT);

    // Compare prk_out_initiator and prk_out_responder
    if (memcmp(prk_out_initiator, prk_out_responder, SHA256_DIGEST_LEN) != 0) {
        printf("Error: prk_out_initiator and prk_out_responder do not match.\n");
        return 1;
    }

    puts("End test: edhoc handshake.");

    return 0;
}
