use std::fmt::Debug;

use iced::{
    Alignment, Background, Color, Element,
    Length::Fill,
    Renderer, Task, Theme,
    widget::{Button, Column, Row, button, row, rule, scrollable, text, text_input},
};
use sqlx::{Pool, Sqlite};

use crate::pokemon::{Ability, Pokemons, Stats, get_pokemons};

const HEIGHT: u32 = 50;

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
    pub fn with_pool() -> (Self, Task<Message>) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt
            .block_on(sqlx::SqlitePool::connect("pokedex.sqlite"))
            .unwrap();

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

pub fn theme(_state: &State) -> Theme {
    // Theme::Nord
    Theme::Light
    // Theme::Dark
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mut to_return = Column::new();

    to_return =
        to_return.push(text_input("Type pokemon name", &state.name).on_input(Message::NameChanged));
    match state.app_state {
        AppState::Initial => {
            for (index, pokemon) in state.pokemons.pokemons.iter().enumerate() {
                let pokemon_name = text(pokemon.name.as_str())
                    .width(200)
                    .height(HEIGHT)
                    .align_x(Alignment::Start)
                    .align_y(Alignment::Center);

                let abilities = compose_ability(&pokemon.abilities).width(260);
                let stats = compose_stats(&pokemon.stats);

                let row = row![pokemon_name, abilities, stats];
                let button: Button<'_, Message> = button(row)
                    .style(|theme: &Theme, status| {
                        let palette = theme.palette();
                        match status {
                            button::Status::Hovered | button::Status::Pressed => {
                                button::Style::default().with_background(palette.danger.inverse())
                            }
                            _ => button::Style::default().with_background(palette.background),
                        }
                    })
                    .into();
                to_return = to_return.push(button.on_press(Message::PokemonSelected(index)));
            }
        }
        AppState::SinglePokemon(_idx) => {}
    }
    scrollable(to_return).height(Fill).into()
}

pub fn compose_ability(abilitites: &Vec<Ability>) -> Row<'_, Message, Theme, Renderer> {
    let mut ability_row: Row<'_, Message, Theme, Renderer> = Row::new();
    ability_row = ability_row.spacing(10);
    if abilitites.len() <= 2 {
        for ability in abilitites.iter() {
            ability_row = ability_row.push(
                text(ability.name.clone().to_uppercase())
                    .width(100)
                    .height(HEIGHT)
                    .size(12)
                    .center(),
            );
        }
        return ability_row;
    }
    let mut ability_column = Column::new();
    for ability in abilitites.iter().take(2) {
        ability_column = ability_column.push(
            text(ability.name.clone().to_uppercase())
                .width(100)
                .height(HEIGHT / 2)
                .size(12)
                .center(),
        );
    }
    ability_row = ability_row.push(ability_column);
    let mut ability_column = Column::new();
    for ability in abilitites.iter().skip(2) {
        ability_column = ability_column.push(
            text(ability.name.clone().to_uppercase())
                .width(100)
                .height(HEIGHT)
                .size(12)
                .center(),
        );
    }
    ability_row = ability_row.push(ability_column);
    return ability_row;
}

pub fn compose_stats(stats: &Stats) -> Row<'_, Message, Theme, Renderer> {
    let mut stats_row: Row<'_, Message, Theme, Renderer> = Row::new();
    for (i, &val) in stats.stats.iter().enumerate() {
        let mut indiv_column = Column::new().width(HEIGHT);
        let txt = match i {
            0 => text("HP"),
            1 => text("Atk"),
            2 => text("Def"),
            3 => text("SpA"),
            4 => text("SpD"),
            5 => text("Spe"),
            _ => unreachable!(),
        };
        indiv_column = indiv_column.push(txt.height(HEIGHT / 2).center());
        indiv_column = indiv_column.push(text(val).height(HEIGHT / 2).center());
        stats_row = stats_row.push(indiv_column);
    }
    let mut indiv_column = Column::new().width(HEIGHT);
    indiv_column = indiv_column.push(text("BST").height(HEIGHT / 2).center());
    indiv_column = indiv_column.push(
        text(stats.stats.iter().sum::<i64>())
            .height(HEIGHT / 2)
            .center(),
    );
    stats_row = stats_row.push(indiv_column);
    stats_row
}
