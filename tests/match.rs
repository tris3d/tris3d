#[test]
fn play_match() {
    let mut tris3d_match = tris3d::Match::new();

    let player_a = tris3d::Player {
        id: String::from("Alice"),
    };

    tris3d_match.add_player(&player_a);

    assert_eq!(tris3d_match.board.has_tris(), false);
}
