const NUM_CHARS: usize = 64;
const SIXBIT2CHAR: [char; NUM_CHARS] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const PADDING: char = '=';

enum RemainderBits {
    Zero,
    Two,
    Four,
}

pub fn encode(bytes: &[u8]) -> String {
    let encoded_size = (bytes.len() / 3 + usize::from(bytes.len() % 3 > 0)) * 4;
    let mut encoded: Vec<char> = Vec::with_capacity(encoded_size);

    let mut iter = bytes.iter();
    let (mut remainder, mut num_bits) = (0, RemainderBits::Zero);
    while let Some(b) = iter.next() {
        (remainder, num_bits) = match num_bits {
            RemainderBits::Zero => {
                let sixbit = (b & 0b11111100) >> 2;
                encoded.push(SIXBIT2CHAR[usize::from(sixbit)]);
                ((b & 0b00000011) << 4, RemainderBits::Two)
            }
            RemainderBits::Two => {
                let sixbit = remainder | ((b & 0b11110000) >> 4);
                encoded.push(SIXBIT2CHAR[usize::from(sixbit)]);
                ((b & 0b00001111) << 2, RemainderBits::Four)
            }
            RemainderBits::Four => {
                let sixbit = remainder | ((b & 0b11000000) >> 6);
                encoded.push(SIXBIT2CHAR[usize::from(sixbit)]);
                encoded.push(SIXBIT2CHAR[usize::from(b & 0b00111111)]);
                (0, RemainderBits::Zero)
            }
        }
    }
    match num_bits {
        RemainderBits::Zero => (),
        RemainderBits::Two => {
            encoded.push(SIXBIT2CHAR[usize::from(remainder)]);
            encoded.push(PADDING);
            encoded.push(PADDING);
        }
        RemainderBits::Four => {
            encoded.push(SIXBIT2CHAR[usize::from(remainder)]);
            encoded.push(PADDING);
        }
    }

    encoded.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn test_one() {
        let one_byte: [u8; 1] = [0b01001101];
        assert_eq!(encode(&one_byte), "TQ==");
    }

    #[test]
    fn test_two() {
        let two_bytes: [u8; 2] = [0b01001101, 0b01100001];
        assert_eq!(encode(&two_bytes), "TWE=");
    }

    #[test]
    fn test_three() {
        let three_bytes: [u8; 3] = [0b01001101, 0b01100001, 0b01101110];
        assert_eq!(encode(&three_bytes), "TWFu");
    }

    #[test]
    fn test_long() {
        let bytes1: [u8; 9] = [
            0b01100001, 0b01100010, 0b01010000, 0b00111001, 0b01111000, 0b00110100, 0b01000100,
            0b01000110, 0b01101111,
        ];
        let bytes2: [u8; 10] = [
            0b01100001, 0b01100010, 0b01010000, 0b00111001, 0b01111000, 0b00110100, 0b01000100,
            0b01000110, 0b01101111, 0b01110110,
        ];
        let bytes3: [u8; 11] = [
            0b01100001, 0b01100010, 0b01010000, 0b00111001, 0b01111000, 0b00110100, 0b01000100,
            0b01000110, 0b01101111, 0b01110110, 0b01001000,
        ];
        assert_eq!(encode(&bytes1), "YWJQOXg0REZv");
        assert_eq!(encode(&bytes2), "YWJQOXg0REZvdg==");
        assert_eq!(encode(&bytes3), "YWJQOXg0REZvdkg=");
    }
}
