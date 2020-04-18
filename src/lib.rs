use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum IniData {
    Value(String),
    Section(IniDataMap)
}

enum State {
    Comment,
    SectionName,
    Key,
    Value
}

#[derive(Clone, Debug, PartialEq)]
struct IniDataMap {
    map: HashMap<String, IniData>,
}

impl IniDataMap {
    pub fn from_bytes(bytes: Vec<u8>) -> IniDataMap {
        let mut map = HashMap::new();
        let mut mapbuf = HashMap::new();
        let mut sbuf = String::new();
        let mut kbuf = String::new();
        let mut vbuf = String::new();
        let mut state = State::Key;
        for i in 0..bytes.len() {
            match bytes[i] as char {
                ';' => {
                    state = State::Comment;
                },
                '[' => {
                    if sbuf.len() > 0 {
                        map.insert(
                            sbuf,
                            IniData::Section(
                                IniDataMap {
                                    map: mapbuf.clone()
                                }
                            )
                        );
                        mapbuf = HashMap::new();
                        sbuf = String::new();
                    }
                    state = State::SectionName;
                },
                ']' => {
                    state = State::Key;
                },
                '=' => {
                    state = State::Value;
                },
                '\n' => {
                    if let State::Key = state {
                        continue;
                    }else if sbuf.len()>0 {
                        mapbuf.insert(kbuf.clone(), IniData::Value(vbuf.clone()));
                    }else{
                        map.insert(kbuf.clone(), IniData::Value(vbuf.clone()));
                    }
                    kbuf = String::new();
                    vbuf = String::new();
                    state = State::Key;
                },
                _ => {
                    match state {
                        State::Comment => {},
                        State::SectionName => {
                            sbuf.push(bytes[i] as char);
                        },
                        State::Key => {
                            kbuf.push(bytes[i] as char);
                        },
                        State::Value => {
                            vbuf.push(bytes[i] as char);
                        },
                    }
                }
            }
        }
        if sbuf.len() > 0 {
            map.insert(
                sbuf,
                IniData::Section(
                    IniDataMap {
                        map: mapbuf.clone()
                    }
                )
            );
        }else if let State::Value = state {
            map.insert(kbuf.clone(), IniData::Value(vbuf.clone()));
        }
        IniDataMap {
            map
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_ini_test() {
        use super::*;
        let mut f = std::fs::File::open("./testcase.ini").unwrap();
        let mut bytes = Vec::new();
        let mut buf = [0; 1024];
        while let Ok(s) = f.read(&mut buf) {
            match s {
                0 => {
                    break;
                },
                1024 => {
                    bytes.append(&mut buf.to_vec());
                },
                s => {
                    bytes.append(&mut buf[0..s].to_vec());
                    break;
                }
            }
        }
        let data = IniDataMap::from_bytes(bytes);
        let mut map = HashMap::new();
        map.insert("foo".to_string(), IniData::Value("bar".to_string()));
        map.insert("spam".to_string(), IniData::Value("ham".to_string()));
        map.insert("heystack".to_string(), IniData::Value("needle".to_string()));
        let mut map_fuga = HashMap::new();
        map_fuga.insert("hoge".to_string(), IniData::Value("piyo".to_string()));
        map_fuga.insert("hogera".to_string(), IniData::Value("hogehoge".to_string()));
        let mut map_toto= HashMap::new();
        map_toto.insert("tete".to_string(), IniData::Value("titi".to_string()));
        map_toto.insert("tata".to_string(), IniData::Value("tutu".to_string()));
        map.insert("fuga".to_string(), IniData::Section(IniDataMap { map: map_fuga }));
        map.insert("toto".to_string(), IniData::Section(IniDataMap { map: map_toto }));
        assert_eq!(data, IniDataMap { map });
    }
}
