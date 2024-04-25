use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 producer 线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    // 因为初始有一份，后面又 clone 了 4 份，所以总共有 5 份 tx
    drop(tx);

    // 创建 consumer 线程
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("msg: {:?}", msg);
        }
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("consumer thread join failed: {:?}", e))?;

    println!("consumer secret: {:?}", secret);
    Ok(())
}

fn producer(idx: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
