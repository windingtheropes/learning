// by default panic will try to clean up before closing 
// abort will force close and the os will be in charnge of cleaning up
// in cargo.toml we add these lines if we want it to abort
//[profile.release]
// panic = 'abort'


fn main() {
    panic!("Crash and burn");

    // panics happen when an unrecoverable error is encountered
    
    let v = vec![1, 2, 3];
    v[99];
}
