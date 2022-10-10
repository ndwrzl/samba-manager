table! {
    logs (id) {
        id -> Integer,
        date -> BigInt,
        server_user -> Text,
        client_ip -> Text,
        client_name -> Text,
        share_name -> Text,
        action -> Text,
        ok -> Nullable<Bool>,
        permissions -> Nullable<Integer>,
        data -> Nullable<Text>,
        path -> Text,
        path2 -> Nullable<Text>,
    }
}
