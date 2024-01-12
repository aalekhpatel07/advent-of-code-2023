#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec<T, const N: usize> {
    inner: [T; N]
}

impl<T, const N: usize> AsRef<[T; N]> for Vec<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.inner
    }
}

impl<T, const N: usize> Vec<T, N> 
where
    T: Copy + Default
{
    pub fn try_from_slice(data: &[T]) -> Result<Self, String> {
        let iter = data.into_iter().copied();
        Self::try_from_iter(iter)
    }
    pub fn try_from_iter<I>(mut iter: I) -> Result<Self, String> 
    where
        I: Iterator<Item=T>
    {

        let mut arr: [T; N] = [Default::default(); N];

        for elem in arr.iter_mut() {
            let Some(item) = iter.next() else {
                return Err("not enough items".to_string());
            };
            *elem = item; 
        }

        Ok(Self { inner: arr })
    }
}

impl<T> Vec<T, 3>
where
    T: Copy
{
    pub fn new(a: T, b: T, c: T) -> Self {
        Self {
            inner: [a, b, c]
        }
    }
}


impl<T, const N: usize> std::fmt::Display for Vec<T, N> 
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.inner.iter().map(|v| v.to_string()).collect::<std::vec::Vec<_>>().join(","))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T, const N: usize, const M: usize> {
    inner: [[T; M]; N]
}

impl<T, const N: usize, const M: usize> std::fmt::Display for Matrix<T, N, M> 
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = write!(f, "");
        for row in self.inner.iter() {
            _ = write!(f, "");
            for val in row {
                _ = write!(f, "{} ", val);
            }
            _ = write!(f, "\n");
        }
        writeln!(f, "")
    }
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(arr: [[T; M]; N]) -> Self {
        Self {
            inner: arr
        }
    }
}


impl<T, const N: usize, const M: usize> Matrix<T, N, M> 
where
    T: Copy + Default
{
    pub fn try_from_blocks<const N1: usize, const M1: usize>(
        top_left: &Matrix<T, N1, M1>,
        top_right: &Matrix<T, N1, M1>,
        bottom_left: &Matrix<T, N1, M1>,
        bottom_right: &Matrix<T, N1, M1>,
    ) -> Result<Self, String> {
        if !(2 * N1 == N && 2 * M1 == M) {
            return Err("Blocks are not even. Expected exactly 4 symmetric blocks.".to_string());
        };
        let mut arr = [[Default::default(); M]; N];

        for row_idx in 0..N1 {
            for col_idx in 0..M1 {
                arr[row_idx][col_idx] = top_left.inner[row_idx][col_idx];
            }
        }
        for row_idx in 0..N1 {
            for col_idx in 0..M1 {
                arr[row_idx][col_idx + M1] = top_right.inner[row_idx][col_idx];
            }
        }

        for row_idx in 0..N1 {
            for col_idx in 0..M1 {
                arr[row_idx + N1][col_idx] = bottom_left.inner[row_idx][col_idx];
            }
        }
        for row_idx in 0..N1 {
            for col_idx in 0..M1 {
                arr[row_idx + N1][col_idx + M1] = bottom_right.inner[row_idx][col_idx];
            }
        }

        Ok(Self { inner: arr })
    }

    pub fn get_block<const N1: usize, const M1: usize>(&self, row: usize, col: usize) -> Result<Matrix<T, N1, M1>, String> {
        if !(row + N1 <= N && col + M1 <= M) {
            return Err("block falls outside the matrix.".to_string());
        }

        let mut data = vec![];
        for row_idx in row..(row + N1) {
            for col_idx in col..(col + M1) {
                data.push(self.inner[row_idx][col_idx]);
            }
        }

        Matrix::<T, N1, M1>::try_from_slice(&data)
    }
}
impl<T, const N: usize, const M: usize> Matrix<T, N, M> 
where
    T: Copy + Default
{
    pub fn try_from_slice(data: &[T]) -> Result<Self, String> {
        let mut iter = data.into_iter();
        let mut arr: [[T; M]; N] = [[T::default(); M]; N];

        for idx in 0..(N * M) {
            let (row, col) = (idx / N, idx % M);
            let Some(item) = iter.next().cloned() else {
                return Err("Not enough items".to_string());
            };
            arr[row][col] = item;
        }

        Ok(Self {
            inner: arr
        })
    }
}

