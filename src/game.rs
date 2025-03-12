use bevy::prelude::*;
use rand::{distr::StandardUniform, prelude::*};

mod components;
mod events;
mod resources;
mod settings;
mod states;
mod systems;
mod ui_components;
mod ui_defaults;

use components::*;
use events::*;
use resources::*;
use settings::*;
use states::*;
use systems::*;
use ui_components::*;
use ui_defaults::*;

// Game
// ================================================================

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, resources::plugin, states::plugin));

    app.add_systems(OnEnter(AppStates::Menu), spawn_start_game_ui);

    app.add_systems(
        Update,
        wait_for_input_to_start_game.run_if(in_state(AppStates::Menu)),
    );

    app.add_systems(OnExit(AppStates::Menu), clean_system::<MenuEntity>);

    app.add_systems(
        OnEnter(AppStates::InGame),
        (
            spawn_players,
            (
                spawn_play_state_text,
                spawn_player_state_text,
                spawn_round_number_text,
                spawn_health_text,
                spawn_bullet_text,
                spawn_dodge_text,
            ),
            launch_game,
        )
            .chain(),
    );

    app.add_systems(
        OnExit(AppStates::InGame),
        (pause_game, reset_game, clean_system::<InGameEntity>).chain(),
    );

    app.add_systems(
        Update,
        (
            listen_game_overs,
            listen_spawn_alert_text,
            player_state_text_update,
            round_number_text_update,
            play_state_text_update,
            health_text_update,
            bullet_text_update,
            dodge_text_update,
        )
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(
        OnEnter(PlayStates::Preparing),
        (check_if_last_round, spawn_press_spacebar_ui),
    );

    app.add_systems(
        Update,
        wait_for_input_to_next_play_state
            .run_if(in_state(PlayStates::Preparing))
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(
        OnExit(PlayStates::Preparing),
        (despawn_press_spacebar_ui, despawn_alert_text),
    );

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

    app.add_systems(
        OnEnter(PlayStates::Betting),
        (reset_betting_timer, spawn_timer_ui).chain(),
    );

    app.add_systems(
        Update,
        (betting_countdown, set_player_state, update_betting_timer_ui)
            .run_if(in_state(PlayStates::Betting))
            .run_if(in_state(AppStates::InGame)),
    );

    app.add_systems(
        OnExit(PlayStates::Betting),
        (
            despawn_timer_ui,
            (
                add_buffes,
                spawn_buff_text,
                (
                    increase_damage_buff,
                    golden_bullet_buff,
                    heal_buff,
                    super_heal_buff,
                    luck_buff,
                    marksmanship_buff,
                ),
            )
                .chain(),
        ),
    );

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
            listen_spawn_player_tick_ui,
            animate_player_tick_text_opacity,
            animate_player_tick_font_size,
            (next_play_state).run_if(check_fighting_phase_ended),
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
            despawn_player_tick_ui,
        ),
    );

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
        (
            listen_spawn_player_tick_ui,
            animate_player_tick_text_opacity,
            animate_player_tick_font_size,
            (next_play_state).run_if(check_rounding_up_phase_ended),
        )
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

    app.add_systems(OnEnter(PlayStates::GameOver), spawn_winner_text);
    // app.add_systems(OnExit(PlayStates::GameOver));

    app.add_systems(
        Update,
        wait_for_input_to_exit_game
            .run_if(in_state(PlayStates::GameOver))
            .run_if(in_state(AppStates::InGame)),
    );
}
