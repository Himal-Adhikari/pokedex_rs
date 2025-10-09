use std::fmt::Debug;

use iced::{
    Alignment, Color, Element,
    Length::Fill,
    Renderer, Task, Theme,
    widget::{
        Button, Column, Container, Row, Space, button, container, row, rule, scrollable, text,
        text_input,
    },
};
use sqlx::{Pool, Sqlite};

use crate::pokemon::{Ability, Pokemon, Pokemons, Stats, get_pokemons};

const HEIGHT: u32 = 50;
const MAX_BASE_STAT: f32 = 255.0;
const MAX_BASE_STAT_SIZE: f32 = 400.0;

const GREEN_START_STAT: f32 = 0.0;
const GREEN_STOP_STAT: f32 = 120.0;
const RED_STOP_STAT: f32 = 60.0;
const BLUE_START_STAT: f32 = 80.0;

const MAX_MOVE_NAME_WIDTH: u32 = 160;
const MAX_TYPE_NAME_WIDTH: u32 = 90;
const BASE_POWER_WIDTH: u32 = 70;
const ACCURACY_WIDTH: u32 = 70;
const PP_WIDTH: u32 = 35;

#[derive(Clone, Debug)]
pub enum Message {
    NameChanged(String),
    PokemonsFound(Option<Pokemons>),
    PokemonSelected(usize),
}

#[derive(Debug)]
pub struct State {
    name: String,
    pokemons: Pokemons,
    pool: Pool<Sqlite>,
    selected_pokemon: Option<Pokemon>,
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
                selected_pokemon: None,
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
            state.selected_pokemon = Some(state.pokemons.pokemons[index].clone());
        }
    }
    return Task::none();
}

pub fn theme(_state: &State) -> Theme {
    Theme::Light
    // Theme::CatppuccinMacchiato
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mut to_return = Column::new();

    to_return =
        to_return.push(text_input("Type pokemon name", &state.name).on_input(Message::NameChanged));
    if let Some(pokemon) = &state.selected_pokemon {
        to_return = to_return.push(default_app_render(&state.pokemons));
        return row![
            scrollable(to_return),
            rule::vertical(5),
            scrollable(pokemon_sidebar_view(pokemon))
        ]
        .spacing(10)
        .into();
    } else {
        to_return = to_return.push(default_app_render(&state.pokemons));
        return scrollable(to_return).height(Fill).into();
    }
}

pub fn default_app_render(pokemons: &Pokemons) -> Column<'_, Message> {
    let mut to_return = Column::new();
    for (index, pokemon) in pokemons.pokemons.iter().enumerate() {
        let pokemon_name = text(pokemon.name.as_str())
            .width(200)
            .height(HEIGHT)
            .align_x(Alignment::Start)
            .align_y(Alignment::Center);

        let abilities = compose_ability_main_view(&pokemon.abilities).width(260);
        let stats = compose_stats_main_view(&pokemon.stats);

        let row = row![pokemon_name, abilities, stats];
        let button: Button<'_, Message> = button(row)
            .style(|theme: &Theme, status| {
                let palette = theme.palette();
                let mut bstyle = match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        button::Style::default().with_background(palette.danger.inverse())
                    }
                    _ => button::Style::default().with_background(palette.background),
                };
                bstyle.text_color = palette.background.inverse();
                bstyle
            })
            .into();
        to_return = to_return.push(button.on_press(Message::PokemonSelected(index)));
    }
    return to_return;
}

