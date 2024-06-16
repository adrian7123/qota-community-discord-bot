pub fn routes() -> Vec<Route> {
    routes![get_players]
}

#[get("/")]
async fn get_players(ctx: &Ctx) -> &'static str {
    "Olá, Mundo!"
}
