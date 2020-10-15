use crate::config::Config;
use crate::errors::IOracleResult;
use crate::models::binding::{Binding, UpdatedBinding};
use crate::models::hexagram::UpdatedHexagram;
use crate::models::trigram::UpdatedTrigram;
use crate::oracle::{ask_question, get_answer};
use crate::views::context::{ItemContext, NoContext};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Question {
    email: String,
    question: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<question>")]
pub fn question(
    config: State<Config>,
    connection: Db,
    question: Form<Question>,
) -> IOracleResult<Redirect> {
    Ok(Redirect::to(format!(
        "/answer/{}",
        ask_question(
            config,
            &connection,
            question.email.to_owned(),
            question.question.to_owned()
        )?
    )))
}

#[get("/answer/<uuid>")]
pub fn answer(connection: Db, uuid: String) -> IOracleResult<Template> {
    Ok(Template::render(
        "answer",
        ItemContext {
            item: get_answer(&connection, uuid)?,
        },
    ))
}

#[get("/settings")]
pub fn settings(connection: Db) -> IOracleResult<Template> {
    Ok(Template::render(
        "settings",
        ItemContext {
            item: Binding::get(&connection)?,
        },
    ))
}

#[post("/save", format = "json", data = "<bindings>")]
pub fn save(connection: Db, bindings: Json<UpdatedBinding>) -> IOracleResult<Redirect> {
    Binding::update(&connection, bindings.into_inner())?;

    Ok(Redirect::to("/settings"))
}

#[get("/csv")]
pub fn csv(_connection: Db) -> IOracleResult<String> {
    // hexagrams
    let file_path = "/home/denis/collector/iora/csv/expanded_gua.csv";
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: UpdatedHexagram = result?;
        println!("{:#?}", record);
        // let _h = crate::models::hexagram::Hexagram::insert(&connection, record)?;
    }

    //trigrams
    let file_path = "/home/denis/collector/iora/csv/trigrams.csv";
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: UpdatedTrigram = result?;
        println!("{:#?}", record);
        // let _t = crate::models::trigram::Trigram::insert(&connection, record)?;
    }

    Ok("Ok".to_string())
}
