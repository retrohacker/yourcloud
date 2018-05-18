extern crate uuid;
use uuid::Uuid;

enum Entry {
    Init { uuid: String, name: String }
}

pub struct Log {
    log: Vec<Entry>,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("");
        for entry in self.log.iter() {
            match *entry {
                Entry::Init{ref uuid, ref name} => {
                    res = format!("{}\ninit {} {}", res, uuid, name);
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
