

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    users,
);