#define ROL64(a, n) (((a) << (n)) | ((a) >> (64 - (n))))

inline ulong bswap64(ulong v) {
    return ((v & 0xff00000000000000UL) >> 56) |
           ((v & 0x00ff000000000000UL) >> 40) |
           ((v & 0x0000ff0000000000UL) >> 24) |
           ((v & 0x000000ff00000000UL) >> 8) |
           ((v & 0x00000000ff000000UL) << 8) |
           ((v & 0x0000000000ff0000UL) << 24) |
           ((v & 0x000000000000ff00UL) << 40) |
           ((v & 0x00000000000000ffUL) << 56);
}

__constant ulong RC[24] = {
    0x0000000000000001UL, 0x0000000000008082UL,
    0x800000000000808aUL, 0x8000000080008000UL,
    0x000000000000808bUL, 0x0000000080000001UL,
    0x8000000080008081UL, 0x8000000000008009UL,
    0x000000000000008aUL, 0x0000000000000088UL,
    0x0000000080008009UL, 0x000000008000000aUL,
    0x000000008000808bUL, 0x800000000000008bUL,
    0x8000000000008089UL, 0x8000000000008003UL,
    0x8000000000008002UL, 0x8000000000000080UL,
    0x000000000000800aUL, 0x800000008000000aUL,
    0x8000000080008081UL, 0x8000000000008080UL,
    0x0000000080000001UL, 0x8000000080008008UL
};

inline void keccak_f1600(ulong *s) {
    for (int r = 0; r < 24; r++) {
        ulong C[5];
        ulong D[5];

        C[0] = s[0] ^ s[5] ^ s[10] ^ s[15] ^ s[20];
        C[1] = s[1] ^ s[6] ^ s[11] ^ s[16] ^ s[21];
        C[2] = s[2] ^ s[7] ^ s[12] ^ s[17] ^ s[22];
        C[3] = s[3] ^ s[8] ^ s[13] ^ s[18] ^ s[23];
        C[4] = s[4] ^ s[9] ^ s[14] ^ s[19] ^ s[24];

        D[0] = C[4] ^ ROL64(C[1], 1);
        D[1] = C[0] ^ ROL64(C[2], 1);
        D[2] = C[1] ^ ROL64(C[3], 1);
        D[3] = C[2] ^ ROL64(C[4], 1);
        D[4] = C[3] ^ ROL64(C[0], 1);

        for (int i = 0; i < 25; i += 5) {
            s[i + 0] ^= D[0];
            s[i + 1] ^= D[1];
            s[i + 2] ^= D[2];
            s[i + 3] ^= D[3];
            s[i + 4] ^= D[4];
        }

        ulong B[25];

        B[0]  = s[0];
        B[10] = ROL64(s[1], 1);
        B[20] = ROL64(s[2], 62);
        B[5]  = ROL64(s[3], 28);
        B[15] = ROL64(s[4], 27);

        B[16] = ROL64(s[5], 36);
        B[1]  = ROL64(s[6], 44);
        B[11] = ROL64(s[7], 6);
        B[21] = ROL64(s[8], 55);
        B[6]  = ROL64(s[9], 20);

        B[7]  = ROL64(s[10], 3);
        B[17] = ROL64(s[11], 10);
        B[2]  = ROL64(s[12], 43);
        B[12] = ROL64(s[13], 25);
        B[22] = ROL64(s[14], 39);

        B[23] = ROL64(s[15], 41);
        B[8]  = ROL64(s[16], 45);
        B[18] = ROL64(s[17], 15);
        B[3]  = ROL64(s[18], 21);
        B[13] = ROL64(s[19], 8);

        B[14] = ROL64(s[20], 18);
        B[24] = ROL64(s[21], 2);
        B[9]  = ROL64(s[22], 61);
        B[19] = ROL64(s[23], 56);
        B[4]  = ROL64(s[24], 14);

        for (int y = 0; y < 5; y++) {
            int o = y * 5;
            ulong a0 = B[o + 0];
            ulong a1 = B[o + 1];
            ulong a2 = B[o + 2];
            ulong a3 = B[o + 3];
            ulong a4 = B[o + 4];

            s[o + 0] = a0 ^ ((~a1) & a2);
            s[o + 1] = a1 ^ ((~a2) & a3);
            s[o + 2] = a2 ^ ((~a3) & a4);
            s[o + 3] = a3 ^ ((~a4) & a0);
            s[o + 4] = a4 ^ ((~a0) & a1);
        }

        s[0] ^= RC[r];
    }
}

__kernel void mine_keccak(
    ulong c0,
    ulong c1,
    ulong c2,
    ulong c3,
    ulong d0,
    ulong d1,
    ulong d2,
    ulong d3,
    ulong nonce_base,
    __global ulong* out_found_nonce,
    __global int* out_found_flag
) {
    ulong nonce = nonce_base + (ulong)get_global_id(0);

    ulong s[25];

    s[0] = c0;
    s[1] = c1;
    s[2] = c2;
    s[3] = c3;

    s[4] = 0;
    s[5] = 0;
    s[6] = 0;
    s[7] = bswap64(nonce);

    s[8] = 0x0000000000000001UL;
    s[9] = 0;
    s[10] = 0;
    s[11] = 0;
    s[12] = 0;
    s[13] = 0;
    s[14] = 0;
    s[15] = 0;
    s[16] = 0x8000000000000000UL;

    s[17] = 0;
    s[18] = 0;
    s[19] = 0;
    s[20] = 0;
    s[21] = 0;
    s[22] = 0;
    s[23] = 0;
    s[24] = 0;

    keccak_f1600(s);

    ulong h0 = bswap64(s[0]);
    if (h0 > d0) return;

    if (h0 == d0) {
        ulong h1 = bswap64(s[1]);
        if (h1 > d1) return;

        if (h1 == d1) {
            ulong h2 = bswap64(s[2]);
            if (h2 > d2) return;

            if (h2 == d2) {
                ulong h3 = bswap64(s[3]);
                if (h3 >= d3) return;
            }
        }
    }

    if (atomic_cmpxchg(out_found_flag, 0, 1) == 0) {
        *out_found_nonce = nonce;
    }
}
