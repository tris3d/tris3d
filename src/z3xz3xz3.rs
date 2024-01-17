use crate::z3;

// Z3xZ3xZ3 is a three dimensional space immersed in euclidian three dimensional space R3.

pub type Z3xZ3xZ3Vector = (u8, u8, u8);

pub fn coordinates_of_index(index: u8) -> Z3xZ3xZ3Vector {
    (
        (index - (index % 9)) / 9,
        ((index - (index % 3)) / 3) % 3,
        index % 3,
    )
}

pub fn index_of_coordinates(vector: Z3xZ3xZ3Vector) -> u8 {
    vector.0 * 9 + vector.1 * 3 + vector.2
}

// Semi-sum operator in Z3xZ3xZ3.
//
// Same as the Z3 semi-sum, the Z3xZ3xZ3 semi-sum operator has the following properties:
// it is commutative, cyclic, and when arguments are equal it is the identity.
//
// Another remarkable property of Z3xZ3xZ3 semi-sum is that if arguments are on the same line,
// the result is aligned with arguments.
pub fn semi_sum(a: Z3xZ3xZ3Vector, b: Z3xZ3xZ3Vector) -> Z3xZ3xZ3Vector {
    (
        z3::semi_sum(a.0, b.0),
        z3::semi_sum(a.1, b.1),
        z3::semi_sum(a.2, b.2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_of_index_works() {
        assert_eq!(coordinates_of_index(0), (0, 0, 0));
        assert_eq!(coordinates_of_index(1), (0, 0, 1));
        assert_eq!(coordinates_of_index(2), (0, 0, 2));
        assert_eq!(coordinates_of_index(3), (0, 1, 0));
        assert_eq!(coordinates_of_index(4), (0, 1, 1));
        assert_eq!(coordinates_of_index(5), (0, 1, 2));
        assert_eq!(coordinates_of_index(6), (0, 2, 0));
        assert_eq!(coordinates_of_index(7), (0, 2, 1));
        assert_eq!(coordinates_of_index(8), (0, 2, 2));
        assert_eq!(coordinates_of_index(9), (1, 0, 0));
        assert_eq!(coordinates_of_index(10), (1, 0, 1));
        assert_eq!(coordinates_of_index(11), (1, 0, 2));
        assert_eq!(coordinates_of_index(12), (1, 1, 0));
        assert_eq!(coordinates_of_index(13), (1, 1, 1));
        assert_eq!(coordinates_of_index(14), (1, 1, 2));
        assert_eq!(coordinates_of_index(15), (1, 2, 0));
        assert_eq!(coordinates_of_index(16), (1, 2, 1));
        assert_eq!(coordinates_of_index(17), (1, 2, 2));
        assert_eq!(coordinates_of_index(18), (2, 0, 0));
        assert_eq!(coordinates_of_index(19), (2, 0, 1));
        assert_eq!(coordinates_of_index(20), (2, 0, 2));
        assert_eq!(coordinates_of_index(21), (2, 1, 0));
        assert_eq!(coordinates_of_index(22), (2, 1, 1));
        assert_eq!(coordinates_of_index(23), (2, 1, 2));
        assert_eq!(coordinates_of_index(24), (2, 2, 0));
        assert_eq!(coordinates_of_index(25), (2, 2, 1));
        assert_eq!(coordinates_of_index(26), (2, 2, 2));
    }

    #[test]
    fn index_of_coordinates_works() {
        assert_eq!(index_of_coordinates((0, 0, 0)), 0);
        assert_eq!(index_of_coordinates((0, 0, 1)), 1);
        assert_eq!(index_of_coordinates((0, 0, 2)), 2);
        assert_eq!(index_of_coordinates((0, 1, 0)), 3);
        assert_eq!(index_of_coordinates((0, 1, 1)), 4);
        assert_eq!(index_of_coordinates((0, 1, 2)), 5);
        assert_eq!(index_of_coordinates((0, 2, 0)), 6);
        assert_eq!(index_of_coordinates((0, 2, 1)), 7);
        assert_eq!(index_of_coordinates((0, 2, 2)), 8);
        assert_eq!(index_of_coordinates((1, 0, 0)), 9);
        assert_eq!(index_of_coordinates((1, 0, 1)), 10);
        assert_eq!(index_of_coordinates((1, 0, 2)), 11);
        assert_eq!(index_of_coordinates((1, 1, 0)), 12);
        assert_eq!(index_of_coordinates((1, 1, 1)), 13);
        assert_eq!(index_of_coordinates((1, 1, 2)), 14);
        assert_eq!(index_of_coordinates((1, 2, 0)), 15);
        assert_eq!(index_of_coordinates((1, 2, 1)), 16);
        assert_eq!(index_of_coordinates((1, 2, 2)), 17);
        assert_eq!(index_of_coordinates((2, 0, 0)), 18);
        assert_eq!(index_of_coordinates((2, 0, 1)), 19);
        assert_eq!(index_of_coordinates((2, 0, 2)), 20);
        assert_eq!(index_of_coordinates((2, 1, 0)), 21);
        assert_eq!(index_of_coordinates((2, 1, 1)), 22);
        assert_eq!(index_of_coordinates((2, 1, 2)), 23);
        assert_eq!(index_of_coordinates((2, 2, 0)), 24);
        assert_eq!(index_of_coordinates((2, 2, 1)), 25);
        assert_eq!(index_of_coordinates((2, 2, 2)), 26);
    }

    #[test]
    fn index_of_coordinates_is_inverse_of_coordinates_of_index() {
        for i in 0..27 {
            assert_eq!(index_of_coordinates(coordinates_of_index(i)), i)
        }
    }

    #[test]
    fn semi_sum_of_equal_values_is_the_identity() {
        for i in 0..27 {
            let vector = coordinates_of_index(i);
            assert_eq!(semi_sum(vector, vector), vector);
        }
    }

    #[test]
    fn semi_sum_of_distinct_values_is_commutative() {
        for i in 0..26 {
            for j in (i + 1)..27 {
                let a = coordinates_of_index(i);
                let b = coordinates_of_index(j);
                assert_eq!(semi_sum(a, b), semi_sum(b, a));
            }
        }
    }

    #[test]
    fn semi_sum_of_distinct_values_is_cyclic() {
        for i in 0..26 {
            for j in (i + 1)..27 {
                let a = coordinates_of_index(i);
                let b = coordinates_of_index(j);
                let c = semi_sum(a, b);
                assert_eq!(semi_sum(b, c), a);
                assert_eq!(semi_sum(c, a), b);
            }
        }
    }

    #[test]
    fn semi_sum_return_element_that_is_on_same_line_in_r3() {
        // Check the five lines passing from origin.
        assert_eq!(semi_sum((0, 0, 0), (1, 0, 0)), (2, 0, 0));
        assert_eq!(semi_sum((0, 0, 0), (0, 1, 0)), (0, 2, 0));
        assert_eq!(semi_sum((0, 0, 0), (0, 0, 1)), (0, 0, 2));
        assert_eq!(semi_sum((0, 0, 0), (1, 1, 0)), (2, 2, 0));
        assert_eq!(semi_sum((0, 0, 0), (1, 1, 1)), (2, 2, 2));
        // Check lines passing from the center and parallel to an axis.
        assert_eq!(semi_sum((0, 1, 1), (1, 1, 1)), (2, 1, 1));
        assert_eq!(semi_sum((1, 0, 1), (1, 1, 1)), (1, 2, 1));
        assert_eq!(semi_sum((1, 1, 0), (1, 1, 1)), (1, 1, 2));
    }
}
