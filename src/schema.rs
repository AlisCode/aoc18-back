table! {
    authprovider (id) {
        id -> Nullable<Integer>,
        prov_name -> Text,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        token -> Text,
        auth_provider -> Integer,
        ext_token -> Text,
    }
}

joinable!(users -> authprovider (auth_provider));

allow_tables_to_appear_in_same_query!(authprovider, users,);
