//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

#[cfg(feature = "v2")]
pub use self::basis_spline::BSpLineWorkspace;
#[cfg(not(feature = "v2"))]
pub use self::basis_spline::{BSpLineDerivWorkspace, BSpLineWorkspace};

pub use self::chebyshev::ChebSeries;
pub use self::combination::Combination;
pub use self::complex::{ComplexF32, ComplexF64};
pub use self::discrete_hankel::DiscreteHankel;
pub use self::eigen_symmetric_workspace::{
    EigenGenHermVWorkspace, EigenGenHermWorkspace, EigenGenSymmVWorkspace, EigenGenSymmWorkspace,
    EigenGenVWorkspace, EigenGenWorkspace, EigenHermitianVWorkspace, EigenHermitianWorkspace,
    EigenNonSymmVWorkspace, EigenNonSymmWorkspace, EigenSymmetricVWorkspace,
    EigenSymmetricWorkspace,
};
pub use self::fast_fourier_transforms::{FftComplexWaveTable, FftComplexWorkspace};
pub use self::histograms::{Histogram, Histogram2D, Histogram2DPdf, HistogramPdf};
pub use self::integration::{
    CquadWorkspace, GLFixedTable, IntegrationQawoTable, IntegrationQawsTable, IntegrationWorkspace,
};
pub use self::interpolation::{Interp, InterpAccel, InterpType, Spline};
pub use self::mathieu::MathieuWorkspace;
pub use self::matrix::{MatrixF32, MatrixF64, MatrixView};
pub use self::matrix_complex::{MatrixComplexF32, MatrixComplexF64};
pub use self::minimizer::{Minimizer, MinimizerType};
pub use self::monte_carlo::{
    MiserMonteCarlo, MiserParams, PlainMonteCarlo, VegasMonteCarlo, VegasParams,
};
pub use self::multifit_solver::{
    MultiFitFdfSolver, MultiFitFdfSolverType, MultiFitFunction, MultiFitFunctionFdf,
};
pub use self::multiset::MultiSet;
pub use self::n_tuples::NTuples;
pub use self::ordinary_differential_equations::{
    ODEiv2Control, ODEiv2Driver, ODEiv2Evolve, ODEiv2Step, ODEiv2StepType, ODEiv2System,
};
pub use self::permutation::Permutation;
pub use self::polynomial::PolyComplex;
pub use self::qrng::{QRng, QRngType};
pub use self::ran_discrete::RanDiscrete;
pub use self::result::{Result, ResultE10};
pub use self::rng::{Rng, RngType};
pub use self::roots::{
    RootFSolver, RootFSolverType, RootFdfSolver, RootFdfSolverType, RootFunction, RootFunctionFdf,
};
pub use self::series_acceleration::{LevinUTruncWorkspace, LevinUWorkspace};
pub use self::siman::{SimAnnealing, SimAnnealingParams};
pub use self::vector::{VectorF32, VectorF64, VectorView};
pub use self::vector_complex::{VectorComplexF32, VectorComplexF64};
pub use self::wavelet_transforms::{Wavelet, WaveletType, WaveletWorkspace};

pub mod basis_spline;
pub mod chebyshev;
pub mod combination;
pub mod complex;
pub mod discrete_hankel;
pub mod eigen_symmetric_workspace;
pub mod fast_fourier_transforms;
pub mod histograms;
pub mod integration;
pub mod interpolation;
pub mod mathieu;
pub mod matrix;
pub mod matrix_complex;
pub mod minimizer;
pub mod monte_carlo;
pub mod multifit_solver;
pub mod multiset;
pub mod n_tuples;
pub mod ordinary_differential_equations;
pub mod permutation;
pub mod polynomial;
pub mod qrng;
pub mod ran_discrete;
pub mod result;
pub mod rng;
pub mod roots;
pub mod series_acceleration;
pub mod siman;
pub mod vector;
pub mod vector_complex;
pub mod wavelet_transforms;
