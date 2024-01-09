#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> std::fmt::Display for Vec3<T> 
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T> std::fmt::Display for M3x3<T> 
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}\n{}\n{}]", self.r1, self.r2, self.r3)
    }
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl<T> std::ops::Neg for Vec3<T> 
where
    T: std::ops::Neg<Output = T>
{
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)      
    }
}

impl<T> std::ops::Div<f64> for Vec3<T>
where
    T: std::ops::Div<f64, Output = T>,
{
    type Output = Vec3<T>;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> std::ops::Add for Vec3<T> 
where
    T: std::ops::Add<Output=T>,
{
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}


impl<T> std::ops::AddAssign for Vec3<T> 
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}


impl<T> std::ops::Sub for Vec3<T> 
where
    T: std::ops::Sub<Output=T>,
{
    type Output = Vec3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}


impl<T> std::ops::SubAssign for Vec3<T> 
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}


impl<T> Vec3<T> 
where
    T: std::ops::Add<Output=T> + std::ops::Mul<Output = T> + Copy
{
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x
        + self.y * other.y
        + self.z * other.z
    }
}

impl<T> Vec3<T> 
where
    T: std::ops::Add<Output=T> + std::ops::Mul<Output = T> + std::ops::Sub<Output=T> + Copy
{
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 { 
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }
}

impl<T> Into<[T; 3]> for &Vec3<T> 
where
    T: Copy
{
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}


#[derive(Debug, Clone, Copy)]
pub struct M3x3<T> {
    r1: Vec3<T>,
    r2: Vec3<T>,
    r3: Vec3<T>
}

impl<T> M3x3<T> 
where
    T: Copy
{
    pub fn new(r1: &Vec3<T>, r2: &Vec3<T>, r3: &Vec3<T>) -> Self {
        Self {
            r1: r1.clone(),
            r2: r2.clone(),
            r3: r3.clone()
        }
    }
}

impl M3x3<f64>
{   
    #[inline(always)]
    pub fn determinant(&self) -> f64 {
        self.r1.x * self.r2.y * self.r3.z 
        + self.r1.y * self.r2.z * self.r3.x
        + self.r1.z * self.r2.x * self.r3.y
        - (
            self.r1.x * self.r2.z * self.r3.y
            + self.r1.y * self.r2.x * self.r3.z
            + self.r1.z * self.r2.y * self.r3.x
        )
    }
    pub fn inverse(&self) -> Option<M3x3<f64>> {

        let determinant = self.determinant();
    
        if determinant == 0. {
            return None;
        }

        let adj = [
            Vec3::<f64>::new(
                self.r2.y * self.r3.z - self.r2.z * self.r3.y, 
                self.r2.z * self.r3.x - self.r2.x * self.r3.z,
                self.r2.x * self.r3.y - self.r2.y * self.r3.x
            ),
            Vec3::new(
                self.r1.z * self.r3.y - self.r1.y * self.r3.z,
                self.r1.x * self.r3.z - self.r1.z * self.r3.x, 
                self.r1.y * self.r1.x - self.r1.x * self.r3.y
            ),
            Vec3::new(
                self.r1.y * self.r2.z - self.r1.z * self.r2.y, 
                self.r1.z * self.r2.x - self.r1.x * self.r2.z, 
                self.r1.x * self.r2.y - self.r1.y * self.r2.x
            )
        ];

        Some(Self::new(&(adj[0] / determinant), &(adj[1] / determinant), &(adj[2] / determinant)))
    }
    // pub fn 
}


impl<T> std::ops::Mul<&Vec3<T>> for M3x3<T> 
where
    T: 
        std::ops::Add<Output=T> 
        + std::ops::Sub<Output=T> 
        + std::ops::Mul<Output=T> 
        + Copy
{
    type Output = Vec3<T>;
    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3::new(
            (&self.r1).dot(&rhs),
            (&self.r2).dot(&rhs),
            (&self.r3).dot(&rhs),
        )
    }
}