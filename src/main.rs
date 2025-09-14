use sqlx::{Pool, Sqlite, sqlite::SqlitePool};

#[derive(Debug)]
struct Pokemon {
    name: String,
    moves: Vec<Move>,
    abilities: Vec<Ability>,
}

#[derive(Debug)]
struct Ability {
    name: String,
}

#[derive(Debug)]
struct Move {
    name: String,
    base_power: Option<i64>,
    generation: i64,
    pp: Option<i64>,
    accuracy: Option<i64>,
    move_type: String,
}

impl Pokemon {
    pub fn from(name: String, moves: Vec<Move>, abilities: Vec<Ability>) -> Pokemon {
        Pokemon {
            name,
            moves,
            abilities,
        }
    }
}

impl Ability {
    pub fn from(name: String) -> Ability {
        Ability { name }
    }
}

impl Move {
    pub fn from(
        name: String,
        base_power: Option<i64>,
        generation: i64,
        pp: Option<i64>,
        accuracy: Option<i64>,
        move_type: String,
    ) -> Move {
        Move {
            name,
            base_power,
            generation,
            pp,
            accuracy,
            move_type,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("pokedex.sqlite").await?;
    let name = String::from("pik");

    let pokemon = get_pokemons(name, &pool).await;
    Ok(())
}

async fn get_pokemons(name: String, pool: &Pool<Sqlite>) -> Option<Vec<Pokemon>> {
    let mut new_name = String::from("%");
    new_name.push_str(&name);
    new_name.push_str("%");
    let mut abilities: Vec<Vec<Ability>> = Vec::new();
    let mut moves: Vec<Vec<Move>> = Vec::new();

    let pokemon_id = match sqlx::query!(
        "
            SELECT
            pokemon.id,
            pokemon.identifier
            FROM
            pokemon
            WHERE
            pokemon.identifier LIKE ?
        ",
        new_name
    )
    .fetch_all(pool)
    .await
    {
        Ok(res) => res,
        Err(_) => {
            return None;
        }
    };

    for p_id in pokemon_id.iter() {
        abilities.push(
            sqlx::query!(
                "
        SELECT
        abilities.identifier
        FROM
        pokemon
        INNER JOIN
        pokemon_abilities ON pokemon.id = pokemon_abilities.pokemon_id
        INNER JOIN
        abilities ON pokemon_abilities.ability_id = abilities.id
        WHERE
        pokemon.id = ?
        ",
                p_id.id
            )
            .fetch_all(pool)
            .await
            .expect("Abilities not found for pokemon id {p_id.id}")
            .into_iter()
            .map(|rec| Ability::from(rec.identifier))
            .collect::<Vec<Ability>>(),
        );

        moves.push(
            sqlx::query!(
                "
            SELECT
            moves.identifier AS move_name,
            moves.power AS base_power,
            moves.generation_id AS generation,
            moves.pp AS pp,
            moves.accuracy AS move_accuracy,
            types.identifier AS move_type
            FROM
            pokemon
            INNER JOIN
            pokemon_moves ON pokemon.id = pokemon_moves.pokemon_id
            INNER JOIN
            moves ON pokemon_moves.move_id = moves.id
            INNER JOIN
            types ON moves.type_id = types.id
            WHERE pokemon.id = ?
            ",
                p_id.id
            )
            .fetch_all(pool)
            .await
            .expect("Moves not found")
            .into_iter()
            .map(|p_moves| {
                Move::from(
                    p_moves.move_name,
                    p_moves.base_power,
                    p_moves.generation,
                    p_moves.pp,
                    p_moves.move_accuracy,
                    p_moves.move_type,
                )
            })
            .collect::<Vec<Move>>(),
        );
    }

    Some(
        pokemon_id
            .into_iter()
            .zip(abilities.into_iter())
            .zip(moves.into_iter())
            .map(|((p_id, abi), p_mov)| Pokemon::from(p_id.identifier, p_mov, abi))
            .collect::<Vec<Pokemon>>(),
    )
}
