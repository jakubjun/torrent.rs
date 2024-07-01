use std::collections::HashMap;

enum BencodeType {
    Int(i32),
    Str(Vec<u8>),
    List(Vec<Bencode>),
    Dict(HashMap<String, Bencode>),
}

struct Bencode {
    node: BencodeType,
    len: usize,
}

impl Bencode {
    fn new(bencoded_input: &str) -> Option<Self> {
        let first_char = bencoded_input.chars().nth(0)?;
        let (node, len) = match first_char {
            'i' => Bencode::parse_int(bencoded_input),
            'l' => Bencode::parse_list(bencoded_input),
            'd' => Bencode::parse_dict(bencoded_input),
            _ if first_char.is_numeric() => Bencode::parse_str(bencoded_input),
            _ => panic!("unsupported bencode type char"),
        };
        Some(Bencode { node, len })
    }

    fn parse_int(bencoded_str: &str) -> (BencodeType, usize) {
        let mut parts = bencoded_str[1..].split("e");
        let part = parts.next().unwrap();
        let len = part.len() + 2;
        let int = part.parse::<i32>().unwrap();
        (BencodeType::Int(int), len)
    }

    fn parse_str(bencoded_str: &str) -> (BencodeType, usize) {
        let mut parts = bencoded_str.split(":");
        let len = parts.next().unwrap();
        let len_of_len = len.len();
        let len = len.parse::<usize>().unwrap();
        let str = &bencoded_str[len_of_len + 1..len_of_len + 1 + len];
        (BencodeType::Str(str.as_bytes().into()), len)
    }

    fn parse_list(bencoded_str: &str) -> (BencodeType, usize) {
        todo!()
    }

    fn parse_dict(bencoded_str: &str) -> (BencodeType, usize) {
        todo!()
    }

    fn get_node(self) -> BencodeType {
        self.node
    }
}

// struct BencodeIterator<'a> {
//     bencode: &'a Bencode,
//     curr: Option<&'a Bencode>,
// }
//
// impl<'a> Iterator for BencodeIterator<'a> {
//     type Item = &'a Bencode;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.curr
//     }
// }

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_int() {
        let b = Bencode::new("i42e").unwrap();
        assert!(matches!(b.get_node(), BencodeType::Int(int) if int == 42));
    }
    #[test]
    fn test_str() {
        let b = Bencode::new("10:spam1spam1").unwrap();
        assert!(
            matches!(b.get_node(), BencodeType::Str(ref str) if String::from_utf8(str.to_owned()).unwrap() == "spam1spam1")
        );
    }
    #[test]
    fn test_list() {
        todo!();
    }
    #[test]
    fn test_dict() {
        todo!();
    }
}
