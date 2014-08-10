/*
 * A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
 */

pub mod Cblas {
    pub mod Level1 {
        pub fn sdsdot(N: i32, alpha: f32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
            unsafe { ::ffi::cblas_sdsdot(N, alpha, x.as_ptr(), incx, y.as_ptr(), incy) }
        }

        pub fn dsdot(N: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f64 {
            unsafe { ::ffi::cblas_dsdot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
        }

        pub fn sdot(N: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
            unsafe { ::ffi::cblas_sdot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
        }

        pub fn ddot(N: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f64 {
            unsafe { ::ffi::cblas_ddot(N, x.as_ptr(), incx, y.as_ptr(), incy) }
        }

        pub fn cdotu_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotu: &mut [T]) {
            unsafe { ::ffi::cblas_cdotu_sub(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_ptr() as *const ::libc::c_void,
                incy, dotu.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn cdotc_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotc: &mut [T]) {
            unsafe { ::ffi::cblas_cdotc_sub(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_ptr() as *const ::libc::c_void,
                incy, dotc.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn zdotu_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotu: &mut [T]) {
            unsafe { ::ffi::cblas_zdotu_sub(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_ptr() as *const ::libc::c_void,
                incy, dotu.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn zdotc_sub<T>(N: i32, x: &[T], incx: i32, y: &[T], incy: i32, dotc: &mut [T]) {
            unsafe { ::ffi::cblas_zdotc_sub(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_ptr() as *const ::libc::c_void,
                incy, dotc.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn snrm2(N: i32, x: &[f32], incx: i32) -> f32 {
            unsafe { ::ffi::cblas_snrm2(N, x.as_ptr(), incx) }
        }

        pub fn sasum(N: i32, x: &[f32], incx: i32) -> f32 {
            unsafe { ::ffi::cblas_sasum(N, x.as_ptr(), incx) }
        }

        pub fn dnrm2(N: i32, x: &[f64], incx: i32) -> f64 {
            unsafe { ::ffi::cblas_dnrm2(N, x.as_ptr(), incx) }
        }

        pub fn dasum(N: i32, x: &[f64], incx: i32) -> f64 {
            unsafe { ::ffi::cblas_dasum(N, x.as_ptr(), incx) }
        }

        pub fn scnrm2<T>(N: i32, x: &[T], incx: i32) -> f32 {
            unsafe { ::ffi::cblas_scnrm2(N, x.as_ptr() as *const ::libc::c_void, incx) }
        }

        pub fn scasum<T>(N: i32, x: &[T], incx: i32) -> f32 {
            unsafe { ::ffi::cblas_scasum(N, x.as_ptr() as *const ::libc::c_void, incx) }
        }

        pub fn dznrm2<T>(N: i32, x: &[T], incx: i32) -> f64 {
            unsafe { ::ffi::cblas_dznrm2(N, x.as_ptr() as *const ::libc::c_void, incx) }
        }

        pub fn dzasum<T>(N: i32, x: &[T], incx: i32) -> f64 {
            unsafe { ::ffi::cblas_dzasum(N, x.as_ptr() as *const ::libc::c_void, incx) }
        }

        pub fn isamax(N: i32, x: &[f32], incx: i32) -> ::types::CblasIndex {
            ::types::CblasIndex(unsafe { ::ffi::cblas_isamax(N, x.as_ptr(), incx) })
        }

        pub fn idamax(N: i32, x: &[f64], incx: i32) -> ::types::CblasIndex {
            ::types::CblasIndex(unsafe { ::ffi::cblas_idamax(N, x.as_ptr(), incx) })
        }

        pub fn icamax<T>(N: i32, x: &[T], incx: i32) -> ::types::CblasIndex {
            ::types::CblasIndex(unsafe { ::ffi::cblas_icamax(N, x.as_ptr() as *const ::libc::c_void, incx) })
        }

        pub fn izamax<T>(N: i32, x: &[T], incx: i32) -> ::types::CblasIndex {
            ::types::CblasIndex(unsafe { ::ffi::cblas_izamax(N, x.as_ptr() as *const ::libc::c_void, incx) })
        }

        pub fn sswap(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_sswap(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn scopy(N: i32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_scopy(N, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn saxpy(N: i32, alpha: f32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_saxpy(N, alpha, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn dswap(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dswap(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn dcopy(N: i32, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dcopy(N, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn daxpy(N: i32, alpha: f64, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_daxpy(N, alpha, x.as_ptr(), incx, y.as_mut_ptr(), incy) }
        }

        pub fn cswap<T>(N: i32, x: &mut [T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_cswap(N, x.as_mut_ptr() as *mut ::libc::c_void, incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn ccopy<T>(N: i32, x: &[T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_ccopy(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn caxpy<T>(N: i32, alpha: &[T], x: &[T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_caxpy(N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void,
                incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zswap<T>(N: i32, x: &mut [T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zswap(N, x.as_mut_ptr() as *mut ::libc::c_void, incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zcopy<T>(N: i32, x: &[T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zcopy(N, x.as_ptr() as *const ::libc::c_void, incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zaxpy<T>(N: i32, alpha: &[T], x: &[T], incx: i32, y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zaxpy(N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void,
                incx, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn srotg(a: &mut [f32], b: &mut [f32], c: &mut [f32], s: &mut [f32]) {
            unsafe { ::ffi::cblas_srotg(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), s.as_mut_ptr()) }
        }

        pub fn srotmg(d1: &mut [f32], d2: &mut [f32], b1: &mut [f32], b2: &[f32], P: &mut [f32]) {
            unsafe { ::ffi::cblas_srotmg(d1.as_mut_ptr(), d2.as_mut_ptr(), b1.as_mut_ptr(), b2.as_ptr(), P.as_mut_ptr()) }
        }

        pub fn srot(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32, c: f32, s: f32) {
            unsafe { ::ffi::cblas_srot(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, c, s) }
        }

        pub fn srotm(N: i32, x: &mut [f32], incx: i32, y: &mut [f32], incy: i32, p: &[f32]) {
            unsafe { ::ffi::cblas_srotm(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, p.as_ptr()) }
        }

        pub fn drotg(a: &mut [f64], b: &mut [f64], c: &mut [f64], s: &mut [f64]) {
            unsafe { ::ffi::cblas_drotg(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), s.as_mut_ptr()) }
        }

        pub fn drotmg(d1: &mut [f64], d2: &mut [f64], b1: &mut [f64], b2: &[f64], P: &mut [f64]) {
            unsafe { ::ffi::cblas_drotmg(d1.as_mut_ptr(), d2.as_mut_ptr(), b1.as_mut_ptr(), b2.as_ptr(), P.as_mut_ptr()) }
        }

        pub fn drot(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32, c: f64, s: f64) {
            unsafe { ::ffi::cblas_drot(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, c, s) }
        }

        pub fn drotm(N: i32, x: &mut [f64], incx: i32, y: &mut [f64], incy: i32, p: &[f64]) {
            unsafe { ::ffi::cblas_drotm(N, x.as_mut_ptr(), incx, y.as_mut_ptr(), incy, p.as_ptr()) }
        }

        /// Multiple each element of a matrix/vector by a constant.
        ///
        /// __Postcondition__: Every incX'th element of X has been multiplied by a factor of alpha
        ///
        /// __Parameters__:
        ///
        /// * N : number of elements in x to scale
        /// * alpha : factor to scale by
        /// * X : pointer to the vector/matrix data
        /// * incx : Amount to increment counter after each scaling, ie incX=2 mean to scale elements {1,3,...}
        ///
        /// Note that the allocated length of X must be incX*N-1 as N indicates the number of scaling operations to perform.
        pub fn sscal(N: i32, alpha: f32, x: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_sscal(N, alpha, x.as_mut_ptr(), incx) }
        }

        /// Multiple each element of a matrix/vector by a constant.
        pub fn dscal(N: i32, alpha: f64, x: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dscal(N, alpha, x.as_mut_ptr(), incx) }
        }

        /// Multiple each element of a matrix/vector by a constant.
        pub fn cscal<T>(N: i32, alpha: &[T], x: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_cscal(N, alpha.as_ptr() as *const ::libc::c_void, x.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }

        /// Multiple each element of a matrix/vector by a constant. 
        pub fn zscal<T>(N: i32, alpha: &[T], x: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_zscal(N, alpha.as_ptr() as *const ::libc::c_void, x.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }

        /// Multiple each element of a matrix/vector by a constant.
        pub fn csscal<T>(N: i32, alpha: f32, x: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_csscal(N, alpha, x.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }

        /// Multiple each element of a matrix/vector by a constant.
        pub fn zdscal<T>(N: i32, alpha: f64, x: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_zdscal(N, alpha, x.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }
    }

    pub mod Level2 {
        /// Multiplies a matrix and a vector.
        ///
        /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor
        /// * transA :  Whether to transpose matrix A. One of enum CblasNoTrans, CBlasTrans.
        /// * M : Rows in matrix A
        /// * N : Columns in matrix A
        /// * alpha : scalar factor for (sigma * op(A) * x)
        /// * A : matrix A
        /// * lda : The size of the first dimension of matrix A
        /// * X : vector X
        /// * incx : use every incX'th element of X
        /// * beta : scalar factor y
        /// * Y : vector Y
        /// * incy : use every incY'th element of Y
        ///
        /// For parameter lda, if you are passing a matrix A[m][n], the value of parameter lda should be m.
        pub fn sgemv(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, alpha: f32, A: &[f32],
            lda: i32, X: &[f32], incx: i32, beta: f32, Y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_sgemv(order, transA, M, N, alpha, A.as_ptr(), lda, X.as_ptr(), incx, beta, Y.as_mut_ptr(), incy) }
        }

        pub fn sgbmv(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, KL: i32, KU: i32,
            alpha: f32, A: &[f32], lda: i32, X: &[f32], incx: i32, beta: f32, Y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_sgbmv(order, transA, M, N, KL, KU, alpha, A.as_ptr(), lda, X.as_ptr(), incx, beta, Y.as_mut_ptr(), incy) }
        }

        pub fn strmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[f32], lda: i32, X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_strmv(order, uplo, transA, diag, N, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn stbmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[f32], lda: i32, X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_stbmv(order, uplo, transA, diag, N, K, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn stpmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[f32], X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_stpmv(order, uplo, transA, diag, N, Ap.as_ptr(), X.as_mut_ptr(), incx) }
        }

        pub fn strsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[f32], lda: i32, X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_strsv(order, uplo, transA, diag, N, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn stbsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[f32], lda: i32, X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_stbsv(order, uplo, transA, diag, N, K, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn stpsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[f32], X: &mut [f32], incx: i32) {
            unsafe { ::ffi::cblas_stpsv(order, uplo, transA, diag, N, Ap.as_ptr(), X.as_mut_ptr(), incx) }
        }

        pub fn dgemv(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, alpha: f64, A: &[f64],
            lda: i32, X: &[f64], incx: i32, beta: f64, Y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dgemv(order, transA, M, N, alpha, A.as_ptr(), lda, X.as_ptr(), incx, beta, Y.as_mut_ptr(), incy) }
        }

        pub fn dgbmv(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, KL: i32, KU: i32,
            alpha: f64, A: &[f64], lda: i32, X: &[f64], incx: i32, beta: f64, Y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dgbmv(order, transA, M, N, KL, KU, alpha, A.as_ptr(), lda, X.as_ptr(), incx, beta, Y.as_mut_ptr(), incy) }
        }

        pub fn dtrmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[f64], lda: i32, X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtrmv(order, uplo, transA, diag, N, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn dtbmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[f64], lda: i32, X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtbmv(order, uplo, transA, diag, N, K, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn dtpmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[f64], X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtpmv(order, uplo, transA, diag, N, Ap.as_ptr(), X.as_mut_ptr(), incx) }
        }

        pub fn dtrsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[f64], lda: i32, X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtrsv(order, uplo, transA, diag, N, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn dtbsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[f64], lda: i32, X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtbsv(order, uplo, transA, diag, N, K, A.as_ptr(), lda, X.as_mut_ptr(), incx) }
        }

        pub fn dtpsv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[f64], X: &mut [f64], incx: i32) {
            unsafe { ::ffi::cblas_dtpsv(order, uplo, transA, diag, N, Ap.as_ptr(), X.as_mut_ptr(), incx) }
        }

        pub fn cgemv<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, alpha: &[T], A: &[T],
            lda: i32, X: &[T], incx: i32, beta: &[T], Y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_cgemv(order, transA, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                X.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, Y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn cgbmv<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, KL: i32, KU: i32,
            alpha: &[T], A: &[T], lda: i32, X: &[T], incx: i32, beta: &[T], Y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_cgbmv(order, transA, M, N, KL, KU, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, X.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, Y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn ctrmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctrmv(order, uplo, transA, diag, N, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ctbmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctbmv(order, uplo, transA, diag, N, K, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ctpmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[T], X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctpmv(order, uplo, transA, diag, N, Ap.as_ptr() as *const ::libc::c_void, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ctrsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctrsv(order, uplo, transA, diag, N, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ctbsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctbsv(order, uplo, transA, diag, N, K, A.as_ptr() as *const ::libc::c_void, lda, X.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ctpsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[T], X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ctpsv(order, uplo, transA, diag, N, Ap.as_ptr() as *const ::libc::c_void, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn zgemv<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, alpha: &[T], A: &[T],
            lda: i32, X: &[T], incx: i32, beta: &[T], Y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zgemv(order, transA, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                X.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, Y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zgbmv<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, M: i32, N: i32, KL: i32, KU: i32,
            alpha: &[T], A: &[T], lda: i32, X: &[T], incx: i32, beta: &[T], Y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zgbmv(order, transA, M, N, KL, KU, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, X.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, Y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn ztrmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztrmv(order, uplo, transA, diag, N, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ztbmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztbmv(order, uplo, transA, diag, N, K, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ztpmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[T], X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztpmv(order, uplo, transA, diag, N, Ap.as_ptr() as *const ::libc::c_void, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ztrsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztrsv(order, uplo, transA, diag, N, A.as_ptr() as *const ::libc::c_void, lda, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ztbsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, K: i32, A: &[T], lda: i32, X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztbsv(order, uplo, transA, diag, N, K, A.as_ptr() as *const ::libc::c_void, lda, X.as_mut_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ztpsv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, N: i32, Ap: &[T], X: &mut [T], incx: i32) {
            unsafe { ::ffi::cblas_ztpsv(order, uplo, transA, diag, N, Ap.as_ptr() as *const ::libc::c_void, X.as_ptr() as *mut ::libc::c_void, incx) }
        }

        pub fn ssymv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, A: &[f32], lda: i32, x: &[f32],
            incx: i32, beta: f32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_ssymv(order, uplo, N, alpha, A.as_ptr(), lda, x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }
        
        pub fn ssbmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, K: i32, alpha: f32, A: &[f32], lda: i32,
            x: &[f32], incx: i32, beta: f32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_ssbmv(order, uplo, N, K, alpha, A.as_ptr(), lda, x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }

        pub fn sspmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, Ap: &[f32], x: &[f32],
            incx: i32, beta: f32, y: &mut [f32], incy: i32) {
            unsafe { ::ffi::cblas_sspmv(order, uplo, N, alpha, Ap.as_ptr(), x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }

        pub fn sger(order: ::enums::CblasOrder, M: i32, N: i32, alpha: f32, x: &[f32],
            incx: i32, y: &[f32], incy: i32, A: &mut [f32], lda: i32) {
            unsafe { ::ffi::cblas_sger(order, M, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr(), lda) }
        }

        pub fn ssyr(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, x: &[f32],
            incx: i32, A: &mut [f32], lda: i32) {
            unsafe { ::ffi::cblas_ssyr(order, uplo, N, alpha, x.as_ptr(), incx, A.as_mut_ptr(), lda) }
        }

        pub fn sspr(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, x: &[f32],
            incx: i32, Ap: &mut [f32]) {
            unsafe { ::ffi::cblas_sspr(order, uplo, N, alpha, x.as_ptr(), incx, Ap.as_mut_ptr()) }
        }

        pub fn ssyr2(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, x: &[f32],
            incx: i32, y: &[f32], incy: i32, A: &mut [f32], lda: i32) {
            unsafe { ::ffi::cblas_ssyr2(order, uplo, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr(), lda) }
        }

        pub fn sspr2(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f32, x: &[f32],
            incx: i32, y: &[f32], incy: i32, A: &mut [f32]) {
            unsafe { ::ffi::cblas_sspr2(order, uplo, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr()) }
        }

        pub fn dsymv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, A: &[f64], lda: i32, x: &[f64],
            incx: i32, beta: f64, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dsymv(order, uplo, N, alpha, A.as_ptr(), lda, x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }
        
        pub fn dsbmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, K: i32, alpha: f64, A: &[f64], lda: i32,
            x: &[f64], incx: i32, beta: f64, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dsbmv(order, uplo, N, K, alpha, A.as_ptr(), lda, x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }

        pub fn dspmv(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, Ap: &[f64], x: &[f64],
            incx: i32, beta: f64, y: &mut [f64], incy: i32) {
            unsafe { ::ffi::cblas_dspmv(order, uplo, N, alpha, Ap.as_ptr(), x.as_ptr(), incx, beta, y.as_mut_ptr(), incy) }
        }

        pub fn dger(order: ::enums::CblasOrder, M: i32, N: i32, alpha: f64, x: &[f64],
            incx: i32, y: &[f64], incy: i32, A: &mut [f64], lda: i32) {
            unsafe { ::ffi::cblas_dger(order, M, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr(), lda) }
        }

        pub fn dsyr(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, x: &[f64],
            incx: i32, A: &mut [f64], lda: i32) {
            unsafe { ::ffi::cblas_dsyr(order, uplo, N, alpha, x.as_ptr(), incx, A.as_mut_ptr(), lda) }
        }

        pub fn dspr(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, x: &[f64],
            incx: i32, Ap: &mut [f64]) {
            unsafe { ::ffi::cblas_dspr(order, uplo, N, alpha, x.as_ptr(), incx, Ap.as_mut_ptr()) }
        }

        pub fn dsyr2(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, x: &[f64],
            incx: i32, y: &[f64], incy: i32, A: &mut [f64], lda: i32) {
            unsafe { ::ffi::cblas_dsyr2(order, uplo, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr(), lda) }
        }

        pub fn dspr2(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: f64, x: &[f64],
            incx: i32, y: &[f64], incy: i32, A: &mut [f64]) {
            unsafe { ::ffi::cblas_dspr2(order, uplo, N, alpha, x.as_ptr(), incx, y.as_ptr(), incy, A.as_mut_ptr()) }
        }

        pub fn csymv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], A: &[T], lda: i32, x: &[T],
            incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_csymv(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }
        
        pub fn csbmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, K: i32, alpha: &[T], A: &[T], lda: i32,
            x: &[T], incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_csbmv(order, uplo, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn cspmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], Ap: &[T], x: &[T],
            incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_cspmv(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, Ap.as_ptr() as *const ::libc::c_void,
                x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn cgeru<T>(order: ::enums::CblasOrder, M: i32, N: i32, alpha: &[T], x: &[T], incx: i32, y: &[T],
            incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_cgeru(order, M, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn cgerc<T>(order: ::enums::CblasOrder, M: i32, N: i32, alpha: &[T], x: &[T], incx: i32, y: &[T],
            incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_cgerc(order, M, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn cher<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_cher(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn chpr<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, Ap: &mut [T]) {
            unsafe { ::ffi::cblas_chpr(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                Ap.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn cher2<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, y: &[T], incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_cher2(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn chpr2<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, y: &[f64], incy: i32, Ap: &mut [f64]) {
            unsafe { ::ffi::cblas_chpr2(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, Ap.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn zsymv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], A: &[T], lda: i32, x: &[T],
            incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zsymv(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }
        
        pub fn zsbmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, K: i32, alpha: &[T], A: &[T], lda: i32,
            x: &[T], incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zsbmv(order, uplo, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zspmv<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], Ap: &[T], x: &[T],
            incx: i32, beta: &[T], y: &mut [T], incy: i32) {
            unsafe { ::ffi::cblas_zspmv(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, Ap.as_ptr() as *const ::libc::c_void,
                x.as_ptr() as *const ::libc::c_void, incx, beta.as_ptr() as *const ::libc::c_void, y.as_mut_ptr() as *mut ::libc::c_void, incy) }
        }

        pub fn zgeru<T>(order: ::enums::CblasOrder, M: i32, N: i32, alpha: &[T], x: &[T], incx: i32, y: &[T],
            incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_zgeru(order, M, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn zgerc<T>(order: ::enums::CblasOrder, M: i32, N: i32, alpha: &[T], x: &[T], incx: i32, y: &[T],
            incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_zgerc(order, M, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn zher<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_zher(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn zhpr<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, Ap: &mut [T]) {
            unsafe { ::ffi::cblas_zhpr(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                Ap.as_mut_ptr() as *mut ::libc::c_void) }
        }

        pub fn zher2<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, y: &[T], incy: i32, A: &mut [T], lda: i32) {
            unsafe { ::ffi::cblas_zher2(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, A.as_mut_ptr() as *mut ::libc::c_void, lda) }
        }

        pub fn zhpr2<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, N: i32, alpha: &[T], x: &[T],
            incx: i32, y: &[f64], incy: i32, Ap: &mut [f64]) {
            unsafe { ::ffi::cblas_zhpr2(order, uplo, N, alpha.as_ptr() as *const ::libc::c_void, x.as_ptr() as *const ::libc::c_void, incx,
                y.as_ptr() as *const ::libc::c_void, incy, Ap.as_mut_ptr() as *mut ::libc::c_void) }
        }
    }

    pub mod Level3 {
        /// General Matrix-Matrix multiplication for single precision float.
        /// 
        /// __Parameters:__
        /// 
        /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor.
        /// * transA : Whether to transpose matrix A. One of enum CblasNoTrans, CBlasTrans, CBlasConjTrans.
        /// * transB : Whether to transpose matrix B. One of enum CblasNoTrans, CBlasTrans, CBlasConjTrans.
        /// * M : Rows in matrices A and C
        /// * N : Columns in Matrices B and C
        /// * K : Columns in matrix A and Rows in matrix B
        /// * alpha : scalar factor for op(A)op(B)
        /// * A : matrix A
        /// * lda : The size of the first dimension of matrix A
        /// * B : matrix B
        /// * ldb : The size of the first dimension of matrix B
        /// * beta : scalar factor for C
        /// * C : matrix C
        /// * ldc : The size of the first dimension of matrix C
        /// 
        /// For parameters lda, ldb, and ldc, if you are passing a matrix D[m][n], the value of parameter lda, ldb, or ldc should be m.
        pub fn sgemm(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, transB: ::enums::CblasTranspose, M: i32, N: i32,
            K: i32, alpha: f32, A: &[f32], lda: i32, B: &[f32], ldb: i32, beta: f32, C: &mut [f32], ldc: i32) {
            unsafe { ::ffi::cblas_sgemm(order, transA, transB, M, N, K, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }
        
        /// Symmetric Matrix-Matrix multiplication for single precision float.
        /// 
        /// __Parameters:__
        /// 
        /// * order : Whether matrices are row major order (C-Style) for column major order (Fortran-style). One of enum CblasRowMajor or CblasColMajor.
        /// * side : If CBlasSideLeft, perform (sigma(A)(B) + beta C). If CBlasSideRight, perform (sigma (B)(A) + beta C)
        /// * uplo : Indicates whether to use the upper (CBlasUpper) or lower (CBlasLower) triangle of matrix A
        /// * M : Rows in matrices A and C
        /// * N : Columns in Matrices B and C
        /// * alpha : scalar factor for op(A)op(B)
        /// * A : matrix A
        /// * lda : The size of the first dimension of matrix A
        /// * B : matrix B
        /// * ldb : The size of the first dimension of matrix B
        /// * beta : scalar factor for C
        /// * C : matrix C
        /// * ldc : The size of the first dimension of matrix C
        pub fn ssymm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: f32,
            A: &[f32], lda: i32, B: &[f32], ldb: i32, beta: f32, C: &mut [f32], ldc: i32) {
            unsafe { ::ffi::cblas_ssymm(order, side, uplo, M, N, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }

        pub fn ssyrk(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: f32,
            A: &[f32], lda: i32, beta: f32, C: &mut [f32], ldc: i32) {
            unsafe { ::ffi::cblas_ssyrk(order, uplo, trans, N, K, alpha, A.as_ptr(), lda, beta, C.as_mut_ptr(), ldc) }
        }

        pub fn ssyr2k(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: f32,
            A: &[f32], lda: i32, B: &[f32], ldb: i32, beta: f32, C: &mut [f32], ldc: i32) {
            unsafe { ::ffi::cblas_ssyr2k(order, uplo, trans, N, K, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }
        
        pub fn strmm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: f32, A: &[f32], lda: i32, B: &mut [f32], ldb: i32) {
            unsafe { ::ffi::cblas_strmm(order, side, uplo, transA, diag, M, N, alpha, A.as_ptr(), lda, B.as_mut_ptr(), ldb) }
        }
        
        pub fn strsm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: f32, A: &[f32], lda: i32, B: &mut [f32], ldb: i32) {
            unsafe { ::ffi::cblas_strsm(order, side, uplo, transA, diag, M, N, alpha, A.as_ptr(), lda, B.as_mut_ptr(), ldb) }
        }

        pub fn dgemm(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, transB: ::enums::CblasTranspose, M: i32, N: i32,
            K: i32, alpha: f64, A: &[f64], lda: i32, B: &[f64], ldb: i32, beta: f64, C: &mut [f64], ldc: i32) {
            unsafe { ::ffi::cblas_dgemm(order, transA, transB, M, N, K, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }

        pub fn dsymm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: f64,
            A: &[f64], lda: i32, B: &[f64], ldb: i32, beta: f64, C: &mut [f64], ldc: i32) {
            unsafe { ::ffi::cblas_dsymm(order, side, uplo, M, N, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }

        pub fn dsyrk(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: f64,
            A: &[f64], lda: i32, beta: f64, C: &mut [f64], ldc: i32) {
            unsafe { ::ffi::cblas_dsyrk(order, uplo, trans, N, K, alpha, A.as_ptr(), lda, beta, C.as_mut_ptr(), ldc) }
        }

        pub fn dsyr2k(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: f64,
            A: &[f64], lda: i32, B: &[f64], ldb: i32, beta: f64, C: &mut [f64], ldc: i32) {
            unsafe { ::ffi::cblas_dsyr2k(order, uplo, trans, N, K, alpha, A.as_ptr(), lda, B.as_ptr(), ldb, beta, C.as_mut_ptr(), ldc) }
        }
        
        pub fn dtrmm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: f64, A: &[f64], lda: i32, B: &mut [f64], ldb: i32) {
            unsafe { ::ffi::cblas_dtrmm(order, side, uplo, transA, diag, M, N, alpha, A.as_ptr(), lda, B.as_mut_ptr(), ldb) }
        }
        
        pub fn dtrsm(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: f64, A: &[f64], lda: i32, B: &mut [f64], ldb: i32) {
            unsafe { ::ffi::cblas_dtrsm(order, side, uplo, transA, diag, M, N, alpha, A.as_ptr(), lda, B.as_mut_ptr(), ldb) }
        }

        pub fn cgemm<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, transB: ::enums::CblasTranspose, M: i32, N: i32,
            K: i32, alpha: &[T], A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_cgemm(order, transA, transB, M, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn csymm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_csymm(order, side, uplo, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn csyrk<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: &[T],
            A: &[T], lda: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_csyrk(order, uplo, trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn csyr2k<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_csyr2k(order, uplo, trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }
        
        pub fn ctrmm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: &[T], A: &[T], lda: i32, B: &mut [T], ldb: i32) {
            unsafe { ::ffi::cblas_ctrmm(order, side, uplo, transA, diag, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_mut_ptr() as *mut ::libc::c_void, ldb) }
        }
        
        pub fn ctrsm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: &[T], A: &[T], lda: i32, B: &mut [T], ldb: i32) {
            unsafe { ::ffi::cblas_ctrsm(order, side, uplo, transA, diag, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_mut_ptr() as *mut ::libc::c_void, ldb) }
        }

        pub fn zgemm<T>(order: ::enums::CblasOrder, transA: ::enums::CblasTranspose, transB: ::enums::CblasTranspose, M: i32, N: i32,
            K: i32, alpha: &[T], A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zgemm(order, transA, transB, M, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn zsymm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zsymm(order, side, uplo, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn zsyrk<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: &[T],
            A: &[T], lda: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zsyrk(order, uplo, trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn zsyr2k<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zsyr2k(order, uplo, trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn ztrmm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: &[T], A: &[T], lda: i32, B: &mut [T], ldb: i32) {
            unsafe { ::ffi::cblas_ztrmm(order, side, uplo, transA, diag, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_mut_ptr() as *mut ::libc::c_void, ldb) }
        }

        pub fn ztrsm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, transA: ::enums::CblasTranspose,
            diag: ::enums::CblasDiag, M: i32, N: i32, alpha: &[T], A: &[T], lda: i32, B: &mut [T], ldb: i32) {
            unsafe { ::ffi::cblas_ztrsm(order, side, uplo, transA, diag, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_mut_ptr() as *mut ::libc::c_void, ldb) }
        }

        pub fn chemm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_chemm(order, side, uplo, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn cherk<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32,
            alpha: &[T],  A: &[T], lda: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_cherk(order, uplo,trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn cher2k<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32,
            alpha: &[T],  A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_cher2k(order, uplo,trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn zhemm<T>(order: ::enums::CblasOrder, side: ::enums::CblasSide, uplo: ::enums::CblasUplo, M: i32, N: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zhemm(order, side, uplo, M, N, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void,
                lda, B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }
    
        pub fn zherk<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: f64,
            A: &[T], lda: i32, beta: f64, C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zherk(order, uplo, trans, N, K, alpha, A.as_ptr() as *const ::libc::c_void, lda, beta,
                C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }

        pub fn zher2k<T>(order: ::enums::CblasOrder, uplo: ::enums::CblasUplo, trans: ::enums::CblasTranspose, N: i32, K: i32, alpha: &[T],
            A: &[T], lda: i32, B: &[T], ldb: i32, beta: &[T], C: &mut [T], ldc: i32) {
            unsafe { ::ffi::cblas_zher2k(order, uplo, trans, N, K, alpha.as_ptr() as *const ::libc::c_void, A.as_ptr() as *const ::libc::c_void, lda,
                B.as_ptr() as *const ::libc::c_void, ldb, beta.as_ptr() as *const ::libc::c_void, C.as_mut_ptr() as *mut ::libc::c_void, ldc) }
        }
    }
}