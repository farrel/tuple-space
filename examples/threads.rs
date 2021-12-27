extern crate tuple_space;

use std::thread::JoinHandle;
use std::{thread, time};

use tuple_space::result::Result;
use tuple_space::space::Space;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::vec_store::VecStore;

fn main() {
    println!("Starting");
    let mut tuple_space = Space::<VecStore>::default();

    let mut writer_tuple_space = tuple_space.clone();
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

    let mut reader_tuple_space = tuple_space.clone();
    let reader_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Reader");
        let mut num_tuples = 0;
        let template = Tuple::builder().any().any().build();
        let reader_sleep = time::Duration::from_millis(500);

        while let Some(tuple) = reader_tuple_space.read(&template)? {
            println!("Reader: Read: {:?}", tuple);
            num_tuples += 1;
            thread::sleep(reader_sleep);
        }

        println!("Reader: Tuple space empty! I read {} tuples.", num_tuples);
        Ok(())
    });

    let mut taker_tuple_space = tuple_space.clone();
    let taker_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Taker");
        let mut num_tuples = 0;
        let template = Tuple::builder()
            .any_integer()
            .any_integer()
            .build();
        let taker_sleep = time::Duration::from_millis(110);

        while let Some(tuple) = taker_tuple_space.take(&template)? {
            println!("Taker: Took: {:?}", tuple);
            num_tuples += 1;
            thread::sleep(taker_sleep);
        }

        println!("Taker: Tuple space empty! I took {} tuples.", num_tuples);
        Ok(())
    });

    let mut writer_tuple_space = tuple_space.clone();
    let writer_2_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Writer 2");
        let writer_sleep = time::Duration::from_millis(100);

        for i in 0..100 {
            let tuple = Tuple::builder().integer(2).integer(i).build();
            println!("Writer 2: Wrote: {:?}", tuple);
            writer_tuple_space.write(&tuple)?;
            thread::sleep(writer_sleep);
        }
        println!("Writer 2: Wrote 100 tuples");

        Ok(())
    });

    writer_1_thread.join();
    writer_2_thread.join();
    taker_thread.join();
    reader_thread.join();

    println!("Finished");
}
