//! # Voracious sort
//!
//! Voracious sort is a [sort algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm), like
//! [Rust standard sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)
//! or
//! [Rust unstable sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable).
//! However it is a [radix sort](https://en.wikipedia.org/wiki/Radix_sort), it is a non comparative sort.
//! It is a **very fast sort** and it compares very well against Rust standard and unstable sorts and other
//! state of the art sorting algorithms (see runtimes below).
//!
//! Voracious sort can sort a
//! [`vector`](https://doc.rust-lang.org/stable/std/vec/)
//! or a
//! [`slice`](https://doc.rust-lang.org/stable/std/primitive.slice.html)
//! of:
//! - [`bool`](https://doc.rust-lang.org/stable/std/primitive.bool.html) (Counting sort),
//! - [`char`](https://doc.rust-lang.org/stable/std/primitive.char.html) (Behave like u32),
//! - [`&str`](https://doc.rust-lang.org/std/primitive.str.html) (Dedicated sort),
//! - [`f32`](https://doc.rust-lang.org/stable/std/primitive.f32.html),
//! [`f64`](https://doc.rust-lang.org/stable/std/primitive.f64.html) (See [link](http://stereopsis.com/radix.html)),
//! - [`i8`](https://doc.rust-lang.org/stable/std/primitive.i8.html),
//! [`i16`](https://doc.rust-lang.org/stable/std/primitive.i16.html),
//! [`i32`](https://doc.rust-lang.org/stable/std/primitive.i32.html),
//! [`i64`](https://doc.rust-lang.org/stable/std/primitive.i64.html),
//! [`i128`](https://doc.rust-lang.org/stable/std/primitive.i128.html) (First bit is flipped),
//! - [`u8`](https://doc.rust-lang.org/stable/std/primitive.u8.html),
//! [`u16`](https://doc.rust-lang.org/stable/std/primitive.u16.html),
//! [`u32`](https://doc.rust-lang.org/stable/std/primitive.u32.html),
//! [`u64`](https://doc.rust-lang.org/stable/std/primitive.u64.html),
//! [`u128`](https://doc.rust-lang.org/stable/std/primitive.u128.html) (Native implementation),
//! - Some [`tuple`](https://doc.rust-lang.org/std/primitive.tuple.html), but
//! they do not have been all implemented (Mapped to a key).
//! - Custom [struct](https://doc.rust-lang.org/std/keyword.struct.html)
//! if a the struct implements both
//! [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
//! and
//! [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
//! traits and Voracious traits (see below) (Mapped to a key).
//!
//! Vocarious sort can only sort in ascending order. You can call the
//! [`reverse`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse)
//! method if desired. Sorts from this crate are
//! [unstable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability) (Unstable sort,
//! not unstable Rust feature !).
//!
//! ## Version
//!
//! Last version tested/used:
//! - Rustc: 1.38.0 stable
//! - Rustfmt: 1.4.4 stable
//! - Cargo: 1.38.0 stable
//! - Clippy: 0.0.212
//!
//! ## License
//!
//! See the license file.
//!
//! ## How to use it
//!
//! Add in `Cargo.toml`:
//! ```toml
//! [dependencies]
//! voracious_radix_sort = "0.1.0"
//! ```
//!
//! Import the crate in your project:
//! ```no_run
//! extern crate voracious_radix_sort;
//! ```
//!
//! ### Environment variable
//!
//! To fully benefit from Voracious sort, it is better to add the environment
//! variable:
//!
//! ```toml
//! export RUSTFLAGS="-C target-cpu=native"
//! ```
//!
//! When the Crate is imported, two methods are added to vectors and slices:
//! - `voracious_sort()` (single thread).
//! - `dlsd_sort()` (single thread).
//!
//! ### Example
//!
//! ```
//! use voracious_radix_sort::*;
//!
//! let mut array = vec![2, 45, 8, 7, 9, 65, 8, 74, 1, 2, 56, 9, 7, 41];
//!
//! // General multipurpose sort, to be used by default.
//! array.voracious_sort();
//!
//! assert_eq!(array, vec![1, 2, 2, 7, 7, 8, 8, 9, 9, 41, 45, 56, 65, 74]);
//! ```
//!
//! ### Example (experimental)
//!
//! ```
//! use voracious_radix_sort::*;
//!
//! let mut array = vec![2, 45, 8, 7, 9, 65, 8, 74, 1, 2, 56, 9, 7, 41];
//!
//! // Experimental sort, faster than Voracious sort on
//! // uniformly distributed data. It uses statistical
//! // distribution hypothesis on the input.
//! array.dlsd_sort();
//!
//! assert_eq!(array, vec![1, 2, 2, 7, 7, 8, 8, 9, 9, 41, 45, 56, 65, 74]);
//! ```
//! ### Panics
//!
//! For
//! [`f32`](https://doc.rust-lang.org/stable/std/primitive.f32.html)
//! and
//! [`f64`](https://doc.rust-lang.org/stable/std/primitive.f64.html)
//! , it `panics` if there is a
//! [`NaN`](https://doc.rust-lang.org/stable/std/f64/constant.NAN.html)
//! value in the
//! [`vector`](https://doc.rust-lang.org/stable/std/vec/)
//! or
//! the
//! [`slice`](https://doc.rust-lang.org/stable/std/primitive.slice.html).
//!
//! ## Dependencies
//!
//! There is no dependency.
//!
//! ## Performances
//!
//! - All tests have been done on a i5 7500 3.4GHz 6MB cache L3 with 40GB DDR4 RAM (November 2019)
//! with `RUSTFLAGS="-C target-cpu=native"`.
//! - Only one run has been done by test.
//! - For more runtimes, please visit our [GitHub](https://github.com/fretlink/voracious_sort).
//! - Times are in micro seconde.
//!
//! - *[RdxSort](https://crates.io/crates/rdxsort) version 0.3.0
//! - *[AfSort](https://crates.io/crates/afsort) version 0.3.1
//!
//! ### For **`u64`** 100_000_000 integers
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `1_969_984` | `1_393_206` | `10_124_103` | `3_372_091 `| `2_842_670` | `4_060_710` |
//! | Zipf | `352_684` | `1_315_713` | `5_584_313` | `331_330`| `5_587_266` | `2_155_202` |
//! | Normal (SD 10^6) | `1_863_536` | `3_008_335` | `9_675_911` | `2_454_208`| `3_874_521` | `4_482_161` |
//!
//! ### For **`f64`** 100_000_000 floats
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `2_358_032` | `2_873_768` | `14_247_551` | `7_108_842`| `4_548_991` | `N/A` |
//! | Zipf | `2_198_049` | `1_221_660` | `6_435_186` | `805_088`| `6_242_734` | `N/A` |
//! | Normal (SD 10^6) | `2_357_697` | `2_334_541` | `14_049_225` | `7_109_580`| `4_309_830` | `N/A` |
//!
//! ### For **`i64`** 100_000_000 integers
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `2_037_479` | `1_347_168` | `9_932_912` | `3_516_609`| `3_302_737` | `N/A` |
//! | Zipf | `401_947` | `1_287_534` | `5_499_072` | `320_038`| `5_807_618` | `N/A` |
//! | Normal (SD 10^6) | `1_856_729` | `3_039_194` | `9_821_670` | `2_602_098`| `4_225_584` | `N/A` |
//!
//! ### For **`char`** 100_000_000 chars
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `777_537` | `802_939` | `6_116_985` | `1_813_057`| `1_933_041` | `N/A` |
//! | All Equal | `47_914` | `47_929` | `47_488` | `41_212`| `2_229_338` | `N/A` |
//! | Small CharSet | `114_896` | `394_616` | `6_197_632` | `689_738`| `2_006_239` | `N/A` |
//! | Medium CharSet | `113_521` | `622_184` | `6_144_734` | `629_989`| `1_982_256` | `N/A` |
//! | Big CharSet | `867_986` | `857_884` | `6_169_368` | `727_749`| `2_029_007` | `N/A` |
//!
//! ### For **`&str`** 10_000_000 &str
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform (length: 20) | `2_526_030` | `N/A` | `3_360_045` | `5_585_305`| `N/A` | `3_096_866` |
//!
//! ### For **`(u32, u32)`** 100_000_000 tuples
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `2_306_188` | `1_605_143` | `12_913_487` | `4_780_460`| `3_147_126` | `N/A` |
//!
//! ### For **`(bool, bool)`** 100_000_000 tuples
//!
//! - For `(bool, bool)`, Voracious sort uses a counting sort.
//!
//! | Distribution | Voracious | DLSD | Rust Std | Rust Unstable | RdxSort* | AFSort* |
//! |-------------:|----------:|-----:|---------:|--------------:|---------:|--------:|
//! | Uniform | `118_116` | `394_691` | `5_223_943` | `846_968`| `456_879` | `N/A` |
//!
//! # For Developers and Researchers
//!
//! ## Logic
//!
//! - Voracious sort is a MSD radix sort. It is an improvement of the
//! [Ska sort](https://probablydance.com/2016/12/27/i-wrote-a-faster-sorting-algorithm/)
//! and it uses the [Verge sort pre-processing heuristic](https://github.com/Morwenn/vergesort).
//! The purpose is to implement a multithread radix sort (see
//! [Regions sort](https://github.com/omarobeya/parallel-inplace-radixsort) and
//! the [article](https://people.csail.mit.edu/jshun/RegionsSort.pdf)).
//!
//! - DLSD (Diverting LSD radix sort) is a simpler version of the
//! [DFR sort](https://github.com/ramou/dfr) with a different diversion and
//! a variable radix (see [article](https://users.encs.concordia.ca/~sthiel/DS/SEA2015_FastRadix.pdf)).
//!
//! - All sorts fallback on the
//! [PDQ sort](https://github.com/stjepang/pdqsort)
//! (Rust Unstable sort) for very small inputs.
//!
//! - For now, both Voracious sort and DLSD sort have generic code. But I notice
//! that dedicated implementation per type is faster (~10-15%). For maintainability
//! reason, this Crate has an as generic code as possible. For research article,
//! dedicated implementation per type is be used.
//!
//! ## Futur work
//!
//! - Add multithread sort.
//! - Improve k-way-merge algorithm.
//! - Use more statistical hypothesis.
//! - Finish 2-arity tuple implementation.
//! - Add more generators (for tests).
//! - Replace the MSD sort for string by a Burstsort or Spreadsort implementation
//! or something else.

pub mod algo;
#[cfg(test)]
pub mod generators;
mod sorts;
#[cfg(test)]
pub mod tests;
mod types;

pub use types::utils::offset_from_bits;
pub use types::{RadixSort, Radixable, RadixableForContainer};

pub use sorts::american_flag_sort::american_flag_sort;
pub use sorts::boolean_sort::boolean_sort;
pub use sorts::comparative_sort::insertion_sort;
pub use sorts::counting_sort::counting_sort;
pub use sorts::dlsd_sort::dlsd_radixsort;
pub use sorts::lsd_sort::lsd_radixsort;
pub use sorts::msd_sort::msd_radixsort;
pub use sorts::msd_string_sort::msd_string_radixsort;
pub use sorts::ska_sort::ska_sort;
pub use sorts::thiel_sort::thiel_radixsort;
pub use sorts::voracious_sort::voracious_sort;
