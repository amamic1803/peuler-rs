//! A module for 3D geometry.

/// A 3D shape.
pub trait Shape3D {
    /// Calculate the surface area of the shape.
    /// # Returns
    /// * The surface area of the shape.
    fn surface_area(&self) -> f64;

    /// Calculate the volume of the shape.
    /// # Returns
    /// * The volume of the shape.
    fn volume(&self) -> f64;
}