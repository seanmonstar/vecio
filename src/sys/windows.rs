extern crate winapi;

use std::io;
use std::os::windows::io::{AsRawSocket, RawSocket};

use ::{Rawv, Writev, Readv};

struct WinSock(RawSocket);

impl<T: AsRawSocket> Rawv for T {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize> {
        WinSock(self.as_raw_socket()).readv(buffers)
    }

    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize> {
        WinSock(self.as_raw_socket()).writev(buffers)
    }
}

impl Writev for WinSock {
    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize> {
        unsafe {
            let mut bytes = 0;
            let ret = WSASend(
                self.0,
                buffers.as_ptr() as LPWSABUF,
                buffers.len() as c_int,
                &mut bytes,
                0,
                0,
                0
            );
            if ret != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(bytes as usize)
            }
        }
    }
}


impl Readv for WinSock {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize> {
        unsafe {
            let mut bytes = 0;
            let ret = WSARecv(
                self.0,
                buffers.as_ptr() as LPWSABUF,
                buffers.len() as c_int,
                &mut bytes,
                0,
                0,
                0
            );
            if ret != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(ret as usize)
            }
        }
    }
}
