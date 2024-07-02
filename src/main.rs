use std::collections::HashMap;

#[derive(Debug)]
enum BencodeType {
    Int(i32),
    Str(Vec<u8>),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
}

struct Bencode {
    node: BencodeType,
    len: usize,
}

impl Bencode {
    fn from_str(bencoded_input: &str) -> Self {
        let first_char = bencoded_input.chars().nth(0).unwrap();
        match first_char {
            'i' => Bencode::parse_int(bencoded_input),
            'l' => Bencode::parse_list(bencoded_input),
            'd' => Bencode::parse_dict(bencoded_input),
            _ if first_char.is_numeric() => Bencode::parse_str(bencoded_input),
            _ => panic!("unsupported bencode type char"),
        }
    }

    fn parse_int(bencoded_str: &str) -> Bencode {
        let mut parts = bencoded_str[1..].split("e");
        let part = parts.next().unwrap();
        let len = part.len() + 2;
        let int = part.parse::<i32>().unwrap();

        Bencode {
            node: BencodeType::Int(int),
            len: len,
        }
    }

    fn parse_str(bencoded_str: &str) -> Bencode {
        let mut parts = bencoded_str.split(":");
        let len = parts.next().unwrap();
        let len_of_len = len.len();
        let len = len.parse::<usize>().unwrap();
        let str = &bencoded_str[len_of_len + 1..len_of_len + 1 + len];
        Bencode {
            node: BencodeType::Str(str.as_bytes().into()),
            len: len + 3,
        }
    }

    fn parse_list(bencoded_str: &str) -> Bencode {
        let mut current_char = bencoded_str.chars().nth(1);
        let mut result: Vec<BencodeType> = vec![];
        let mut len = 1;

        while let Some(ch) = current_char {
            if ch == 'e' {
                break;
            }
            let b = Bencode::from_str(&bencoded_str[1..]);
            result.push(b.node);
            len += b.len;
            current_char = bencoded_str.chars().nth(b.len);
        }

        Bencode {
            node: BencodeType::List(result),
            len: len + 1,
        }
    }

    fn parse_dict(bencoded_str: &str) -> Bencode {
        todo!()
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

// impl<'a> Iterator for BencodeType {
//     type Item = BencodeType;
//
//     fn next(&mut self) -> Option<Self::Item> {}
// }
//
// impl BencodeType {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_int() {
        let b = Bencode::from_str("i42e");
        let n = b.node;
        assert!(matches!(n, BencodeType::Int(_)));
        if let BencodeType::Int(int) = n {
            assert_eq!(int, 42);
            assert_eq!(b.len, 4);
        }
    }
    #[test]
    fn test_str() {
        let b = Bencode::from_str("10:spam1spam1");
        let n = b.node;
        assert!(matches!(n, BencodeType::Str(_)));
        if let BencodeType::Str(str) = n {
            assert_eq!(String::from_utf8(str).unwrap(), String::from("spam1spam1"));
            assert_eq!(b.len, 13);
        }
    }
    #[test]
    fn test_list() {
        let b = Bencode::from_str("li42ee");
        let n = b.node;
        assert!(matches!(n, BencodeType::List(_)));
        assert_eq!(b.len, 6);
        if let BencodeType::List(list) = n {
            if let BencodeType::Int(int) = list.get(0).unwrap() {
                assert_eq!(int, &42);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
    #[test]
    fn test_dict() {
        todo!();
    }
}
