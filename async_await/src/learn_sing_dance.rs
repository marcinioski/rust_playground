use futures::executor::block_on;
use std::thread;
use std::time;

struct Song();

async fn learn_song() -> Song {
    println!("learn_song()");
    Song()
}

async fn sing_song(_song: Song) {
    println!("sing_song()");
    thread::sleep(time::Duration::from_millis(1000));
}

async fn dance() {
    println!("dance()");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

pub async fn learn_and_sing_dance() {
    // dance happens seperatelly, do not need to wait until
    // learn and sing finished!
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f2, f1);
}

pub fn learn_sing_dance() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