impl<T, const N: usize, const M: usize> std::ops::Add for Matrix<T, N, M> 
where
    T: std::ops::Add<Output=T> + Copy + Clone + Default
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {

        let mut res: [[T; M]; N] = [[Default::default(); M]; N];

        for idx in 0..M * N {
            let (row, col) = (idx / N, idx % M);
            res[row][col] = self.inner[row][col] + rhs.inner[row][col]
        }

        Self {
            inner: res
        }
    }
}


impl<T, const N: usize, const M: usize> std::ops::Div<T> for Matrix<T, N, M> 
where
    T: std::ops::Div<Output=T> + Copy + Clone + Default
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {

        let mut res: [[T; M]; N] = [[Default::default(); M]; N];

        for idx in 0..M * N {
            let (row, col) = (idx / N, idx % M);
            res[row][col] = self.inner[row][col] / rhs;
        }

        Self {
            inner: res
        }
    }
}



impl<T, const N: usize, const M: usize> std::ops::Sub for Matrix<T, N, M> 
where
    T: std::ops::Sub<Output=T> + Copy + Clone + Default
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {

        let mut res: [[T; M]; N] = [[Default::default(); M]; N];

        for idx in 0..M * N {
            let (row, col) = (idx / N, idx % M);
            res[row][col] = self.inner[row][col] - rhs.inner[row][col]
        }

        Self {
            inner: res
        }
    }
}

pub trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

pub trait One: Sized {
    fn one() -> Self;
    fn is_one(&self) -> bool;
    fn set_one(&mut self) {
        *self = One::one();
    }
}


impl Zero for f64 {
    fn zero() -> Self {
        0.
    }
    fn is_zero(&self) -> bool {
        self.abs() == 0.
    }
    fn set_zero(&mut self) {
        
    }
}

impl One for f64 {
    fn one() -> Self {
        1.
    }
    fn is_one(&self) -> bool {
        *self == 1.
    }
    fn set_one(&mut self) {
        *self = One::one()
    }
}

fn invert_3d<T, const N: usize>(matrix: &Matrix<T, N, N>) -> Option<Matrix<T, N, N>>
where
    T: Copy + Clone + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Default + Zero + std::ops::Div<Output=T>
{
    if N != 3 {
        unreachable!("expected N to be 3.");
    }

    let r1 = matrix.inner[0].clone();
    let r2 = matrix.inner[1].clone();
    let r3 = matrix.inner[2].clone();

    let (r1x, r1y, r1z) = (r1[0], r1[1], r1[2]);
    let (r2x, r2y, r2z) = (r2[0], r2[1], r2[2]);
    let (r3x, r3y, r3z) = (r3[0], r3[1], r3[2]);

    let determinant = r1x * r2z * r3y + r1y * r2x * r3z + r1z * r2y * r3x;
    if determinant.is_zero() {
        return None;
    }

    let adj: Matrix<T, N, N> = Matrix::try_from_slice(&[
        r2y * r3z - r2z * r3y,
        r2z * r3x - r2x * r3z,
        r2x * r3y - r2y * r3x,
        
        r1z * r3y - r1y * r3z,
        r1x * r3z - r1z * r3x,
        r1y * r1x - r1x * r3y,

        r1y * r2z - r1z * r2y,
        r1z * r2x - r1x * r2z,
        r1x * r2y - r1y * r2x
    ]).unwrap();

    Some(adj / determinant)
}


