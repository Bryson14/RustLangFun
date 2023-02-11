use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct QueryRoot; // (1)

#[Object] // (2)
impl QueryRoot { // (3)
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str { // (4)
        "Hello world"
    }
}