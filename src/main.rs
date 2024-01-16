mod challanges;
use crate::challanges::set_one_challanges;


fn main() {
    //set one challange 1
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let base_out= set_one_challanges::convert_hex_to_base64(hex_string);
    assert_eq!(decoded, base_out);
    println!("{}", base_out);
   
    //set two challange 2
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";
    let result = set_one_challanges::fixed_xor(&hex1, &hex2);
    println!("{}", result);

}




