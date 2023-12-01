use bevy::prelude::*;
use super::components::*;
use super::cards::*; 
use super::buttons::*;
use super::hard_ai_logic::select_action_for_hand;
use crate::options::components::OptionsResult;
// use rand::Rng;
use super::easy_ai_logic::*;
use bevy::text::BreakLineOn;
use crate::AppState;
use bevy::input::keyboard::KeyboardInput;
use std::fs::File;
use std::io::prelude::*;
use serde_json::*;
use std::{thread, time};

const PLAYER_SIZE: f32 =  60.;
const PLAYER_POS: (f32, f32, f32) = (140., -175., 2.);
const PLAYER_BLIND_POS: (f32, f32, f32) = (140., -220., 2.);

pub fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_num_mut: ResMut<NumPlayers>,
    mut poker_turn: ResMut<PokerTurn>,
    options_result: Res<OptionsResult>,
) {
    let mut player_money = options_result.money_per_player;
    let mut player_bet = 0;
    let pot = 0;
    let top_bet = 0;

    if options_result.is_loaded_game {
        //open the game file and separate into lines
        let mut game_file = File::open("saved_game.txt").expect("Can't open file");
        let mut contents = String::new();
        game_file.read_to_string(&mut contents).expect("Cannot read from file");
        let lines: Vec<&str> = contents.split('\n').collect();

        //get the players from the file
        let players: usize = lines[0].parse().unwrap();
        player_num_mut.player_count = players;
        //spawn the players
        for i in 0..players {
            let player: Player = from_str(lines[i + 1]).unwrap();
            if player.player_id == 0 {
                player_money = player.cash;
                player_bet = player.current_bet;
            }
            commands.spawn(player);
        }
        
    }
    else {
        poker_turn.small_blind_val = options_result.small_blind_amount;
        poker_turn.big_blind_val = options_result.big_blind_amount;
        player_num_mut.player_count = options_result.num_players;
    }

    

    commands.spawn(SpriteBundle {
        texture: asset_server.load("game_screen.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).insert(Background);

    commands.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("Cash: ${}\n", player_money),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                TextSection {
                    value: format!("Your Current Bet: ${}\n", player_bet),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                TextSection {
                    value: format!("Current Pot: ${}\n", pot),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                TextSection {
                    value: format!("Current Top Bet: ${}", top_bet),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
            ],
            alignment: TextAlignment::Center,
            linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
        },
        ..Default::default()
    }).insert(VisText);

    commands.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            //align_items: AlignItems::Center,
            //justify_content: JustifyContent::Center,
            left: Val::Px(540.),
            width: Val::Px(400.),
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "It is AI 1's Turn!\n".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
            ],
            alignment: TextAlignment::Center,
            linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
        },
        ..Default::default()
    }).insert(VisText);
    
    spawn_option_buttons(&mut commands, &asset_server);
    spawn_players(&mut commands, &asset_server, &player_num_mut);
}

fn spawn_players(commands: &mut Commands, asset_server: &Res<AssetServer>, player_num: &ResMut<NumPlayers>) {
    let ai_pos: Vec<(f32, f32, f32)> = vec![(225., 170., 2.), (435., 10., 2.), (-140., -175., 2.), (-435., 10., 2.), (-225., 170., 2.)];

    //spawn player in the same spot every game
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(PLAYER_POS.0, PLAYER_POS.1, PLAYER_POS.2),
        ..default()
    }).insert(VisPlayers)
    .with_children(|parent| {
        parent.spawn(Text2dBundle {
                text: Text::from_section("You",
                TextStyle {
                    font: asset_server.load("fonts/Lato-Black.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                }),
                transform: Transform::from_xyz(0., 0., 3.),
                ..default()
        });
    });

    //spawn the AI players
    for i in 0..player_num.player_count - 1 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(ai_pos[i].0, ai_pos[i].1, ai_pos[i].2),
            ..default()
        }).insert(VisPlayers)
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                    text: Text::from_section(String::from("AI ") + &(i + 1).to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }),
                    transform: Transform::from_xyz(0., 0., 3.),
                    ..default()
            });
        });
    }
}

