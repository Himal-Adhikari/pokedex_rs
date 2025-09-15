use iced::{
    Element, Task,
    widget::{Column, button, text_input},
};
use sqlx::{Pool, Sqlite};

use crate::pokemon::{Pokemons, get_pokemons};

#[derive(Clone, Debug)]
pub enum Message {
    NameChanged(String),
    PokemonsFound(Option<Pokemons>),
    PokemonSelected(usize),
}

#[derive(Clone, Debug)]
pub enum AppState {
    Initial,
    SinglePokemon,
}

#[derive(Debug)]
pub struct State {
    name: String,
    pokemons: Pokemons,
    pool: Pool<Sqlite>,
    app_state: AppState,
}

impl State {
    pub fn with_pool(pool: Pool<Sqlite>) -> (Self, Task<Message>) {
        (
            State {
                name: Default::default(),
                pokemons: Default::default(),
                app_state: AppState::Initial,
                pool,
            },
            Task::none(),
        )
    }

    pub fn search_pokemons(&self) -> Task<Message> {
        Task::perform(
            get_pokemons(self.name.clone(), self.pool.clone()),
            Message::PokemonsFound,
        )
    }
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::NameChanged(name) => {
            state.app_state = AppState::Initial;
            state.name = name;
            state.search_pokemons()
        }
        Message::PokemonsFound(pokemons) => match pokemons {
            Some(pkm) => {
                state.pokemons = pkm;
                Task::none()
            }
            None => {
                state.pokemons = Pokemons::default();
                Task::none()
            }
        },
        Message::PokemonSelected(_index) => {
            state.app_state = AppState::SinglePokemon;
            Task::none()
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mut to_return = Column::new();
    to_return =
        to_return.push(text_input("Type pokemon name", &state.name).on_input(Message::NameChanged));
    for (index, pokemon) in state.pokemons.pokemons.iter().enumerate() {
        to_return =
            to_return.push(button(pokemon.name.as_str()).on_press(Message::PokemonSelected(index)));
    }
    to_return.into()
}
