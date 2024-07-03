use std::collections::HashMap;

#[derive(Debug)]
enum BencodeType {
    Int(i32),
    Str(Vec<u8>),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
}

#[derive(Debug)]
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
            len,
        }
    }

    fn parse_str(bencoded_str: &str) -> Bencode {
        let first_part = bencoded_str.split(":").next().unwrap();
        let len_first_part = first_part.len();
        let len_second_part = first_part.parse::<usize>().unwrap();
        let str = &bencoded_str[len_first_part + 1..len_first_part + 1 + len_second_part];
        Bencode {
            node: BencodeType::Str(str.as_bytes().into()),
            len: len_first_part + 1 + len_second_part,
        }
    }

    fn parse_list(bencoded_str: &str) -> Bencode {
        let mut chars = bencoded_str.chars();
        let mut result: Vec<BencodeType> = vec![];

        // skip 0th because of the 'l' list start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        chars.next();

        while let Some(ch) = chars.next() {
            if ch == 'e' {
                break;
            }
            let b = Bencode::from_str(remaining_bencoded_str);
            remaining_bencoded_str = &remaining_bencoded_str[b.len..];
            result.push(b.node);
            chars.nth(b.len);
        }

        Bencode {
            node: BencodeType::List(result),
            len: bencoded_str.len() - remaining_bencoded_str.len() + 1,
        }
    }

    fn parse_dict(bencoded_str: &str) -> Bencode {
        let mut chars = bencoded_str.chars();
        let mut result: HashMap<String, BencodeType> = HashMap::new();

        // skip 0th because of the 'd' dict start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        chars.next();

        while let Some(ch) = chars.next() {
            if ch == 'e' {
                break;
            }

            let key = Bencode::from_str(remaining_bencoded_str);
            remaining_bencoded_str = &remaining_bencoded_str[key.len..];

            let val = Bencode::from_str(remaining_bencoded_str);
            remaining_bencoded_str = &remaining_bencoded_str[val.len..];

            if let BencodeType::Str(str) = key.node {
                result.insert(String::from_utf8(str).unwrap(), val.node);
            }
            chars.nth(key.len + val.len);
        }

        Bencode {
            node: BencodeType::Dict(result),
            // 1 is the ':' delimiter
            len: bencoded_str.len() - remaining_bencoded_str.len() + 1,
        }
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
        let b = Bencode::from_str("li42ei15e5:abcdee");
        let n = b.node;
        assert!(matches!(n, BencodeType::List(_)));
        assert_eq!(b.len, 17);
        let list = if let BencodeType::List(list) = n {
            list
        } else {
            panic!();
        };

        let int = if let BencodeType::Int(int) = list.first().unwrap() {
            int
        } else {
            panic!();
        };

        assert_eq!(int, &42);

        let int = if let BencodeType::Int(int) = list.get(1).unwrap() {
            int
        } else {
            panic!();
        };
        assert_eq!(int, &15);

        let str = if let BencodeType::Str(str) = list.get(2).unwrap() {
            str
        } else {
            panic!();
        };
        assert_eq!(
            String::from_utf8(str.clone()).unwrap(),
            String::from("abcde")
        );
    }
    #[test]
    fn test_dict() {
        let b = Bencode::from_str("d3:bar4:spam3:fooi42ee");
        let n = b.node;

        assert!(matches!(n, BencodeType::Dict(_)));
        assert_eq!(b.len, 22);
        if let BencodeType::Dict(dict) = n {
            let bar = dict.get("bar").unwrap();
            let foo = dict.get("foo").unwrap();

            if let BencodeType::Str(str) = bar {
                assert_eq!(
                    String::from_utf8(str.clone()).unwrap(),
                    String::from("spam")
                );
            } else {
                panic!()
            }
        }
    }
}
