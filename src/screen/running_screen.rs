

use crate::{options::components::OptionsResult, game::components::Player};

use super::{AppState, Message, User, UserInfo, Users, UiInfo, UiInfoString};
use bevy::prelude::*;
use bevy_egui::{egui::RichText, *};


pub(super) fn running_screen_update(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<AppState>>,
    users: Res<Users>,
    players: &mut Query<(Entity, &mut Player)>,
    ui_info:ResMut<UiInfo>,
    option:Res<OptionsResult>,
    
) {
    egui::TopBottomPanel::top("hall").show(contexts.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("Game Running").size(30.0).strong())
        });
    });
    let Users { ref users } = users.as_ref();
    let users = users.blocking_read();
    let users_list = users
        .iter()
        .map(|User { ip, name, .. }| {
            let (ip, name) = (ip.to_string(), name.to_string());
            UserInfo { ip, name }
        })
        .collect::<Vec<UserInfo>>();

    egui::SidePanel::left("left").show(contexts.ctx_mut(), |ui| {
        if ui.button(RichText::new("Over").size(16.0)).clicked() {

            // game over
            users.iter().for_each(|m|{m.send_message.blocking_send(Message::Over);});
            state.set(AppState::GameEnd);
        }
    });
    egui::TopBottomPanel::bottom("info").show(contexts.ctx_mut(),|ui|{
        match &ui_info.info{
            UiInfoString::Info(i)=>{
                ui.label(RichText::new(i).color(egui::Color32::GREEN));
            }
            UiInfoString::Warn(w)=>{
                ui.label(RichText::new(w).color(egui::Color32::YELLOW));
            }
            UiInfoString::Error(e)=>{
                ui.label(RichText::new(e).color(egui::Color32::RED));
            }
            UiInfoString::None=>{

            }
        }
    });


        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("User")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for (index,UserInfo { ip, name }) in users_list.iter().enumerate() {
                            ui.label(RichText::new(name).size(24.0))
                                .on_hover_text(format!("target:{ip:?}"));
    
                            // stop
                            // ui.label(RichText::new(format!("Money:{}",player)).size(16.0).weak());
                            ui.end_row();
                        }
                    });
            })
        });

}