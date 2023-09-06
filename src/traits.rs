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

    use crate::Angle;


//#######################
// D E F I N I T I O N S
//#######################

    pub trait Number<Rhs=Self, Output=Self>:
        Copy
        + Sized
        + PartialEq + PartialOrd
        + Add<Rhs, Output=Output> + AddAssign
        + Sub<Rhs, Output=Output> + SubAssign
        + Mul<Rhs, Output=Output> + MulAssign
        + Div<Rhs, Output=Output> + DivAssign
        + Zero + One {}


    pub trait Zero { const ZERO: Self; }
    pub trait One  { const ONE: Self; }

    pub trait Signed<Output=Self>: Neg<Output=Output> + Number { type Unsigned; }
    pub trait Unsigned: Number                                 { type Signed; }

    pub trait Integer: Number {}
    pub trait Float: Number   {}
    pub trait Trigonometry<Rhs=Self, Output=Self>: Neg + Number { fn sin(self) -> f32; fn cos(self) -> f32; }

    pub trait Sqrt { fn sqrt(self) -> Self; }


    const fn isqrt8(x: u8) -> u8 {

        let mut v      = x;
        let mut b      = 1u8 << 3u8; // Bit set in the middle of the sequence
        let mut b_shft = 3u8;        // Half the bit width
        let mut n_hat  = 0u8;


        while b != 1u8 {

            let temp = ((n_hat << 1u8) + b) << b_shft;
            if v >= temp {

                n_hat += b;
                v     -= temp;

            } // if ..


            b      >>= 1u8;
            b_shft -=  1u8;

        } // while ..


        n_hat + (x & 1u8) // Add one if odd

    } // fn ..


    const fn isqrt16(x: u16) -> u16 {

        let mut v      = x;
        let mut b      = 1u16 << 7u8; // Bit set in the middle of the sequence
        let mut b_shft = 7u8;         // Half the bit width
        let mut n_hat  = 0u16;


        while b != 1u16 {

            let temp = ((n_hat << 1u8) + b) << b_shft;
            if v >= temp {

                n_hat += b;
                v     -= temp;

            } // if ..


            b      >>= 1u8;
            b_shft -=  1u8;

        } // while ..


        n_hat + (x & 1u16) // Add one if odd

    } // fn ..


    const fn isqrt32(x: u32) -> u32 {

        let mut v      = x;
        let mut b      = 1u32 << 15u8; // Bit set in the middle of the sequence
        let mut b_shft = 15u8;         // Half the bit width
        let mut n_hat  = 0u32;
    
    
        while b != 1u32 {
    
            let temp = ((n_hat << 1u8) + b) << b_shft;
            if v >= temp {
    
                n_hat += b;
                v     -= temp;
    
            } // if ..
    
    
            b      >>= 1u8;
            b_shft -=  1u8;
    
        } // while ..
    
    
        n_hat + (x & 1u32) // Add one if odd
    
    } // fn ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<T, Rhs, Output> Number<Rhs, Output> for T where T:
        Copy
        + Sized
        + PartialEq + PartialOrd
        + Add<Rhs, Output=Output> + AddAssign
        + Sub<Rhs, Output=Output> + SubAssign
        + Mul<Rhs, Output=Output> + MulAssign
        + Div<Rhs, Output=Output> + DivAssign
        + Zero + One {}

    impl Signed for i8  { type Unsigned = u8; }
    impl Signed for i16 { type Unsigned = u16; }
    impl Signed for i32 { type Unsigned = u32; }
    impl Signed for f32 { type Unsigned = f32; }
    impl Unsigned for u8  { type Signed = i8; }
    impl Unsigned for u16 { type Signed = i16; }
    impl Unsigned for u32 { type Signed = i32; }

    impl Integer for u8  {}
    impl Integer for u16 {}
    impl Integer for u32 {}
    impl Integer for i8  {}
    impl Integer for i16 {}
    impl Integer for i32 {}

    impl Float for f32 {}


    impl Zero for u8  { const ZERO: Self = 0u8; }
    impl Zero for u16 { const ZERO: Self = 0u16; }
    impl Zero for u32 { const ZERO: Self = 0u32; }
    impl Zero for i8  { const ZERO: Self = 0i8; }
    impl Zero for i16 { const ZERO: Self = 0i16; }
    impl Zero for i32 { const ZERO: Self = 0i32; }

    impl Zero for f32 { const ZERO: Self = 0f32; }

    impl Zero for Angle    { const ZERO: Self = Angle(0u8); }

    impl One for u8  { const ONE: Self = 1u8; }
    impl One for u16 { const ONE: Self = 1u16; }
    impl One for u32 { const ONE: Self = 1u32; }
    impl One for i8  { const ONE: Self = 1i8; }
    impl One for i16 { const ONE: Self = 1i16; }
    impl One for i32 { const ONE: Self = 1i32; }

    impl One for f32 { const ONE: Self = 1f32; }

    impl One for Angle    { const ONE: Self = Angle(128u8); }


    impl Sqrt for u8  { fn sqrt(self) -> Self { isqrt8(self) }}
    impl Sqrt for u16 { fn sqrt(self) -> Self { isqrt16(self) }}
    impl Sqrt for u32 { fn sqrt(self) -> Self { isqrt32(self) }}

    impl Sqrt for f32 { fn sqrt(self) -> Self { f32::sqrt(self) }}

    impl Trigonometry for f32   { fn sin(self) -> f32 { f32::sin(self) } fn cos(self) -> f32 { f32::cos(self) }}
    impl Trigonometry for Angle { fn sin(self) -> f32 { self.sinf() }    fn cos(self) -> f32 { self.cosf() }}
