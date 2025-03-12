use super::*;

fn countdown(
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

fn reset_countdown_timer(mut countdown_timer: ResMut<CountdownTimer>) {
    countdown_timer.0.reset()
}

// UI
fn update_countdown_timer_ui(
    betting_timer: Res<CountdownTimer>,
    mut query: Query<&mut Text, With<TimerUIText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", betting_timer.0.remaining_secs().round());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayStates::Countdown),
        (reset_countdown_timer, spawn_timer_ui).chain(),
    );

    app.add_systems(
        Update,
        (countdown, update_countdown_timer_ui)
            .chain()
            .run_if(in_state(PlayStates::Countdown))
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(OnExit(PlayStates::Countdown), despawn_timer_ui);
}
