pub mod errors;

use std::{
    io::{Write, Read, self},
    process::{
        Command,
        Child,
        ChildStdout,
        ChildStdin,
        ChildStderr,
    },
};
use anyhow::anyhow;
use crate::errors::*;

#[derive(Debug)]
pub struct Qrexec { 
    pub child: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
    pub stderr: ChildStderr,
}

impl Qrexec {
    /// Calls qrexec-client-vm with the arguments provided through the args parameter.
    ///
    /// # Examples 
    ///
    /// ```
    /// let qrx = Qrexec::new(&[
    ///     "--buffer-size=10",
    ///     "target_vmname",
    ///     "rpc_service_on_target_vmname",
    /// ]);
    ///
    /// Qrexec::write(&mut qrx.stdin, &[0, 1, 2])?;
    /// ```
    pub fn new(args: &[&str]) -> QRXRes<Self> {
        const STDOUT_ERR: &str = 
            "Error: child proc failed to produce stdout";
        const STDIN_ERR: &str = 
            "Error: child proc failed to produce stdin";
        const STDERR_ERR: &str =
            "Error: child proc failed to produce stderr";

        let mut child = Command::new("qrexec-client-vm")
            .args(args)
            .spawn()?;
        return Ok(Self {
            stdout: child.stdout.take().ok_or(
                anyhow!(STDOUT_ERR))?,
            stdin: child.stdin.take().ok_or(
                anyhow!(STDIN_ERR))?,
            stderr: child.stderr.take().ok_or(
                anyhow!(STDERR_ERR))?,
            child,
        })
    }

    // wrote these so I can put error handling in here
    // once i test the errors that pop out. 

    /// returns the number of bytes read into the buffer 
    /// currently this is a direct inlined call the  
    /// standard libraries read function. 
    #[inline(always)]
    pub fn read(
        read: &mut impl Read,
        buf: &mut [u8],
    ) -> Result<usize, io::Error> {
        match read.read(buf) {
            Ok(nb) => Ok(nb),
            Err(e) => Err(e),
        }
    } 

    /// returns the number of bytes written into the buffer
    /// currently this is a direct inlined call the  
    /// standard libraries read function. 
    #[inline(always)]
    pub fn write(
        written: &mut impl Write,
        buf: &[u8],
    ) -> Result<usize, io::Error> {
        match written.write(buf) {
            Ok(nb) => Ok(nb),
            Err(e) => Err(e),
        }
    }
}

impl Drop for Qrexec {
    fn drop(&mut self) {
        let _ = self.child.kill();   
    }
}
