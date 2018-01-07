use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct ParseHexError;

impl fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "string is not a hexidecimal number")
    }
}

impl error::Error for ParseHexError {
    fn description(&self) -> &str {
        "string is not a hexidecimal number"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn hex_to_dec(hex: &str) -> Result<usize, ParseHexError> {
    let mut dec = 0;
    let mut i = 1;
    
    for ch in hex.chars() {
        dec += match ch {
            '0' => 0, 
            '1' => 1,   
            '2' => 2,   
            '3' => 3,   
            '4' => 4,   
            '5' => 5,   
            '6' => 6,   
            '7' => 7,   
            '8' => 8,   
            '9' => 9,            
            'A' | 'a' => 10,
            'B' | 'b' => 11,
            'C' | 'c' => 12,
            'D' | 'd' => 13,
            'E' | 'e' => 14,
            'F' | 'f' => 15,
            _ => return Err(ParseHexError),
        } * 16usize.pow((hex.len() - i) as u32);        
        i += 1;     
    }

    Ok(dec)
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseHexError> {
    let mut hex_digit: String = String::with_capacity(2);
    let mut bytes: Vec<u8> = Vec::with_capacity((hex.len() as f32 / 2f32).ceil() as usize);
    
    for ch in hex.chars() {
        hex_digit.push(ch);

        if hex_digit.len() == 2 {
            let b: u8 = match hex_to_dec(&hex_digit) {
                Ok(num) => num as u8,
                Err(e) => return Err(e),
            };
            bytes.push(b);
            hex_digit.clear();
        }
    }

    if hex_digit.len() == 1 {
        let b: u8 = match hex_to_dec(&hex_digit) {
            Ok(num) => num as u8,
            Err(e) => return Err(e),
        };
        bytes.push(b);
    }

    Ok(bytes)
}

fn bytes_to_base64(bytes: &[u8]) -> Vec<u8> {
    let mut trio: Vec<u8> = Vec::with_capacity(3);
    let mut result: Vec<u8> = Vec::with_capacity(((bytes.len() as f32 / 3f32).ceil() * 4f32) as usize);

    let alphabet = [        
        b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',
        b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',
        b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',
        b'Y',b'Z',b'a',b'b',b'c',b'd',b'e',b'f',
        b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',
        b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',
        b'w',b'x',b'y',b'z',b'0',b'1',b'2',b'3',
        b'4',b'5',b'6',b'7',b'8',b'9',b'+',b'/',
    ];
    
    for b in bytes {
        trio.push(*b);

        if trio.len() == 3 {
            result.push(alphabet[((trio[0] & 0b11111100) >> 2) as usize]);
            result.push(alphabet[(((trio[0] & 0b00000011) << 4) | ((trio[1] & 0b11110000) >> 4)) as usize]);
            result.push(alphabet[(((trio[1] & 0b00001111) << 2) | ((trio[2] & 0b11000000) >> 6)) as usize]);
            result.push(alphabet[(trio[2] & 0b00111111) as usize]);

            trio.clear();
        }
    }

    if trio.len() == 1 {
        result.push(alphabet[((trio[0] & 0b11111100) >> 2) as usize]);
        result.push(alphabet[((trio[0] & 0b00000011) << 4) as usize]);
        result.push(b'=');
        result.push(b'=');
    }
    else if trio.len() == 2 {
        result.push(alphabet[((trio[0] & 0b11111100) >> 2) as usize]);
        result.push(alphabet[(((trio[0] & 0b00000011) << 4) | ((trio[1] & 0b11110000) >> 4)) as usize]);
        result.push(alphabet[((trio[1] & 0b00001111) << 2) as usize]);
        result.push(b'=');
    }

    return result;
}

fn bytes_to_hex(b: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for byte in b {
        write!(&mut out, "{:x}", byte).expect("Unable to write");
    }
    out
}

fn fixed_xor(bytes: &[u8], mask: &[u8]) -> Vec<u8> {
    if bytes.len() != mask.len() {
        panic!("Input arrays must have the same length");
    }

    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut mask_iter = mask.iter();

    for b in bytes {
        result.push(b ^ mask_iter.next().unwrap());
    }

    result
}

fn main() {
    match hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
        Ok(bytes) =>  {
            let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

            if String::from_utf8(bytes_to_base64(&bytes)).expect("Not UTF-8") == expected {
                println!("Task 1 from set 1: Done");
            } else {
                println!("Task 1 from set 1: Fail");
            }
        },
        Err(e) => println!("Error 1.1: {}", e),
    }

    let expected = "746865206b696420646f6e277420706c6179";
    let bytes = hex_to_bytes("1c0111001f010100061a024b53535009181c").unwrap();
    let mask = hex_to_bytes("686974207468652062756c6c277320657965").unwrap();

    if bytes_to_hex(&fixed_xor(&bytes, &mask)) == expected {
        println!("Task 2 from set 1: Done");
    } else {
        println!("Task 2 from set 1: Fail");
    }
}
