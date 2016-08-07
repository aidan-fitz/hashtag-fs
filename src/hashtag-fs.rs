extern crate fuse;

use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

pub struct HashtagFS {
    
}

impl Filesystem for HashtagFS {


}
