# tuple-space - Tuple Spaces for rust

# What is a Tuple Space

A tuple space is a method for coordinating data between different processes in an asynchronous manner. Processes write tuples of data to the tuple space and then read or remove data from the tuple space using a tuple as template to match against.

## Example

```rust
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
    // Arc<Mutex<VecStore>>
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
        let adder_template = Tuple::builder()
            .any_integer()
            .any_integer()
            .build();
        let sleep = time::Duration::from_millis(110);
        while let Ok(Some(tuple)) = adder_tuple_space.take(&adder_template) {
            if let (Types::Integer(num_1), Types::Integer(num_2)) = (&tuple[0], &tuple[1]) {
                let sum_tuple = Tuple::builder().add_integer(num_1 + num_2).build();
                adder_tuple_space.write(&sum_tuple)?;
            }
            thread::sleep(sleep);
        }
        Ok(())
    });

    // Printer thread that removes single element integer tuples and prints them to stdout.
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

```


## License (3-Clause BSD License)

Copyright 2021 Farrel Lifson

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
