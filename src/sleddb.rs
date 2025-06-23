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