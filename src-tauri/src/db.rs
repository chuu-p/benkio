use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use crate::{
        db::establish_connection,
        models::{Flashcard, NewCard},
    };
    use chrono::Utc;
    use diesel::prelude::*;

    pub fn create_card(conn: &mut SqliteConnection, front_a: &str, back_a: &str) -> Flashcard {
        use crate::schema::flashcards;

        let new_card = NewCard {
            content_front: front_a,
            content_back: back_a,
            interval: 1,
            next_review: Utc::now().naive_utc(),
        };

        diesel::insert_into(flashcards::table)
            .values(&new_card)
            .returning(Flashcard::as_returning())
            .get_result(conn)
            .expect("Error saving new flashcard")
    }

    #[test]
    fn it_create_card() {
        let connection = &mut establish_connection();

        diesel::delete(crate::schema::flashcards::table)
            .execute(connection)
            .unwrap();

        let contents = vec![
            // Day 1
            // 1 vocals
            ("あ", "a"),
            ("い", "i"),
            ("う", "u"),
            ("え", "e"),
            ("お", "o"),
            // 2 k
            ("か", "ka"),
            ("き", "ki"),
            ("く", "ku"),
            ("け", "ke"),
            ("こ", "ko"),
            // 3 s
            ("さ", "sa"),
            ("し", "shi"),
            ("す", "su"),
            ("せ", "se"),
            ("そ", "so"),
            // 4 t
            ("た", "ta"),
            ("ち", "chi"),
            ("つ", "tsu"),
            ("て", "te"),
            ("と", "to"),
            // Day 2
            // 5 n
            ("な", "na"),
            ("に", "ni"),
            ("ぬ", "nu"),
            ("ね", "ne"),
            ("の", "no"),
            // 6 h
            ("は", "ha"),
            ("ひ", "hi"),
            ("ふ", "fu"),
            ("へ", "he"),
            ("ほ", "ho"),
            // 7 m
            ("ま", "ma"),
            ("み", "mi"),
            ("む", "mu"),
            ("め", "me"),
            ("も", "mo"),
            // 8 y
            ("や", "ya"),
            ("ゆ", "yu"),
            ("よ", "yo"),
            // Day 3
            // 9 r
            ("ら", "ra"),
            ("り", "ri"),
            ("る", "ru"),
            ("れ", "re"),
            ("ろ", "ro"),
            // 10 w + n
            ("わ", "wa"),
            ("を", "wo"),
            ("ん", "n"),
        ];

        for (front_a, back_a) in contents {
            create_card(connection, front_a, back_a);
        }
    }

    // #[test]
    fn it_works() {
        let connection = &mut establish_connection();

        let results = crate::flashcards::dsl::flashcards
            .limit(5)
            .select(Flashcard::as_select())
            .load(connection)
            .expect("Error loading cards");

        println!("Displaying {} cards", results.len());
        for flashcard in results {
            println!("{}", flashcard.content_front);
            println!("{}", flashcard.content_back);
            println!("-----------");
        }
    }
}
