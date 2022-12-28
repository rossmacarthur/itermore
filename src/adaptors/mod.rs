#[cfg(feature = "array_combinations")]
pub mod array_combinations;
#[cfg(feature = "cartesian_product")]
pub mod cartesian_product;
#[cfg(feature = "combinations")]
pub mod combinations;
#[cfg(any(feature = "array_combinations", feature = "combinations"))]
mod generic_combinations;
