//mod hashtagfs;

extern crate fuse;

use self::fuse::Filesystem;
//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

use std::path::Path;

pub struct HashtagFS {
    pub base_dir: Path,
}

impl HashtagFS {
    pub fn new<P: AsRef<Path>>(base_dir: &P) -> HashtagFS {

    }
}

impl Filesystem for HashtagFS {


}