pub fn tick_ai_timer(
    mut timer_query: Query<&mut AITimer>,
    time: Res<Time>,
    state: ResMut<PokerTurn>
) {
    if state.current_player != 0 && timer_query.iter().count() > 0{
        let mut timer = timer_query.single_mut();
        timer.timer.tick(time.delta());
    }
}
pub fn tear_down_game_screen(
    mut commands: Commands, 
    mut background_query: Query<Entity, With<Background>>, 
    mut node_query: Query<Entity, With<NBundle>>,
    player_entity_query: Query<Entity,  With<Player>>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    com_entity_query: Query<Entity, With<CommunityCards>>,
    vis_player_query: Query<Entity, With<VisPlayers>>,
    mut blinds_query: Query<Entity, With<Blind>>,
    vis_text_query: Query<Entity, With<VisText>>,
    mut state: ResMut<PokerTurn>,
) {

    //let node = node_query.single_mut();
    for node in node_query.iter_mut() {
        commands.entity(node).despawn_recursive();
    }

    //commands.entity(node).despawn_recursive();

    let background = background_query.single_mut();
    
    commands.entity(background).despawn_recursive();


    for entity in blinds_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    if player_entity_query.iter().next().is_some() {
        for entity in player_entity_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

    }

    if vis_text_query.iter().next().is_some() {
        for entity in vis_text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }


    if !player_card_query.is_empty() {
        //let player_card = player_card_query.single_mut(); 
        for player_card in player_card_query.iter_mut() {
            commands.entity(player_card).despawn_recursive();
        }
    }

    if vis_player_query.iter().next().is_some() {
        for entity in vis_player_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
   
    if com_entity_query.iter().next().is_some() {
        for entity in com_entity_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    state.current_player = 1;
    state.phase = PokerPhase::PreFlop;
    state.round_started = false;
    state.pot = 0;
    state.current_top_bet = 0;
    state.bet_made = false;
    state.pot_raised = false;
    state.small_blind = 1;
    state.big_blind = 0;
    state.small_blind_val = 25;
    state.big_blind_val = 50;
}

fn process_player_turn(
    commands: &mut Commands,
    current_player: usize,
    state: &mut ResMut<PokerTurn>,
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    player_count: &ResMut<NumPlayers>,
    mut last_action: ResMut<LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
    community_query: &mut Query<&CommunityCards>,
    timer_query: &mut Query<(Entity, &mut AITimer)>,
) {
    let mut player_raised = false;
    for (_entity, mut player) in player_entity_query.iter_mut() {
        if player.player_id == current_player {
            let mut text_iter = text_query.iter_mut();
            let _money_text = text_iter.next();
            let mut turn_text = text_iter.next().unwrap();

            if player.player_id != 0 {
                if !player.has_folded && !player.is_all_in {
                    turn_text.sections[0].value = format!("It is AI {}'s turn!\n", player.player_id);
                    if timer_query.iter().count() == 0 {
                        commands.spawn(AITimer{timer: Timer::from_seconds(5.0, TimerMode::Once)});
                    }
                    else {
                        let (timer_entity, timer) = timer_query.single_mut();
                        if timer.timer.just_finished() {
                            // let player_move: String = generate_move(&mut player, &state, community_query);
                            let mut hand_category: usize = 0;

                            if state.phase == PokerPhase::PreFlop {
                                hand_category = 0;
                            } else {
                                hand_category = (generate_post_flop_hand_strength(&mut player.cards, community_query)) as usize;
                            }

                            let player_move: String = select_action_for_hand(&mut player, hand_category);
                            if player_move.eq("Raise") {
                                state.current_top_bet += 50;
                                println!("Current top bet is now: ${}", state.current_top_bet);
                                player_raised = raise_action(state, player, player_count, &mut last_action, text_query);
                            }
                            else if player_move.eq("Call") {
                                call_action(state, player, player_count, &mut last_action, text_query);
                            }
                            else if player_move.eq("Fold") {
                                fold_action(state, player, player_count, &mut last_action, text_query);
                            }
                            else {
                                check_action(state, player, player_count, &mut last_action, text_query);
                            }
                            commands.entity(timer_entity).despawn_recursive();
                        }
                    }
                } else {
                    state.current_player = (current_player + 1) % player_count.player_count;
                    player.has_moved = true;
                }
            } else if !player.has_folded && !player.is_all_in {
                turn_text.sections[0].value = "It is your turn!\n".to_string();
                if let Some(PlayerAction::Check) = last_action.action {
                    check_action(state, player, player_count, &mut last_action, text_query);
                    break;
                } else if let Some(PlayerAction::Raise) = last_action.action {
                    player_raised = raise_action(state, player, player_count, &mut last_action, text_query);
                    break;
                } else if let Some(PlayerAction::Fold) = last_action.action {
                    fold_action(state, player, player_count, &mut last_action, text_query);
                    break;
                } else if let Some(PlayerAction::Call) = last_action.action {
                    call_action(state, player, player_count, &mut last_action, text_query);
                    break;
                }
            } else {
                state.current_player = (state.current_player + 1) % player_count.player_count;
                player.has_moved = true;
            }
        }
    }
    if player_raised {
        for (_entity, mut player) in player_entity_query.iter_mut() {
            if player.player_id != current_player {
                player.has_moved = false;
            } else {
                player.has_raised = false;
            }
        }
    }
}

pub fn check_action (
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let _money_text = text_iter.next();
    let mut turn_text = text_iter.next().unwrap();

    if state.current_top_bet > player.current_bet {
        if player.player_id == 0 {
            turn_text.sections[1].value = "You cannot check".to_string();
        }
        else {
            turn_text.sections[1].value = format!("AI {} cannot check", player.player_id);
        }
        println!("Cannot check since top_bet ({}) is > your current bet ({})!", state.current_top_bet, player.current_bet);
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
    } else {
        if player.player_id == 0 {
            turn_text.sections[1].value = "You have checked".to_string();
        }
        else {
            turn_text.sections[1].value = format!("AI {} has checked", player.player_id);
        }
        println!("Player {} has checked!", player.player_id);
        player.has_moved = true;
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
    }
}

pub fn raise_action (
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) -> bool {
    let mut text_iter = text_query.iter_mut();
    let mut money_text = text_iter.next().unwrap();
    let mut turn_text = text_iter.next().unwrap();

    if player.cash >= state.current_top_bet - player.current_bet {
        state.pot += state.current_top_bet - player.current_bet;
        println!("Player {} has raised the bet to {}", player.player_id, state.current_top_bet);
        if player.player_id == 0 {
            turn_text.sections[1].value = format!("You raised the bet to {}", state.current_top_bet);
        }
        else {
            turn_text.sections[1].value = format!("AI {} raised the bet to {}", player.player_id, state.current_top_bet);
        }
        println!("Player {} has raised the bet to {}", player.player_id, state.current_top_bet);

        player.has_moved = true;
        player.has_raised = true;
        player.cash -= state.current_top_bet - player.current_bet;
        player.current_bet = state.current_top_bet;
        money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
        money_text.sections[3].value = format!("Current Top Bet: ${}\n", state.current_top_bet);

        if player.player_id == 0 {
            money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
            money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);
        }
        if player.cash == 0 {
            player.is_all_in = true;
            if player.player_id == 0 {
                turn_text.sections[1].value = "You have gone all in!".to_string();
            }
            else {
                turn_text.sections[1].value = format!("AI {} has gona all in!", player.player_id);
            }
            println!("Player {} has gone all in!", player.player_id);
        }
        
        last_action.action = Some(PlayerAction::None);
        state.current_player = (state.current_player + 1) % player_count.player_count;
        true
    } else {
        if player.player_id == 0 {
            turn_text.sections[1].value = "You cannot raise due to going negative".to_string();
        }
        println!("Player {} cannot raise due to going negative", player.player_id);
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        false
    }
}

pub fn fold_action(
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let _money_text = text_iter.next();
    let mut turn_text = text_iter.next().unwrap();

    if player.player_id == 0 {
        turn_text.sections[1].value = "You folded!".to_string();
    }
    else {
        turn_text.sections[1].value = format!("AI {} has folded!", player.player_id);
    }
    println!("Player {} has folded!", player.player_id);
    player.has_moved = true;
    player.has_folded = true;
    if player.player_id == 0 {
        last_action.action = Some(PlayerAction::None);
    }
    state.current_player = (state.current_player + 1) % player_count.player_count;
}

pub fn call_action(
    state: &mut ResMut<PokerTurn>,
    mut player: Mut<'_, Player>,
    player_count: &ResMut<NumPlayers>,
    last_action: &mut ResMut<'_, LastPlayerAction>,
    text_query: &mut Query<&mut Text, With<VisText>>,
) {
    let mut text_iter = text_query.iter_mut();
    let mut money_text = text_iter.next().unwrap();
    let mut turn_text = text_iter.next().unwrap();

    if player.cash >= state.current_top_bet - player.current_bet {
        if player.player_id == 0 {
            turn_text.sections[1].value = "You have called!".to_string();
        }
        else {
            turn_text.sections[1].value = format!("AI {} has called!", player.player_id);
        }
        println!("Player {} has called!", player.player_id);
        player.has_moved = true;
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        state.pot += state.current_top_bet - player.current_bet;
        player.cash -= state.current_top_bet - player.current_bet;
        if player.player_id == 0 {
            money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
            money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);
        }
        if player.cash == 0 {
            player.is_all_in = true;
            if player.player_id == 0 {
                turn_text.sections[1].value = "You have gone all in!".to_string();
            }
            else {
                turn_text.sections[1].value = format!("AI {} has gone all in!", player.player_id);
            }
            println!("Player {} has gone all in!", player.player_id);
        }
        player.current_bet = state.current_top_bet;
        state.current_player = (state.current_player + 1) % player_count.player_count;
    } else {
        if player.player_id == 0 {
            turn_text.sections[1].value = "You have gone all in!".to_string();
        }
        else {
            turn_text.sections[1].value = format!("AI {} has gone all in!", player.player_id);
        }
        println!("Player {} has gone all in!", player.player_id);
        player.has_moved = true;
        player.is_all_in = true;
        if player.player_id == 0 {
            last_action.action = Some(PlayerAction::None);
        }
        state.pot += player.cash;
        player.current_bet += player.cash;
        player.cash = 0;
        state.current_player = (state.current_player + 1) % player_count.player_count;
        if player.player_id == 0 {
            money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
            money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);
        }
    }
    money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
    money_text.sections[3].value = format!("Current Top Bet: ${}\n", state.current_top_bet);
}

