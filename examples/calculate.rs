extern crate tuple_space;

use std::thread::JoinHandle;
use std::{thread, time};

use tuple_space::result::Result;
use tuple_space::space::Space;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::types::Types;
use tuple_space::vec_store::VecStore;

fn main() {
    // Create a tuple space with a VecStore tuple store.
    let mut writer_tuple_space = Space::<VecStore>::default();
    // Create clones of the tuple space. The underlying VecStore is shared in a
    // Arc<Mutex<VecStore>> and will the shared between the clones.
    let mut adder_tuple_space = writer_tuple_space.clone();
    let mut print_tuple_space = writer_tuple_space.clone();

    // Writer thread that writes 100 tuples of two integers into the tuple space every 100ms.
    let writer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        let sleep = time::Duration::from_millis(100);
        for i in 0..100 {
            let tuple = Tuple::builder().integer(i).integer(i).build();
            writer_tuple_space.write(&tuple)?;
            thread::sleep(sleep);
        }
        Ok(())
    });

    thread::sleep(time::Duration::from_millis(200));

    // Adder tread that finds and removes a tuple with two integers, and writes their sum back into the tuple
    // space.
    let adder_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        // Template tuple with two Integer wild card elements.
        let adder_template = Tuple::builder().any_integer().any_integer().build();
        let sleep = time::Duration::from_millis(110);
        while let Ok(Some(tuple)) = adder_tuple_space.take(&adder_template) {
            if let (Types::Integer(num_1), Types::Integer(num_2)) = (&tuple[0], &tuple[1]) {
                let sum_tuple = Tuple::builder().integer(num_1 + num_2).build();
                adder_tuple_space.write(&sum_tuple)?;
            }
            thread::sleep(sleep);
        }
        Ok(())
    });

    // Printer thread that removes single elemen integer tuples and prints them to stdout.
    let printer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
        let printer_template = Tuple::builder().any_integer().build();
        let sleep = time::Duration::from_millis(120);
        while let Ok(Some(tuple)) = print_tuple_space.take(&printer_template) {
            if let Types::Integer(num) = &tuple[0] {
                println!("Printer: {}", num);
            }
            thread::sleep(sleep);
        }
        Ok(())
    });

    writer_thread.join();
    adder_thread.join();
    printer_thread.join();
}
