use crate::utils::types::*;
use std::io::{Error, ErrorKind};

pub struct Process {
    pid: Dword,
    handle: Handle,
}

impl Process {

    // Send bytes
    // what: Send {what} bytes.
    fn Send(what: &[u8]) -> Option<Error> {
        None
    }

    // Send bytes, includes '\n'
    fn SendLn(what: &[u8]) -> Option<Error> {
        let mut to_send: Vec<u8> = what.to_vec();
        to_send.push('\n' as u8);
        Self::Send(to_send.as_slice())
    }
    
    // Send bytes, includes '\r\n'
    fn SendRfln(what: &[u8]) -> Option<Error> {
        let mut to_send: Vec<u8> = what.to_vec();
        to_send.append(&mut vec!['\r' as u8, '\n' as u8]);
        Self::Send(to_send.as_slice())
    }

    // Recieve bytes from process.
    // until: Recieve until byte {until} appears.
    // limit: Recieve max {limit} bytes, *{limit} not inclusive.*
    // ignore: Option that ignores {until}.
    fn Recv(until: u8, limit: u64, ignore: bool) -> Result<Vec<u8>, Error> {
        return Err(Error::new(ErrorKind::Other, "test"));
    }

    // Recieve All
    fn RecvAll() -> Result<Vec<u8>, Error> {
        Self::Recv(0, u64::MAX, true)
    }

    // Recieve Line
    fn RecvLine() -> Result<Vec<u8>, Error> {
        Self::Recv('\n' as u8, u64::MAX, false)
    }

    fn RecvNBytes(n: u64) -> Result<Vec<u8>, Error> {
        Self::Recv(0, n+1, true)
    }

    fn RecvUntil(what: u8) -> Result<Vec<u8>, Error> {
        Self::Recv(what, u64::MAX, false)
    }

}