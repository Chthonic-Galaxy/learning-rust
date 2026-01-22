use std::mem::ManuallyDrop;

use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::fd::FromRawFd;

use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main() {
    let f1 = File::open("this_file.txt").unwrap();
    println!("File Descriptor(1st time): {}", f1.as_raw_fd());

    let f2 = File::open("this_file.txt").unwrap();
    println!("File Descriptor(2nd time): {}", f2.as_raw_fd());
    let fd2 = f2.as_raw_fd();

    println!("File Descriptor(1st open time): {}", f1.as_raw_fd());

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("this_file.txt")
        .unwrap();

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };

    mmap[1] = b'a';

    mmap.flush().unwrap();

    println!("File modified! Check this_file.txt content.");

    let mut file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd2) });
    println!("this_file.txt content: {}", {
        let mut buffer = [0; 52];

        let n = file.read(&mut buffer).unwrap();

        String::from_utf8_lossy(&buffer[..n]).to_string()
    });
}
