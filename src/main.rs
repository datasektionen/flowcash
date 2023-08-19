use flowcash::*;

fn main() {
    let mut game_state = initial_game_state();

    // Game Loop
    loop {
        let input_state = capture_input_state();

        player_control_system(&mut game_state, &input_state)
        ai_behavior_system(&mut game_state);

        render_system(&mut game);
        audio_system(&mut game);

        // wait for vsync?
    }
}
