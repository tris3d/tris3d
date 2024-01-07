// Semi-sum operator in Z3.
//
// Z3 is the group of arithmetic modulo 3.
//
// Notice that in Z3, mutliply and divide by 2 has the same result: indeed
//
// ```
// 0 -> 0
// 1 -> 2
// 2 -> 1
// ```
//
// So, in Z3
//
// ```
// (a + b) * 2 = (a + b) / 2
// ```
//
// Since I'm working with integers I prefer to multiply by 2 to avoid floats.
//
// Other remarkable properties of semi-sum in Z3 are:
//
// - if a and b are equal it is the identity,
// - otherwise it is cyclic.
pub fn semi_sum_in_z3(a: u8, b: u8) -> u8 {
    ((a + b) * 2) % 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semi_sum_in_z3_works() {
        // If a and b are equal, semi-sum is the identity.
        assert_eq!(semi_sum_in_z3(0, 0), 0);
        assert_eq!(semi_sum_in_z3(1, 1), 1);
        assert_eq!(semi_sum_in_z3(2, 2), 2);

        // Otherwise it is  cyclic.
        //
        // ```
        // 0, 1 -> 2
        // 1, 2 -> 0
        // 2, 0 -> 1
        // ```
        //
        // ```
        // 2, 1 -> 0
        // 1, 0 -> 2
        // 0, 2 -> 1
        // ```
        assert_eq!(semi_sum_in_z3(0, 1), 2);
        assert_eq!(semi_sum_in_z3(1, 2), 0);
        assert_eq!(semi_sum_in_z3(2, 0), 1);
        assert_eq!(semi_sum_in_z3(0, 2), 1);
        assert_eq!(semi_sum_in_z3(1, 0), 2);
        assert_eq!(semi_sum_in_z3(2, 1), 0);
    }
}
