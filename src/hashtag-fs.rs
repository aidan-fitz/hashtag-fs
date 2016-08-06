extern crate fuse;

use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

impl Filesystem for HashtagFS {

}
