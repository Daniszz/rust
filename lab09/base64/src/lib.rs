pub fn encode(input: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded = String::new();
    let mut i = 0;

    while i < input.len() {
        let b1 = *input.get(i).unwrap_or(&0);
        let b2 = *input.get(i + 1).unwrap_or(&0);
        let b3 = *input.get(i + 2).unwrap_or(&0);

        let combined: u32 = (u32::from(b1) << 16) | (u32::from(b2) << 8) | u32::from(b3);

        let v1 = (combined >> 18) & 0b111111;
        let v2 = (combined >> 12) & 0b111111;
        let v3 = (combined >> 6) & 0b111111;
        let v4 = combined & 0b111111;

        encoded.push(alphabet[v1 as usize] as char);
        encoded.push(alphabet[v2 as usize] as char);

        encoded.push(if i + 1 < input.len() {
            alphabet[v3 as usize] as char
        } else {
            '='
        });
        encoded.push(if i + 2 < input.len() {
            alphabet[v4 as usize] as char
        } else {
            '='
        });

        i += 3;
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = b"DAA";
        assert_eq!(encode(input), "REFB");
    }

    #[test]
    fn test2() {
        let input = b"nu";
        assert_eq!(encode(input), "bnU=");
    }

    #[test]
    fn test3() {
        let input = b"D";
        assert_eq!(encode(input), "RA==");
    }
}
