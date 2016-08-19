//! `crypto_stream_chacha20` (Chacha20)

use ffi::{crypto_stream_chacha20, crypto_stream_chacha20_KEYBYTES,
          crypto_stream_chacha20_NONCEBYTES, crypto_stream_chacha20_xor};

stream_module!(crypto_stream_chacha20,
               crypto_stream_chacha20_xor,
               crypto_stream_chacha20_KEYBYTES,
               crypto_stream_chacha20_NONCEBYTES);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_1() {
        // the fifth test from
        // https://tools.ietf.org/html/draft-agl-tls-chacha20poly1305-04#section-7
        let key = Key([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
                       0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                       0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]);
        let nonce = Nonce([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        let expected =
            [0xf7, 0x98, 0xa1, 0x89, 0xf1, 0x95, 0xe6, 0x69, 0x82, 0x10, 0x5f, 0xfb, 0x64, 0x0b,
             0xb7, 0x75, 0x7f, 0x57, 0x9d, 0xa3, 0x16, 0x02, 0xfc, 0x93, 0xec, 0x01, 0xac, 0x56,
             0xf8, 0x5a, 0xc3, 0xc1, 0x34, 0xa4, 0x54, 0x7b, 0x73, 0x3b, 0x46, 0x41, 0x30, 0x42,
             0xc9, 0x44, 0x00, 0x49, 0x17, 0x69, 0x05, 0xd3, 0xbe, 0x59, 0xea, 0x1c, 0x53, 0xf1,
             0x59, 0x16, 0x15, 0x5c, 0x2b, 0xe8, 0x24, 0x1a, 0x38, 0x00, 0x8b, 0x9a, 0x26, 0xbc,
             0x35, 0x94, 0x1e, 0x24, 0x44, 0x17, 0x7c, 0x8a, 0xde, 0x66, 0x89, 0xde, 0x95, 0x26,
             0x49, 0x86, 0xd9, 0x58, 0x89, 0xfb, 0x60, 0xe8, 0x46, 0x29, 0xc9, 0xbd, 0x9a, 0x5a,
             0xcb, 0x1c, 0xc1, 0x18, 0xbe, 0x56, 0x3e, 0xb9, 0xb3, 0xa4, 0xa4, 0x72, 0xf8, 0x2e,
             0x09, 0xa7, 0xe7, 0x78, 0x49, 0x2b, 0x56, 0x2e, 0xf7, 0x13, 0x0e, 0x88, 0xdf, 0xe0,
             0x31, 0xc7, 0x9d, 0xb9, 0xd4, 0xf7, 0xc7, 0xa8, 0x99, 0x15, 0x1b, 0x9a, 0x47, 0x50,
             0x32, 0xb6, 0x3f, 0xc3, 0x85, 0x24, 0x5f, 0xe0, 0x54, 0xe3, 0xdd, 0x5a, 0x97, 0xa5,
             0xf5, 0x76, 0xfe, 0x06, 0x40, 0x25, 0xd3, 0xce, 0x04, 0x2c, 0x56, 0x6a, 0xb2, 0xc5,
             0x07, 0xb1, 0x38, 0xdb, 0x85, 0x3e, 0x3d, 0x69, 0x59, 0x66, 0x09, 0x96, 0x54, 0x6c,
             0xc9, 0xc4, 0xa6, 0xea, 0xfd, 0xc7, 0x77, 0xc0, 0x40, 0xd7, 0x0e, 0xaf, 0x46, 0xf7,
             0x6d, 0xad, 0x39, 0x79, 0xe5, 0xc5, 0x36, 0x0c, 0x33, 0x17, 0x16, 0x6a, 0x1c, 0x89,
             0x4c, 0x94, 0xa3, 0x71, 0x87, 0x6a, 0x94, 0xdf, 0x76, 0x28, 0xfe, 0x4e, 0xaa, 0xf2,
             0xcc, 0xb2, 0x7d, 0x5a, 0xaa, 0xe0, 0xad, 0x7a, 0xd0, 0xf9, 0xd4, 0xb6, 0xad, 0x3b,
             0x54, 0x09, 0x87, 0x46, 0xd4, 0x52, 0x4d, 0x38, 0x40, 0x7a, 0x6d, 0xeb, 0x3a, 0xb7,
             0x8f, 0xab, 0x78, 0xc9];
        let output = stream(expected.len(), &nonce, &key);
        assert!(output[..] == expected[..]);
    }
}
