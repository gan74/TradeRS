#![allow(dead_code)]
#![allow(unused_imports)] 

use iced::executor;
use iced::widget::{column, row, text, button, horizontal_space, container, scrollable};
use iced::{Application, Command, Element, Settings, Theme, Length};


use std::fs::File;
use std::sync::{Arc, Mutex};

extern crate core;

use core::utils::*;
use core::api::*;
use core::agent::*;
use core::ship::*;
use core::waypoint::*;
use core::request_manager::*;

pub fn main() -> iced::Result {
    TraderApp::run(Settings::default())
}



struct TraderApp {
    api: Arc<Api>,
    agent: Option<Agent>,
    ships: Option<Vec<Ship>>,

    errors: Vec<ApiError>,
}

impl TraderApp {
    pub fn from_token(token: String) -> Self {
        Self::from_api(api_from_token(token))
    }

    pub fn from_stored_token() -> Self {
        let token = read_text_file("./token.txt").expect("Could not find token file");
        Self::from_token(token)
    }

    fn from_api(api: Api) -> Self {
        TraderApp {
            api: Arc::new(api),
            agent: None,
            ships: None,
            errors: Vec::new(),
        }
    }

    fn process_error<T>(&mut self, result: Result<T, ApiError>) -> Option<T> {
        match result {
            Ok(obj) => Some(obj),
            Err(error) => {
                self.errors.push(error);
                None
            },
        }
    }
}



#[derive(Debug, Clone)]
enum Message {
    RefreshAll,
    AgentReceived(Result<Agent, ApiError>),
    ShipsReceived(Result<Vec<Ship>, ApiError>),
}

impl Application for TraderApp {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (TraderApp::from_stored_token(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Trade-RS")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RefreshAll => {
                return Command::batch([
                    Command::perform(get_agent(self.api.clone()), Message::AgentReceived),
                    Command::perform(get_ships(self.api.clone()), Message::ShipsReceived),
                ]);
            }

            Message::AgentReceived(result) => self.agent = self.process_error(result),
            Message::ShipsReceived(result) => self.ships = self.process_error(result),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let agent = if let Some(agent) = &self.agent {
            text(format!("{} credits", agent.credits()))
        } else {
            text("? credits")
        };

        let ships = column(self.ships.as_ref().unwrap_or(&Vec::new()).iter().map(|ship| {
            row![
                text(ship.name()), horizontal_space(Length::Fill), 
                text(format!("{:?}", ship.status)), horizontal_space(Length::Fill),
                text(format!("fuel {}/{}", ship.fuel, ship.max_fuel)).width(150), horizontal_space(Length::Fill),
                text(format!("cargo {}/{}", ship.inventory.used, ship.inventory.capacity)).width(150),
            ].spacing(20).into()
        }).collect());

        column![
            row![
                agent, horizontal_space(Length::Fill), button("Refresh All").on_press(Message::RefreshAll),
            ],
            scrollable(ships),
        ].padding(20).into()
    }
}




async fn get_agent(api: Arc<Api>) -> Result<Agent, ApiError> {
    api.agent()
}

async fn get_ships(api: Arc<Api>) -> Result<Vec<Ship>, ApiError> {
    api.ships()
}






fn create_request_manager() -> Arc<Mutex<RequestManager>> {
    let file = File::create("api.log").expect("Unable to create log file");
    Arc::new(Mutex::new(RequestManager::new(file)))
}

fn api_from_token(token: String) -> Api {
    Api::new(token, create_request_manager())
}

