use std::collections::HashMap;

#[derive(Debug)]
enum BencodeType {
    Int(i32),
    Str(Vec<u8>),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
}

#[derive(Debug)]
struct BencodeError {
    message: String,
}

impl BencodeError {
    fn new(msg: &str) -> Self {
        BencodeError {
            message: String::from(msg),
        }
    }
}

#[derive(Debug)]
struct Bencode {
    node: BencodeType,
    len: usize,
}

impl Bencode {
    fn from_str(bencoded_input: &str) -> Result<Self, BencodeError> {
        let first_char = bencoded_input.chars().nth(0).unwrap();
        match first_char {
            'i' => Bencode::parse_int(bencoded_input),
            'l' => Bencode::parse_list(bencoded_input),
            'd' => Bencode::parse_dict(bencoded_input),
            _ if first_char.is_numeric() => Bencode::parse_str(bencoded_input),
            _ => Err(BencodeError::new(
                "bencoded_input should be a properly formatted BEncode string",
            )),
        }
    }

    fn from_file(path: &str) -> Result<Self, BencodeError> {
        todo!()
    }

    fn parse_int(bencoded_str: &str) -> Result<Self, BencodeError> {
        let mut parts = bencoded_str[1..].split('e');
        let part = parts.next().unwrap();
        let len = part.len() + 2;

        match part.parse::<i32>() {
            Ok(int) => Ok(Bencode {
                node: BencodeType::Int(int),
                len,
            }),
            Err(_) => Err(BencodeError::new(
                "part between 'i' and 'e' should be a properly formatted integer",
            )),
        }
    }

    fn parse_str(bencoded_str: &str) -> Result<Self, BencodeError> {
        let first_part = if let Some(first_part) = bencoded_str.split(':').next() {
            first_part
        } else {
            return Err(BencodeError::new("string element should contain a ':'"));
        };

        let len_second_part = if let Ok(len_second_part) = first_part.parse::<usize>() {
            len_second_part
        } else {
            return Err(BencodeError::new(
                "the part before ':' should be a properly formatted integer",
            ));
        };

        let str = &bencoded_str[first_part.len() + 1..first_part.len() + 1 + len_second_part];

        Ok(Bencode {
            node: BencodeType::Str(str.as_bytes().into()),
            len: first_part.len() + 1 + len_second_part,
        })
    }

    fn parse_list(bencoded_str: &str) -> Result<Self, BencodeError> {
        let mut chars = bencoded_str.chars();
        let mut result: Vec<BencodeType> = vec![];

        // skip 0th because of the 'l' list start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        chars.next();

        while let Some(ch) = chars.next() {
            if ch == 'e' {
                break;
            }
            let b = Bencode::from_str(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[b.len..];
            result.push(b.node);
            chars.nth(b.len);
        }

        Ok(Bencode {
            node: BencodeType::List(result),
            len: bencoded_str.len() - remaining_bencoded_str.len() + 1,
        })
    }

    fn parse_dict(bencoded_str: &str) -> Result<Self, BencodeError> {
        let mut chars = bencoded_str.chars();
        let mut result: HashMap<String, BencodeType> = HashMap::new();

        // skip 0th because of the 'd' dict start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        chars.next();

        while let Some(ch) = chars.next() {
            if ch == 'e' {
                break;
            }

            let key = Bencode::from_str(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[key.len..];

            let val = Bencode::from_str(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[val.len..];

            if let BencodeType::Str(str) = key.node {
                result.insert(String::from_utf8(str).unwrap(), val.node);
            }
            chars.nth(key.len + val.len);
        }

        Ok(Bencode {
            node: BencodeType::Dict(result),
            // 1 is the ':' delimiter
            len: bencoded_str.len() - remaining_bencoded_str.len() + 1,
        })
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
    fn test_parse_int() {
        let b = Bencode::from_str("i42e").unwrap();
        let n = b.node;
        assert!(matches!(n, BencodeType::Int(_)));
        if let BencodeType::Int(int) = n {
            assert_eq!(int, 42);
            assert_eq!(b.len, 4);
        }
    }
    #[test]
    fn test_parse_str() {
        let b = Bencode::from_str("10:spam1spam1").unwrap();
        let n = b.node;
        assert!(matches!(n, BencodeType::Str(_)));
        if let BencodeType::Str(str) = n {
            assert_eq!(String::from_utf8(str).unwrap(), String::from("spam1spam1"));
            assert_eq!(b.len, 13);
        }
    }
    #[test]
    fn test_parse_list() {
        let b = Bencode::from_str("li42ei15e5:abcdee").unwrap();
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
    fn test_parse_dict() {
        let b = Bencode::from_str("d3:bar4:spam3:fooi42ee").unwrap();
        let n = b.node;

        assert!(matches!(n, BencodeType::Dict(_)));
        assert_eq!(b.len, 22);
        let dict = if let BencodeType::Dict(dict) = n {
            dict
        } else {
            panic!()
        };

        let bar = dict.get("bar").unwrap();

        if let BencodeType::Str(str) = bar {
            assert_eq!(
                String::from_utf8(str.clone()).unwrap(),
                String::from("spam")
            );
        } else {
            panic!()
        }

        let foo = dict.get("foo").unwrap();

        if let BencodeType::Int(int) = foo {
            assert_eq!(int, &42);
        } else {
            panic!()
        }
    }
}
