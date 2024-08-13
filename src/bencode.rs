use core::str;
use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum BencodeType {
    Int(i32),
    Str(Vec<u8>),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BencodeError {
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
pub struct Bencode<'a> {
    pub node: BencodeType,
    len: usize,
    pub slice: &'a [u8],
}

impl<'a> Bencode<'a> {
    pub fn from_u8(bencoded_input: &'a [u8]) -> Result<Self, BencodeError> {
        let first_char = bencoded_input.first().unwrap();
        let first_char = *first_char as char;
        let res = match first_char {
            'i' => Bencode::parse_int(bencoded_input),
            'l' => Bencode::parse_list(bencoded_input),
            'd' => Bencode::parse_dict(bencoded_input),
            _ if first_char.is_numeric() => Bencode::parse_str(bencoded_input),
            _ => Err(BencodeError::new(
                "bencoded_input should be a properly formatted BEncode string",
            )),
        };

        res
    }

    // pub fn from_str(bencoded_input: &str) -> Result<Self, BencodeError> {
    //     unimplemented!();
    //     let first_char = bencoded_input.chars().nth(0).unwrap();
    //     match first_char {
    //         'i' => Bencode::parse_int(bencoded_input),
    //         'l' => Bencode::parse_list(bencoded_input),
    //         'd' => Bencode::parse_dict(bencoded_input),
    //         _ if first_char.is_numeric() => Bencode::parse_str(bencoded_input),
    //         _ => Err(BencodeError::new(
    //             "bencoded_input should be a properly formatted BEncode string",
    //         )),
    //     }
    // }

    // pub fn from_file(file_path: &Path) -> Result<Self, BencodeError> {
    //     let contents = if let Ok(str) = fs::read(file_path) {
    //         str
    //     } else {
    //         return Err(BencodeError::new("Should have been able to read the file"));
    //     };
    //
    //     Bencode::from_u8(&contents)
    // }

    fn parse_int(bencoded_str: &'a [u8]) -> Result<Self, BencodeError> {
        let mut parts = bencoded_str[1..].split(|b| *b as char == 'e');
        // TODO
        let part = str::from_utf8(parts.next().unwrap()).unwrap();
        let len = part.len() + 2;

        match part.parse::<i32>() {
            Ok(int) => Ok(Bencode {
                node: BencodeType::Int(int),
                slice: &bencoded_str[..len],
                len,
            }),
            Err(_) => Err(BencodeError::new(
                "part between 'i' and 'e' should be a properly formatted integer",
            )),
        }
    }

    fn parse_str(bencoded_str: &'a [u8]) -> Result<Self, BencodeError> {
        let first_part = if let Some(first_part) = bencoded_str.split(|b| *b as char == ':').next()
        {
            first_part
        } else {
            return Err(BencodeError::new("string element should contain a ':'"));
        };

        let first_part = if let Ok(first_part) = str::from_utf8(first_part) {
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
        let len = first_part.len() + 1 + len_second_part;

        Ok(Bencode {
            node: BencodeType::Str(str.to_vec()),
            len,
            slice: &bencoded_str[..len],
        })
    }

    fn parse_list(bencoded_str: &'a [u8]) -> Result<Self, BencodeError> {
        let mut chars = bencoded_str.iter();
        let mut result: Vec<BencodeType> = vec![];

        // skip 0th because of the 'l' list start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        let mut chh = chars.nth(1);

        while let Some(ch) = chh {
            if *ch as char == 'e' {
                break;
            }
            let b = Bencode::from_u8(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[b.len..];
            result.push(b.node);
            chh = chars.nth(b.len - 1);
        }
        let len = bencoded_str.len() - remaining_bencoded_str.len() + 1;

        Ok(Bencode {
            node: BencodeType::List(result),
            len,
            slice: &bencoded_str[..len],
        })
    }

    fn parse_dict(bencoded_str: &'a [u8]) -> Result<Self, BencodeError> {
        let mut chars = bencoded_str.iter();
        let mut result: HashMap<String, BencodeType> = HashMap::new();

        // skip 0th because of the 'd' dict start char
        let mut remaining_bencoded_str = &bencoded_str[1..];

        let mut chh = chars.nth(1);

        while let Some(ch) = chh {
            if *ch as char == 'e' {
                break;
            }

            let key = Bencode::from_u8(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[key.len..];

            let val = Bencode::from_u8(remaining_bencoded_str)?;
            remaining_bencoded_str = &remaining_bencoded_str[val.len..];

            if let BencodeType::Str(str) = key.node {
                result.insert(String::from_utf8(str).unwrap(), val.node);
            }
            chh = chars.nth(key.len + val.len - 1);
        }
        let len = bencoded_str.len() - remaining_bencoded_str.len() + 1;

        Ok(Bencode {
            node: BencodeType::Dict(result),
            // 1 is the ':' delimiter
            len,
            slice: &bencoded_str[..len],
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

#[cfg(test)]
mod test {
    use crate::sha1::sha1;

    use super::*;
    #[test]
    fn test_parse_int() {
        let b = Bencode::from_u8("i42e".as_bytes()).unwrap();
        let n = b.node;
        assert!(matches!(n, BencodeType::Int(_)));
        if let BencodeType::Int(int) = n {
            assert_eq!(int, 42);
            assert_eq!(b.len, 4);
        }
    }
    #[test]
    fn test_parse_str() {
        let b = Bencode::from_u8("10:spam1spam1".as_bytes()).unwrap();
        let n = b.node;
        assert!(matches!(n, BencodeType::Str(_)));
        if let BencodeType::Str(str) = n {
            assert_eq!(String::from_utf8(str).unwrap(), String::from("spam1spam1"));
            assert_eq!(b.len, 13);
        }
    }
    #[test]
    fn test_parse_list() {
        let b = Bencode::from_u8("li42ei15e5:abcdee".as_bytes()).unwrap();
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
        let b = Bencode::from_u8("d3:bar4:spam3:fooi42ee".as_bytes()).unwrap();
        let n = b.node;

        assert!(matches!(n, BencodeType::Dict(_)));
        assert_eq!(b.len, 22);
        let dict = if let BencodeType::Dict(dict) = n {
            dict
        } else {
            panic!()
        };

        if let BencodeType::Str(str) = dict.get("bar").unwrap() {
            assert_eq!(
                String::from_utf8(str.clone()).unwrap(),
                String::from("spam")
            );
        } else {
            panic!()
        }

        if let BencodeType::Int(int) = dict.get("foo").unwrap() {
            assert_eq!(int, &42);
        } else {
            panic!()
        }
    }
    #[test]
    fn test_parse_list_recursive() {
        let b = Bencode::from_u8("lli42ei15e5:abcdeee".as_bytes()).unwrap();
        let n = b.node;
        assert!(matches!(n, BencodeType::List(_)));
        assert_eq!(b.len, 19);
        let list = if let BencodeType::List(list) = n {
            list
        } else {
            panic!();
        };

        let _inner_list = if let BencodeType::List(list) = list.first().unwrap() {
            list
        } else {
            panic!();
        };
    }
    #[test]
    fn test_parse_dict_recursive() {
        let b = Bencode::from_u8("d1:ad1:ai25eee".as_bytes()).unwrap();
        assert_eq!(b.len, 14);
        println!("------------------------------------------------------------------------------");
        let dict = if let BencodeType::Dict(dict) = b.node {
            println!("{:?}", String::from_utf8(b.slice.to_vec()));
            let sss: String = sha1(b.slice)
                .iter()
                .map(|b| format!("{:02x}", b).to_string())
                .collect();
            println!("{:?}", sss);
            dict
        } else {
            panic!();
        };

        let _inner_dict = if let BencodeType::Dict(dict) = dict.get("a").unwrap() {
            dict
        } else {
            panic!();
        };
    }
    #[test]
    fn test_complex_parse() {
        let _b = Bencode::from_u8(
            "d3:abcld1:ad1:al5:heheh5:hehehd5:heheh5:hehehei25ed2:abi10eeeeeee".as_bytes(),
        )
        .unwrap();
    }

    #[test]
    fn test_from_u8_1() {
        let _b = Bencode::from_u8("lli5eel1:bi1eee".as_bytes()).unwrap();
    }

    #[test]
    fn test_from_u8_2() {
        let _b = Bencode::from_u8("d3:abcli12ei14ee2:aa3:aaae".as_bytes()).unwrap();
    }

    #[test]
    fn test_torrent_1() {
        let _b = Bencode::from_u8("d4:infod5:filesld4:pathl36:Fedora-Budgie-Live-x86_64-38-1.6.isoeed6:lengthi2562eee4:name28:Fedora-Budgie-Live-x86_64-38ee".as_bytes()).unwrap();
    }

    #[test]
    fn test_torrent_2() {
        let _b = Bencode::from_u8("d8:announce46:http://torrent.fedoraproject.org:6969/announce13:creation datei1681726664e4:infod5:filesld6:lengthi1967298560e4:pathl36:Fedora-Budgie-Live-x86_64-38-1.6.isoeed6:lengthi2562e4:pathl35:Fedora-Spins-38-1.6-x86_64-CHECKSUMeee4:name28:Fedora-Budgie-Live-x86_64-3812:piece lengthi262144eee".as_bytes()).unwrap();
    }
}
