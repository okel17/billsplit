use async_graphql::*;
struct Query;

#[Object]
impl Query {
    async fn make_email(&self, name: String) -> String {
        name + "@gmail.com"
    }
}

#[actix_web::main]
async fn main() -> serde_json::Result<()> {

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ makeEmail( name: \"om\" ) }").await;
    
    let json = serde_json::to_string(&res)?;
    println!("{}", json);
    
    Ok(())
}