fn invert_4d<T, const N: usize>(matrix: &Matrix<T, N, N>) -> Option<Matrix<T, N, N>>
where
    T: Copy + Clone + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Default + Zero + One + std::ops::Div<Output=T> + std::iter::Sum
{
    if N != 4 {
        unreachable!("expected N to be 4.");
    }

    let a = matrix.get_block::<2, 2>(0, 0).unwrap();
    let b = matrix.get_block::<2, 2>(0, 3).unwrap();
    let c = matrix.get_block::<2, 2>(2, 0).unwrap();
    let d = matrix.get_block::<2, 2>(2, 2).unwrap();

    let Some(a_inv) = a.inverse() else {
        return None;
    };
    let dcab = d - c.mul(&a_inv.mul(&b));
    let Some(dcab_inv) = dcab.inverse() else {
        return None;
    };

    let top_left = a_inv + (a_inv.mul(&b.mul(&dcab_inv).mul(&c).mul(&a_inv)));
    let top_right =  a_inv.mul(&b.mul(&dcab_inv)) / (T::zero() - T::one());
    let bottom_left = (dcab_inv.mul(&c.mul(&a_inv))) / (T::zero() - T::one());
    let bottom_right = dcab_inv;

    Matrix::try_from_blocks::<2, 2>(
        &top_left, 
        &top_right, 
        &bottom_left, 
        &bottom_right
    ).ok()
}

fn invert_6d<T, const N: usize>(matrix: &Matrix<T, N, N>) -> Option<Matrix<T, N, N>>
where
    T: Copy + Clone + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Default + Zero + One + std::ops::Div<Output=T> + std::iter::Sum
{
    if N != 6 {
        unreachable!("expected N to be 6.");
    }

    let a = matrix.get_block::<3, 3>(0, 0).unwrap();
    let b = matrix.get_block::<3, 3>(0, 3).unwrap();
    let c = matrix.get_block::<3, 3>(3, 0).unwrap();
    let d = matrix.get_block::<3, 3>(3, 3).unwrap();

    let Some(a_inv) = a.inverse() else {
        return None;
    };
    let dcab = d - c.mul(&a_inv.mul(&b));
    let Some(dcab_inv) = dcab.inverse() else {
        return None;
    };

    let top_left = a_inv + (a_inv.mul(&b.mul(&dcab_inv).mul(&c).mul(&a_inv)));
    let top_right =  a_inv.mul(&b.mul(&dcab_inv)) / (T::zero() - T::one());
    let bottom_left = (dcab_inv.mul(&c.mul(&a_inv))) / (T::zero() - T::one());
    let bottom_right = dcab_inv;

    Matrix::try_from_blocks::<3, 3>(
        &top_left, 
        &top_right, 
        &bottom_left, 
        &bottom_right
    ).ok()
}

#[inline(always)]
fn invert_2d<T, const N: usize>(matrix: &Matrix<T, N, N>) -> Option<Matrix<T, N, N>>
where
    T: Copy + Clone + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Default + Zero + std::ops::Div<Output=T>
{
    if N != 2 {
        unreachable!("expected N to be 2.");
    }

    let r1 = matrix.inner[0].clone();
    let r2 = matrix.inner[1].clone();
    let determinant = r2[1] * r1[0] - r1[1] * r2[0];
    if determinant.is_zero() {
        return None;
    }

    let m: Matrix<T, N, N> = Matrix::try_from_slice(&[
        r2[1],
        T::zero() - r1[1],
        T::zero() - r2[0],
        r1[2]
    ]).unwrap();

    Some(m / determinant)
}



impl<T, const N: usize> Matrix<T, N, N> 
where
    T: Copy + Clone + Default + Zero + One + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Div<Output=T> + std::iter::Sum
{
    pub fn inverse(&self) -> Option<Self> {
        match N {
            0 => panic!("unsupported dimension."),
            1 => {
                if self.inner[0][0].is_zero() {
                    return None;
                }
                return Matrix::<T, N, N>::try_from_slice(&[T::one() / self.inner[0][0]]).ok()
            },
            2 => {
                return invert_2d(self);
            },
            3 => {
                return invert_3d(self);
            },
            4 => {
                return invert_4d(self);
            }
            6 => {
                return invert_6d(self);
            }
            _ => {

                unimplemented!("")
            }
        }
    }
}


