extern crate tuple_space;

use std::thread::JoinHandle;
use std::{thread, time};

use tuple_space::error::Error;
use tuple_space::space::Space;
use tuple_space::tuple::Tuple;

fn main() {
    println!("Starting");
    let mut tuple_space = Space::default();

    let mut writer_tuple_space = tuple_space.clone();
    let writer_thread: JoinHandle<Result<(), Error>> = thread::spawn(move || {
        let writer_sleep = time::Duration::from_millis(100);

        for i in 0..100 {
            let tuple = Tuple::builder().add_integer(i).build();
            println!("Wrote: {:?}", tuple);
            writer_tuple_space.write(tuple)?;
            thread::sleep(writer_sleep);
        }

        Ok(())
    });

    thread::sleep(time::Duration::from_millis(200));

    let mut reader_tuple_space = tuple_space.clone();
    let reader_thread: JoinHandle<Result<(), Error>> = thread::spawn(move || {
        let template = Tuple::builder().add_integer_type().build();
        let reader_sleep = time::Duration::from_millis(110);

        while tuple_space.len()? > 0 {
            println!("Took: {:?}", reader_tuple_space.take(&template)?);
            thread::sleep(reader_sleep);
        }

        println!("Tuple space empty!");
        Ok(())
    });

    writer_thread.join();
    reader_thread.join();

    println!("Finished");
}
