use diesel::table;

table! {
    schools {
        id -> Text,
        name -> Text
    }
}