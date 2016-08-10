mod hashtagfs;

//use hashtagfs::HashtagFS;


fn main() {
    // Syntax: hashtag-fs base_dir mountpoint (same convention as EncFS)
    let argv = env::args_os();

    let base_dir = argv.nth(1).unwrap();
    let mountpoint = argv.nth(2).unwrap();

    // mounts a new HashtagFS
    let fs = hashtagfs::HashtagFS::new(&base_dir);

    fuse::mount(fs, &mountpoint, &[]);
}
