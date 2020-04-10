#[cfg(test)]
mod tests {
  static HEX_CHARS: &str = "0123456789abcdef";

  fn hex_to_byte(first: char, second: char) -> u8 {
    let n1 = HEX_CHARS.find(first).unwrap() as u8;
    let n2 = HEX_CHARS.find(second).unwrap() as u8;
    return (n1 << 4) | n2;
  }

  fn read_hex(src: &str) -> Vec<u8> {
    let mut out : Vec<u8> = Vec::new();
    let mut iter = src.chars();
    loop {
      let first = match iter.next() {
        Some(val) => val,
        None => break,
      };
      let second = iter.next().unwrap();
      out.push(hex_to_byte(first, second));
    }
    out
  }

  static B64 : &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

  fn to_base64<T>(data: T) -> String
  where T: IntoIterator<Item=u8> {
    let mut r = String::new();
    let mut iter = data.into_iter();

    loop {
      let b1 = match iter.next() {
        Some(val) => val,
        None => break,
      };
      let b2 = iter.next().unwrap();
      let b3 = iter.next().unwrap();
      let mut val =
        ((b1 as u32) << 16)
        | ((b2 as u32) << 8)
        | ((b3 as u32));
      for _i in 0..4 {
        let v = ((val >> 18) & 0x3F) as usize;
        r.push(B64[v] as char);
        val <<= 6;
      }
    }

    return r;
  }

  fn xor<T>(v1: T, v2: T) -> Vec<u8>
  where T: IntoIterator<Item=u8> {
    let mut i1 = v1.into_iter();
    let mut i2 = v2.into_iter();
    let mut r : Vec<u8> = Vec::new();

    loop {
      let b1 = match i1.next() {
        Some(val) => val,
        None => break,
      };
      let b2 = i2.next().unwrap();
      r.push(b1 ^ b2);
    }
  
    r
  }

  #[test]
  fn test_01_01_hex_to_base64() {
    let src = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let binary = read_hex(src);

    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", to_base64(binary));
  }

  #[test]
  fn test_01_02_fixed_xor() {
    let binary = read_hex("1c0111001f010100061a024b53535009181c");
    let xor_value = read_hex("686974207468652062756c6c277320657965");

    assert_eq!(read_hex("746865206b696420646f6e277420706c6179"), xor(binary, xor_value));
  }

}
