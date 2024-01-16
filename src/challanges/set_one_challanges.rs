use base64::{engine::general_purpose, Engine as _};
/* 
 Challenge 1: Convert Hex to Base64
 The first challenge is quite simple and straightforward. We will add the `base64` package to our project.
 We create a function `convert_hex_to_base64` that takes a hexadecimal value as a string parameter and returns a string
 encoded as base64.
*/

pub fn convert_hex_to_base64(hex: &str) -> String {
    // The `hex::decode` function takes a hexadecimal string and decodes it into a byte slice.
    // The `unwrap` function call is used to handle potential decoding errors.
    let hex_decode = hex::decode(hex).unwrap();

    // The `general_purpose::STANDARD` enum defines the default encoding alphabet and its corresponding.
    general_purpose::STANDARD.encode(hex_decode)
}


/*
challange 2
 Fixed XOR
 Write a function that takes two equal-length buffers and produces their XOR combination.

 If your function works properly, then when you feed it the string:

 1c0111001f010100061a024b53535009181c
 ... after hex decoding, and when XOR'd against:

 686974207468652062756c6c277320657965
 ... should produce:
 746865206b696420646f6e277420706c6179
*/

fn xor_buffers(buffer1: &[u8], buffer2: &[u8]) -> Vec<u8> {
    buffer1.iter().zip(buffer2.iter()).map(|(&a, &b)| a ^ b).collect()
}
pub fn fixed_xor(hex1: &str, hex2: &str) -> String{
     let buffer1=hex::decode(hex1).unwrap();
     let buffer2=hex::decode(hex2).unwrap();
     let xor_result = xor_buffers(&buffer1, &buffer2);
     xor_result.iter().map(|&byte| format!("{:02x}", byte)).collect()
      
 }
