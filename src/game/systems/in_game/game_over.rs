use super::*;

pub fn spawn_winner_text(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut commands: Commands,
    query: Query<&Window>,
) {
    let window = query.single();
    // let dimensions = [window.width(), window.height()];

    for ev in ev_game_over.read() {
        let mut game_over_screen = commands.spawn((
            Node {
                width: Val::Px(window.width()),
                height: Val::Px(window.height()),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(25., 0., 255., 0.5)),
            InGameEntity,
        ));

        match ev.state {
            GameOvers::Winner => {
                game_over_screen.with_child((
                    Node {
                        width: Val::Px(window.width()),
                        ..default()
                    },
                    Text::new(format!("Player {} won", ev.player.unwrap())),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont::from_font_size(125.),
                    InGameEntity,
                ));
            }
            GameOvers::Tie => {
                game_over_screen.with_child((
                    Node {
                        width: Val::Px(window.width()),
                        ..default()
                    },
                    Text::new("It's a tie!"),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont::from_font_size(125.),
                    InGameEntity,
                ));
            }
        }
    }
}

pub fn wait_for_input_to_exit_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Enter {
            next_app_state.set(AppStates::Menu);
        }
    }
}
