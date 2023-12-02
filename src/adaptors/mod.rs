#[cfg(feature = "array_chunks")]
pub mod array_chunks;
#[cfg(feature = "array_combinations")]
pub mod array_combinations;
#[cfg(feature = "array_combinations_with_reps")]
pub mod array_combinations_with_reps;
#[cfg(feature = "array_windows")]
pub mod array_windows;
#[cfg(feature = "cartesian_product")]
pub mod cartesian_product;
#[cfg(feature = "circular_array_windows")]
pub mod circular_array_windows;
#[cfg(feature = "combinations")]
pub mod combinations;
#[cfg(feature = "combinations_with_reps")]
pub mod combinations_with_reps;
#[cfg(any(
    feature = "array_combinations",
    feature = "array_combinations_with_reps",
    feature = "combinations",
    feature = "combinations_with_reps"
))]
mod generic_combinations;
