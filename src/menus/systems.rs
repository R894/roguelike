use belly::prelude::*;
use bevy::prelude::*;

use crate::states::MainState;

use super::StartGameEvent;

pub fn main_menu(mut commands: Commands) {
    commands.add(eml! {
        <body id="menu" s:width="100%" s:height="100%">
            <div s:font="bold" s:font-size="90px">"Roguelike"</div>
            <button on:press=|ctx|{ctx.send_event(StartGameEvent)}><div s:font="bold" s:padding="5px 10px 5px 10px" s:font-size="45px">"START"</div></button>
        </body>
    });
}

pub fn game_over_menu(mut commands: Commands) {
    commands.add(eml! {
        <body id="menu" s:width="100%" s:height="100%">
            <div s:font="bold" s:font-size="90px">"Game Over!"</div>
            <button on:press=|ctx|{ctx.send_event(StartGameEvent)}><div s:font="bold" s:font-size="45px">"RESTART"</div></button>
        </body>
    });
}

pub fn despawn_menu(mut elements: Elements) {
    elements.select("#menu").remove();
}

pub fn start_game_event_system(mut state: ResMut<NextState<MainState>>) {
    state.set(MainState::Game);
}
