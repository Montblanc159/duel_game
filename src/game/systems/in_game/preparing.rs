use super::*;

pub fn spawn_press_spacebar_ui(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();
    let dimensions = [500., 50.];

    commands.spawn((
        Node {
            width: Val::Px(dimensions[0]),
            height: Val::Px(dimensions[1]),
            position_type: PositionType::Absolute,
            bottom: Val::Px(DEFAULT_MARGIN * 2. - (dimensions[1] / 2.)),
            left: Val::Px(window.width() / 2. - (dimensions[0] / 2.)),
            ..default()
        },
        Text::new("Press space bar"),
        TextFont {
            font_size: 30.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        PressSpacebarText,
        InGameEntity,
    ));
}

pub fn despawn_press_spacebar_ui(
    mut commands: Commands,
    query: Query<Entity, With<PressSpacebarText>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn check_if_last_round(round: Res<RoundCounter>, mut ev_last_round: EventWriter<AlertEvent>) {
    if round.0 == N_MAX_ROUND {
        ev_last_round.send(AlertEvent {
            value: "Last round!".into(),
        });
    }
}
