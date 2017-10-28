//! Library to simplify using code generated by
//! [entity_store_code_gen](https://crates.io/crates/entity_store_code_gen).
extern crate serde;
#[macro_use] extern crate serde_derive;

// handy re-exports
pub extern crate direction;
pub extern crate cgmath;
pub extern crate num;

mod neighbour_count;

pub use self::neighbour_count::NeighbourCount;

/// Expands to the code generated by
/// [entity_store_code_gen](https://crates.io/crates/entity_store_code_gen).
/// Example usage:
///
/// ```
/// mod entity_store {
///     include_entity_store();
/// }
/// ```
#[macro_export]
macro_rules! include_entity_store {
    () => {
        include!(concat!(env!("OUT_DIR"), "/entity_store_code_gen_out.rs"));
    }
}