pub fn turn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PokerTurn>,
    mut player_entity_query: Query<(Entity, &mut Player)>,
    mut player_card_query: Query<Entity, With<VisPlayerCards>>,
    mut community_query: Query<&CommunityCards>,
    com_entity_query: Query<Entity, With<CommunityCards>>,
    mut deck: ResMut<Deck>,
    player_count: ResMut<NumPlayers>,
    last_action: ResMut<LastPlayerAction>,
    mut blind_text_query: Query<Entity, With<Blind>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    sprite_data: Res<SpriteData>,
    mut options_result: ResMut<OptionsResult>,
    mut text_query: Query<&mut Text, With<VisText>>,
    mut timer_query: Query<(Entity, &mut AITimer)>,
) {
    let mut text_iter = text_query.iter_mut();
    let mut money_text = text_iter.next().unwrap();
    let mut turn_text = text_iter.next().unwrap();

    let ai_blind_pos: Vec<(f32, f32, f32)> = vec![(225., 215., 2.), (435., 55., 2.), (-140., -220., 2.), (-435., 55., 2.), (-225., 215., 2.)];

    if options_result.is_loaded_game {
        options_result.is_loaded_game = false;

        //spawn player cards
        let mut players_hands: Vec<Player> = Vec::new();
        for (_, player) in player_entity_query.iter_mut() {
            if player.player_id == 0 {
                if player.big_blind {
                    commands.spawn(Text2dBundle {
                        text: Text::from_section("BB", TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 25.,
                            color: Color::WHITE,
                        }),
                        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                        ..default()
                    })
                    .insert(Blind);
                }
                else if player.small_blind {
                    commands.spawn(Text2dBundle {
                        text: Text::from_section("SB", TextStyle {
                            font: asset_server.load("fonts/Lato-Black.ttf"),
                            font_size: 25.,
                            color: Color::WHITE,
                        }),
                        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                        ..default()
                    })
                    .insert(Blind);
                }
            }
            else if player.big_blind {
                commands.spawn(Text2dBundle {
                    text: Text::from_section("BB", TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 25.,
                        color: Color::WHITE,
                    }),
                    transform: Transform::from_xyz(
                        ai_blind_pos[player.player_id - 1].0,
                        ai_blind_pos[player.player_id - 1].1,
                        ai_blind_pos[player.player_id - 1].2),
                    ..default()
                })
                .insert(Blind);
            }
            else if player.small_blind {
                commands.spawn(Text2dBundle {
                    text: Text::from_section("SB", TextStyle {
                        font: asset_server.load("fonts/Lato-Black.ttf"),
                        font_size: 25.,
                        color: Color::WHITE,
                    }),
                    transform: Transform::from_xyz(
                        ai_blind_pos[player.player_id - 1].0,
                        ai_blind_pos[player.player_id - 1].1,
                        ai_blind_pos[player.player_id - 1].2),
                    ..default()
                })
                .insert(Blind);
            }
            players_hands.push(Player {
                player_id: player.player_id,
                cards: player.cards.clone(),
                cash: player.cash,
                current_bet: player.current_bet,
                has_folded: player.has_folded,
                has_moved: player.has_moved,
                is_all_in: player.is_all_in,
                has_raised: player.has_raised,
                hand_strength: player.hand_strength,
                move_dist: player.move_dist.clone(),
                big_blind: false,
                small_blind: false,
                cfr_data: player.cfr_data.clone(),
            });
        } 
        spawn_player_cards(&mut commands, &players_hands, &mut player_entity_query, &sprite_data);
        
        //get file contents
        let mut game_file = File::open("saved_game.txt").expect("Can't open file");
        let mut contents = String::new();
        game_file.read_to_string(&mut contents).expect("Cannot read from file");
        let lines: Vec<&str> = contents.split('\n').collect();

        //spawn community cards
        let mut com_cards: Vec<Vec<Card>> = Vec::new();
        let com_card_count: usize = lines[player_count.player_count + 1].parse().unwrap();
        for i in 0..com_card_count {
            let com_card: CommunityCards = from_str(lines[player_count.player_count + 2 + i]).unwrap();
            com_cards.push(com_card.cards);
        }
        println!("{}", to_string(&com_cards).unwrap());
        spawn_community_cards(&mut commands, com_cards, &community_query, &sprite_data);

        //update state resource
        let new_state: PokerTurn = from_str(lines[player_count.player_count + com_card_count + 2]).unwrap();
        state.current_player = new_state.current_player;
        state.phase = new_state.phase;
        state.round_started = new_state.round_started;
        state.pot = new_state.pot;
        state.current_top_bet = new_state.current_top_bet;
        state.pot_raised = new_state.pot_raised;
        state.bet_made = new_state.bet_made;
        state.small_blind = new_state.small_blind;
        state.big_blind = new_state.big_blind;
        state.small_blind_val = new_state.small_blind_val;
        state.big_blind_val = new_state.big_blind_val;

        //update text
        money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
        money_text.sections[3].value = format!("Current Top Bet: ${}", state.current_top_bet);

        //update deck
        let new_deck: Deck = from_str(lines[player_count.player_count + com_card_count + 3]).unwrap();
        deck.cards = new_deck.cards;
    }

    let current_player_moved = player_entity_query.iter()
        .find_map(|(_entity, player)| {
            if player.player_id == state.current_player {
                Some(player.has_moved)
            } else {
                None
            }
        }).unwrap_or(false);

    
    let players_no_cash = player_entity_query.iter().filter(|(_entity, player)| player.cash == 0).count();
   
    
    // If only one player left go straight to showdown phase
    let active_players_count = player_entity_query.iter().filter(|(_entity, player)| !player.has_folded).count();
    if active_players_count == 1 {
        state.phase = PokerPhase::Showdown;
    }

    
    

    match state.phase {
        PokerPhase::PreFlop => {
                if !state.round_started {
                    if !state.is_first_round {
                        thread::sleep(time::Duration::from_secs(5));
                    }
                    let mut game_over: bool = false;
                    if players_no_cash ==  player_count.player_count -1 {
                        println!("Only one player with money left game over");
                        let mut game_result = GameResult {
                            id: 0,
                        };

                        for (_, player) in player_entity_query.iter_mut() {
                            if player.cash != 0 {
                                game_result.id = player.player_id;
                            }
                        }
                        commands.insert_resource(game_result);
                        app_state_next_state.set(AppState::GameOver);
                        game_over = true;
                    }

                    println!("Phase is now in PreFlop!");
                    let cards = &mut deck.cards;
                    shuffle_cards(cards);
                    let players_hands = deal_hands(player_count.player_count, cards, options_result.money_per_player);
                    spawn_player_cards(&mut commands, &players_hands, &mut player_entity_query, &sprite_data);

                    //loops through the players to find the big and small blinds
                    if player_entity_query.iter().count() > 0 && !game_over {
                        for (_, mut player) in player_entity_query.iter_mut() {
                            if player.player_id == state.small_blind {
                                player.small_blind = true;
                                if player.cash <= state.small_blind_val {
                                    state.pot += player.cash;
                                    player.current_bet = player.cash;
                                    player.cash = 0;
                                    player.is_all_in = true;
                                } else {
                                    player.cash -= state.small_blind_val;
                                    player.current_bet = state.small_blind_val;
                                    state.pot += state.small_blind_val;
                                }
                                money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
                                
                                
                                //spawn the blind text
                                if player.player_id == 0 {
                                    //update player's visible cash amount
                                    money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
                                    money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);

                                    commands.spawn(Text2dBundle {
                                        text: Text::from_section("SB", TextStyle {
                                            font: asset_server.load("fonts/Lato-Black.ttf"),
                                            font_size: 25.,
                                            color: Color::WHITE,
                                        }),
                                        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                                        ..default()
                                    })
                                    .insert(Blind);
                                }
                                else {
                                    commands.spawn(Text2dBundle {
                                        text: Text::from_section("SB", TextStyle {
                                            font: asset_server.load("fonts/Lato-Black.ttf"),
                                            font_size: 25.,
                                            color: Color::WHITE,
                                        }),
                                        transform: Transform::from_xyz(
                                            ai_blind_pos[player.player_id - 1].0,
                                            ai_blind_pos[player.player_id - 1].1,
                                            ai_blind_pos[player.player_id - 1].2),
                                        ..default()
                                    })
                                    .insert(Blind);
                                }
                            }
                            else if player.player_id == state.big_blind {
                                player.big_blind = true;
                                if player.cash <= state.big_blind_val {
                                    state.pot += player.cash;
                                    player.current_bet = player.cash;
                                    player.cash = 0;
                                    player.is_all_in = true;
                                    state.current_top_bet = player.current_bet;
                                } else {
                                    player.cash -= state.big_blind_val;
                                    player.current_bet = state.big_blind_val;
                                    state.pot += state.big_blind_val;
                                    state.current_top_bet = state.big_blind_val;
                                }
                                money_text.sections[2].value = format!("Current Pot: ${}\n", state.pot);
                                money_text.sections[3].value = format!("Current Top Bet: ${}", state.current_top_bet);

                                //spawn blind text
                                if player.player_id == 0 {
                                    //update player's visible cash amount
                                    money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
                                    money_text.sections[1].value = format!("Your Current Bet: ${}\n", player.current_bet);

                                    commands.spawn(Text2dBundle {
                                        text: Text::from_section("BB", TextStyle {
                                            font: asset_server.load("fonts/Lato-Black.ttf"),
                                            font_size: 25.,
                                            color: Color::WHITE,
                                        }),
                                        transform: Transform::from_xyz(PLAYER_BLIND_POS.0, PLAYER_BLIND_POS.1, PLAYER_BLIND_POS.2),
                                        ..default()
                                    })
                                    .insert(Blind);
                                }
                                else {
                                    commands.spawn(Text2dBundle {
                                        text: Text::from_section("BB", TextStyle {
                                            font: asset_server.load("fonts/Lato-Black.ttf"),
                                            font_size: 25.,
                                            color: Color::WHITE,
                                        }),
                                        transform: Transform::from_xyz(
                                            ai_blind_pos[player.player_id - 1].0,
                                            ai_blind_pos[player.player_id - 1].1,
                                            ai_blind_pos[player.player_id - 1].2),
                                        ..default()
                                    })
                                    .insert(Blind);
                                }
                            }
                        }
                        state.round_started = true;
                        println!("Pot is: {}", state.pot);
                    }
                    state.current_player = (state.big_blind + 1) % player_count.player_count;
                }

            if !current_player_moved {
                process_player_turn(&mut commands, state.current_player, &mut state, &mut player_entity_query, &player_count, last_action, &mut text_query, &mut community_query, &mut timer_query);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count, &mut text_query);
        }
        PokerPhase::Flop => {
            if community_query.iter().count() < 3 {
                println!("Phase is now in flop!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, flop, &community_query, &sprite_data);
            }
            if !current_player_moved {
                process_player_turn(&mut commands, state.current_player, &mut state, &mut player_entity_query, &player_count, last_action, &mut text_query, &mut community_query, &mut timer_query);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count, &mut text_query);
        }
        PokerPhase::Turn => {
            if community_query.iter().count() < 4 {
                println!("Phase is now in Turn!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, flop, &community_query, &sprite_data);
            }
            if !current_player_moved {
                process_player_turn(&mut commands, state.current_player, &mut state, &mut player_entity_query, &player_count, last_action, &mut text_query, &mut community_query, &mut timer_query);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count, &mut text_query); 
        }
        PokerPhase::River => {
            if community_query.iter().count() < 5 {
                println!("Phase is now in River!");
                let cards = &mut deck.cards;
                let flop = deal_com_function(cards, &community_query);
                spawn_community_cards(&mut commands, flop, &community_query, &sprite_data);
            }
            if !current_player_moved {
                process_player_turn(&mut commands, state.current_player, &mut state, &mut player_entity_query, &player_count, last_action, &mut text_query, &mut community_query, &mut timer_query);
            }
            next_player_turn(&mut state, &mut player_entity_query, player_count.player_count, &mut text_query);
        }
        PokerPhase::Showdown => {
            // Check the winners using poorly named card_function, the players is derived from the Entity Player query and iterated over to just return the players
            // and remove the entities so that player_entity_query can be used in this instance
            let mut winner = card_function(&community_query, &player_entity_query.iter().map(|(_, player)| player).collect::<Vec<&Player>>());

            if active_players_count == 1 {
                for (_, player) in player_entity_query.iter_mut() {
                    if !player.has_folded {
                        winner = player.player_id;
                    }
                }
            }

            // This is all to reinitialize the cards so another round may begin
            deck.cards = init_cards();
            //let player_card_bundle = player_card_query.single_mut();
            //commands.entity(player_card_bundle).despawn_recursive();

            for player_card_bundle in player_card_query.iter_mut() {
                commands.entity(player_card_bundle).despawn_recursive();
            }

            for entity in com_entity_query.iter() {
                commands.entity(entity).despawn();
            }

            for (_, mut player) in player_entity_query.iter_mut() {
                if winner == 0 {
                    if player.player_id == 0 {
                        turn_text.sections[0].value = "You won!".to_string();
                        turn_text.sections[1].value = String::new();
                        println!("Player 0 wins and gains a pot of {}\n", state.pot);
                        player.cash += state.pot;
                        money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
                        money_text.sections[1].value = format!("Your Current Bet: ${}\n", 0);
                    }
                } else if winner == 1 {
                    if player.player_id == 1 {
                        turn_text.sections[0].value = format!("AI {} won!", player.player_id);
                        turn_text.sections[1].value = String::new();
                        println!("Player 1 wins and gains a pot of {}\n", state.pot);
                        player.cash += state.pot;
                    }
                } else {
                    println!("Player {} ties and gains a pot of {}\n", player.player_id, state.pot/player_count.player_count);
                    player.cash += state.pot/player_count.player_count;
                    money_text.sections[0].value = format!("Your Cash: ${}\n", player.cash);
                    money_text.sections[1].value = format!("Your Current Bet: ${}\n", 0);
                }
           }
           

            state.pot = 0;
            state.current_top_bet = 0;
            state.small_blind = (state.small_blind + 1) % player_count.player_count;
            state.big_blind = (state.big_blind + 1) % player_count.player_count;
            state.current_player = state.big_blind + 1 % player_count.player_count;

            money_text.sections[2].value = format!("Current Pot: ${}\n", 0);
            money_text.sections[3].value = format!("Current Top Bet: ${}\n", 0);
            //turn_text.sections[0].value = format!("");
            //turn_text.sections[1].value = format!("");

            for blind in blind_text_query.iter_mut() {
                commands.entity(blind).despawn_recursive();
            }


            for (_, mut player) in player_entity_query.iter_mut() {
                player.has_folded = false;
                player.current_bet = 0;
                player.has_moved = false;
                player.is_all_in = false;
                player.has_raised = false;
                player.small_blind = false;
            }
            state.round_started = false;
            state.phase = PokerPhase::PreFlop;
            state.is_first_round = false;
        }
    }
}

