use super::*;

pub fn fight(
    query: Query<(&PlayerState, &Player, &Target, &Damage, &Marksmanship), With<Player>>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut ev_depleted: EventWriter<DepletedEvent>,
) {
    for (player_state, player, target, damage, marksmanship) in &query {
        match player_state.0 {
            PlayerStates::Attacking => {
                ev_attack.send(AttackEvent {
                    player: target.value,
                    damage: damage.value,
                    marksmanship: marksmanship.value,
                });
            }
            PlayerStates::NotAttacking => {
                ev_depleted.send(DepletedEvent {
                    player: player.value,
                    source: DepletedSources::Bullets,
                });
            }
            _ => {}
        }
    }
}

pub fn listen_attack_event(
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_dodged: EventWriter<DodgedEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_depleted: EventWriter<DepletedEvent>,
    query: Query<(&Player, &PlayerState), With<Player>>,
) {
    for ev in ev_attack.read() {
        for (player, player_state) in &query {
            if ev.player == player.value {
                match player_state.0 {
                    PlayerStates::Dodging => {
                        ev_dodged.send(DodgedEvent {
                            player: player.value,
                        });
                    }
                    state => {
                        if state == PlayerStates::NotDodging {
                            ev_depleted.send(DepletedEvent {
                                player: player.value,
                                source: DepletedSources::Dodges,
                            });
                        }

                        ev_damage.send(DamageEvent {
                            player: player.value,
                            value: ev.damage,
                            marksmanship: ev.marksmanship,
                        });
                    }
                }
            }
        }
    }
}

pub fn listen_damage_event(
    mut ev_damage: EventReader<DamageEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    mut ev_missed: EventWriter<MissedEvent>,
    mut query: Query<(&mut Health, &Player, &Luck), With<Player>>,
) {
    for ev in ev_damage.read() {
        for (mut health, player, luck) in &mut query {
            if player.value == ev.player {
                if ev.marksmanship.roll() > luck.value.roll() {
                    health.value = health.value.checked_sub(ev.value).unwrap_or(0);

                    ev_tick_player.send(TickPlayerEvent {
                        player: player.value,
                        value: format!("\n-{}", ev.value),
                    });
                } else {
                    ev_missed.send(MissedEvent {
                        player: player.value,
                    });
                }
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

pub fn listen_depleted_event(
    mut ev_dodged: EventReader<DepletedEvent>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
    query: Query<&Player>,
) {
    for ev in ev_dodged.read() {
        for player in &query {
            if player.value == ev.player {
                let msg = match ev.source {
                    DepletedSources::Dodges => "\nCan't dodge!",
                    DepletedSources::Bullets => "\nOut of mana!",
                };

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: msg.into(),
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
            listen_attack_event,
            listen_damage_event,
            listen_missed_event,
            listen_dodged_event,
            listen_depleted_event,
            (next_play_state)
                .run_if(check_fighting_phase_ended)
                .run_if(events_empty::<AttackEvent>)
                .run_if(events_empty::<DodgedEvent>)
                .run_if(events_empty::<DepletedEvent>)
                .run_if(events_empty::<MissedEvent>)
                .run_if(events_empty::<DamageEvent>)
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
