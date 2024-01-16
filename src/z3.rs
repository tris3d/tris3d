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
// Of course, semi-sum in Z3 is commutative by definition.
// Other remarkable properties of semi-sum in Z3 are:
//
// - If a and b are equal it is the identity,
// - otherwise it is cyclic.
//
// Being cyclic on distinct values means the following.
//
// ```
// 0, 1 -> 2
// 1, 2 -> 0
// 2, 0 -> 1
// ```
//
// And on the opposite direction, the result is the following.
//
// ```
// 2, 1 -> 0
// 1, 0 -> 2
// 0, 2 -> 1
// ```
pub fn semi_sum(a: u8, b: u8) -> u8 {
    ((a + b) * 2) % 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z3_semi_sum_of_equal_values_is_identity() {
        assert_eq!(semi_sum(0, 0), 0);
        assert_eq!(semi_sum(1, 1), 1);
        assert_eq!(semi_sum(2, 2), 2);
    }

    #[test]
    fn z3_semi_sum_of_distinct_values_is_cyclic() {
        assert_eq!(semi_sum(0, 1), 2);
        assert_eq!(semi_sum(1, 2), 0);
        assert_eq!(semi_sum(2, 0), 1);
        assert_eq!(semi_sum(0, 2), 1);
        assert_eq!(semi_sum(1, 0), 2);
        assert_eq!(semi_sum(2, 1), 0);
    }
}
