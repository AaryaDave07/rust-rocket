// @generated automatically by Diesel CLI.

diesel::table! {
    buyers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Nullable<Integer>,
        shop_id -> Integer,
        name -> Text,
        price -> Float,
    }
}

diesel::table! {
    orders (id) {
        id -> Nullable<Integer>,
        buyer_id -> Integer,
        item_id -> Integer,
        rider_id -> Nullable<Integer>,
        status -> Text,
    }
}

diesel::table! {
    payments (id) {
        id -> Nullable<Integer>,
        order_id -> Integer,
        amount -> Float,
        status -> Text,
    }
}

diesel::table! {
    riders (id) {
        id -> Integer,
        name -> Text,
        phone -> Text,
    }
}

diesel::table! {
    shops (id) {
        id -> Nullable<Integer>,
        name -> Text,
        location -> Text,
    }
}

diesel::joinable!(items -> shops (shop_id));
diesel::joinable!(orders -> buyers (buyer_id));
diesel::joinable!(orders -> items (item_id));
diesel::joinable!(orders -> riders (rider_id));
diesel::joinable!(payments -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    buyers,
    items,
    orders,
    payments,
    riders,
    shops,
);
