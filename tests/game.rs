#[test]
fn play() {
    let mut game = tris3d::new_game();

    let player_1 = String::from("Alice");
    let player_2 = String::from("Bob");
    let player_3 = String::from("Neuromancer");

    match game.add_player(player_1) {
        Ok(_) => {}
        Err(_) => {}
    }
    match game.add_player(player_2) {
        Ok(_) => {}
        Err(_) => {}
    }
    match game.add_player(player_3) {
        Ok(_) => {}
        Err(_) => {}
    }

    assert_eq!(game.board.has_tris(), false);
}
