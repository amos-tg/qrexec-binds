#[cfg(target_arch = "x86_64")]

pub mod errors;
mod header;

use std::{
    io::{
        Write, 
        Read,
        Stdin,
        Stdout,
        self,
    },
    process::{
        Command,
        Child,
        ChildStdout,
        ChildStdin,
        ChildStderr,
        Stdio,
    },
};
use anyhow::anyhow;
use crate::{
    errors::*,
    header::*,
};

pub trait QIO {
    /// returns the number of bytes read into the buffer 
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;

    /// returns the number of bytes written from the buffer.
    /// You cannot send more data than BUF_LEN - 8 in a 
    /// single call to this function as this would result in an  
    /// overflow. 
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
}

#[inline(always)]
fn inner_read(read: &mut impl Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut hbuf = [0u8; HEADER_LEN];
    read.read_exact(&mut hbuf)?;

    let msg_len = header_len(&hbuf) as usize;
    read.read_exact(&mut buf[..msg_len])?;

    return Ok(msg_len);
} 

#[inline(always)]
fn inner_write(
    written: &mut impl Write,
    data_buf: &[u8],
) -> io::Result<usize> {
    let header = header(data_buf);
    written.write_all(&header)?;
    written.write_all(&data_buf)?;
    written.flush()?;
    return Ok(data_buf.len() + HEADER_LEN);
}

#[derive(Debug)]
pub struct QrexecServer {
    read: Stdin,
    written: Stdout,
}

impl QIO for QrexecServer {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        return inner_read(&mut self.read, buf);
    }

    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        return inner_write(&mut self.written, buf);
    }
}

impl QrexecServer {
    pub fn new() -> Self {
        let read = io::stdin();  
        let written = io::stdout();
        Self {
            read, written,
        }
    } 
}

/// BUF_LEN is the size of the buffer used with the 
/// qrexec-client-vm call, --buffer-size=BUF_LEN, argument, 
/// and the size of the buffer used to write into the 
/// qrexec-client-vm file descriptors behind the scenes. 
/// The only thing you need to know when you set this is that 
/// 8 extra bytes are taken up by the header therefore you cannot 
/// send more data than <BUF_LEN - 8> in a single write call.  
#[derive(Debug)]
pub struct QrexecClient { 
    pub child: Child,
    pub read: ChildStdout,
    pub written: ChildStdin,
    pub stderr: ChildStderr,
}

impl QrexecClient {
    /// Calls qrexec-client-vm with the arguments provided through the args parameter.
    /// Arguments:
    ///
    /// target_vmname: 
    /// self explanatory 
    ///
    /// rpc_service: 
    /// the service you are calling on the target vm, this can include 
    /// an argument for the service using this syntax: some.service+argument. 
    ///
    /// local_program: 
    /// Full path to local program to be connected with remote service.
    ///
    /// local_program_args: 
    /// Arguments for the local program.
    pub fn new<const BUF_LEN: usize>(
        target_vmname: &str, 
        rpc_service: &str,
        local_program: Option<&str>,
        local_program_args: Option<&[&str]>,
    ) -> QRXRes<Self> {
        let mut child = Command::new("qrexec-client-vm");
        child.stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());

        child.args([
            &format!("--buffer-size={}", &BUF_LEN.to_string()),
            target_vmname, rpc_service]);

        if let Some(local_program) = local_program {
            child.arg(local_program);
        }

        if let Some(args) = local_program_args {
            child.args(args);
        }

        let mut child = child.spawn()?;

        return Ok(Self {
            read: child.stdout.take().ok_or(
                anyhow!(STDOUT_ERR))?,
            written: child.stdin.take().ok_or(
                anyhow!(STDIN_ERR))?,
            stderr: child.stderr.take().ok_or(
                anyhow!(STDERR_ERR))?,
            child,
        })
    }
}

impl QIO for QrexecClient {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        return inner_read(&mut self.read, buf);
    }  

    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        return inner_write(&mut self.written, buf);
    }
}

impl Drop for QrexecClient { 
    fn drop(&mut self) {
        let _ = self.child.kill();   
    }
}
