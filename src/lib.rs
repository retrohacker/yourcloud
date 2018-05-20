extern crate uuid;
extern crate sha3;
extern crate hex;
extern crate time;
use uuid::Uuid;
use sha3::{Digest, Sha3_512};

pub enum Entry {
    Init { uuid: String, name: String },
    Mkdir { time: time::Tm, id: u64, path: String },
    Create { time: time::Tm, id: u64, path: String },
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Entry::Init{ref uuid, ref name} => {
                write!(f, "init {} \"{}\"", uuid, name)
            },
            Entry::Mkdir{ref time, ref id, ref path} => {
                write!(f, "mkdir {} {} \"{}\"",
                       time.rfc3339(), id, path)
            }
            Entry::Create {ref time, ref id, ref path} => {
                write!(f, "create {} {} \"{}\"",
                       time.rfc3339(), id, path)
            }
        }
    }
}

pub struct Log {
    log: Vec<Entry>,
    id_gen: u64,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("");
        let mut hash = format!("");
        let mut index = 0;
        for entry in self.log.iter() {
            let mut line = format!("{}", entry);
            let mut hasher = Sha3_512::default();
            // The first line doesn't have a previous hash!
            if index != 0 {
                line = format!("{}\n{}", line, hash);
            }
            hasher.input(&line.into_bytes());
            hash = hex::encode(hasher.result());
            res = format!("{} {}\n{}", hash, entry, res);
            index = index + 1;
        }
        write!(f, "{}", res)
    }
}

impl Log {
    pub fn new(name: &str) -> Log {
        let uuid = Uuid::new_v4();
        let mut res = Log {
            log: vec!(Entry::Init {
                uuid: format!("{}", uuid),
                name: name.clone().to_string(),
            }),
            id_gen: 1,
        };
        res.mkdir("/");
        res
    }
    pub fn mkdir(&mut self, path: &str) {
        let id = self.id_gen;
        self.id_gen = self.id_gen + 1;
        self.log.push(Entry::Mkdir {
            time: time::now_utc(),
            path: String::from(path),
            id,
        });
    }
    pub fn create(&mut self, path: &str) {
        let id = self.id_gen;
        self.id_gen = self.id_gen + 1;
        self.log.push(Entry::Create {
            time: time::now_utc(),
            path: String::from(path),
            id,
        });
    }
    pub fn get(&self, id: u64) -> Option<Entry> {
        let mut res = None;
        for entry in self.log.iter() {
            match *entry {
                Entry::Mkdir{id: i, ref time, ref path} if i == id => {
                    res = Some(Entry::Mkdir {
                        time: time.clone(),
                        id: id,
                        path: path.clone(),
                    });
                    break;
                },
                _ => ()
            }
        }
        res
    }
}
