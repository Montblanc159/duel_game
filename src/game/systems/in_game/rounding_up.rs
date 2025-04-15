use super::*;

fn prepare_player_for_next_round(
    mut query: Query<(&mut PlayerState, &Player), With<Player>>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
) {
    for (mut player_state, player) in &mut query {
        player_state.0 = PlayerStates::Idle;
        ev_change_player_state.send(PlayerStateChangeEvent {
            player: player.value,
        });
    }
}

fn is_not_game_over(game_over: Res<GameOver>) -> bool {
    !game_over.0
}

fn check_if_dead(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Health, &Player)>,
    mut game_over: ResMut<GameOver>,
) {
    let mut dead = [false, false];

    for (health, player) in &mut query {
        if health.value == 0 {
            let index: usize = (player.value - 1).into();
            dead[index] = true
        }
    }

    match dead {
        [true, true] => {
            ev_game_over.send(GameOverEvent {
                player: None,
                state: GameOvers::Tie,
            });
            game_over.0 = true;
        }
        [true, false] => {
            ev_game_over.send(GameOverEvent {
                player: Some(2),
                state: GameOvers::Winner,
            });
            game_over.0 = true;
        }
        [false, true] => {
            ev_game_over.send(GameOverEvent {
                player: Some(1),
                state: GameOvers::Winner,
            });
            game_over.0 = true;
        }
        [false, false] => {}
    }
}

fn check_if_out_of_ammo(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Bullets, &Health, &Player)>,
    mut game_over: ResMut<GameOver>,
) {
    let mut query_mut = query.iter_combinations_mut();
    while let Some([(bullets_0, health_0, player_0), (bullets_1, health_1, player_1)]) =
        query_mut.fetch_next()
    {
        let ammo = [bullets_0.value, bullets_1.value];

        if ammo.iter().all(|&x| x == 0) {
            game_over.0 = true;

            let winner: Option<u8>;

            match health_0.value < health_1.value {
                true => winner = Some(player_1.value),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.value)
                    }
                }
            }

            if winner.is_some() {
                ev_game_over.send(GameOverEvent {
                    player: winner,
                    state: GameOvers::Winner,
                });
            } else {
                ev_game_over.send(GameOverEvent {
                    player: None,
                    state: GameOvers::Tie,
                });
            }
        }
    }
}

fn check_if_no_more_rounds(
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut query: Query<(&Health, &Player)>,
    mut game_over: ResMut<GameOver>,
    round: Res<RoundCounter>,
) {
    if round.0 == N_MAX_ROUND {
        game_over.0 = true;

        let mut winner: Option<u8>;

        let mut query_mut = query.iter_combinations_mut();
        while let Some([(health_0, player_0), (health_1, player_1)]) = query_mut.fetch_next() {
            match health_0.value < health_1.value {
                true => winner = Some(player_1.value),
                false => {
                    if health_0.value == health_1.value {
                        winner = None;
                    } else {
                        winner = Some(player_0.value)
                    }
                }
            }

            if winner.is_some() {
                ev_game_over.send(GameOverEvent {
                    player: winner,
                    state: GameOvers::Winner,
                });
            } else {
                ev_game_over.send(GameOverEvent {
                    player: None,
                    state: GameOvers::Tie,
                });
            }
        }
    }
}

fn restore_dodge(
    round_counter: Res<RoundCounter>,
    mut query: Query<(&mut Dodges, &Luck, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    if round_counter.0 % 2 == 0 {
        for (mut dodges, luck, player) in &mut query {
            if luck.value.roll() >= 25 {
                dodges.value += 1;
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "+1 dodge".into(),
                });
            } else {
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "+0 dodge".into(),
                });
            }
        }
    }
}

fn restore_bullet(
    round_counter: Res<RoundCounter>,
    mut query: Query<(&mut Bullets, &Luck, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    if round_counter.0 % 2 == 0 {
        for (mut bullets, luck, player) in &mut query {
            if luck.value.roll() >= 25 {
                bullets.value += 1;
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "\n+1 bullet".into(),
                });
            } else {
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "\n+0 bullet".into(),
                });
            }
        }
    }
}

fn check_rounding_up_phase_ended(query: Query<&TextColor, With<PlayerTickText>>) -> bool {
    let mut conditions: Vec<bool> = vec![];

    for color in &query {
        conditions.push(color.0.alpha() <= 0.);
    }

    conditions.iter().all(|condition| *condition)
}

fn increase_round_counter(mut round_counter: ResMut<RoundCounter>) {
    round_counter.0 += 1;
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayStates::RoundingUp),
        (
            check_if_dead,
            check_if_no_more_rounds.run_if(is_not_game_over),
            check_if_out_of_ammo.run_if(is_not_game_over),
            restore_bullet.run_if(is_not_game_over),
            restore_dodge.run_if(is_not_game_over),
        )
            .chain(),
    );

    app.add_systems(
        Update,
        (next_play_state
            .run_if(check_rounding_up_phase_ended)
            .run_if(events_empty::<TickPlayerEvent>),)
            .run_if(in_state(PlayStates::RoundingUp))
            .run_if(is_not_game_over)
            .chain()
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(
        OnExit(PlayStates::RoundingUp),
        (
            prepare_player_for_next_round,
            increase_round_counter,
            despawn_player_tick_ui,
        )
            .run_if(is_not_game_over),
    );
}
