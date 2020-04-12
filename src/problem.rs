use nalgebra::{storage::ContiguousStorageMut, Dim, Matrix, RealField, Vector};
use num_traits::FromPrimitive;

/// A least-squares minimization problem.
///
/// This is what [`LevenbergMarquardt`](struct.LevenbergMarquardt.html) needs
/// to compute the residuals and the Jacobian. See the [module documentation](index.html)
/// for a usage example.
pub trait LeastSquaresProblem<F, N, M>
where
    F: RealField + FromPrimitive,
    N: Dim,
    M: Dim,
{
    /// Storage type used for the residuals. Use `nalgebra::storage::Owned<F, M>`
    /// if you want to use `VectorN` or `MatrixMN`.
    type ResidualStorage: ContiguousStorageMut<F, M>;
    type JacobianStorage: ContiguousStorageMut<F, M, N>;
    type ParameterStorage: ContiguousStorageMut<F, N> + Clone;

    /// Set the stored parameters `$\vec{x}$`.
    ///
    /// The passed input can be normalized.
    fn set_params(&mut self, x: &mut Vector<F, N, Self::ParameterStorage>);

    /// Compute the residual vector.
    fn residuals(&self) -> Option<Vector<F, M, Self::ResidualStorage>>;

    /// Compute the Jacobian for the residual vector.
    fn jacobian(&self) -> Option<Matrix<F, M, N, Self::JacobianStorage>>;
}