pub fn pokemon_sidebar_view(pokemon: &Pokemon) -> Column<'_, Message> {
    let mut to_return = Column::new().spacing(10);

    to_return = to_return.push(text(pokemon.name.as_str()).size(40).width(Fill));

    to_return = to_return.push(text("Abilities:"));
    let mut abilities = Row::new();

    let ability_num = pokemon.abilities.len();
    abilities = abilities.push(text("    "));
    for i in 0..ability_num {
        let mut ability_name = pokemon.abilities[i].name.clone();
        if i != ability_num - 1 {
            ability_name.push_str(" | ");
        }
        abilities = abilities.push(text(ability_name));
    }
    to_return = to_return.push(abilities);

    to_return = to_return.push(text("\nBase stats:\n"));

    for (idx, &val) in pokemon.stats.stats.iter().enumerate() {
        let mut stat_row = Row::new();
        stat_row = stat_row.push(
            match idx {
                0 => text("HP:"),
                1 => text("Attack:"),
                2 => text("Defense:"),
                3 => text("Sp. Atk:"),
                4 => text("Sp. Def:"),
                5 => text("Speed:"),
                _ => unreachable!(),
            }
            .width(100)
            .align_x(Alignment::End),
        );
        let val_string = format!("    {}  ", val);
        stat_row = stat_row.push(text(val_string).width(80).align_x(Alignment::End));
        let stat_display_box = container("")
            .width(val as f32 / MAX_BASE_STAT * MAX_BASE_STAT_SIZE)
            .style(move |_theme| {
                container::Style::default().background(Color::from_rgb8(
                    (255.0 * (1.0 - (val as f32 / (RED_STOP_STAT)))) as u8,
                    ((255.0 / (GREEN_STOP_STAT - GREEN_START_STAT))
                        * (val as f32 - GREEN_START_STAT)) as u8,
                    ((255.0 / (MAX_BASE_STAT - BLUE_START_STAT)) * (val as f32 - BLUE_START_STAT))
                        as u8,
                ))
            });
        stat_row = stat_row.push(stat_display_box);

        to_return = to_return.push(stat_row);
    }
    let base_stat_total_row = Row::new()
        .push(text("Total:").width(100).align_x(Alignment::End))
        .push({
            let val_string = format!("    {}  ", pokemon.stats.stats.iter().sum::<i64>());
            text(val_string).width(80).align_x(Alignment::End)
        });

    to_return = to_return.push(base_stat_total_row);
    to_return = to_return.push(text("Moves"));

    for pokemon_move in pokemon.moves.iter() {
        let mut move_row = Row::new().height(40);
        move_row = move_row.push(text(pokemon_move.name.as_str()).width(MAX_MOVE_NAME_WIDTH));
        move_row = move_row.push(
            compose_type_container(pokemon_move.move_type.as_str()).width(MAX_TYPE_NAME_WIDTH),
        );

        let mut base_power_column = Column::new()
            .width(BASE_POWER_WIDTH)
            .align_x(Alignment::Center);

        if let Some(move_power) = pokemon_move.base_power {
            base_power_column = base_power_column
                .push(text("Power"))
                .push(text(move_power.to_string()));
        }
        move_row = move_row.push(base_power_column);

        let mut accuracy_column = Column::new()
            .width(ACCURACY_WIDTH)
            .align_x(Alignment::Center);
        move_row = move_row.push(Space::new().width(10));

        if let Some(accuracy) = pokemon_move.accuracy {
            accuracy_column = accuracy_column
                .push(text("Accuracy"))
                .push(text(accuracy.to_string()));
        }
        move_row = move_row.push(accuracy_column);

        let mut pp_column = Column::new().width(PP_WIDTH).align_x(Alignment::Center);
        move_row = move_row.push(Space::new().width(20));
        if let Some(pp) = pokemon_move.pp {
            pp_column = pp_column.push(text("PP")).push(text(pp.to_string()));
        }
        move_row = move_row.push(pp_column);

        to_return = to_return.push(move_row);
    }

    return to_return;
}

fn compose_type_container<'a>(typ: impl Into<String>) -> Container<'a, Message> {
    let mut name: String = typ.into();
    name = name.to_lowercase().trim().to_string();

    match name.as_str() {
        "normal" => container("NORMAL").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(170, 170, 153))
        }),
        "fire" => container("FIRE")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(255, 68, 34))),
        "water" => container("WATER")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(51, 153, 255))),
        "electric" => container("ELECTRIC")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(255, 204, 51))),
        "grass" => container("GRASS")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(119, 204, 85))),
        "ice" => container("ICE").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(102, 204, 255))
        }),
        "fighting" => container("FIGHTING")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(187, 85, 68))),
        "poison" => container("POISON")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(170, 85, 153))),
        "ground" => container("GROUND")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(221, 187, 85))),
        "flying" => container("FLYING").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(136, 153, 255))
        }),
        "psychic" => container("PSYCHIC")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(255, 85, 153))),
        "bug" => container("BUG")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(170, 187, 34))),
        "rock" => container("ROCK").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(187, 170, 102))
        }),
        "ghost" => container("GHOST").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(102, 102, 187))
        }),
        "dragon" => container("DRAGON").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(119, 102, 238))
        }),
        "dark" => container("DARK")
            .style(|_theme| container::Style::default().background(Color::from_rgb8(119, 85, 68))),
        "steel" => container("STEEL").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(170, 170, 187))
        }),
        "fairy" => container("FAIRY").style(|_theme| {
            container::Style::default().background(Color::from_rgb8(238, 153, 238))
        }),
        _ => unreachable!(),
    }
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
}

pub fn compose_ability_main_view(abilitites: &Vec<Ability>) -> Row<'_, Message, Theme, Renderer> {
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

pub fn compose_stats_main_view(stats: &Stats) -> Row<'_, Message, Theme, Renderer> {
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
