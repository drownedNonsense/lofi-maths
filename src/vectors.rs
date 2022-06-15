//#########################
// D E P E N D E N C I E S
//#########################

    use std::ops::{
        Neg,
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Rem, RemAssign,
    }; // use ..

    use crate::traits::{Number, Signed, Unsigned, Trigonometry, Sqrt};
    use crate::Angle;


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Vec2<T: Number>(pub T, pub T);


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<T: Number> Vec2<T> {
        pub fn x(&self) -> T { self.0 }
        pub fn y(&self) -> T { self.1 }
    } // impl Vec2


    impl<T: Signed> Vec2<T> {
        pub fn distance(a: Self, b: Self) -> T::Unsigned where T: Into<T::Unsigned>, T::Unsigned: Sqrt { let dif = b - a; ((dif.0 * dif.0 + dif.1 * dif.1).into()).sqrt() }
    } // impl Vec2


    impl From<Angle> for Vec2<f32> {
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


    impl<T: Number> Rem for Vec2<T>  {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self::Output { Vec2(self.0 % rhs.0, self.1 % rhs.1) }
    } // impl Rem ..


    impl<T: Number> RemAssign for Vec2<T>  {
        fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0; self.1 %= rhs.1; }
    } // impl RemAssign ..


    impl<T: Number> Rem<T> for Vec2<T>  {
        type Output = Self;
        fn rem(self, rhs: T) -> Self::Output { Vec2(self.0 % rhs, self.1 % rhs) }
    } // impl Rem ..


    impl<T: Number> RemAssign<T> for Vec2<T>  {
        fn rem_assign(&mut self, rhs: T) { self.0 %= rhs; self.1 %= rhs; }
    } // impl RemAssign ..
