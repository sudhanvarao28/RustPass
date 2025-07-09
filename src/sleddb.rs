use std::{collections::HashMap, vec};

use aes_gcm::Key;
use once_cell::sync::Lazy;
use sled::{Db, IVec};
use crate::encrypt_decrypt;

static DB: Lazy<Db> = Lazy::new(||{
    sled::open("rustpass_db").expect("Unable to create sled db")
});

pub fn insert(key:&str, value:&Vec<u8>)-> sled::Result<()>{
    let value:Vec<u8> = value.clone();
    DB.insert(key, value)?;
    DB.flush()?;
    Ok(())
}

pub fn remove(key: &str) -> sled::Result<()> {
    DB.remove(key)?;  // Deletes the key if it exists
    DB.flush()?;       // Ensures changes are persisted
    Ok(())
}


pub fn get(key:&str)-> Option<IVec>{
    match DB.get(key) {
        Ok(n) => {
            return n;
        },
        Err(e) => {
            println!("Unable to get DB - Error:{}",e);
            return None;
        }
    }
}

pub fn iter_get_passwords(masterpassword: &[u8]) -> Result<HashMap<String, Vec<u8>>, anyhow::Error>{
    let mut result_map:HashMap<String, Vec<u8>> = HashMap::new();
    for result in DB.iter(){
        let (key,value) = result?;
        let key_str = String::from_utf8(key.to_vec())?;
        if key_str == "salt".to_string() || key_str == "hash".to_string(){
            continue;
        }
        let value_vec = value.to_vec();
        let decrypted_value = encrypt_decrypt::decrypt(&value_vec.as_slice(), masterpassword)?;
        result_map.insert(key_str, decrypted_value.to_vec());
    }
    Ok(result_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_get(){
        let key = "Gmail";
        let value = b"Blueblue";
        let value = encrypt_decrypt::encrypt(value, b"shakalaka").expect("unable to encrypt");
        let _ = insert(key, &value);
        if let Some(sled_value) = get(key){
            println!("Seld_value: {:?}", &sled_value);
            assert_eq!(&sled_value , &value);
        }
    }
}