fn next_player_turn(
    state: &mut ResMut<PokerTurn>,
    player_entity_query: &mut Query<(Entity, &mut Player)>,
    _total_players: usize,
    text_query: &mut Query<&mut Text, With<VisText>>,
    
) {
    let mut text = text_query.iter_mut().next().unwrap();
    let players_moved_count = player_entity_query.iter().filter(|(_entity, player)| player.has_moved && !player.has_folded).count();

    let active_players_count = player_entity_query.iter().filter(|(_entity, player)| !player.has_folded).count();

    if players_moved_count == active_players_count && player_entity_query.iter().count() > 0{
        match state.phase {
            PokerPhase::PreFlop => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::Flop;
                state.current_top_bet = 0;
            }
            PokerPhase::Flop => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::Turn;
                state.current_top_bet = 0;
            }
            PokerPhase::Turn => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::River;
                state.current_top_bet = 0;
            }
            PokerPhase::River => {
                for (_entity, mut player) in player_entity_query.iter_mut() {
                    player.has_moved = false;
                    player.current_bet = 0;
                    player.has_raised = false;
                }
                state.phase = PokerPhase::Showdown;
                state.current_top_bet = 0;
            }
            PokerPhase::Showdown => {}
        }
        text.sections[1].value = format!("Your Current Bet: ${}\n", 0);
        text.sections[3].value = format!("Current Top Bet: ${}\n", 0);

        state.current_player = (state.big_blind + 1) % _total_players;
    }
}

