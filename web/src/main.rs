mod lib;
mod thread_pool;

use crate::lib::handle_connection_slow;
use crate::thread_pool::ThreadPool;
use futures::{SinkExt, StreamExt};
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_stream_by_single_thread(stream: TcpStream) {
    handle_connection_slow(stream);
}

fn handle_stream_by_threads(stream: TcpStream) {
    thread::spawn(|| {
        handle_connection_slow(stream);
    });
}

fn handle_stream_by_limit_threads(stream: TcpStream, pool: &mut ThreadPool) {
    pool.execute(move || {
        handle_connection_slow(stream);
    })
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // handle_stream_by_threads(stream.unwrap());
        handle_stream_by_limit_threads(stream.unwrap(), &mut pool);
    }
}
