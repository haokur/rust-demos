use std::thread::sleep;
use std::time::Duration;
use futures::executor::block_on;

async fn do_something() {
    println!("I'm a async function!");
}

async fn hello_world() {
    hello_cat().await;
    println!("hello world!");
}

async fn hello_cat() {
    println!("hello kitty");
}

#[test]
fn test_async() {
    let future = do_something();
    block_on(future);

    let future = hello_world();
    block_on(future);
}

async fn run_all_wait() {
    do_something().await;
    hello_world().await;
}

#[test]
fn test_await() {
    block_on(run_all_wait());
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!("演唱歌曲,{}", song.name);
}

async fn dance() {
    println!("跳舞~")
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn sing_and_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

#[test]
fn test_example() {
    block_on(sing_and_dance());
}
