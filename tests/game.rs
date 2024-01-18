#[test]
fn play() {
    let mut game = tris3d::new_game();

    let player_1 = String::from("Alice");
    let player_2 = String::from("Bob");
    let player_3 = String::from("Neuromancer");

    game.add_player(player_1).unwrap();
    game.add_player(player_2).unwrap();
    game.add_player(player_3).unwrap();

    assert_eq!(game.add_move(String::from("Alice"), 'A').unwrap(), 0);
}
