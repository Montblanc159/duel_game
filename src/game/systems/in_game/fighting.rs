use super::*;

pub fn fight(
    query: Query<(&PlayerState, &Luck, &Marksmanship, &Damage, &Player), With<Player>>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_dodged: EventWriter<DodgedEvent>,
    mut ev_missed: EventWriter<MissedEvent>,
) {
    let mut query_mut = query.iter_combinations();
    while let Some(
        [(state_0, luck_0, marksmanship_0, damage_0, player_0), (state_1, luck_1, marksmanship_1, damage_1, player_1)],
    ) = query_mut.fetch_next()
    {
        match [state_0.0, state_1.0] {
            [PlayerStates::Attacking, PlayerStates::Attacking] => {
                if marksmanship_1.value.roll() > luck_0.value.roll() {
                    ev_damage.send(DamageEvent {
                        player: player_0.value,
                        value: damage_1.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_0.value,
                    });
                }

                if marksmanship_0.value.roll() > luck_1.value.roll() {
                    ev_damage.send(DamageEvent {
                        player: player_1.value,
                        value: damage_0.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_1.value,
                    });
                }
            }
            [PlayerStates::Attacking, PlayerStates::Idle
            | PlayerStates::NotAttacking
            | PlayerStates::NotDodging
            | PlayerStates::Buffing] => {
                if marksmanship_0.value.roll() > luck_1.value.roll() {
                    ev_damage.send(DamageEvent {
                        player: player_1.value,
                        value: damage_0.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_1.value,
                    });
                }
            }
            [PlayerStates::Idle
            | PlayerStates::NotAttacking
            | PlayerStates::NotDodging
            | PlayerStates::Buffing, PlayerStates::Attacking] => {
                if marksmanship_1.value.roll() > luck_0.value.roll() {
                    ev_damage.send(DamageEvent {
                        player: player_0.value,
                        value: damage_1.value,
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player_0.value,
                    });
                }
            }
            [PlayerStates::Dodging, PlayerStates::Attacking] => {
                ev_dodged.send(DodgedEvent {
                    player: player_0.value,
                });
            }
            [PlayerStates::Attacking, PlayerStates::Dodging] => {
                ev_dodged.send(DodgedEvent {
                    player: player_1.value,
                });
            }
            _ => {} // [PlayerStates::Dodging, PlayerStates::Idle] => {},
                    // [PlayerStates::Dodging, PlayerStates::Dodging] => {},
                    // [PlayerStates::Idle, PlayerStates::Dodging] => {},
                    // [PlayerStates::Idle, PlayerStates::Idle] => {},
        }
    }
}

pub fn listen_damage_event(
    mut ev_damage: EventReader<DamageEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    mut query: Query<(&mut Health, &Player), With<Player>>,
) {
    for ev in ev_damage.read() {
        for (mut health, player) in &mut query {
            if player.value == ev.player {
                health.value = health.value.checked_sub(ev.value).unwrap_or(0);

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: format!("\n-{}", ev.value),
                });
            }
        }
    }
}

pub fn listen_missed_event(
    mut ev_missed: EventReader<MissedEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    query: Query<&Player>,
) {
    for ev in ev_missed.read() {
        for player in &query {
            if player.value == ev.player {
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "\nMissed!".into(),
                });
            }
        }
    }
}

pub fn listen_dodged_event(
    mut ev_dodged: EventReader<DodgedEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    query: Query<&Player>,
) {
    for ev in ev_dodged.read() {
        for player in &query {
            if player.value == ev.player {
                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "\nDodged!".into(),
                });
            }
        }
    }
}

pub fn decrease_dodges(mut query: Query<(&PlayerState, &mut Dodges), With<Player>>) {
    for (player_state, mut dodges) in &mut query {
        if player_state.0 == PlayerStates::Dodging {
            dodges.value = dodges.value.checked_sub(1).unwrap_or(0)
        }
    }
}

pub fn decrease_bullets(mut query: Query<(&PlayerState, &mut Bullets), With<Player>>) {
    for (player_state, mut bullets) in &mut query {
        if player_state.0 == PlayerStates::Attacking {
            bullets.value = bullets.value.checked_sub(1).unwrap_or(0)
        }
    }
}

fn remove_buffes(mut query: Query<&mut Buff, With<Player>>) {
    for mut buff in &mut query {
        buff.value = None;
    }
}

fn despawn_buff_text(
    mut commands: Commands,
    query: Query<(Entity, &BuffText), With<BuffText>>,
    query_player: Query<(&Player, &Buff)>,
) {
    for (entity, buff_text) in &query {
        for (player, buff) in &query_player {
            if player.value == buff_text.value && buff.value.is_none() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn damage_reset(mut query: Query<(&mut Damage, &PlayerState), With<Player>>) {
    for (mut damage, player_state) in &mut query {
        if player_state.0 == PlayerStates::Attacking {
            damage.value = DEFAULT_DAMAGE;
        }
    }
}

pub fn luck_reset(mut query: Query<(&mut Luck, &PlayerState), With<Player>>) {
    for (mut luck, player_state) in &mut query {
        if player_state.0 != PlayerStates::Buffing {
            luck.value = Dice {
                value: DEFAULT_LUCK,
            };
        }
    }
}

pub fn marksmanship_reset(mut query: Query<(&mut Marksmanship, &PlayerState), With<Player>>) {
    for (mut marksmanship, player_state) in &mut query {
        if player_state.0 != PlayerStates::Buffing {
            marksmanship.value = Dice {
                value: DEFAULT_MARKSMANSHIP,
            };
        }
    }
}

pub fn check_fighting_phase_ended(query: Query<&TextColor, With<PlayerTickText>>) -> bool {
    let mut conditions: Vec<bool> = vec![];

    for color in &query {
        conditions.push(color.0.alpha() <= 0.);
    }

    conditions.iter().all(|condition| *condition)
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayStates::Fighting),
        (fight, decrease_bullets, decrease_dodges),
    );

    app.add_systems(
        Update,
        (
            listen_damage_event,
            listen_missed_event,
            listen_dodged_event,
            (next_play_state)
                .run_if(check_fighting_phase_ended)
                .run_if(events_empty::<TickPlayerEvent>),
        )
            .run_if(in_state(PlayStates::Fighting))
            .chain()
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(
        OnExit(PlayStates::Fighting),
        (
            (remove_buffes, despawn_buff_text).chain(),
            damage_reset,
            marksmanship_reset,
            luck_reset,
        ),
    );
}
