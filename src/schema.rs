table! {
    rtts (id) {
        id -> Char,
        src -> Varchar,
        dst -> Varchar,
        sid -> Nullable<Varchar>,
        rtt -> Unsigned<Integer>,
    }
}
