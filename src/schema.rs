table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Nullable<Varchar>,
        password -> Varchar,
    }
}
