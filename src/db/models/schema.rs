// @generated automatically by Diesel CLI.

diesel::table! {
    deals (uuid) {
        uuid -> Uuid,
        name -> Text,
        image -> Text,
        price -> Int4,
        description -> Text,
    }
}

diesel::table! {
    issuers (uuid) {
        uuid -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    jwt_issuers (uuid) {
        uuid -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    nonces (session_id) {
        nonce -> Text,
        session_id -> Text,
        key -> Text,
        created_at -> Int8,
    }
}

diesel::table! {
    roles (uuid) {
        uuid -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        session_data -> Nullable<Text>,
        expires_at -> Timestamp,
        user_agent -> Nullable<Text>,
        last_activity -> Timestamp,
        ip -> Nullable<Text>,
        user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        email -> Text,
        password -> Text,
        role -> Uuid,
    }
}

diesel::joinable!(nonces -> sessions (session_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(users -> roles (role));

diesel::allow_tables_to_appear_in_same_query!(
    deals,
    issuers,
    jwt_issuers,
    nonces,
    roles,
    sessions,
    users,
);
