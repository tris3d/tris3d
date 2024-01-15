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
pub fn z3_semi_sum(a: u8, b: u8) -> u8 {
    ((a + b) * 2) % 3
}

pub fn z3xz3xz3_coordinates_of_index(index: u8) -> (u8, u8, u8) {
    (0, (index - (index % 3)) / 3, index % 3)
}

pub fn index_of_z3xz3xz3_coordinates(x: u8, y: u8, z: u8) -> u8 {
    x * 9 + y * 3 + z
}

// pub fn z3xz3xz3_semi_sum(a: u8, b: u8) -> u8 {}

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

    // #[test]
    // fn z3xz3xz3_semi_sum_of_equal_values_is_identity() {
    //     assert_eq!(z3xz3xz3_semi_sum(0, 0), 0);
    //     // TODO...
    // }

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
        // TODO assert_eq!(z3xz3xz3_coordinates_of_index(9), (0, 0, 0));
    }

    #[test]
    fn index_of_z3xz3xz3_coordinates_works() {
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 0, 0), 0);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 0, 1), 1);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 0, 2), 2);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 1, 0), 3);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 1, 1), 4);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 1, 2), 5);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 2, 0), 6);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 2, 1), 7);
        assert_eq!(index_of_z3xz3xz3_coordinates(0, 2, 2), 8);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 0, 0), 9);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 0, 1), 10);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 0, 2), 11);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 1, 0), 12);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 1, 1), 13);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 1, 2), 14);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 2, 0), 15);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 2, 1), 16);
        assert_eq!(index_of_z3xz3xz3_coordinates(1, 2, 2), 17);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 0, 0), 18);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 0, 1), 19);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 0, 2), 20);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 1, 0), 21);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 1, 1), 22);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 1, 2), 23);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 2, 0), 24);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 2, 1), 25);
        assert_eq!(index_of_z3xz3xz3_coordinates(2, 2, 2), 26);
    }
}
