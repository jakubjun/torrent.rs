use std::mem;

fn sha1(input: String) -> Vec<u8> {
    let h0: u32 = 0x67452301;
    let h1: u32 = 0xEFCDAB89;
    let h2: u32 = 0x98BADCFE;
    let h3: u32 = 0x10325476;
    let h4: u32 = 0xC3D2E1F0;

    let bytes_to_pad_to = 64;

    let mut input = input.as_bytes().to_vec();

    let ml_bytes_orig = input.len();

    input.push(0x80);

    let ml_bytes = input.len() as i32;

    let pad_to_add = 56 - (ml_bytes % bytes_to_pad_to);

    for _ in 0..pad_to_add {
        input.push(0x00);
    }

    let original_len = ((ml_bytes * 8) as u64).to_be_bytes();

    input.extend(original_len);

    // break message into 512-bit chunks

    // I copy-pasted this code from StackOverflow without reading the answer
    // surrounding it that told me to write a comment explaining why this code
    // is actually safe for my own use case.
    let chunks = unsafe {
        let ratio = mem::size_of::<u8>() / mem::size_of::<u32>();

        let length = input.len() / 4;
        let capacity = input.capacity() / 4;
        let ptr = input.as_mut_ptr() as *mut u32;

        // Don't run the destructor for vec32
        mem::forget(input);

        // Construct new Vec
        Vec::from_raw_parts(ptr, length, capacity)
    }; // broken into 32 bit chunks ... 16 of these is a 512 bit chunk
       //
       //
    dbg!(chunks.len());

    for i in 0..chunks.len() / 16 {
        // break chunk into sixteen 32-bit big-endian words w[i], 0 ≤ i ≤ 15
        let ch = &chunks[i * 16..(i + 1) * 16]; // 512 bit chunk
    }

    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sha1() {
        sha1(String::from("hello there"));
    }
}
