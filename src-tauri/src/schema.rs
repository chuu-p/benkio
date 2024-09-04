// @generated automatically by Diesel CLI.

diesel::table! {
    flashcards (id) {
        id -> Integer,
        content_front -> Text,
        content_back -> Text,
        interval -> Integer,
        next_review -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        card_id -> Integer,
        review_date -> Timestamp,
        duration_front -> Float,
        duration_back -> Float,
        correct -> Bool,
        interval -> Integer,
    }
}

diesel::joinable!(reviews -> flashcards (card_id));

diesel::allow_tables_to_appear_in_same_query!(
    flashcards,
    reviews,
);
