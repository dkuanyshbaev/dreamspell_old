table! {
    glyphs (id) {
        id -> Int4,
        num -> Int4,
        name -> Varchar,
        image -> Varchar,
        preview -> Text,
        description -> Text,
    }
}

table! {
    kins (id) {
        id -> Int4,
        num -> Int4,
        name -> Varchar,
        image -> Varchar,
    }
}

table! {
    tones (id) {
        id -> Int4,
        num -> Int4,
        name -> Varchar,
        image -> Varchar,
        preview -> Text,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    glyphs,
    kins,
    tones,
    users,
);
