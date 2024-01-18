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
    assert_eq!(game.add_move(String::from("Bob"), 'H').unwrap(), 0);
    assert_eq!(game.add_move(String::from("Neuromancer"), 'G').unwrap(), 0);
    assert_eq!(game.add_move(String::from("Alice"), '*').unwrap(), 0);
    assert_eq!(game.add_move(String::from("Bob"), 'I').unwrap(), 0);
    assert_eq!(game.add_move(String::from("Alice"), 'F').unwrap(), 0);
    assert_eq!(game.add_move(String::from("Neuromancer"), 'V').unwrap(), 1);

    assert_eq!(game.status, tris3d::game::Status::IsOver);
}
