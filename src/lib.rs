//###############
// M O D U L E S
//###############

    mod traits;
    mod angle;
    mod vectors;

    pub use angle::Angle;

    #[test]
    fn test() {

        use vectors::Vec2;
        assert_eq!(Vec2::distance(Vec2(1, 0), Vec2(-1, 0)), 2u32)

    }