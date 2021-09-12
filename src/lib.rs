fn convert_pad(input: &str) -> (Vec<u32>, u32, u32) {
    let mut output = Vec::new();
    // first, we split the string into chars (treating them as bytes)
    let chars = input.chars().collect::<Vec<_>>();
    for slice in chars.chunks_exact(4) {
        // and stack 4 chars into a single u32 block, in reverse byte order
        output.push(
            (slice[0] as u32)
                + (slice[1] as u32).rotate_left(8)
                + (slice[2] as u32).rotate_left(16)
                + (slice[3] as u32).rotate_left(24),
        );
    }
    // then, we grab the remainder
    let rem = chars.chunks_exact(4).remainder();
    // and stack those into a single u32 in rev byte order as well
    let output_rem = match rem.len() {
        1 => (rem[0] as u32),
        2 => (rem[0] as u32) + (rem[1] as u32).rotate_left(8),
        3 => (rem[0] as u32) + (rem[1] as u32).rotate_left(8) + (rem[2] as u32).rotate_left(16),
        _ => 0,
    };
    // return values are:
    // 1. input as 4-byte chunks
    // 2. remainder chunk
    // 3. number of bytes in input
    (output, output_rem, chars.len() as u32)
}

fn murmur3_hash(key: Vec<u32>, mut rem: u32, len: u32, seed: u32) -> u32 {
    // defining key constants
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    // seed the hash
    let mut hash = seed;

    for &chunk in key.iter() {
        // load chunk into temp register
        let mut k = chunk;

        // twiddle it
        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        // XOR it into the hash
        hash ^= k;

        // fuddle the hash
        hash = hash.rotate_left(R2);
        hash = hash.wrapping_mul(M).wrapping_add(N);
    }

    // twiddle the remainder
    rem = rem.wrapping_mul(C1);
    rem = rem.rotate_left(R1);
    rem = rem.wrapping_mul(C2);

    // XOR it into the hash
    hash ^= rem;

    // XOR the length into the hash
    hash ^= len;

    // final bit of hash dickery
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    // and we're done!
    hash
}

pub fn murmur3(input: &str, seed: u32) -> u32 {
    let triple = convert_pad(input);
    murmur3_hash(triple.0, triple.1, triple.2, seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "The quick brown fox jumps over the lazy dog.";
        let seed = 123;
        assert_eq!(murmur3(input, seed), 4039614496);
        let input = "Dagoth Ur was a hotep, I swear it by Azura.";
        let seed = 6;
        assert_eq!(murmur3(input, seed), 1596804321);
    }
}
