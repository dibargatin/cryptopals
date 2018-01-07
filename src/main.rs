use std::error;
use std::fmt;
use std::collections::HashMap;

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

fn count_symbol_freq(text: &str) -> HashMap<char, f32> {
    let mut symbols: HashMap<char, u32> = HashMap::new();
    let mut total: u32 = 0;

    for ch in text.chars().filter(|c| c.is_alphabetic()) {
        let counter = symbols.entry(ch).or_insert(0);
        *counter += 1;
        total += 1;
    }

    symbols.into_iter().map(|(ch, count)| (ch, count as f32 / total as f32)).collect()
}

fn main() {
    // Task 1 from set 1
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

    // Task 2 from set 1
    let expected = "746865206b696420646f6e277420706c6179";
    let bytes = hex_to_bytes("1c0111001f010100061a024b53535009181c").unwrap();
    let mask = hex_to_bytes("686974207468652062756c6c277320657965").unwrap();

    if bytes_to_hex(&fixed_xor(&bytes, &mask)) == expected {
        println!("Task 2 from set 1: Done");
    } else {
        println!("Task 2 from set 1: Fail");
    }

    // Task 3 from set 1
    let source = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let source_len = source.len();
    let mut decoded: HashMap<char, Vec<u8>> = HashMap::with_capacity(26);

    for key in b'A'..b'Z' {
        let xored = decoded.entry(key as char).or_insert(Vec::with_capacity(source_len));

        for b in &source {
            xored.push(b ^ key);
        }
    }

    let eng_freq: HashMap<char, f32> = [
        ('t', 0.15978),('a', 0.11680),('o', 0.07631),('i', 0.07294),('s', 0.06686),
        ('w', 0.05497),('c', 0.05238),('b', 0.04434),('p', 0.04319),('h', 0.04200),
        ('f', 0.04027),('m', 0.03826),('d', 0.03174),('r', 0.02826),('e', 0.02799),
        ('l', 0.02415),('n', 0.02284),('g', 0.01642),('u', 0.01183),('v', 0.00824),
        ('y', 0.00763),('j', 0.00511),('k', 0.00456),('q', 0.00222),('x', 0.00045),
        ('z', 0.00045),
    ].iter().cloned().collect();

    let distance = |symb_freq: &HashMap<char, f32>| -> f32 {
        let mut result: f32 = 0.0;
        for (ch, freq) in symb_freq {
            result += (freq - eng_freq.get(ch).unwrap()).abs();
        }
        result
    };

    let mut dist = decoded.into_iter().map(|(k,v)| {
            let text = String::from_utf8(v).expect("Not UTF-8");
            (k, distance(&count_symbol_freq(&text.to_lowercase())), text)
        }
    ).collect::<Vec<(char, f32, String)>>();

    use std::cmp::Ordering;
    dist.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    let &(key, _, ref text) = dist.last().unwrap();

    println!("Task 3 from set 1: Done. Key: {}. Message: {}", key, text);
}
