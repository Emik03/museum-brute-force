mod test {
    #[allow(unused_imports)]
    use crate::vault::Vault;
    #[allow(unused_imports)]
    use crate::vault::Vault::*;

    #[test]
    fn is_valid_solution() {
        const X: Vault = Bronze;
        const Y: Vault = Silver;
        const Z: Vault = Gold;

        let vaults = [
            (X, 0),
            (Y, 1),
            (Z, 0),
            (Z, 0),
            (Z, 0),
            (X, 3),
            (Z, 3),
            (Y, 4),
            (Y, 3),
        ];

        assert_eq!(Vault::run(vaults).map(|x| x.iter().sum()), Some(0));
    }
}
