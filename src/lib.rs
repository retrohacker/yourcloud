extern crate uuid;
extern crate sha3;
extern crate hex;
use uuid::Uuid;
use sha3::{Digest, Sha3_512};

enum Entry {
    Init { uuid: String, name: String }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Entry::Init{ref uuid, ref name} => {
                write!(f, "init {} \"{}\"", uuid, name)
            }
        }
    }
}

pub struct Log {
    log: Vec<Entry>,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("");
        for entry in self.log.iter() {
            let line = format!("{}", entry);
            let mut hasher = Sha3_512::default();
            hasher.input(&line.into_bytes());
            let hash = hex::encode(hasher.result());
            match *entry {
                Entry::Init{..} => {
                    res = format!("{}\n{} {}", res, hash, entry);
                },
            }
        }
        write!(f, "{}", res)
    }
}

impl Log {
    pub fn new(name: &str) -> Log {
        let uuid = Uuid::new_v4();
        Log {
            log: vec!(Entry::Init {
                uuid: format!("{}", uuid),
                name: name.clone().to_string(),
            }),
        }
    }
}
