use super::*;

pub fn prepare_player_for_next_round(
    mut query: Query<&mut PlayerState, With<Player>>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
) {
    for mut player_state in &mut query {
        player_state.0 = PlayerStates::Idle;
        ev_change_player_state.send(PlayerStateChangeEvent);
    }
}

pub fn is_not_game_over(game_over: Res<GameOver>) -> bool {
    !game_over.0
}

pub fn check_if_dead(
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

pub fn check_if_out_of_ammo(
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

pub fn check_if_no_more_rounds(
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

pub fn restore_dodge(
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

pub fn restore_bullet(
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

pub fn check_rounding_up_phase_ended(query: Query<&TextColor, With<PlayerTickText>>) -> bool {
    let mut conditions: Vec<bool> = vec![];

    for color in &query {
        conditions.push(color.0.alpha() <= 0.);
    }

    conditions.iter().all(|condition| *condition)
}

pub fn increase_round_counter(mut round_counter: ResMut<RoundCounter>) {
    round_counter.0 += 1;
}
