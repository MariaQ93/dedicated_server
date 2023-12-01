use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{sync::Arc};
use tokio::sync::{mpsc, RwLock};

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Kick(String),
    Start,
    Close,
    Join(UserInfo),
    BeKick,
    Over,
    Raise(u64),
    Call,
    Fold,
    Check,
    Reset,
}

pub struct User {
    /// client ip
    pub ip: String,
    /// client name
    pub name: String,
    /// when ui get message,it will call send_message function
    pub send_message: mpsc::Sender<Message>,
}

#[derive(Resource)]
pub struct GameInteraction {
    /// this is server ip
    pub server_ip: String,

    /// this is server code
    pub code: String,
}

impl Default for GameInteraction {
    fn default() -> Self {
        GameInteraction {
            server_ip: "127.0.0.1:3000".to_owned(),
            code: "TEST".to_owned(),
        }
    }
}
#[derive(Resource, Default)]
pub struct Users {
    pub users: Arc<RwLock<Vec<User>>>,
}

#[derive(Resource)]
pub struct GameSigned {
    /// if it is false,server will close
    pub sd: tokio::sync::mpsc::Sender<()>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub ip: String,
}

pub enum UiInfoString{
    Info(String),
    Warn(String),
    Error(String),
    None,
}

#[derive(Resource)]
pub struct UiInfo{
    pub info:UiInfoString,
    timer:Timer,
}

impl Default for UiInfo{
    fn default() -> Self {
        UiInfo { info: UiInfoString::None, timer: Timer::from_seconds(5.0, TimerMode::Repeating) }
    }
}
impl UiInfo{
    pub fn set(&mut self,info:UiInfoString){
        self.info=info;
        self.timer.reset();
    }
}

pub fn reset_infostring(time: Res<Time>, mut uiinfo: ResMut<UiInfo>){
    if uiinfo.timer.tick(time.delta()).just_finished(){
        uiinfo.info=UiInfoString::None;
    }
}

#[derive(Serialize,Deserialize,Clone,Default)]
enum GameOperation{
    #[default]
    Check,
    Call,
    Raise(u64),
    Fold,
}


#[derive(Clone,Default)]
pub struct Player{
    pub money:u64,
    // 0 is UTG,number -1 is BB,number -2 is SB
    pub pos:u64,
    // op
    pub op:GameOperation,
    pub pay_money:u64,
}

#[derive(Resource)]
pub struct PlayerInfos{
    pub players:Vec<Player>,
    pub game_pos:u64,
    pub money:u64,
}

impl PlayerInfos{
    pub fn init(size:usize)->Self{
        let mut player=Vec::with_capacity(size);
        player.resize(size,Player::default());
        PlayerInfos{
            players:player,
            game_pos:0,
            money:0,
        }
    }
}