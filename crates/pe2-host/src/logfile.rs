//! The log's time stamps. When the error output is not a console (pe2.log,
//! or a file it was redirected to), it goes through a pipe instead, both the
//! host's (eprintln) and the C side's (the game's printf, the crash
//! report), and a thread writes each line to where it went before with the
//! local time in front: `[HH:MM:SS.mmm] ...`.
//!
//! What is still in the pipe when the process ends would be lost: the exit
//! paths and the crash report call `drain` first.

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use windows_sys::Win32::Foundation::{DUPLICATE_SAME_ACCESS, DuplicateHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{FILE_TYPE_CHAR, GetFileType, ReadFile, WriteFile};
use windows_sys::Win32::System::Console::{GetStdHandle, STD_ERROR_HANDLE, SetStdHandle};
use windows_sys::Win32::System::Pipes::{CreatePipe, PeekNamedPipe};

unsafe extern "C" {
    fn _open_osfhandle(handle: isize, flags: i32) -> i32;
    fn _dup2(from: i32, to: i32) -> i32;
}

/// The pipe's read end (0: not started), and whether the writer thread is
/// between reading and writing a chunk.
static READ_END: AtomicIsize = AtomicIsize::new(0);
static BUSY: AtomicBool = AtomicBool::new(false);

/// Starts stamping the error output, unless it is a console.
pub fn start() {
    // SAFETY: plain handle queries and a pipe of this process's own.
    unsafe {
        let out = GetStdHandle(STD_ERROR_HANDLE);
        if out.is_null() || out == INVALID_HANDLE_VALUE || GetFileType(out) == FILE_TYPE_CHAR {
            return;
        }
        // A handle of its own: redirecting descriptor 2 closes the one it had.
        let process = windows_sys::Win32::System::Threading::GetCurrentProcess();
        let mut own: HANDLE = std::ptr::null_mut();
        if DuplicateHandle(process, out, process, &mut own, 0, 0, DUPLICATE_SAME_ACCESS) == 0 {
            return;
        }
        let out = own;
        let (mut read, mut write): (HANDLE, HANDLE) = (std::ptr::null_mut(), std::ptr::null_mut());
        if CreatePipe(&mut read, &mut write, std::ptr::null(), 1 << 16) == 0 {
            return;
        }
        // The C runtime's stderr (file descriptor 2) and the process's.
        let fd = _open_osfhandle(write as isize, 0);
        if fd < 0 || _dup2(fd, 2) != 0 {
            return;
        }
        SetStdHandle(STD_ERROR_HANDLE, write);
        READ_END.store(read as isize, Ordering::SeqCst);
        let (read, out) = (read as isize, out as isize);
        std::thread::Builder::new().name("log".into()).spawn(move || copy(read, out)).expect("spawn log thread");
    }
}

/// Copies the pipe to `out`, line by line, each with the time in front.
fn copy(read: isize, out: isize) {
    let mut chunk = vec![0u8; 1 << 14];
    let mut line = Vec::new();
    loop {
        let mut n = 0u32;
        // SAFETY: the pipe's read end, into a buffer of the given size.
        let ok =
            unsafe { ReadFile(read as HANDLE, chunk.as_mut_ptr(), chunk.len() as u32, &mut n, std::ptr::null_mut()) };
        if ok == 0 || n == 0 {
            return;
        }
        BUSY.store(true, Ordering::SeqCst);
        let mut text = Vec::new();
        for &b in &chunk[..n as usize] {
            if line.is_empty() && b != b'\r' {
                line.extend_from_slice(stamp().as_bytes());
            }
            if b == b'\r' {
                continue;
            }
            line.push(b);
            if b == b'\n' {
                text.extend_from_slice(&line);
                line.clear();
            }
        }
        // A line not ended yet is written once it is.
        if !text.is_empty() {
            write_all(out, &text);
        }
        BUSY.store(false, Ordering::SeqCst);
    }
}

fn write_all(out: isize, mut bytes: &[u8]) {
    while !bytes.is_empty() {
        let mut n = 0u32;
        // SAFETY: the original error output, from a live buffer.
        let ok = unsafe { WriteFile(out as HANDLE, bytes.as_ptr(), bytes.len() as u32, &mut n, std::ptr::null_mut()) };
        if ok == 0 || n == 0 {
            return;
        }
        bytes = &bytes[n as usize..];
    }
}

fn pipe_empty(read: isize) -> bool {
    let mut avail = 0u32;
    // SAFETY: a peek at the pipe's read end.
    let ok = unsafe {
        PeekNamedPipe(read as HANDLE, std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut avail, std::ptr::null_mut())
    };
    ok == 0 || avail == 0
}

/// `[HH:MM:SS.mmm] `, the local time.
fn stamp() -> String {
    // SAFETY: fills a plain struct.
    let t = unsafe {
        let mut t = std::mem::zeroed();
        windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut t);
        t
    };
    format!("[{:02}:{:02}:{:02}.{:03}] ", t.wHour, t.wMinute, t.wSecond, t.wMilliseconds)
}

/// Waits (a second at most) until what was written so far is in the log.
pub fn drain() {
    let read = READ_END.load(Ordering::SeqCst);
    if read == 0 {
        return;
    }
    let until = std::time::Instant::now() + std::time::Duration::from_secs(1);
    while (!pipe_empty(read) || BUSY.load(Ordering::SeqCst)) && std::time::Instant::now() < until {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// `drain` for the C side (the crash report).
#[unsafe(no_mangle)]
pub extern "C" fn pc_log_drain() {
    drain();
}
