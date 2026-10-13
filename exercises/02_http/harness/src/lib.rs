//! Test support for the HTTP exercises: starts `spin up` for an exercise, waits until it
//! serves requests and stops it again when the test process exits.
//!
//! You should not need to modify this crate.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The address the test instance of `spin up` listens on.
/// The port differs from Spin's default (3000) so tests don't collide with a `spin up` you
/// started yourself.
pub const LISTEN: &str = "127.0.0.1:3333";

static SPIN: Mutex<Option<Child>> = Mutex::new(None);

/// Starts `spin up --build` in `app_dir` (the directory containing `spin.toml`) with the given
/// extra arguments and environment variables. Subsequent calls in the same process return
/// immediately; the application is stopped when the process exits.
pub fn start(app_dir: impl AsRef<Path>, args: &[&str], env: &[(&str, &str)]) {
    let mut spin = SPIN.lock().unwrap_or_else(|e| e.into_inner());
    if spin.is_some() {
        return;
    }

    assert!(
        TcpStream::connect(LISTEN).is_err(),
        "something is already listening on {LISTEN}, stop it before running the tests"
    );

    let mut cmd = Command::new("spin");
    cmd.args(["up", "--build", "--listen", LISTEN])
        .args(args)
        .envs(env.iter().copied())
        .current_dir(app_dir)
        .stdin(Stdio::null());
    // `spin up` spawns `spin trigger http` as a child; a process group lets us stop both
    #[cfg(unix)]
    cmd.process_group(0);

    let mut child = cmd.spawn().unwrap_or_else(|err| {
        panic!("failed to run `spin`: {err}. Is the Spin CLI installed and on your PATH?")
    });

    let deadline = Instant::now() + Duration::from_secs(300);
    loop {
        if TcpStream::connect(LISTEN).is_ok() {
            break;
        }
        if let Ok(Some(status)) = child.try_wait() {
            panic!("`spin up` exited with {status} before serving requests, see the output above");
        }
        assert!(
            Instant::now() < deadline,
            "`spin up` did not start within 5 minutes"
        );
        std::thread::sleep(Duration::from_millis(200));
    }

    // the test harness ends the process with `exit`, which runs `atexit` handlers
    unsafe { libc::atexit(stop) };
    *spin = Some(child);
}

/// Stops `spin up` and the trigger process it spawned.
extern "C" fn stop() {
    let mut spin = SPIN.lock().unwrap_or_else(|e| e.into_inner());
    let Some(mut child) = spin.take() else {
        return;
    };
    #[cfg(unix)]
    let _ = Command::new("kill")
        .args(["-TERM", "--", &format!("-{}", child.id())])
        .status();
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/T", "/F", "/PID", &child.id().to_string()])
        .status();
    let _ = child.wait();
}

/// A minimal HTTP response.
#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

/// Sends a single HTTP/1.0 request to the test instance and reads the response. HTTP/1.0 keeps
/// the response simple: no chunked encoding, the server closes the connection after the body.
pub fn request(method: &str, path_and_query: &str, body: &str) -> Response {
    let mut stream = TcpStream::connect(LISTEN).expect("spin is not running");
    stream
        .set_read_timeout(Some(Duration::from_secs(60)))
        .unwrap();
    write!(
        stream,
        "{method} {path_and_query} HTTP/1.0\r\nHost: {LISTEN}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
    .unwrap();

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let raw = String::from_utf8_lossy(&raw);
    let (head, body) = raw.split_once("\r\n\r\n").expect("malformed HTTP response");
    let status = head
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .expect("malformed status line");

    Response {
        status,
        body: body.to_string(),
    }
}

/// Serves `body` with status 200 for every request on `127.0.0.1:{port}` until the test process
/// exits. Used to stand in for third-party HTTP APIs.
pub fn mock_server(port: u16, body: &'static str) {
    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => listener,
        // already running in this process
        Err(_) => return,
    };
    std::thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let _ = write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
        }
    });
}
