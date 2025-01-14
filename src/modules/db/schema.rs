use diesel::table;

table! {
    todos {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        metadata -> Nullable<Jsonb>,
        title -> Text,
        description -> Nullable<Text>
    }
}