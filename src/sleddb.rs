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

pub fn iter_get_passwords(masterpassword: &[u8]) -> Result<Vec<(String, Vec<u8>)>, anyhow::Error> {
    let mut result_vec: Vec<(String, Vec<u8>)> = Vec::new();

    for result in DB.iter() {
        let (key, value) = result?;
        let key_str = String::from_utf8(key.to_vec())?;
        if key_str == "salt" || key_str == "hash" {
            continue;
        }
        let decrypted_value = encrypt_decrypt::decrypt(&value, masterpassword)?;
        result_vec.push((key_str, decrypted_value));
    }

    Ok(result_vec)
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