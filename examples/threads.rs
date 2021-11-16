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
            let tuple = Tuple::builder().add_integer(1).add_integer(i).build();
            println!("Writer 1: Wrote: {:?}", tuple);
            writer_tuple_space.write(&tuple)?;
            thread::sleep(writer_sleep);
        }

        Ok(())
    });

    let mut writer_tuple_space = tuple_space.clone();
    let writer_2_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Writer 2");
        let writer_sleep = time::Duration::from_millis(100);

        for i in 0..100 {
            let tuple = Tuple::builder().add_integer(2).add_integer(i).build();
            println!("Writer 2: Wrote: {:?}", tuple);
            writer_tuple_space.write(&tuple)?;
            thread::sleep(writer_sleep);
        }

        Ok(())
    });

    thread::sleep(time::Duration::from_millis(200));

    let mut reader_tuple_space = tuple_space.clone();
    let reader_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        println!("Spawning Reader");
        let mut num_tuples = 0;
        let template = Tuple::builder().add_any().add_any().build();
        let reader_sleep = time::Duration::from_millis(500);

        while reader_tuple_space.len()? > 0 {
            println!("Reader: Read: {:?}", reader_tuple_space.read(&template)?);
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
            .add_integer_type()
            .add_integer_type()
            .build();
        let taker_sleep = time::Duration::from_millis(110);

        while taker_tuple_space.len()? > 0 {
            println!("Taker: Took: {:?}", taker_tuple_space.take(&template)?);
            num_tuples += 1;
            thread::sleep(taker_sleep);
        }

        println!("Taker: Tuple space empty! I took {} tuples.", num_tuples);
        Ok(())
    });

    writer_1_thread.join();
    writer_2_thread.join();
    taker_thread.join();
    reader_thread.join();

    println!("Finished");
}
