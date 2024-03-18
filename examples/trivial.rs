use sha2::Digest;
use std::io::Write;

fn find_32_leading_zeros(prefix: &str) -> (u64, [u8; 32]) {
    let mut buf = prefix.as_bytes().to_vec();
    let prefix_len = buf.len();

    for nonce in 0..u64::MAX {
        buf.truncate(prefix_len);
        write!(&mut buf, "{}", nonce).unwrap();
        let hash = sha2::Sha256::digest(&buf);
        if u32::from_be_bytes(hash.as_slice()[..4].try_into().unwrap()) == 0 {
            let digest = hash.as_slice().try_into().unwrap();
            return (nonce, digest);
        }
    }
    panic!("Trivial hasher didn't find a solution");
}

fn main() {
    let prefix = "Hello, sha256:";
    let (nonce, hash) = find_32_leading_zeros(prefix);
    println!("SHA256({}{}) = {}", prefix, nonce, hex::encode(hash));
    // SHA256(Hello, sha256:4066804119) = 000000005436e20c3518f6a9d6c9c82445d5905c7ffca51b8ed6e31c62a3c448
}