impl<T, const N: usize> std::ops::Neg for Vec<T, N> 
where
    T: std::ops::Neg<Output = T> + Default + Copy
{
    type Output = Vec<T, N>;
    fn neg(self) -> Self::Output {
        Vec::try_from_iter(self.inner.map(|v| -v).into_iter()).unwrap()
    }
}

impl<T, const N: usize> std::ops::Div<f64> for Vec<T, N>
where
    T: std::ops::Div<f64, Output = T> + Copy + Default
{
    type Output = Vec<T, N>;
    fn div(self, rhs: f64) -> Self::Output {
        Vec::try_from_iter(self.inner.map(|v| v / rhs).into_iter()).unwrap()
    }
}

impl<T, const N: usize> std::ops::Add for Vec<T, N> 
where
    T: std::ops::Add<Output=T> + Copy + Default,
{
    type Output = Vec<T, N>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec::try_from_iter(self.inner.iter().zip(rhs.inner.iter()).map(|(&v, &u)| v + u)).unwrap()
    }
}

impl<T, const I: usize, const J: usize> Matrix<T, I, J> 
where
    T: std::ops::Add<Output=T> + Copy + Default + std::ops::Mul<Output=T> + Zero + std::iter::Sum
{
    pub fn mul<const K: usize>(self, rhs: &Matrix<T, J, K>) -> Matrix<T, I, K> {
        let mut res: [[T; K]; I] = [[Default::default(); K]; I];

        for i in 0..I {
            for k in 0..K {
                res[i][k] =
                (0..J)
                .into_iter()
                .map(|j| self.inner[i][j] * rhs.inner[j][k])
                .sum();
            }
        }

        Matrix { inner: res }
    }

}


impl<T, const N: usize> std::ops::AddAssign for Vec<T, N> 
where
    T: std::ops::AddAssign + Copy + Default,
{
    fn add_assign(&mut self, rhs: Self) {
        self.inner
        .iter_mut()
        .zip(rhs.inner.iter())
        .for_each(|(v, u)| {
            *v += *u
        });
    }
}

impl<T, const N: usize> std::ops::Sub for Vec<T, N> 
where
    T: std::ops::Sub<Output=T> + Copy + Default,
{
    type Output = Vec<T, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec::try_from_iter(self.inner.iter().zip(rhs.inner.iter()).map(|(&v, &u)| v - u)).unwrap()
    }
}

impl<T, const N: usize> std::ops::SubAssign for Vec<T, N> 
where
    T: std::ops::SubAssign + Copy + Default,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.inner
        .iter_mut()
        .zip(rhs.inner.iter())
        .for_each(|(v, u)| {
            *v -= *u
        });
    }
}


impl<T, const N: usize> Vec<T, N> 
where
    T: std::ops::Add<Output=T> + std::ops::Mul<Output = T> + Copy + std::iter::Sum
{
    pub fn dot(&self, other: &Vec<T, N>) -> T {
        self.inner
        .iter()
        .zip(other.inner.iter())
        .map(|(&v, &u)| v * u)
        .sum()
    }
}

impl<T> Vec<T, 3> 
where
    T: std::ops::Add<Output=T> + std::ops::Mul<Output = T> + std::ops::Sub<Output=T> + Copy
{
    pub fn cross(&self, other: &Vec<T, 3>) -> Vec<T, 3> {

        let x = self.inner[1] * other.inner[2] - other.inner[1] * self.inner[2];
        let y = self.inner[2] * other.inner[0] - other.inner[2] * self.inner[0];
        let z = self.inner[0] * other.inner[1] - other.inner[0] * self.inner[1];

        Vec::<T, 3>::new(x, y, z)
    }
}

impl<T, const N: usize> Into<[T; N]> for &Vec<T, N> 
where
    T: Copy
{
    fn into(self) -> [T; N] {
        self.inner
    }
}

