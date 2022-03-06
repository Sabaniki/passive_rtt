table! {
    raw_rtts (id) {
        id -> Unsigned<Bigint>,
        src -> Varchar,
        dst -> Varchar,
        sid -> Nullable<Varchar>,
        rtt -> Unsigned<Integer>,
    }
}
