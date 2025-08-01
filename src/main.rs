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

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("pokedex.sqlite").await?;
    let name = String::from("ivysaur");

    let pokemon = get_pokemon(name, &pool).await;
    dbg!(pokemon);
    Ok(())
}

async fn get_pokemon(name: String, pool: &Pool<Sqlite>) -> Option<Pokemon> {
    let mut abilities: Vec<Ability> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    let pokemon_id = match sqlx::query!(
        "
            SELECT
            pokemon.id
            FROM
            pokemon
            WHERE
            pokemon.identifier = ?
        ",
        name
    )
    .fetch_one(pool)
    .await
    {
        Ok(res) => res.id,
        Err(_) => {
            return None;
        }
    };

    for ability in sqlx::query!(
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
        pokemon_id
    )
    .fetch_all(pool)
    .await
    .expect("Ability not found")
    {
        abilities.push(Ability {
            name: ability.identifier,
        });
    }

    for pokemon_move in sqlx::query!(
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
        pokemon_id
    )
    .fetch_all(pool)
    .await
    .expect("Moves not found")
    {
        moves.push(Move {
            name: pokemon_move.move_name,
            base_power: pokemon_move.base_power,
            generation: pokemon_move.generation,
            pp: pokemon_move.pp,
            accuracy: pokemon_move.move_accuracy,
            move_type: pokemon_move.move_type,
        });
    }

    Some(Pokemon {
        name,
        moves,
        abilities,
    })
}
