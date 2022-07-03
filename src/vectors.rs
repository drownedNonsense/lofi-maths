//#########################
// D E P E N D E N C I E S
//#########################

    use std::ops::{
        Neg,
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
    }; // use ..

    use crate::traits::{Number, Signed, Unsigned, Trigonometry, Sqrt};
    use crate::Angle;


//#############
// M A C R O S
//#############

    macro_rules! impl_vec_left_mul(
        ($($T: ty),*$(,)*) => {$(

            impl Mul<Vec2<$T>> for $T {
                type Output = Vec2<$T>;
                fn mul(self, other: Vec2<$T>) -> Self::Output {
                    other * self
                } // fn mul()
            } // impl Mul ..


            impl Mul<Vec3<$T>> for $T {
                type Output = Vec3<$T>;
                fn mul(self, other: Vec3<$T>) -> Self::Output {
                    other * self
                } // fn mul()
            } // impl Mul ..


            impl Mul<Vec4<$T>> for $T {
                type Output = Vec4<$T>;
                fn mul(self, other: Vec4<$T>) -> Self::Output {
                    other * self
                } // fn mul()
            } // impl Mul ..

        )*}
    ); // impl_vec_left_mul()


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Vec2<T: Number>(pub T, pub T);


    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Vec3<T: Number>(pub T, pub T, pub T);


    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Vec4<T: Number>(pub T, pub T, pub T, pub T);


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl_vec_left_mul!(u8, u16, u32, i8, i16, i32, f32);


    //#########
    // V E C 2
    //#########

    impl<T: Number> Vec2<T> {
        pub fn x(&self) -> T { self.0 }
        pub fn y(&self) -> T { self.1 }
        pub fn dot(a: Self, b: Self) -> T { a.0 * b.0 + a.1 * b.1 }
        pub fn squared_magnitude(&self) -> T { self.0 * self.0 + self.1 * self.1 }
    } // impl Vec2


    impl<T: Unsigned> Vec2<T> {
        pub fn uvec2_magnitude(&self)           -> T where T: Sqrt { (self.0 * self.0 + self.1 * self.1).sqrt() }
        pub fn uvec2_distance(a: Self, b: Self) -> T where T: Sqrt {{ if b > a { b - a } else { a - b }}.uvec2_magnitude() }
    } // impl Vec2


    impl<T: Signed> Vec2<T> {
        pub fn ivec2_magnitude(&self)           -> T::Unsigned where T: Into<T::Unsigned>, T::Unsigned: Sqrt { ((self.0 * self.0 + self.1 * self.1).into()).sqrt() }
        pub fn ivec2_distance(a: Self, b: Self) -> T::Unsigned where T: Into<T::Unsigned>, T::Unsigned: Sqrt { (b - a).ivec2_magnitude() }
    } // impl Vec2


    impl Vec2<f32> {
        pub fn vec2_magnitude(&self)           -> f32 { (self.0 * self.0 + self.1 * self.1).sqrt() }
        pub fn vec2_distance(a: Self, b: Self) -> f32 { (b - a).vec2_magnitude() }
    } // impl Vec2


    impl From<Angle> for Vec2<f32> {
        /// Creates a `Vec2` from polar coordinates
        fn from(angle: Angle) -> Self {
            Vec2(angle.cos(), angle.sin())
        } // fn from()
    } // impl From ..


    impl<T: Signed> Neg for Vec2<T>  {
        type Output = Self;
        fn neg(self) -> Self::Output { Vec2(-self.0, -self.1) }
    } // impl Neg ..


    impl<T: Number> Add for Vec2<T>  {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { Vec2(self.0 + rhs.0, self.1 + rhs.1) }
    } // impl Add ..


    impl<T: Number> AddAssign for Vec2<T>  {
        fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; }
    } // impl AddAssign ..


    impl<T: Number> Sub for Vec2<T>  {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { Vec2(self.0 - rhs.0, self.1 - rhs.1) }
    } // impl Sub ..


    impl<T: Number> SubAssign for Vec2<T>  {
        fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; }
    } // impl AddAssign ..


    impl<T: Number> Mul<T> for Vec2<T>  {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output { Vec2(self.0 * rhs, self.1 * rhs) }
    } // impl Mul ..


    impl<T: Number> MulAssign<T> for Vec2<T>  {
        fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; self.1 *= rhs; }
    } // impl MulAssign ..


    impl<T: Number> Div<T> for Vec2<T>  {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output { Vec2(self.0 / rhs, self.1 / rhs) }
    } // impl Div ..


    impl<T: Number> DivAssign<T> for Vec2<T>  {
        fn div_assign(&mut self, rhs: T) { self.0 /= rhs; self.1 /= rhs; }
    } // impl DivAssign ..


    //#########
    // V E C 3
    //#########
    
    impl<T: Number> Vec3<T> {
        pub fn x(&self) -> T { self.0 }
        pub fn y(&self) -> T { self.1 }
        pub fn z(&self) -> T { self.2 }
        pub fn xy(&self) -> Vec2<T> { Vec2(self.0, self.1) }
        pub fn yz(&self) -> Vec2<T> { Vec2(self.1, self.2) }
        pub fn xz(&self) -> Vec2<T> { Vec2(self.0, self.2) }
    } // impl Vec3


    impl<T: Signed> Neg for Vec3<T>  {
        type Output = Self;
        fn neg(self) -> Self::Output { Vec3(-self.0, -self.1, -self.2) }
    } // impl Neg ..


    impl<T: Number> Add for Vec3<T>  {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
    } // impl Add ..


    impl<T: Number> AddAssign for Vec3<T>  {
        fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; self.2 += rhs.2; }
    } // impl AddAssign ..


    impl<T: Number> Sub for Vec3<T>  {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
    } // impl Sub ..


    impl<T: Number> SubAssign for Vec3<T>  {
        fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; self.2 -= rhs.2; }
    } // impl AddAssign ..


    impl<T: Number> Mul<T> for Vec3<T>  {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output { Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs) }
    } // impl Mul ..


    impl<T: Number> MulAssign<T> for Vec3<T>  {
        fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; self.1 *= rhs; self.2 *= rhs; }
    } // impl MulAssign ..


    impl<T: Number> Div<T> for Vec3<T>  {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output { Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs) }
    } // impl Div ..


    impl<T: Number> DivAssign<T> for Vec3<T>  {
        fn div_assign(&mut self, rhs: T) { self.0 /= rhs; self.1 /= rhs; self.2 /= rhs; }
    } // impl DivAssign ..


    //#########
    // V E C 4
    //#########

    impl<T: Number> Vec4<T> {
        pub fn x(&self) -> T { self.0 }
        pub fn y(&self) -> T { self.1 }
        pub fn z(&self) -> T { self.2 }
        pub fn w(&self) -> T { self.3 }
        pub fn xy(&self) -> Vec2<T> { Vec2(self.0, self.1) }
        pub fn xz(&self) -> Vec2<T> { Vec2(self.0, self.2) }
        pub fn xw(&self) -> Vec2<T> { Vec2(self.0, self.3) }
        pub fn yz(&self) -> Vec2<T> { Vec2(self.1, self.2) }
        pub fn yw(&self) -> Vec2<T> { Vec2(self.1, self.3) }
        pub fn zw(&self) -> Vec2<T> { Vec2(self.2, self.3) }
        pub fn xyz(&self) -> Vec3<T> { Vec3(self.0, self.1, self.2) }
        pub fn xyw(&self) -> Vec3<T> { Vec3(self.0, self.1, self.3) }
        pub fn xzw(&self) -> Vec3<T> { Vec3(self.0, self.2, self.3) }
        pub fn yzw(&self) -> Vec3<T> { Vec3(self.1, self.2, self.3) }
    } // impl Vec4


    impl<T: Signed> Neg for Vec4<T>  {
        type Output = Self;
        fn neg(self) -> Self::Output { Vec4(-self.0, -self.1, -self.2, -self.3) }
    } // impl Neg ..


    impl<T: Number> Add for Vec4<T>  {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { Vec4(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3) }
    } // impl Add ..


    impl<T: Number> AddAssign for Vec4<T>  {
        fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; self.2 += rhs.2; self.3 += rhs.3; }
    } // impl AddAssign ..


    impl<T: Number> Sub for Vec4<T>  {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { Vec4(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3) }
    } // impl Sub ..


    impl<T: Number> SubAssign for Vec4<T>  {
        fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; self.2 -= rhs.2; self.3 -= rhs.3; }
    } // impl AddAssign ..


    impl<T: Number> Mul<T> for Vec4<T>  {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output { Vec4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs) }
    } // impl Mul ..


    impl<T: Number> MulAssign<T> for Vec4<T>  {
        fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; self.1 *= rhs; self.2 *= rhs; self.3 *= rhs; }
    } // impl MulAssign ..


    impl<T: Number> Div<T> for Vec4<T>  {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output { Vec4(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs) }
    } // impl Div ..


    impl<T: Number> DivAssign<T> for Vec4<T>  {
        fn div_assign(&mut self, rhs: T) { self.0 /= rhs; self.1 /= rhs; self.2 /= rhs; self.3 /= rhs; }
    } // impl DivAssign ..
