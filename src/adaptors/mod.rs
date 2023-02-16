#[cfg(feature = "array_combinations")]
pub mod array_combinations;
#[cfg(feature = "cartesian_product")]
pub mod cartesian_product;
#[cfg(feature = "combinations")]
pub mod combinations;
#[cfg(feature = "power_set")]
pub mod power_set;

#[cfg(any(
    feature = "array_combinations",
    feature = "combinations",
    feature = "power_set"
))]
mod generic_combinations;
