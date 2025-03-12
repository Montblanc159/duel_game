use super::*;

pub fn countdown(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
    time: Res<Time>,
    mut countdown_timer: ResMut<CountdownTimer>,
) {
    countdown_timer.0.tick(time.delta());

    if countdown_timer.0.just_finished() {
        next_play_state.set(play_state.get().next());
        countdown_timer.0.reset()
    }
}

pub fn reset_countdown_timer(mut countdown_timer: ResMut<CountdownTimer>) {
    countdown_timer.0.reset()
}

// UI
pub fn update_countdown_timer_ui(
    betting_timer: Res<CountdownTimer>,
    mut query: Query<&mut Text, With<TimerUIText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", betting_timer.0.remaining_secs().round());
    }
}
