// use async_graphql::*;

// struct Query;

// #[Object]
// impl Query {
//     /// Returns the sum of a and b
//     async fn add(&self, a: i32, b: i32) -> i32 {
//         a + b
//     }
// }

// let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
// let res = schema.execute("{ add(a: 10, b: 20) }").await;
