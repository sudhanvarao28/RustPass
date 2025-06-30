use aes_gcm::{aead::{consts::{U12}, generic_array::GenericArray, Aead}, Aes256Gcm, Key, KeyInit, Nonce};
use anyhow::Ok;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::{rngs::OsRng, TryRngCore};
use crate::sleddb;



pub fn encrypt(plaintext_pass: &[u8],masterpass: &[u8]) -> Result<Vec<u8>, anyhow::Error>{
    let mut salt= [0u8; 16];
    OsRng.try_fill_bytes(&mut salt)?;

    let mut key_str = [0u8;32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(&masterpass, &salt, &mut key_str).expect("Unable to hash key");

    let mut nonce_byte = [0u8;12];
    OsRng.try_fill_bytes(&mut nonce_byte)?;
    let nonce:&GenericArray<u8,U12> = Nonce::from_slice(&nonce_byte);

    
    let key = Key::<Aes256Gcm>::from_slice(&key_str);
    let cipher = Aes256Gcm::new(&key);
    let cipher_text = cipher.encrypt(nonce, plaintext_pass).expect("Unable to encrypt");

    let mut output = Vec::new();
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_byte);
    output.extend_from_slice(&cipher_text);


    Ok(output)
}

pub fn decrypt(encrypted_text:&[u8], masterpass: &[u8]) -> Result<Vec<u8>, anyhow::Error>{
    if encrypted_text.len() < 16 + 12{
        return Err(anyhow::anyhow!("Encrypted blob to short"));
    }   
    let (salt_bytes, rest) = encrypted_text.split_at(16);
    let (nonce_bytes, ciphertext) = rest.split_at(12);

    let mut key = [0u8;32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(&masterpass, &salt_bytes, &mut key).expect("Unable to create key");

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce: &GenericArray<u8,U12> = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("Unable to Decrypt");

    Ok(plaintext)
}

pub fn store_master_password(password: &[u8]) -> Result<(), anyhow::Error> {
    let mut salt_bytes = [0u8; 16];
    OsRng.try_fill_bytes(&mut salt_bytes)?;
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(password, &salt_bytes, &mut key).expect("Unable to hash password");
    sleddb::insert("salt", &salt_bytes.to_vec())?;
    sleddb::insert("hash", &key.to_vec())?;
    Ok(())
}

pub fn verify_master_password(password: &[u8]) -> Result<bool, anyhow::Error>{
    let mut verified = false;
    let salt = match sleddb::get("salt") {
        Some(n) =>{
            n.to_vec()
        },
        None => {
           return Err(anyhow::anyhow!("Missing Salt"));
        }
    };

    let hash = match sleddb::get("hash") {
        Some(n) =>{
            n.to_vec()
        },
        None => {
           return Err(anyhow::anyhow!("Missing hash"));
        }
    };
    let mut derived_hash = [0u8; 32];
    Argon2::default().hash_password_into(password, &salt, &mut derived_hash).expect("Unable to generate hash");

    if hash == derived_hash.to_vec(){
        verified = true;
    }
    
    Ok(verified)
}

pub fn is_master_password_configured() -> Result<bool, anyhow::Error> {
    let has_salt = sleddb::get("salt").is_some(); 
    let has_hash = sleddb::get("hash").is_some();
    Ok(has_salt && has_hash)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = b"hello";
        let masterpass = b"Shakalaka";
        if let std::result::Result::Ok(encrypted) = encrypt(plaintext, masterpass){
            println!("Encrypted Bytes: {:?}", &encrypted);

            if let std::result::Result::Ok(decrypted) = decrypt(encrypted.as_slice(), masterpass){
                let decrypted_str = std::str::from_utf8(&decrypted).expect("Unable to convert to utf8");
                println!("Decrypted message: {}",decrypted_str );       
                assert_eq!(decrypted_str, std::str::from_utf8(plaintext).unwrap());
            } else {
                println!("Decryption failed");
            }
        } else {
            println!("Encryption failed");
        }
    
    }

    #[test]
    fn test_password_verificaiton()-> Result<(),anyhow::Error>{
        let password = b"Floroma";
        store_master_password(password).expect("unable to store password");
        let verified = verify_master_password(password).expect("unable to verify password");
        assert_eq!(verified,true);

        let verified = verify_master_password(b"Bloromo").expect("unable to verify password");
        assert_eq!(verified,false);

        Ok(())

    }
}