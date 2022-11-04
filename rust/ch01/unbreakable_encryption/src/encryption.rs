use rand::{thread_rng, RngCore};

pub fn random_key(length: usize) -> Vec<u8> {
  let mut token_bytes: Vec<u8> = vec![0; length];
    
  thread_rng().fill_bytes(&mut token_bytes);

  token_bytes
}

pub fn encrypt(original: &String) -> (Vec<u8>, Vec<u8>) {
  
  let original_bytes: Vec<u8> = original.as_bytes().to_vec();
    
  let dummy: Vec<u8> = random_key(original_bytes.len());
  
  let mut dummy_iterator = dummy.iter();

  let encrypted: Vec<u8> = original_bytes.into_iter().map(|original_byte| original_byte ^ dummy_iterator.next().unwrap()).
  collect();

  (dummy, encrypted)
}

pub fn decrypt(key1: &Vec<u8>, key2: &Vec<u8>) -> String {
  let mut key2_iterator = key2.iter();

  let decrypted: Vec<u8> = key1.into_iter().map(|key1_byte| key1_byte ^ key2_iterator.next().unwrap()).collect();

  String::from_utf8(decrypted).unwrap()
}
