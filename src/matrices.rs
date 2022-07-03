//#########################
// D E P E N D E N C I E S
//#########################

    use std::ops::{Mul, MulAssign};

    use crate::traits::{Number, Signed, Trigonometry};
    use crate::vectors::{Vec2, Vec3, Vec4};


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Mat3<T: Number>(pub Vec3<T>, pub Vec3<T>, pub Vec3<T>);


    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Mat4<T: Number>(pub Vec4<T>, pub Vec4<T>, pub Vec4<T>, pub Vec4<T>);


//###############################
// I M P L E M E N T A T I O N S
//###############################

    //#########
    // M A T 3
    //#########

    impl<T: Number> Mat3<T> {
        pub const IDENTITY: Self =
            Mat3(
                Vec3(T::ONE,  T::ZERO, T::ZERO),
                Vec3(T::ZERO, T::ONE,  T::ZERO),
                Vec3(T::ZERO, T::ZERO, T::ONE),
            ); // const ..


        pub fn new_2d_homogeneous_scaling_mat(vector: Vec2<T>) -> Self {
            Mat3(
                Vec3(vector.0, T::ZERO,  T::ZERO),
                Vec3(T::ZERO,  vector.1, T::ZERO),
                Vec3(T::ZERO,  T::ZERO,  T::ONE),
            ) // Mat3
        } // fn new_2d_homogeneous_scaling_mat()


        pub fn new_2d_homogeneous_translation_mat(vector: Vec2<T>) -> Self {
            Mat3(
                Vec3(T::ONE,   T::ZERO,  T::ZERO),
                Vec3(T::ZERO,  T::ONE,   T::ZERO),
                Vec3(vector.0, vector.1, T::ONE),
            ) // Mat3
        } // fn new_2d_homogeneous_translation_mat()
    } // impl Mat3 ..


    impl Mat3<f32> {
        pub fn new_2d_homogeneous_rotation_mat<T: Trigonometry>(angle: T) -> Self {
            Mat3(
                Vec3(angle.cos(), -angle.sin(), 0f32),
                Vec3(angle.sin(),  angle.cos(), 0f32),
                Vec3(0f32,         0f32,        1f32),
            ) // Mat3
        } // fn new_2d_homogeneous_rotation_mat()
    } // impl Mat3


    impl<T: Number> Mul for Mat3<T> {
        type Output = Self;
        fn mul(self, other: Self) -> Self::Output {
            Mat3(
                self.0 * other.0.0 + self.1 * other.0.1 + self.2 * other.0.2,
                self.1 * other.1.0 + self.1 * other.1.1 + self.2 * other.1.2,
                self.2 * other.2.0 + self.1 * other.2.1 + self.2 * other.2.2,
            ) // Mat3
        } // fn mul()
    } // impl Mul ..


    impl<T: Number> MulAssign for Mat3<T> {
        fn mul_assign(&mut self, other: Self) {
            self.0 = self.0 * other.0.0 + self.1 * other.0.1 + self.2 * other.0.2;
            self.1 = self.1 * other.1.0 + self.1 * other.1.1 + self.2 * other.1.2;
            self.2 = self.2 * other.2.0 + self.1 * other.2.1 + self.2 * other.2.2;
        } // fn mul_assign()
    } // impl MulAssign ..


    impl<T: Number> Mul<Vec2<T>> for Mat3<T> {
        type Output = Vec3<T>;
        fn mul(self, other: Vec2<T>) -> Self::Output {
            Vec3(
                self.0.0 * other.0 + self.1.0 * other.1 + self.2.0,
                self.0.1 * other.0 + self.1.1 * other.1 + self.2.1,
                self.0.2 * other.0 + self.1.2 * other.1 + self.2.2,
            ) // Vec3
        } // fn mul()
    } // impl Mul ..


    impl<T: Number> Mul<Vec3<T>> for Mat3<T> {
        type Output = Vec3<T>;
        fn mul(self, other: Vec3<T>) -> Self::Output {
            Vec3(
                self.0.0 * other.0 + self.1.0 * other.1 + self.2.0 * other.2,
                self.0.1 * other.0 + self.1.1 * other.1 + self.2.1 * other.2,
                self.0.2 * other.0 + self.1.2 * other.1 + self.2.2 * other.2,
            ) // Vec3
        } // fn mul()
    } // impl Mul ..


    impl<T: Number + Into<f32>> From<Mat3<T>> for [f32; 9usize] {
        fn from(mat4: Mat3<T>) -> Self {[
            mat4.0.0.into(), mat4.0.1.into(), mat4.0.2.into(),
            mat4.1.0.into(), mat4.1.1.into(), mat4.1.2.into(),
            mat4.2.0.into(), mat4.2.1.into(), mat4.2.2.into(),
        ]} // fn from()
    } // impl From ..


    //#########
    // M A T 4
    //#########

    impl<T: Number> Mat4<T> {
        pub const IDENTITY: Self =
            Mat4(
                Vec4(T::ONE,  T::ZERO, T::ZERO, T::ZERO),
                Vec4(T::ZERO, T::ONE,  T::ZERO, T::ZERO),
                Vec4(T::ZERO, T::ZERO, T::ONE,  T::ZERO),
                Vec4(T::ZERO, T::ZERO, T::ZERO, T::ONE),
            ); // const ..


        pub fn new_orthogonal_projection_mat(horizontal: (T, T), vertical: (T, T), depth: (T, T)) -> Self
        where T: Signed {
            Mat4(
                Vec4( (T::ONE + T::ONE) / (horizontal.1 - horizontal.0),              T::ZERO,                                                T::ZERO,                                   T::ZERO),
                Vec4( T::ZERO,                                                        (T::ONE + T::ONE) / (vertical.1 - vertical.0),          T::ZERO,                                   T::ZERO),
                Vec4( T::ZERO,                                                        T::ZERO,                                                (-T::ONE - T::ONE) / (depth.1 - depth.0),  T::ZERO),
                Vec4(-(horizontal.1 + horizontal.0) / (horizontal.1 - horizontal.0), -(vertical.1 + vertical.0) / (vertical.1 - vertical.0), -(depth.1 + depth.0) / (depth.1 - depth.0), T::ONE),
            ) // Mat4
        } // fn new_orthogonal_projection_mat()
    } // impl Mat4

    impl<T: Number> Mul for Mat4<T> {
        type Output = Self;
        fn mul(self, other: Self) -> Self::Output {
            Mat4(
                self.0 * other.0.0 + self.1 * other.0.1 + self.2 * other.0.2 + self.3 * other.0.3,
                self.1 * other.1.0 + self.1 * other.1.1 + self.2 * other.1.2 + self.3 * other.1.3,
                self.2 * other.2.0 + self.1 * other.2.1 + self.2 * other.2.2 + self.3 * other.2.3,
                self.3 * other.3.0 + self.1 * other.3.1 + self.2 * other.3.2 + self.3 * other.3.3,
            ) // Mat4
        } // fn mul()
    } // impl Mul ..


    impl<T: Number> MulAssign for Mat4<T> {
        fn mul_assign(&mut self, other: Self) {
            self.0 = self.0 * other.0.0 + self.1 * other.0.1 + self.2 * other.0.2 + self.3 * other.0.3;
            self.1 = self.1 * other.1.0 + self.1 * other.1.1 + self.2 * other.1.2 + self.3 * other.1.3;
            self.2 = self.2 * other.2.0 + self.1 * other.2.1 + self.2 * other.2.2 + self.3 * other.2.3;
            self.3 = self.3 * other.3.0 + self.1 * other.3.1 + self.2 * other.3.2 + self.3 * other.3.3;
        } // fn mul_assign()
    } // impl MulAssign ..


    impl<T: Number> Mul<Vec3<T>> for Mat4<T> {
        type Output = Vec4<T>;
        fn mul(self, other: Vec3<T>) -> Self::Output {
            Vec4(
                self.0.0 * other.0 + self.1.0 * other.1 + self.2.0 * other.2 + self.3.0,
                self.0.1 * other.0 + self.1.1 * other.1 + self.2.1 * other.2 + self.3.1,
                self.0.2 * other.0 + self.1.2 * other.1 + self.2.2 * other.2 + self.3.2,
                self.0.3 * other.0 + self.1.3 * other.1 + self.2.3 * other.2 + self.3.3,
            ) // Vec4
        } // fn mul()
    } // impl Mul ..


    impl<T: Number> Mul<Vec4<T>> for Mat4<T> {
        type Output = Vec4<T>;
        fn mul(self, other: Vec4<T>) -> Self::Output {
            Vec4(
                self.0.0 * other.0 + self.1.0 * other.1 + self.2.0 * other.2 + self.3.0 * other.3,
                self.0.1 * other.0 + self.1.1 * other.1 + self.2.1 * other.2 + self.3.1 * other.3,
                self.0.2 * other.0 + self.1.2 * other.1 + self.2.2 * other.2 + self.3.2 * other.3,
                self.0.3 * other.0 + self.1.3 * other.1 + self.2.3 * other.2 + self.3.3 * other.3,
            ) // Vec4
        } // fn mul()
    } // impl Mul ..


    impl<T: Number + Into<f32>> From<Mat4<T>> for [f32; 16usize] {
        fn from(mat4: Mat4<T>) -> Self {[
            mat4.0.0.into(), mat4.0.1.into(), mat4.0.2.into(), mat4.0.3.into(),
            mat4.1.0.into(), mat4.1.1.into(), mat4.1.2.into(), mat4.1.3.into(),
            mat4.2.0.into(), mat4.2.1.into(), mat4.2.2.into(), mat4.2.3.into(),
            mat4.3.0.into(), mat4.3.1.into(), mat4.3.2.into(), mat4.3.3.into(),
        ]} // fn from()
    } // impl From ..
