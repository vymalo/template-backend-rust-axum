use diesel::table;

table! {
    todos {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        metadata -> Nullable<Jsonb>,
        title -> Text,
        description -> Nullable<Text>
    }
}