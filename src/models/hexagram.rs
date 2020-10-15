use super::schema::hexagrams;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Hexagram {
    id: i32,
    binary: String,
    king_wen_order: i32,
    shao_yong_order: i32,
    gua: String,
    pin_yin: String,
    character: String,
    wilheim: String,
    huang: String,
    hatcher: String,
    no2do: String,
    inner_ba_gua: String,
    outer_ba_gua: String,
    host_yao: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
#[table_name = "hexagrams"]
#[serde(rename_all = "PascalCase")]
pub struct UpdatedHexagram {
    binary: String,
    king_wen_order: i32,
    shao_yong_order: i32,
    gua: String,
    pin_yin: String,
    character: String,
    wilheim: String,
    huang: String,
    hatcher: String,
    no2do: String,
    inner_ba_gua: String,
    outer_ba_gua: String,
    host_yao: String,
}

impl Hexagram {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<Hexagram>> {
        hexagrams::table.order(hexagrams::id.asc()).load(connection)
    }

    pub fn get(connection: &SqliteConnection, id: i32) -> QueryResult<Hexagram> {
        hexagrams::table.find(id).get_result(connection)
    }

    pub fn insert(
        connection: &SqliteConnection,
        new_hexagram: UpdatedHexagram,
    ) -> QueryResult<usize> {
        diesel::insert_into(hexagrams::table)
            .values(new_hexagram)
            .execute(connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: i32,
        new_hexagram: UpdatedHexagram,
    ) -> QueryResult<usize> {
        diesel::update(hexagrams::table.find(id))
            .set(new_hexagram)
            .execute(connection)
    }
}
