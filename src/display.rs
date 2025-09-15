use iced::{
    Alignment, Element,
    Length::Fill,
    Renderer, Task, Theme,
    widget::{Column, Row, Rule, Themer, button, row, scrollable, text, text_input},
};
use sqlx::{Pool, Sqlite};

use crate::pokemon::{Ability, Pokemons, get_pokemons};

#[derive(Clone, Debug)]
pub enum Message {
    NameChanged(String),
    PokemonsFound(Option<Pokemons>),
    PokemonSelected(usize),
}

#[derive(Clone, Debug)]
pub enum AppState {
    Initial,
    SinglePokemon(usize),
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
            if !state.name.is_empty() {
                return state.search_pokemons();
            }
            state.pokemons = Default::default();
        }
        Message::PokemonsFound(pokemons) => match pokemons {
            Some(pkm) => {
                state.pokemons = pkm;
            }
            None => {
                state.pokemons = Pokemons::default();
            }
        },
        Message::PokemonSelected(index) => {
            state.app_state = AppState::SinglePokemon(index);
        }
    }
    return Task::none();
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mut to_return = Column::new();

    to_return =
        to_return.push(text_input("Type pokemon name", &state.name).on_input(Message::NameChanged));
    // to_return = to_return.align_x(Alignment::Center);
    match state.app_state {
        AppState::Initial => {
            for (index, pokemon) in state.pokemons.pokemons.iter().enumerate() {
                let pokemon_name = text(pokemon.name.as_str())
                    .width(150)
                    .align_x(Alignment::Start);

                let abilities = compose_ability(&pokemon.abilities).width(260);

                let row = row![pokemon_name, abilities];
                to_return = to_return.push(button(row).on_press(Message::PokemonSelected(index)));
                to_return = to_return.push(Rule::horizontal(3));
            }
        }
        AppState::SinglePokemon(_idx) => {}
    }
    scrollable(to_return).height(Fill).into()
    // to_return.into()
}

pub fn compose_ability(abilitites: &Vec<Ability>) -> Row<'_, Message, Theme, Renderer> {
    let mut ability_row: Row<'_, Message, Theme, Renderer> = Row::new();
    if abilitites.len() <= 2 {
        for ability in abilitites.iter() {
            ability_row = ability_row.push(
                text(ability.name.clone().to_uppercase())
                    .width(130)
                    .size(12)
                    .align_x(Alignment::Center),
            );
            ability_row = ability_row.spacing(10);
        }
        return ability_row;
    }
    let mut ability_column = Column::new();
    for ability in abilitites.iter().take(2) {
        ability_column = ability_column.push(
            text(ability.name.clone().to_uppercase())
                .width(130)
                .size(12)
                .align_x(Alignment::Center),
        );
    }
    ability_row = ability_row.push(ability_column);
    let mut ability_column = Column::new();
    for ability in abilitites.iter().skip(2) {
        ability_column = ability_column.push(
            text(ability.name.clone().to_uppercase())
                .width(130)
                .size(12)
                .align_x(Alignment::Center),
        );
    }
    ability_row = ability_row.push(ability_column);
    return ability_row;
}