pub fn handle_keyboard(
    mut events: EventReader<KeyboardInput>,
    mut text_query: Query<&mut Text, With<TextBoxTag>>,
    mut char_events: EventReader<ReceivedCharacter>,
    text_input_query: Query<(Entity, &TextBox)>,
    children_query: Query<&Children>,
) {

    for (input_entity, textbox) in &text_input_query {
        if !textbox.active {
            continue;
        }

        for descendant in children_query.iter_descendants(input_entity) {
            if let Ok(mut text) = text_query.get_mut(descendant) {
                for event in char_events.iter() {
                    if !(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&event.char)) {
                        continue;
                    }

                    text.sections[0].value.push(event.char);
                }

                for event in events.iter() {
                    match event.key_code {
                        Some(KeyCode::Return) => {
                            if event.state.is_pressed() {
                                return;
                            }; // repeats for some reason without this
                            debug!("result = {}", text.sections[0].value);
                        }
                        Some(KeyCode::Back) => {
                            text.sections[0].value.pop();
                        }
                        _ => {} // produces a compile error without this
                    }
                }
            }
        }
    }
}

pub fn activate(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_query: Query<(Entity, &mut TextBox, &mut BackgroundColor)>,
) {
    // if a thingy is clicked, set it to active and make all the other ones inactive
    // idk if we have a color scheme or something so it's just gonna be kinda greyed out if inactive
    for (target_entity, interaction) in &interaction_query {
        debug!("{:?} ----- {:?}", target_entity, interaction);
        match *interaction {
            Interaction::Pressed => {
                for (entity, mut text_box, mut color) in &mut text_query {
                    if target_entity == entity {
                        // if this one was clicked, set it active and highlight it
                        *color = Color::WHITE.into();
                        text_box.active = true;
                    } else {
                        // darken and deactivate all the other ones
                        *color = Color::rgb(0.7, 0.7, 0.7).into();
                        text_box.active = false;
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn make_scrolly(
    mut commands: Commands,
    query: Query<(Entity, &TextBox), Added<TextBox>>,
) {
    /*
    aight so basically this pretty much only runs once
    it gets called every loop because its tied to the update event in mod.rs but Added<TextBox>
    is only nonempty once (at the beginning, after the text boxes are spawned)
    this is the easiest way i could think of to be able to run this query in order to loop over all
    the text boxes
    my b if this makes absolutely no sense and theres an easier way to do it
    */

    // why is box a reserved keyword
    for (entity, textbox) in &query {

        commands.entity(entity).insert(Interaction::None); // make it responsive to click interactions

        // make the area for the text to be in and identify it with the TextBoxTag component
        let text_area = commands
            .spawn((
                TextBundle {
                    text: Text {
                        linebreak_behavior: BreakLineOn::NoWrap,
                        sections: vec![
                            TextSection {
                                value: "".to_string(),
                                style: textbox.text_style.clone(),
                            },
                        ],
                        ..default()
                    },
                    ..default()
                },
                TextBoxTag {
                    id: textbox.id,
                },
            ))
            .id();
        
        // define overflow behavior
        let overflow_fixer = commands
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexEnd, // shove it all to the left
                    max_width: Val::Percent(100.), // make it go all the way to the end
                    overflow: Overflow::clip(), // cut it off so it ain't visible
                    ..default()
                },
                ..default()
            })
            .id();

        // add the s c r o l l e r to the textbox
        commands.entity(overflow_fixer).add_child(text_area);
        commands.entity(entity).add_child(overflow_fixer);
    }
}