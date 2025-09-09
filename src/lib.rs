#[cfg(target_arch = "x86_64")]

pub mod errors;
mod header;

use std::{
    io::{Write, Read, self},
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

pub enum Model {
    Client,
    Server,
}

/// WRITE_BUF_SIZE is the size of the buffer used with the 
/// qrexec-client-vm call, --buffer-size=WRITE_BUF_SIZE, argument, 
/// and the size of the buffer used to write into the 
/// qrexec-client-vm file descriptors behind the scenes. 
/// The only thing you need to know when you set this is that 
/// 8 extra bytes are taken up by the header therefore you cannot 
/// send more data than <WRITE_BUF_SIZE - 8> in a single write call.  
#[derive(Debug)]
pub struct Qrexec<const WRITE_BUF_SIZE: usize> { 
    wbuf: [u8; WRITE_BUF_SIZE],
    pub child: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
    pub stderr: ChildStderr,
}

impl<const WRITE_BUF_SIZE: usize> Qrexec<WRITE_BUF_SIZE> {
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
    pub fn new(
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
            &WRITE_BUF_SIZE.to_string(), target_vmname, rpc_service]);

        if let Some(local_program) = local_program {
            child.arg(local_program);
        }

        if let Some(args) = local_program_args {
            child.args(args);
        }

        let mut child = child.spawn()?;

        return Ok(Self {
            wbuf: [0u8; WRITE_BUF_SIZE],
            stdout: child.stdout.take().ok_or(
                anyhow!(STDOUT_ERR))?,
            stdin: child.stdin.take().ok_or(
                anyhow!(STDIN_ERR))?,
            stderr: child.stderr.take().ok_or(
                anyhow!(STDERR_ERR))?,
            child,
        })
    }
    
    /// returns the number of bytes read into the buffer 
    #[inline(always)]
    pub fn read(read: &mut impl Read, buf: &mut [u8]) -> io::Result<usize> {
        let mut hbuf = [0u8; HEADER_LEN];
        read.read_exact(&mut hbuf)?;

        let msg_len = header_len(&hbuf) as usize;
        read.read_exact(&mut buf[..msg_len])?;

        return Ok(msg_len);
    } 

    /// returns the number of bytes written from the buffer.
    /// You cannot send more data than WRITE_BUF_SIZE - 8 in a 
    /// single call to this function as this would result in an  
    /// overflow. 
    #[inline(always)]
    pub fn write(&mut self, written: &mut impl Write, buf: &[u8]) -> QRXRes<usize> {
        let total_nb = buf.len() + HEADER_LEN;
        if (total_nb) > WRITE_BUF_SIZE {
            Err(anyhow!(WBUF_LEN_ERR))?;
        } 

        let mut i = 0;
        let header = header(buf);

        for val in header {
            self.wbuf[i] = val;
            i += 1;
        }

        i = HEADER_LEN;
        for val in buf {
            self.wbuf[i] = *val;
            i += 1;
        }

        written.write_all(&self.wbuf[..i])?;

        return Ok(total_nb);
    }
}

impl<const WRITE_BUF_SIZE: usize> Drop for Qrexec<WRITE_BUF_SIZE> {
    fn drop(&mut self) {
        let _ = self.child.kill();   
    }
}
