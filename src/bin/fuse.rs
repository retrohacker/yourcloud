extern crate fuse;
use fuse::*;

extern crate time;
use time::Timespec;

extern crate yourcloud;
use yourcloud::Log;
use yourcloud::Entry;

use std::ffi::OsStr;
use std::path::Path;

struct FileSystem {
    log: Log
}

impl FileSystem {
    fn new(name: &str) -> FileSystem {
        FileSystem {
            log: Log::new(name)
        }
    }
}

impl fuse::Filesystem for FileSystem {
    fn init(&mut self,req: &Request) -> Result<(), i32> {
        println!("init");
        println!("{:?}", req);
        Ok(())
    }
    fn getattr(&mut self,req: &Request,ino: u64,reply: ReplyAttr) {
        println!("getattr");
        let dir = self.log.get(ino).unwrap();
        match dir {
            Entry::Mkdir{ time, id, path } => {
                let t = time.to_timespec();
                return reply.attr(&time::get_time(), &FileAttr {
                    ino: ino,
                    size: 0,
                    blocks: 0,
                    atime: t,
                    mtime: t,
                    ctime: t,
                    crtime: t,
                    kind: FileType::Directory,
                    perm: 0o666,
                    nlink: 0,
                    uid: 0,
                    gid: 0,
                    rdev: 0,
                    flags: 0,
                });
            },
            _ => {
                return reply.error(14);
            }
        }
        return reply.error(2);
    }
    fn access(
        &mut self,
        _req: &Request,
        _ino: u64,
        _mask: u32,
        reply: ReplyEmpty
    ) {
        println!("access");
        reply.ok();
    }
    fn opendir(
        &mut self,
        _req: &Request,
        ino: u64,
        flags: u32,
        reply: ReplyOpen
    ) {
        println!("opendir");
        println!("ino: {}", ino);
        let dir = self.log.get(ino).unwrap();
        println!("{}", dir);
        match dir {
            Entry::Mkdir{ time, id, path } => {
                return reply.opened(0, flags);
            },
            _ => {
                return reply.error(14);
            }
        }
        return reply.error(2);
    }
    fn lookup(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _reply: ReplyEntry
    ) {
        println!("lookup");
    }
    fn destroy(&mut self,_req: &Request) {
        println!("destroy");
    }
    fn forget(&mut self,_req: &Request,_ino: u64,_nlookup: u64) {
        println!("forget");
    }
    fn setattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        _size: Option<u64>,
        _atime: Option<Timespec>,
        _mtime: Option<Timespec>,
        _fh: Option<u64>,
        _crtime: Option<Timespec>,
        _chgtime: Option<Timespec>,
        _bkuptime: Option<Timespec>,
        _flags: Option<u32>,
        _reply: ReplyAttr
    ) {
        println!("setattr");
    }
    fn readlink(&mut self,_req: &Request,_ino: u64,_reply: ReplyData) {
        println!("readlink");
    }
    fn mknod(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _rdev: u32,
        _reply: ReplyEntry
    ) {
        println!("mknod");
    }
    fn mkdir(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _reply: ReplyEntry
    ) {
        println!("mkdir");
    }
    fn unlink(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _reply: ReplyEmpty
    ) {
        println!("unlink");
    }
    fn rmdir(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _reply: ReplyEmpty
    ) {
        println!("rmdir");
    }
    fn symlink(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _link: &Path,
        _reply: ReplyEntry
    ) {
        println!("symlink");
    }
    fn rename(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _newparent: u64,
        _newname: &OsStr,
        _reply: ReplyEmpty
    ) {
        println!("rename");
    }
    fn link(
        &mut self,
        _req: &Request,
        _ino: u64,
        _newparent: u64,
        _newname: &OsStr,
        _reply: ReplyEntry
    ) {
        println!("link");
    }
    fn open(&mut self,_req: &Request,_ino: u64,_flags: u32,_reply: ReplyOpen) {
        println!("open");
    }
    fn read(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _reply: ReplyData
    ) {
        println!("read");
    }
    fn write(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _data: &[u8],
        _flags: u32,
        _reply: ReplyWrite
    ) {
        println!("write");
    }
    fn flush(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _reply: ReplyEmpty
    ) {
        println!("flush");
    }
    fn release(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
        _reply: ReplyEmpty
    ) {
        println!("release");
    }
    fn fsync(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _datasync: bool,
        _reply: ReplyEmpty
    ) {
        println!("fsync");
    }
    fn readdir(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _reply: ReplyDirectory
    ) {
        println!("readdir");
    }
    fn releasedir(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _reply: ReplyEmpty
    ) {
        println!("releasedir");
    }
    fn fsyncdir(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _datasync: bool,
        _reply: ReplyEmpty
    ) {
        println!("fsyncdir");
    }
    fn statfs(&mut self,_req: &Request,_ino: u64,_reply: ReplyStatfs) {
        println!("statfs");
    }
    fn setxattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _name: &OsStr,
        _value: &[u8],
        _flags: u32,
        _position: u32,
        _reply: ReplyEmpty
    ) {
        println!("setxattr");
    }
    fn getxattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _name: &OsStr,
        _size: u32,
        _reply: ReplyXattr
    ) {
        println!("getxattr");
    }
    fn listxattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _size: u32,
        _reply: ReplyXattr
    ) {
        println!("listxattr");
    }
    fn removexattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _name: &OsStr,
        _reply: ReplyEmpty
    ) {
        println!("removexattr");
    }
    fn create(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _flags: u32,
        _reply: ReplyCreate
    ) {
        println!("create");
    }
    fn getlk(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: u32,
        _pid: u32,
        _reply: ReplyLock
    ) {
        println!("getlk");
    }
    fn setlk(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: u32,
        _pid: u32,
        _sleep: bool,
        _reply: ReplyEmpty
    ) {
        println!("setlk");
    }
    fn bmap(
        &mut self,
        _req: &Request,
        _ino: u64,
        _blocksize: u32,
        _idx: u64,
        _reply: ReplyBmap
    ) {
        println!("bmap");
    }
}

fn main() {
    let fs = FileSystem::new("Hello World!");
    let mp = Path::new("./foobar");
    fuse::mount(fs,&mp,&[]);
}
