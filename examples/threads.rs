extern crate tuple_store;

use std::thread::JoinHandle;
use std::{thread, time};

use tuple_store::mutex_store::MutexStore;
use tuple_store::query_tuple::QueryTuple;
use tuple_store::result::Result;
use tuple_store::store::Store;
use tuple_store::tuple::Tuple;
use tuple_store::vec_store::VecStore;

fn main() {
    println!("Starting");
    let mutex_store = MutexStore::<VecStore>::default();

    let mut writer_tuple_space = mutex_store.clone();
    let writer_1_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Writer 1");
        let writer_sleep = time::Duration::from_millis(100);

        for i in 0..100 {
            let tuple = Tuple::builder().integer(1).integer(i).build();
            println!("Writer 1: Wrote: {:?}", tuple);
            writer_tuple_space.write(&tuple)?;
            thread::sleep(writer_sleep);
        }
        println!("Writer 1: Wrote 100 tuples");

        Ok(())
    });

    thread::sleep(time::Duration::from_millis(200));

    let reader_mutex_store = mutex_store.clone();
    let reader_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Reader");
        let mut num_tuples = 0;
        let query_tuple = QueryTuple::builder().any().any().build();
        let reader_sleep = time::Duration::from_millis(500);

        while let Some(tuple) = reader_mutex_store.read(&query_tuple)? {
            println!("Reader: Read: {:?}", tuple);
            num_tuples += 1;
            thread::sleep(reader_sleep);
        }

        println!("Reader: Tuple space empty! I read {} tuples.", num_tuples);
        Ok(())
    });

    let mut taker_mutex_store = mutex_store.clone();
    let taker_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Taker");
        let mut num_tuples = 0;
        let query_tuple = QueryTuple::builder().any_integer().any_integer().build();
        let taker_sleep = time::Duration::from_millis(110);

        while let Some(tuple) = taker_mutex_store.take(&query_tuple)? {
            println!("Taker: Took: {:?}", tuple);
            num_tuples += 1;
            thread::sleep(taker_sleep);
        }

        println!("Taker: Tuple space empty! I took {} tuples.", num_tuples);
        Ok(())
    });

    let mut writer_mutex_store = mutex_store.clone();
    let writer_2_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Writer 2");
        let writer_sleep = time::Duration::from_millis(100);

        for i in 0..100 {
            let tuple = Tuple::builder().integer(2).integer(i).build();
            println!("Writer 2: Wrote: {:?}", tuple);
            writer_mutex_store.write(&tuple)?;
            thread::sleep(writer_sleep);
        }
        println!("Writer 2: Wrote 100 tuples");

        Ok(())
    });

    if let Err(_) = writer_1_thread.join() {
        panic!("Writer 1 panic")
    };
    if let Err(_) = writer_2_thread.join() {
        panic!("Writer 2 panic")
    };
    if let Err(_) = taker_thread.join() {
        panic!("Taker panic")
    };
    if let Err(_) = reader_thread.join() {
        panic!("Reader panic")
    };

    println!("Finished");
}
