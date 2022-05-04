extern crate tuple_store;

use std::thread::JoinHandle;
use std::{thread, time};

use tuple_store::mutex_store::MutexStore;
use tuple_store::query_tuple::QueryTuple;
use tuple_store::result::Result;
use tuple_store::store::Store;
use tuple_store::tuple::Tuple;
use tuple_store::types::Types;
use tuple_store::vec_store::VecStore;

fn main() {
    // Create a tuple space with a VecStore tuple store.
    let mut writer_mutex_store = MutexStore::<VecStore>::default();
    // Create clones of the tuple space. The underlying VecStore is shared in a
    // Arc<Mutex<VecStore>> and will the shared between the clones.
    let mut adder_mutex_store = writer_mutex_store.clone();
    let mut print_mutex_store = writer_mutex_store.clone();

    // Writer thread that writes 100 tuples of two integers into the tuple space every 100ms.
    let writer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        let sleep = time::Duration::from_millis(100);
        for i in 0..100 {
            let tuple = Tuple::builder().integer(i).integer(i).build();
            writer_mutex_store.write(&tuple)?;
            thread::sleep(sleep);
        }
        Ok(())
    });

    thread::sleep(time::Duration::from_millis(200));

    // Adder thread that finds and removes a tuple with two integers, and writes their sum back into the tuple
    // space.
    let adder_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        // Template tuple with two Integer wild card elements.
        let adder_query_tuple = QueryTuple::builder().any_integer().any_integer().build();
        let sleep = time::Duration::from_millis(110);
        while let Ok(Some(tuple)) = adder_mutex_store.take(&adder_query_tuple) {
            if let (Types::Integer(num_1), Types::Integer(num_2)) = (&tuple[0], &tuple[1]) {
                let sum_tuple = Tuple::builder().integer(num_1 + num_2).build();
                adder_mutex_store.write(&sum_tuple)?;
            }
            thread::sleep(sleep);
        }
        Ok(())
    });

    // Printer thread that removes single elemen integer tuples and prints them to stdout.
    let printer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        let printer_query_tuple = QueryTuple::builder().any_integer().build();
        let sleep = time::Duration::from_millis(120);
        while let Ok(Some(tuple)) = print_mutex_store.take(&printer_query_tuple) {
            if let Types::Integer(num) = &tuple[0] {
                println!("Printer: {}", num);
            }
            thread::sleep(sleep);
        }
        Ok(())
    });

    if let Err(_) = writer_thread.join() {
        panic!("Writer panic")
    };
    if let Err(_) = adder_thread.join() {
        panic!("Adder panic")
    };
    if let Err(_) = printer_thread.join() {
        panic!("Printer panic")
    };
}
