fn sha1(input: &[u8]) -> Vec<u8> {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let bytes_to_pad_to = 64;

    let mut input = input.to_vec();

    let original_length_in_bits = input.len() * 8;

    input.push(0x80);

    let ml_bytes = input.len() as i32;

    let pad_to_add = 56 - (ml_bytes % bytes_to_pad_to);

    input.resize(input.len() + pad_to_add as usize, 0x00);

    let original_len = ((original_length_in_bits) as u64).to_be_bytes();

    input.extend(original_len);

    // break message into 512-bit chunks

    let mut chunks: Vec<u32> = vec![];

    for i in 0..input.len() / 4 {
        chunks.push(u32::from_be_bytes(
            input[i * 4..(i + 1) * 4].try_into().unwrap(),
        ));
    }

    // for each 512 chunk
    for i in 0..chunks.len() / 16 {
        let ch = &mut chunks[i * 16..(i + 1) * 16]; // 16 words
        let mut ch = ch.to_vec();
        for j in 16..80 {
            ch.push((ch[j - 3] ^ ch[j - 8] ^ ch[j - 14] ^ ch[j - 16]).rotate_left(1));
        }
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for j in 0..80 {
            let f: u32;
            let k: u32;
            if j <= 19 {
                f = (b & c) | ((!b) & d);
                k = 0x5a827999;
            } else if (20 <= j) && (j <= 39) {
                f = b ^ c ^ d;
                k = 0x6ed9eba1;
            } else if (40 <= j) && (j <= 59) {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8f1bbcdc;
            } else {
                f = b ^ c ^ d;
                k = 0xca62c1d6;
            }

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(ch[j]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut res: Vec<u8> = vec![];

    res.extend(h0.to_be_bytes());
    res.extend(h1.to_be_bytes());
    res.extend(h2.to_be_bytes());
    res.extend(h3.to_be_bytes());
    res.extend(h4.to_be_bytes());

    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sha1() {
        let cases = [
            (
                "The quick brown fox jumps over the lazy dog",
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            ),
            (
                "The quick brown fox jumps over the lazy cog",
                "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            ),
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        ];

        for (input, output) in cases.into_iter() {
            let bytes = sha1(input.as_bytes());
            let out: String = bytes
                .iter()
                .map(|b| format!("{:02x}", b).to_string())
                .collect();
            assert_eq!(out, output);
        }
    }
}
