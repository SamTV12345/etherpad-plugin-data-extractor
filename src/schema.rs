// @generated automatically by Diesel CLI.

diesel::table! {
    datas (id) {
        id -> Text,
        plugin_name -> Text,
        _id -> Text,
        _rev -> Text,
        name -> Text,
        license -> Nullable<Text>,
        downloads -> Integer,
    }
}

diesel::table! {
    ep_changes (name) {
        name -> Text,
        seq_id -> BigInt,
    }
}

diesel::table! {
    keywords (id) {
        id -> Text,
        version_id -> Text,
        keyword -> Text,
    }
}

diesel::table! {
    officialRepositories (id) {
        id -> Text,
    }
}

diesel::table! {
    plugin_shorts (name) {
        name -> Text,
        description -> Nullable<Text>,
        time_downloaded -> Nullable<Text>,
        version -> Text,
        official -> Bool,
        downloads -> Nullable<Integer>,
    }
}

diesel::table! {
    plugins (name) {
        name -> Text,
        description -> Text,
        time -> Timestamp,
        version -> Text,
        official -> Bool,
    }
}

diesel::table! {
    sequences (id) {
        id -> Text,
        val -> BigInt,
    }
}

diesel::table! {
    timestamp_sync (id) {
        id -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    versions (id) {
        id -> Text,
        data_id -> Text,
        name -> Text,
        version -> Text,
        description -> Text,
        time -> Timestamp,
        author_name -> Text,
        author_email -> Text,
        license -> Nullable<Text>,
        repository_type -> Nullable<Text>,
        repository_url -> Nullable<Text>,
        keywords -> Nullable<Text>,
        image -> Nullable<Text>,
        readme -> Nullable<Text>,
    }
}

diesel::joinable!(datas -> plugins (plugin_name));
diesel::joinable!(keywords -> versions (version_id));
diesel::joinable!(versions -> datas (data_id));

diesel::allow_tables_to_appear_in_same_query!(
    datas,
    ep_changes,
    keywords,
    officialRepositories,
    plugin_shorts,
    plugins,
    sequences,
    timestamp_sync,
    versions,
);
