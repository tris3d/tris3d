#[test]
fn play_match() {
    let match_id = String::from("match_id1");
    let tris3d_match = tris3d::Match::new(match_id);
    // TODO add moves and play a match
    assert_eq!(tris3d_match.board.has_tris(), false);
}
