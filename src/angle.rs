//#########################
// D E P E N D E N C I E S
//#########################

    use std::ops::{
        Neg,
        Add, AddAssign,
        Sub, SubAssign,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    pub struct Angle(pub u8);

    const TRIGO_LOOKUP: [f32; 65usize] = [
        0f32,
        0.024541229f32,
        0.049067676f32,
        0.07356457f32,
        0.09801714f32,
        0.12241068f32,
        0.14673047f32,
        0.1709619f32,
        0.19509032f32,
        0.21910124f32,
        0.2429802f32,
        0.26671278f32,
        0.29028466f32,
        0.31368175f32,
        0.33688986f32,
        0.35989505f32,
        0.38268346f32,
        0.40524134f32,
        0.42755508f32,
        0.44961134f32,
        0.47139674f32,
        0.49289823f32,
        0.51410276f32,
        0.53499764f32,
        0.55557024f32,
        0.5758082f32,
        0.5956993f32,
        0.61523163f32,
        0.63439333f32,
        0.65317285f32,
        0.671559f32,
        0.68954057f32,
        0.70710677f32,
        0.7242471f32,
        0.7409512f32,
        0.7572089f32,
        0.77301043f32,
        0.7883464f32,
        0.8032075f32,
        0.8175848f32,
        0.83146966f32,
        0.8448536f32,
        0.85772866f32,
        0.87008697f32,
        0.8819213f32,
        0.8932243f32,
        0.9039893f32,
        0.9142098f32,
        0.9238795f32,
        0.9329928f32,
        0.94154406f32,
        0.9495282f32,
        0.95694035f32,
        0.96377605f32,
        0.97003126f32,
        0.9757021f32,
        0.9807853f32,
        0.98527765f32,
        0.9891765f32,
        0.99247956f32,
        0.9951847f32,
        0.99729043f32,
        0.99879545f32,
        0.9996988f32,
        1f32,
    ]; // const ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl Angle {
        pub(crate) fn sinf(self) -> f32 {
            match self.0 & 0xC0 {
                0x00 =>  TRIGO_LOOKUP[self.0 as usize],
                0x40 =>  TRIGO_LOOKUP[(0x40 - (self.0 & 0x3F)) as usize],
                0x80 => -TRIGO_LOOKUP[(self.0 & 0x3F) as usize],
                _    => -TRIGO_LOOKUP[(0x40 - (self.0 & 0x3F)) as usize],
            } // match ..
        } // fn sinf()


        pub(crate) fn cosf(self) -> f32 {
            match self.0 & 0xC0 {
                0x00 =>  TRIGO_LOOKUP[(0x40 - self.0) as usize],
                0x40 => -TRIGO_LOOKUP[(self.0 & 0x3F) as usize],
                0x80 => -TRIGO_LOOKUP[(0x40 - (self.0 & 0x3F)) as usize],
                _    =>  TRIGO_LOOKUP[(self.0 & 0x3F) as usize],
            } // match ..
        } // fn cosf()
    } // impl Angle


    impl Neg for Angle {
        type Output = Self;
        fn neg(self) -> Self::Output { Angle(0u8.wrapping_sub(self.0)) }
    } // impl Neg ..


    impl Add for Angle {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { Angle(self.0.wrapping_add(rhs.0)) }
    } // impl Add ..


    impl AddAssign for Angle {
        fn add_assign(&mut self, rhs: Self) { self.0 = self.0.wrapping_add(rhs.0) }
    } // impl AddAssign ..


    impl Sub for Angle {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { Angle(self.0.wrapping_sub(rhs.0)) }
    } // impl Sub ..


    impl SubAssign for Angle {
        fn sub_assign(&mut self, rhs: Self) { self.0 = self.0.wrapping_sub(rhs.0) }
    } // impl SubAssign ..
