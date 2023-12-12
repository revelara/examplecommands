#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();

    type Handle = InOut<CyberMessageIn, CyberMessageOut>;

    type Reply = ();

    type Others = ();

    type Signal = ();

    type State = Out<CyberState>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum CyberMessageIn {
    AddNewUser(ActorId),
    ModifyUser(ActorId, String),
    AddNewGame(String, CyberLevels),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum CyberMessageOut {
    UserCreated,
    UserModified,
    GameCreated,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberState {
    pub players: Vec<CyberPlayer>,
    pub games: Vec<CyberGames>,
    pub sponsors: Vec<CyberSponsors>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberPlayer {
    pub id: ActorId,
    pub name: String,
    pub points: u64,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberGames {
    pub name: String,
    pub levels: CyberLevels,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberLevels {
    pub name: String,
    pub modules: Vec<CyberModules>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberModules {
    pub level: u8,
    pub questions: Vec<CyberQuestion>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberQuestion {
    question: String,
    options: Vec<String>,
    correct_answer: u8,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct CyberSponsors;
