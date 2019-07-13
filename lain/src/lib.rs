//! This crate provides functionality one may find useful while developing a fuzzer. A recent
//! nightly Rust build is required for the specialization feature.
//! 
//! Please consider this crate in "beta" and subject to breaking changes for minor version releases for pre-1.0.
//! 
//! ## Documentation
//! 
//! Please refer to [the wiki](https://github.com/microsoft/lain/wiki) for a high-level overview.
//! 
//! For API documentation: ![docs.rs documentation](https://docs.rs/lain/badge.svg).
//!
//! ## Example Usage
//!
//! ```rust
//! extern crate lain;
//!
//! use lain::prelude::*;
//! use lain::rand;
//!
//! #[derive(Debug, Mutatable, NewFuzzed, BinarySerialize)]
//! struct MyStruct {
//!     field_1: u8,
//!
//!     #[bitfield(backing_type = "u8", bits = 3)]
//!     field_2: u8,
//!
//!     #[bitfield(backing_type = "u8", bits = 5)]
//!     field_3: u8,
//!
//!     #[fuzzer(min = 5, max = 10000)]
//!     field_4: u32,
//!
//!     #[fuzzer(ignore)]
//!     ignored_field: u64,
//! }
//!
//! fn main() {
//!     let mut mutator = Mutator::new(rand::thread_rng());
//!
//!     let mut instance = MyStruct::new_fuzzed(&mut mutator, None);
//!
//!     let mut serialized_data = Vec::with_capacity(instance.serialized_size());
//!     instance.push_to_buffer::<_, BigEndian>(&mut serialized_data);
//!
//!     println!("{:?}", instance);
//!
//!     // perform small mutations on the instance
//!     instance.mutate(&mut mutator, None);
//!
//!     println!("{:?}", instance);
//! }
//! ```
//!
//! A complete example of a fuzzer and its target can be found in the [examples](examples/)
//! directory. The server is written in C and takes data over a TCP socket, parses a message, and
//! mutates some state. The fuzzer has Rust definitions of the C data structure and will send fully
//! mutated messages to the server and utilizes the `Driver` object to manage fuzzer threads and
//! state.
//!
//! # Contributing
//!
//! This project welcomes contributions and suggestions.  Most contributions require you to agree to
//! a Contributor License Agreement (CLA) declaring that you have the right to, and actually do,
//! grant us the rights to use your contribution. For details, visit https://cla.microsoft.com.
//!
//! When you submit a pull request, a CLA-bot will automatically determine whether you need to
//! provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the
//! instructions provided by the bot. You will only need to do this once across all repos using our
//! CLA.
//!
//! This project has adopted the [Microsoft Open Source Code of
//! Conduct](https://opensource.microsoft.com/codeofconduct/). For more information see the [Code of
//! Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact
//! [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or
//! comments.


#![feature(specialization)]
#![feature(const_fn)]

// TODO: Uncomment once const generics are more stable
// #![feature(const_generics)]
// #![feature(maybe_uninit)]

extern crate num;
extern crate num_derive;
extern crate num_traits;
extern crate self as lain;

pub extern crate byteorder;
pub extern crate field_offset;
pub extern crate lain_derive;
pub extern crate lazy_static;
pub extern crate rand;

pub use lain_derive::*;

#[macro_use]
pub extern crate log;
#[macro_use]
extern crate mashup;

#[doc(hidden)]
pub mod buffer;
#[doc(hidden)]
pub mod dangerous_numbers;
pub mod driver;
#[doc(hidden)]
pub mod mutatable;
pub mod mutator;
pub mod prelude;
#[doc(hidden)]
pub mod new_fuzzed;
pub mod traits;
pub mod types;

pub fn hexdump(data: &[u8]) -> String {
    let mut ret = "------".to_string();
    for i in 0..16 {
        ret += &format!("{:02X} ", i);
    }

    let mut ascii = String::new();
    for (i, b) in data.iter().enumerate() {
        if i % 16 == 0 {
            ret += &format!("\t{}", ascii);
            ascii.clear();
            ret += &format!("\n{:04X}:", i);
        }

        ret += &format!(" {:02X}", b);
        // this is the printable ASCII range
        if *b > 0x21 && *b != 0x7f {
            ascii.push(*b as char);
        } else {
            ascii.push('.');
        }
    }

    if data.len() % 16 != 0 {
        for _i in 0..16 - (data.len() % 16) {
            ret += &format!("   ");
        }
    }

    ret += &format!("\t{}", ascii);
    ascii.clear();

    ret
}