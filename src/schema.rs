table! {
    config (id) {
        id -> Int4,
        chat_id -> Int8,
        chat_name -> Varchar,
        chat_username -> Varchar,
        threshold -> Int2,
        timeout -> Int2,
        timezone -> Int2,
        lang -> Varchar,
    }
}

table! {
    message (id) {
        id -> Int4,
        chat_id -> Int8,
        fwd_msg_id -> Int8,
        msg_id -> Int8,
        content -> Varchar,
        create_time -> Timestamptz,
    }
}

table! {
    record (id) {
        id -> Int4,
        chat_id -> Int8,
        msg_id -> Int8,
        msg_ids -> Jsonb,
        create_time -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    config,
    message,
    record,
);
