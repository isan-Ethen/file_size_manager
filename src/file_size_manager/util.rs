use sha2::{Digest, Sha256};

pub fn get_content_hash<P: AsRef<[u8]>>(content: P) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content.as_ref());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
