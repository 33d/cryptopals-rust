#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

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

  fn freq<T>(data: Vec<T>) -> BTreeMap<T, u32>
  where T: Ord {
    let mut r: BTreeMap<T, u32> = BTreeMap::new();
  
    for c in data {
      r.entry(c)
        .and_modify(|n| *n += 1 )
        .or_insert(1);
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

  #[test]
  fn test_01_03_xor_single() {
    let src = read_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut answer = String::new();

    for x in 0..=255 {
      let xor : Vec<u8> = src.iter().map(|n| n ^ x).collect();
      let freqs : BTreeMap<u8, u32> = freq(xor.clone());
      let mut freq_in_order : Vec<u8> = freqs.keys().cloned().collect();
      // Look for the top 3 letters in some common ones
      freq_in_order.sort_by(|a, b| freqs.get(b).unwrap().partial_cmp(freqs.get(a).unwrap()).unwrap());
      freq_in_order.truncate(3);
      if freq_in_order.iter().all(|x| b" etarsno".contains(x)) {
        answer.insert_str(0, std::str::from_utf8(&xor).unwrap());
        break;
      }
    }

    assert_eq!("Cooking MC\'s like a pound of bacon", answer);
  }

}

