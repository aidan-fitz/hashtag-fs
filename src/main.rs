use hashtag-fs::HashtagFS;

fn main() {
    // Syntax: hashtag-fs base_dir mountpoint
    let argv = env::args_os();

    let base_dir = argv.nth(1).unwrap();
    let mountpoint = argv.nth(2).unwrap();

    fuse::mount(HashtagFS, &mountpoint, &[]);
}
