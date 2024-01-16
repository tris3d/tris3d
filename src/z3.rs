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
pub fn z3_semi_sum(a: u8, b: u8) -> u8 {
    ((a + b) * 2) % 3
}

pub fn z3xz3xz3_coordinates_of_index(index: u8) -> (u8, u8, u8) {
    (
        (index - (index % 9)) / 9,
        ((index - (index % 3)) / 3) % 3,
        index % 3,
    )
}

pub fn index_of_z3xz3xz3_coordinates(vector: (u8, u8, u8)) -> u8 {
    vector.0 * 9 + vector.1 * 3 + vector.2
}

// Semi-sum operator in Z3xZ3xZ3.
//
// Z3xZ3xZ3 is a three dimensional spaces immersed in euclidian three dimensional space R3.
//
// Same as the Z3 semi-sum, the Z3xZ3xZ3 semi-sum operator has the following properties:
// it is commutative, cyclic, and when arguments are equal it is the identity.
//
// Another remarkable property of Z3xZ3xZ3 semi-sum is that if arguments are on the same line,
// the result is aligned with arguments.
pub fn z3xz3xz3_semi_sum(a: (u8, u8, u8), b: (u8, u8, u8)) -> (u8, u8, u8) {
    (
        z3_semi_sum(a.0, b.0),
        z3_semi_sum(a.1, b.1),
        z3_semi_sum(a.2, b.2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z3_semi_sum_of_equal_values_is_identity() {
        // Z3 semi-sum of two equal values is the identity.
        assert_eq!(z3_semi_sum(0, 0), 0);
        assert_eq!(z3_semi_sum(1, 1), 1);
        assert_eq!(z3_semi_sum(2, 2), 2);
    }

    #[test]
    fn z3_semi_sum_of_distinct_values_is_cyclic() {
        // Z3 semi-sum of distinct values is cyclic.
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
        assert_eq!(z3_semi_sum(0, 1), 2);
        assert_eq!(z3_semi_sum(1, 2), 0);
        assert_eq!(z3_semi_sum(2, 0), 1);
        assert_eq!(z3_semi_sum(0, 2), 1);
        assert_eq!(z3_semi_sum(1, 0), 2);
        assert_eq!(z3_semi_sum(2, 1), 0);
    }

    #[test]
    fn z3xz3xz3_coordinates_of_index_works() {
        assert_eq!(z3xz3xz3_coordinates_of_index(0), (0, 0, 0));
        assert_eq!(z3xz3xz3_coordinates_of_index(1), (0, 0, 1));
        assert_eq!(z3xz3xz3_coordinates_of_index(2), (0, 0, 2));
        assert_eq!(z3xz3xz3_coordinates_of_index(3), (0, 1, 0));
        assert_eq!(z3xz3xz3_coordinates_of_index(4), (0, 1, 1));
        assert_eq!(z3xz3xz3_coordinates_of_index(5), (0, 1, 2));
        assert_eq!(z3xz3xz3_coordinates_of_index(6), (0, 2, 0));
        assert_eq!(z3xz3xz3_coordinates_of_index(7), (0, 2, 1));
        assert_eq!(z3xz3xz3_coordinates_of_index(8), (0, 2, 2));
        assert_eq!(z3xz3xz3_coordinates_of_index(9), (1, 0, 0));
        assert_eq!(z3xz3xz3_coordinates_of_index(10), (1, 0, 1));
        assert_eq!(z3xz3xz3_coordinates_of_index(11), (1, 0, 2));
        assert_eq!(z3xz3xz3_coordinates_of_index(12), (1, 1, 0));
    }

    #[test]
    fn index_of_z3xz3xz3_coordinates_works() {
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 0, 0)), 0);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 0, 1)), 1);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 0, 2)), 2);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 1, 0)), 3);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 1, 1)), 4);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 1, 2)), 5);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 2, 0)), 6);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 2, 1)), 7);
        assert_eq!(index_of_z3xz3xz3_coordinates((0, 2, 2)), 8);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 0, 0)), 9);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 0, 1)), 10);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 0, 2)), 11);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 1, 0)), 12);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 1, 1)), 13);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 1, 2)), 14);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 2, 0)), 15);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 2, 1)), 16);
        assert_eq!(index_of_z3xz3xz3_coordinates((1, 2, 2)), 17);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 0, 0)), 18);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 0, 1)), 19);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 0, 2)), 20);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 1, 0)), 21);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 1, 1)), 22);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 1, 2)), 23);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 2, 0)), 24);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 2, 1)), 25);
        assert_eq!(index_of_z3xz3xz3_coordinates((2, 2, 2)), 26);
    }

    #[test]
    fn index_of_z3xz3xz3_coordinates_is_inverse_of_z3xz3xz3_coordinates_of_index() {
        for i in 0..27 {
            assert_eq!(
                index_of_z3xz3xz3_coordinates(z3xz3xz3_coordinates_of_index(i)),
                i
            )
        }
    }

    #[test]
    fn z3xz3xz3_semi_sum_of_equal_values_is_the_identity() {
        for i in 0..27 {
            let vector = z3xz3xz3_coordinates_of_index(i);
            assert_eq!(z3xz3xz3_semi_sum(vector, vector), vector);
        }
    }

    #[test]
    fn z3xz3xz3_semi_sum_of_distinct_values_is_commutative() {
        for i in 0..26 {
            for j in (i + 1)..27 {
                let a = z3xz3xz3_coordinates_of_index(i);
                let b = z3xz3xz3_coordinates_of_index(j);
                assert_eq!(z3xz3xz3_semi_sum(a, b), z3xz3xz3_semi_sum(b, a));
            }
        }
    }

    #[test]
    fn z3xz3xz3_semi_sum_of_distinct_values_is_cyclic() {
        for i in 0..26 {
            for j in (i + 1)..27 {
                let a = z3xz3xz3_coordinates_of_index(i);
                let b = z3xz3xz3_coordinates_of_index(j);
                let c = z3xz3xz3_semi_sum(a, b);
                assert_eq!(z3xz3xz3_semi_sum(b, c), a);
                assert_eq!(z3xz3xz3_semi_sum(c, a), b);
            }
        }
    }

    #[test]
    fn z3xz3xz3_semi_sum_return_element_that_is_on_same_line_in_r3() {
        // Check the five lines passing from origin.
        assert_eq!(z3xz3xz3_semi_sum((0, 0, 0), (1, 0, 0)), (2, 0, 0));
        assert_eq!(z3xz3xz3_semi_sum((0, 0, 0), (0, 1, 0)), (0, 2, 0));
        assert_eq!(z3xz3xz3_semi_sum((0, 0, 0), (0, 0, 1)), (0, 0, 2));
        assert_eq!(z3xz3xz3_semi_sum((0, 0, 0), (1, 1, 0)), (2, 2, 0));
        assert_eq!(z3xz3xz3_semi_sum((0, 0, 0), (1, 1, 1)), (2, 2, 2));
        // Check lines passing from the center and parallel to an axis.
        assert_eq!(z3xz3xz3_semi_sum((0, 1, 1), (1, 1, 1)), (2, 1, 1));
        assert_eq!(z3xz3xz3_semi_sum((1, 0, 1), (1, 1, 1)), (1, 2, 1));
        assert_eq!(z3xz3xz3_semi_sum((1, 1, 0), (1, 1, 1)), (1, 1, 2));
    }
}
