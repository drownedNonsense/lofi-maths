//#########################
// D E P E N D E N C I E S
//#########################

    use std::cmp::Ordering;
    use std::ops::{
        Neg,
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Rem, RemAssign,
    }; // use ..

    use rusty_toolkit::WrappingFrom;


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Clone, Copy, Hash, Default, Debug)]
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


    const BYTE_TO_RAD_LOOKUP: [f32; 256usize] = [
        0f32,
        0.002496548f32,
        0.004993096f32,
        0.007489644f32,
        0.009986192f32,
        0.01248274f32,
        0.014979288f32,
        0.017475836f32,
        0.019972384f32,
        0.022468932f32,
        0.02496548f32,
        0.027462028f32,
        0.029958576f32,
        0.032455124f32,
        0.034951672f32,
        0.03744822f32,
        0.039944768f32,
        0.042441316f32,
        0.044937864f32,
        0.047434412f32,
        0.04993096f32,
        0.052427508f32,
        0.054924056f32,
        0.057420604f32,
        0.059917152f32,
        0.0624137f32,
        0.06491025f32,
        0.067406796f32,
        0.069903344f32,
        0.07239989f32,
        0.07489644f32,
        0.07739299f32,
        0.079889536f32,
        0.082386084f32,
        0.08488263f32,
        0.08737918f32,
        0.08987573f32,
        0.092372276f32,
        0.094868824f32,
        0.09736537f32,
        0.09986192f32,
        0.10235847f32,
        0.104855016f32,
        0.107351564f32,
        0.10984811f32,
        0.11234466f32,
        0.11484121f32,
        0.117337756f32,
        0.119834304f32,
        0.12233085f32,
        0.1248274f32,
        0.12732396f32,
        0.1298205f32,
        0.13231704f32,
        0.13481359f32,
        0.13731015f32,
        0.13980669f32,
        0.14230323f32,
        0.14479978f32,
        0.14729634f32,
        0.14979288f32,
        0.15228942f32,
        0.15478598f32,
        0.15728253f32,
        0.15977907f32,
        0.16227561f32,
        0.16477217f32,
        0.16726872f32,
        0.16976526f32,
        0.1722618f32,
        0.17475836f32,
        0.17725492f32,
        0.17975146f32,
        0.182248f32,
        0.18474455f32,
        0.1872411f32,
        0.18973765f32,
        0.19223419f32,
        0.19473074f32,
        0.1972273f32,
        0.19972384f32,
        0.20222038f32,
        0.20471694f32,
        0.20721349f32,
        0.20971003f32,
        0.21220657f32,
        0.21470313f32,
        0.21719968f32,
        0.21969622f32,
        0.22219276f32,
        0.22468932f32,
        0.22718588f32,
        0.22968242f32,
        0.23217896f32,
        0.23467551f32,
        0.23717207f32,
        0.23966861f32,
        0.24216515f32,
        0.2446617f32,
        0.24715826f32,
        0.2496548f32,
        0.25215134f32,
        0.2546479f32,
        0.25714445f32,
        0.259641f32,
        0.26213753f32,
        0.26463407f32,
        0.26713064f32,
        0.26962718f32,
        0.27212372f32,
        0.2746203f32,
        0.27711684f32,
        0.27961338f32,
        0.28210992f32,
        0.28460646f32,
        0.28710303f32,
        0.28959957f32,
        0.2920961f32,
        0.29459268f32,
        0.29708922f32,
        0.29958576f32,
        0.3020823f32,
        0.30457884f32,
        0.3070754f32,
        0.30957195f32,
        0.3120685f32,
        0.31456506f32,
        0.3170616f32,
        0.31955814f32,
        0.32205468f32,
        0.32455122f32,
        0.3270478f32,
        0.32954434f32,
        0.33204088f32,
        0.33453745f32,
        0.337034f32,
        0.33953053f32,
        0.34202707f32,
        0.3445236f32,
        0.34702018f32,
        0.34951672f32,
        0.35201326f32,
        0.35450983f32,
        0.35700637f32,
        0.3595029f32,
        0.36199945f32,
        0.364496f32,
        0.36699256f32,
        0.3694891f32,
        0.37198564f32,
        0.3744822f32,
        0.37697875f32,
        0.3794753f32,
        0.38197184f32,
        0.38446838f32,
        0.38696495f32,
        0.3894615f32,
        0.39195803f32,
        0.3944546f32,
        0.39695114f32,
        0.39944768f32,
        0.40194422f32,
        0.40444076f32,
        0.40693733f32,
        0.40943387f32,
        0.4119304f32,
        0.41442698f32,
        0.41692352f32,
        0.41942006f32,
        0.4219166f32,
        0.42441314f32,
        0.42690971f32,
        0.42940626f32,
        0.4319028f32,
        0.43439937f32,
        0.4368959f32,
        0.43939245f32,
        0.441889f32,
        0.44438553f32,
        0.4468821f32,
        0.44937864f32,
        0.45187518f32,
        0.45437175f32,
        0.4568683f32,
        0.45936483f32,
        0.46186137f32,
        0.4643579f32,
        0.46685448f32,
        0.46935102f32,
        0.47184756f32,
        0.47434413f32,
        0.47684067f32,
        0.47933722f32,
        0.48183376f32,
        0.4843303f32,
        0.48682687f32,
        0.4893234f32,
        0.49181995f32,
        0.49431652f32,
        0.49681306f32,
        0.4993096f32,
        0.50180614f32,
        0.5043027f32,
        0.5067992f32,
        0.5092958f32,
        0.51179236f32,
        0.5142889f32,
        0.51678544f32,
        0.519282f32,
        0.5217785f32,
        0.52427506f32,
        0.5267716f32,
        0.52926815f32,
        0.53176475f32,
        0.5342613f32,
        0.5367578f32,
        0.53925437f32,
        0.5417509f32,
        0.54424745f32,
        0.546744f32,
        0.5492406f32,
        0.5517371f32,
        0.5542337f32,
        0.5567302f32,
        0.55922675f32,
        0.5617233f32,
        0.56421983f32,
        0.5667164f32,
        0.5692129f32,
        0.5717095f32,
        0.57420605f32,
        0.5767026f32,
        0.57919914f32,
        0.5816957f32,
        0.5841922f32,
        0.58668876f32,
        0.58918536f32,
        0.5916819f32,
        0.59417844f32,
        0.596675f32,
        0.5991715f32,
        0.60166806f32,
        0.6041646f32,
        0.60666114f32,
        0.6091577f32,
        0.6116543f32,
        0.6141508f32,
        0.61664736f32,
        0.6191439f32,
        0.62164044f32,
        0.624137f32,
        0.6266335f32,
        0.6291301f32,
        0.63162667f32,
        0.6341232f32,
        0.63661975f32,
    ]; // const ..


    const RAD_TO_BYTE_RATIO: f32 = 255f32 / std::f32::consts::FRAC_2_PI;


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
        } // fn ..


        pub(crate) fn cosf(self) -> f32 {
            match self.0 & 0xC0 {
                0x00 =>  TRIGO_LOOKUP[(0x40 - self.0) as usize],
                0x40 => -TRIGO_LOOKUP[(self.0 & 0x3F) as usize],
                0x80 => -TRIGO_LOOKUP[(0x40 - (self.0 & 0x3F)) as usize],
                _    =>  TRIGO_LOOKUP[(self.0 & 0x3F) as usize],
            } // match ..
        } // fn ..
    } // impl ..


    impl Neg for Angle {
        type Output = Self;
        fn neg(self) -> Self::Output { Angle(0u8.wrapping_sub(self.0)) }
    } // impl ..


    impl Add for Angle {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { Angle(self.0.wrapping_add(rhs.0)) }
    } // impl ..


    impl AddAssign for Angle {
        fn add_assign(&mut self, rhs: Self) { self.0 = self.0.wrapping_add(rhs.0) }
    } // impl ..


    impl Sub for Angle {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { Angle(self.0.wrapping_sub(rhs.0)) }
    } // impl ..


    impl SubAssign for Angle {
        fn sub_assign(&mut self, rhs: Self) { self.0 = self.0.wrapping_sub(rhs.0) }
    } // impl ..


    impl Mul for Angle {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output { Angle(self.0.wrapping_mul(rhs.0)) }
    } // impl ..


    impl MulAssign for Angle {
        fn mul_assign(&mut self, rhs: Self) { self.0 = self.0.wrapping_mul(rhs.0) }
    } // impl ..


    impl Div for Angle {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output { Angle(self.0 / rhs.0) }
    } // impl ..


    impl DivAssign for Angle {
        fn div_assign(&mut self, rhs: Self) { self.0 = self.0 / rhs.0 }
    } // impl ..


    impl Rem for Angle {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self::Output { Angle(self.0 % rhs.0) }
    } // impl ..


    impl RemAssign for Angle {
        fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0 }
    } // impl ..


    impl From<Angle> for f32 {
        /// Converts a single byte angle to floating point radians
        fn from(angle: Angle) -> f32 { BYTE_TO_RAD_LOOKUP[angle.0 as usize] }
    } // impl ..


    impl From<f32> for Angle {
        /// Converts a floating point radian to a single byte angle
        fn from(angle: f32) -> Self { Angle(u8::wrapping_from((angle * RAD_TO_BYTE_RATIO) as i32)) }
    } // impl ..


    impl PartialEq for Angle {
        fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
    } // impl ..


    impl PartialOrd for Angle {
        fn ge(&self, other: &Self) -> bool { self.0 >= other.0 }
        fn gt(&self, other: &Self) -> bool { self.0 >  other.0 }
        fn le(&self, other: &Self) -> bool { self.0 <= other.0 }
        fn lt(&self, other: &Self) -> bool { self.0 <  other.0 }
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.eq(other)      { Some(Ordering::Equal) }
            else if self.gt(other) { Some(Ordering::Greater) }
            else                   { Some(Ordering::Less) }
        } // fn ..
    } // imp ..
