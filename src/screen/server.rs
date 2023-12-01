use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::options::components::OptionsResult;
use crate::screen::resource::GameSignType;


use super::{AppState, GameInteraction, GameSigned, Message, User, UserInfo, Users, S2D};
use bevy::prelude::*;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::time::interval;
use tokio::{
    io::{AsyncWriteExt},
    net::TcpStream,
    runtime::Runtime,
    select,
    sync::{
        mpsc::{self},
        RwLock,
    },
};
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TokioRuntime>()
        
            .add_systems(OnEnter(AppState::ServerRunning), create_server)
            // tell thread clear
            .add_systems(
                OnEnter(AppState::GameEnd),
                |mut command: Commands,
                 signed: Res<GameSigned>,
                 mut state: ResMut<NextState<AppState>>| {
                    signed.sd.blocking_send(GameSignType::End);
                    thread::sleep(Duration::from_millis(100));
                    command.remove_resource::<GameSigned>();
                    state.set(AppState::StartScreen)
                },
            );
    }
}

#[derive(Deserialize)]
struct NetConnect {
    name: String,
    code: String,
}

#[derive(Serialize)]
enum NetReceiver {
    Success(Vec<UserInfo>),
    Failed,
    Full,
}
#[derive(Resource)]
struct TokioRuntime {
    rt: Runtime,
}


impl Default for TokioRuntime {
    fn default() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
        }
    }
}

/// when game on enter lobby,create server
fn create_server(
    mut command: Commands,
    interaction: Res<GameInteraction>,
    users: Res<Users>,
    tokio_runtime: Res<TokioRuntime>,
    option:Res<OptionsResult>,

    setting_option:Res<OptionsResult>,

) {
    let server_ip = interaction.server_ip.clone();
    let interaction_code = interaction.code.clone();
    let num_players=option.num_players;
    let (sd, mut rx) = mpsc::channel(1);
    command.insert_resource(GameSigned { sd });
    let users = users.users.clone();

    tokio_runtime.rt.spawn(async move{

        // init tcp server
        let listener = TcpListener::bind(server_ip).await.unwrap();
        let my_local_ip = local_ip().unwrap();
        println!("You created a server with IP {:?}", my_local_ip);

        let message=loop{
            select! {
                client=listener.accept()=>{
                    let (client,ipaddr)=client.unwrap();
                    let mut client=S2D::from(client);
                    let ip=ipaddr.to_string();
                    let interaction_code=interaction_code.clone();
                    let users=users.clone();

                    let NetConnect{name,code}=match client.recv().await{
                        Ok(data)=>{
                            let Ok(data) = bincode::deserialize::<NetConnect>(&data) else{
                                println!("data error!");
                                continue;
                            };
                            data
                        }
                        Err(e)=>{
                            println!("{e}");
                            continue;
                        }
                    };

                    if code!=interaction_code{
                        let encoded: Vec<u8>=bincode::serialize(&NetReceiver::Failed).expect("serde error!");
                        client.send(&encoded).await;
                        continue
                    }
                    let mut users2=users.write().await;
                    if users2.len()>=num_players{
                        let encoded: Vec<u8>=bincode::serialize(&NetReceiver::Full).expect("serde error!");
                        client.send(&encoded).await;
                        
                    }
                    let (send_message,recever)=mpsc::channel::<Message>(50);
                    let (sender,recv_message)=mpsc::channel::<Message>(50);
                    // for every send
                    for user in users2.iter(){
                        let (ip,name)=(ip.to_string(),name.to_string());
                        user.send_message.send(Message::Join(UserInfo { name, ip })).await;
                    }
                    let ip2=ip.clone();
                    users2.push(super::User { ip, name, send_message,recv_message });
                    let users_list=users2.iter().map(|User{ip,name,..
                    }|{
                        let (ip,name)=(ip.to_string(),name.to_string());
                        UserInfo{
                            ip,name
                        }
                    }).collect::<Vec<UserInfo>>();
                    drop(users2);

                    // send ok
                    let encoded: Vec<u8>=bincode::serialize(&NetReceiver::Success(users_list)).expect("serde error!");

                    client.send(&encoded).await;

                    tokio::spawn(handle_connection(client,recever,users,ip2,sender));
                }
                game_message=rx.recv()=>{
                    break game_message;
                }
            }
        };
        if let Some(game_message)=message{
            if game_message==GameSignType::Start{
                let mut interval = interval(Duration::from_secs(1));
                let users=users.clone();

                // if is game start signed
                loop{
                    // every second per one loop

                    tokio::select! {
                        _=interval.tick()=>{
                            
                        }
                        game_message=rx.recv()=>{
                            break;
                        }
                    }
                }
            }
        }
        let mut users=users.write().await;
        for user in users.iter(){
            user.send_message.send_timeout(Message::Close, Duration::from_millis(500)).await;
        }
        users.clear();
        drop(users);
    });
}

/// do everything for handle
async fn handle_connection(
    mut stream: S2D<TcpStream>,
    mut recever: mpsc::Receiver<Message>,
    users: Arc<RwLock<Vec<User>>>,
    ip: String,
    sender:mpsc::Sender<Message>,
) {
    loop {
        select! {
            result=stream.recv()=> {
                match result{
                    Ok(data) => {
                        let message:Message=bincode::deserialize(&data).expect("serde error!");
                        match message{
                            // Game Exit
                            Message::Close=>{
                                let mut users=users.write().await;
                            users.retain(|user|user.ip!=ip);
                            for user in users.iter(){
                                let ip=ip.clone();
                                user.send_message.send(Message::Kick(ip)).await;
                            }
                            drop(users);
                            println!("user exit");
                            return;
                            }
                            // oprate
                            Message::Raise(_)|Message::Call|Message::Fold|Message::Check=>{

                                let users=users.write().await;
                                for user in users.iter(){
                                    if user.ip!=ip{
                                        user.send_message.send(message.clone()).await;
                                    }
                                }
                                drop(users);

                            }
                            else_message=>{
                                sender.send(else_message).await;
                            }
                        }
                    },
                    Err(_e)=>{
                        let mut users=users.write().await;
                            users.retain(|user|user.ip!=ip);
                            for user in users.iter(){
                                let ip=ip.clone();
                                user.send_message.send(Message::Kick(ip)).await;
                            }
                            drop(users);
                            println!("user exit");
                            return;
                    }
                }
            }


            message=recever.recv()=> {
                    let Some(message)=message else{
                        continue
                    };
                        match message{
                            Message::Kick(this_ip)=>{
                                if this_ip==ip{
                                    let encoded: Vec<u8>=bincode::serialize(&Message::BeKick).expect("serde error!");
                                    stream.send(&encoded).await;
                                    break;
                                }else{
                                    let encoded: Vec<u8>=bincode::serialize(&Message::Kick(this_ip)).expect("serde error!");
                                    stream.send(&encoded).await;
                                }

                            }
                            Message::Close=>{
                                break
                            }
                            tell=>{
                                let encode:Vec<u8>=bincode::serialize(&tell).expect("serde error!");
                                stream.send(&encode).await;
                            }
                        }

            }
        };
    }
    let mut users = users.write().await;
    users.retain(|user| user.ip != ip);
    drop(users);
}