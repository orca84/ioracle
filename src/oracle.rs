use crate::errors::IOracleResult;
use crate::models::record::{Record, UpdatedRecord};
use crate::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::State;
use rocket_contrib::databases::diesel::SqliteConnection;
use uuid::Uuid;

pub fn ask_question(
    config: State<Config>,
    connection: &SqliteConnection,
    email: String,
    question: String,
) -> IOracleResult<String> {
    let answer = ioracle(&question)?;
    let answer_uuid = save(connection, &email, &question, &answer)?;
    send(config, &email, &question, &answer)?;

    Ok(answer_uuid)
}

pub fn ioracle(question: &String) -> IOracleResult<String> {
    // TODO: question -> answer

    Ok(question.to_string())
}

pub fn get_answer(connection: &SqliteConnection, uuid: String) -> IOracleResult<String> {
    let record = Record::get_by_uuid(&connection, uuid)?;

    Ok(record.answer)
}

pub fn save(
    connection: &SqliteConnection,
    email: &String,
    question: &String,
    answer: &String,
) -> IOracleResult<String> {
    let uuid = Uuid::new_v4();
    let uuid = uuid.to_string();

    let _r = Record::insert(
        &connection,
        UpdatedRecord {
            uuid: uuid.to_string(),
            email: email.to_string(),
            question: question.to_string(),
            answer: answer.to_string(),
        },
    )?;

    Ok(uuid)
}

pub fn send(
    config: State<Config>,
    email: &String,
    _question: &String,
    answer: &String,
) -> IOracleResult<()> {
    let body_text = format!("Your answer: {}", answer);

    let email = Message::builder()
        .from(config.email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("I Oracle")
        .body(body_text)
        .unwrap();

    let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());

    let mailer = SmtpTransport::relay("mail.privateemail.com")
        .unwrap()
        .credentials(creds)
        .build();

    //TODO: return result
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    Ok(())
}
