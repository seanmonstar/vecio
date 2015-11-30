use std::io;
use std::os::raw::{c_int, c_void};
use std::os::unix::io::{AsRawFd, RawFd};

use ::{Rawv, Writev, Readv};

struct UnixFd(RawFd);

impl<T: AsRawFd> Rawv for T {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize> {
        UnixFd(self.as_raw_fd()).readv(buffers)
    }

    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize> {
        UnixFd(self.as_raw_fd()).writev(buffers)
    }
}

impl Writev for UnixFd {
    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize> {
        unsafe {
            let ret = writev(self.0, buffers.as_ptr() as *const IoVec, buffers.len() as c_int);
            if ret == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(ret as usize)
            }
        }
    }
}


impl Readv for UnixFd {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize> {
        unsafe {
            let ret = readv(self.0, buffers.as_ptr() as *const IoVec, buffers.len() as c_int);
            if ret == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(ret as usize)
            }
        }
    }
}

#[repr(C)]
struct IoVec {
    iov_base: *mut c_void,
    iov_len: c_int,

}

extern {
    fn readv(fd: RawFd, bufs: *const IoVec, count: c_int) -> c_int;
    fn writev(fd: RawFd, bufs: *const IoVec, count: c_int) -> c_int;
}

#[test]
fn test_unix() {
    use std::io::Write;
    use std::fs::File;
    let mut f = File::create("foo.txt").unwrap();
    assert_eq!(f.writev(&[b"foo", b"bar"]).unwrap(), 6);
    f.flush().unwrap();

    let mut f = File::open("foo.txt").unwrap();

    let mut first = [0u8; 2];
    let mut second = [0u8; 4];
    assert_eq!(f.readv(&[&mut first, &mut second]).unwrap(), 6);
    assert_eq!(&first, b"fo");
    assert_eq!(&second, b"obar");
}
