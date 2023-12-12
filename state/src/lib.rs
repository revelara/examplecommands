#![no_std]
use cyberclub_io::*;
use gmeta::metawasm;
use gstd::prelude::*;

#[metawasm]
pub mod metafns {
    pub type State = CyberState;

    //TODO, implement the functions to get specific part of the state
    pub fn get_players(state: State) -> Vec<CyberPlayer> {
        state.players
    }

    pub fn get_games(state: State) -> Vec<CyberGames> {
        state.games
    }

    pub fn get_sponsors(state: State) -> Vec<CyberSponsors> {
        state.sponsors
    }
}
