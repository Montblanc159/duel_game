use super::*;

pub fn betting_countdown(
    play_state: Res<State<PlayStates>>,
    mut next_play_state: ResMut<NextState<PlayStates>>,
    time: Res<Time>,
    mut betting_timer: ResMut<BettingTimer>,
) {
    betting_timer.0.tick(time.delta());

    if betting_timer.0.just_finished() {
        next_play_state.set(play_state.get().next());
    }
}

pub fn reset_betting_timer(mut betting_timer: ResMut<BettingTimer>) {
    betting_timer.0.reset()
}

pub fn set_player_state(
    mut query: Query<(&KeyAssignment, &mut PlayerState, &Dodges, &Bullets)>,
    mut ev_change_player_state: EventWriter<PlayerStateChangeEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        for (key_assignements, mut player_state, dodges, bullets) in &mut query {
            let requested_state = key_assignements
                .derive_player_state(key)
                .unwrap_or(player_state.0);

            ev_change_player_state.send(PlayerStateChangeEvent);

            match requested_state {
                PlayerStates::Attacking => {
                    if bullets.value > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotAttacking;
                    }
                }
                PlayerStates::Dodging => {
                    if dodges.value > 0 {
                        player_state.0 = requested_state;
                    } else {
                        player_state.0 = PlayerStates::NotDodging;
                    }
                }
                other_player_state => player_state.0 = other_player_state,
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Buffes {
    GoldenBulletBuff,
    IncreaseDamageBuff,
    HealBuff,
    SuperHealBuff,
    LuckBuff,
    MarksmanshipBuff,
}

impl Distribution<Buffes> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Buffes {
        let index = rng.random_range(0..=5);
        match index {
            0 => Buffes::GoldenBulletBuff,
            1 => Buffes::IncreaseDamageBuff,
            2 => Buffes::HealBuff,
            3 => Buffes::SuperHealBuff,
            4 => Buffes::LuckBuff,
            5 => Buffes::MarksmanshipBuff,
            _ => unreachable!(),
        }
    }
}

pub fn add_buffes(mut query: Query<(&mut Buff, &PlayerState), With<Player>>) {
    for (mut buff, player_state) in &mut query {
        if player_state.0 == PlayerStates::Buffing {
            let random_buff: Buffes = rand::random();
            buff.value = Some(random_buff);
        }
    }
}

pub fn remove_buffes(mut query: Query<&mut Buff, With<Player>>) {
    for mut buff in &mut query {
        buff.value = None;
    }
}

pub fn increase_damage_buff(
    mut query: Query<(&Buff, &mut Damage, &mut PlayerState, &mut Bullets, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut damage, mut player_state, mut bullets, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::IncreaseDamageBuff {
                damage.value += 1;
                bullets.value += 1;
                player_state.0 = PlayerStates::Attacking;

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "Double shot".into(),
                });
            }
        }
    }
}

pub fn golden_bullet_buff(
    mut query: Query<(&Buff, &mut Damage, &mut PlayerState, &mut Bullets, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut damage, mut player_state, mut bullets, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::GoldenBulletBuff {
                damage.value = 5;
                bullets.value += 1;
                player_state.0 = PlayerStates::Attacking;

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "Golden bullet".into(),
                });
            }
        }
    }
}

pub fn heal_buff(
    mut query: Query<(&Buff, &mut Health, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut health, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::HealBuff {
                if health.value < DEFAULT_HEALTH {
                    health.value += 1;

                    ev_tick_player.send(TickPlayerEvent {
                        player: player.value,
                        value: "Healing".into(),
                    });
                } else {
                    health.value = DEFAULT_HEALTH;

                    ev_tick_player.send(TickPlayerEvent {
                        player: player.value,
                        value: "Maxed HP".into(),
                    });
                }
            }
        }
    }
}

pub fn super_heal_buff(
    mut query: Query<(&Buff, &mut Health, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut health, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::SuperHealBuff {
                if health.value < DEFAULT_HEALTH - 1 {
                    health.value += 2;

                    ev_tick_player.send(TickPlayerEvent {
                        player: player.value,
                        value: "Super healing".into(),
                    });
                } else {
                    health.value = DEFAULT_HEALTH;

                    ev_tick_player.send(TickPlayerEvent {
                        player: player.value,
                        value: "Maxed HP".into(),
                    });
                }
            }
        }
    }
}

pub fn luck_buff(
    mut query: Query<(&Buff, &mut Luck, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut luck, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::LuckBuff {
                luck.value += Dice { value: 50 };

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "Lucky charm".into(),
                });
            }
        }
    }
}

pub fn marksmanship_buff(
    mut query: Query<(&Buff, &mut Marksmanship, &Player), With<Player>>,
    mut ev_tick_player: EventWriter<TickPlayerEvent>,
) {
    for (&buff, mut marksmanship, player) in &mut query {
        if let Some(buff_value) = buff.value {
            if buff_value == Buffes::MarksmanshipBuff {
                marksmanship.value += Dice { value: 50 };

                ev_tick_player.send(TickPlayerEvent {
                    player: player.value,
                    value: "Sharpshooter".into(),
                });
            }
        }
    }
}

pub fn spawn_buff_text(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<(&Player, &PlayerState, &Buff), With<Player>>,
) {
    let window = window_query.single();
    let dimensions = [25., 200.];

    for (player, player_state, buff) in &query {
        if buff.value.is_some() && player_state.0 == PlayerStates::Buffing {
            let left_position = if player.value == 1 {
                DEFAULT_MARGIN + 50.
            } else {
                window.width() - DEFAULT_MARGIN - 50.
            };

            commands.spawn((
                Node {
                    width: Val::Px(dimensions[0]),
                    height: Val::Px(dimensions[1]),
                    position_type: PositionType::Relative,
                    top: Val::Px(window.height() / 2.),
                    left: Val::Px(left_position - (dimensions[0] / 2.)),
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::new(format!("{:?}", buff.value.unwrap())),
                TextFont {
                    font_size: 10.,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                BuffText {
                    value: player.value,
                },
                InGameEntity,
            ));
        }
    }
}

pub fn despawn_buff_text(
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

// UI
pub fn update_betting_timer_ui(
    betting_timer: Res<BettingTimer>,
    mut query: Query<&mut Text, With<TimerUIText>>,
) {
    for mut text in &mut query {
        **text = format!("{}", betting_timer.0.remaining_secs().round());
    }
}
