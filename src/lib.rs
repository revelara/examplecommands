#![no_std]
use cyberclub_io::{CyberMessageIn, CyberPlayer, CyberState};
use gstd::{debug, msg, prelude::*};

static mut STATE: Option<CyberState> = None;

#[no_mangle]
extern "C" fn init() {
    let _state = state_mut();
    _state.players = vec![];
    _state.games = vec![];
    _state.sponsors = vec![];
}

#[no_mangle]
extern "C" fn handle() {
    let _event: CyberMessageIn = msg::load().expect("Error while loading the event in handle");

    let _state = state_mut();

    match _event {
        CyberMessageIn::AddNewUser(actor_id) => {
            debug!("A new user will be added with actor Id: {:?}", actor_id);

            let _new_cyber_player = CyberPlayer {
                id: actor_id,
                name: "".to_string(),
                points: 0,
            };
            _state.players.push(_new_cyber_player);
        }
        CyberMessageIn::ModifyUser(_, _) => todo!(),
        CyberMessageIn::AddNewGame(_, _) => todo!(),
    }

    debug!("State: {:?}", unsafe { STATE.as_mut() });
}

fn state_mut() -> &'static mut CyberState {
    let _state = unsafe { STATE.as_mut() };
    unsafe { _state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn state() {
    let greeting = unsafe { STATE.as_ref().expect("The contract is not initialized") };
    msg::reply(greeting, 0).expect("Failed to share state");
}
