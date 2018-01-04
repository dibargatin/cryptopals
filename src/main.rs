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

fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let mut trio: Vec<u8> = Vec::with_capacity(3);
    let mut result: String = String::with_capacity(((bytes.len() as f32 / 3f32).ceil() * 4f32) as usize);

    let alphabet = [        
        'A','B','C','D','E','F','G','H',
        'I','J','K','L','M','N','O','P',
        'Q','R','S','T','U','V','W','X',
        'Y','Z','a','b','c','d','e','f',
        'g','h','i','j','k','l','m','n',
        'o','p','q','r','s','t','u','v',
        'w','x','y','z','0','1','2','3',
        '4','5','6','7','8','9','+','/',
    ];
    
    for b in bytes {
        trio.push(b);

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
        result.push('=');
        result.push('=');
    }
    else if trio.len() == 2 {
        result.push(alphabet[((trio[0] & 0b11111100) >> 2) as usize]);
        result.push(alphabet[(((trio[0] & 0b00000011) << 4) | ((trio[1] & 0b11110000) >> 4)) as usize]);
        result.push(alphabet[((trio[1] & 0b00001111) << 2) as usize]);
        result.push('=');
    }

    return result;
}

fn main() {
    match hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
        Ok(bytes) =>  {
            if bytes_to_base64(bytes) == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t" {
                println!("Done!");
            } else {
                println!("Fail!");
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
