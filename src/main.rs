extern crate rusqlite;

use rusqlite::Connection;

#[derive(Debug)]
struct Data {
    id: u32,
    name: String,
    used: bool,
}

fn mark_used(conn: &Connection, db: &str, d: &Data)-> rusqlite::Result<()>{
    let mut stmt = conn.prepare(&format!("update {} set used = 1 where id=?", db))?;
    stmt.execute(&[&d.id])?;
    Ok(())
}

fn reset_table(conn: &Connection, db: &str) -> rusqlite::Result<()>{
    let mut stmt = conn.prepare(&format!("update {} set used = 0", db))?;
    stmt.execute(&[])?;
    Ok(())
}

fn find_next(conn: &Connection, db: &str) -> rusqlite::Result<Data>{
    let mut stmt = conn.prepare(
        &format!("select id, name, used from {} where used=0 limit 1", db))?;
    let mut rows = stmt.query(&[])?;

    match rows.next() {
        Some(result_row) => {
            let row = result_row?;
            let d = Data {
                id: row.get(0),
                name: row.get(1),
                used: row.get(2),
            };
            return Ok(d);
        }
        None => {
            // Reset all the values to 0 and start over
            reset_table(conn, db)?;
            return find_next(conn, db);
        }
    };
}

fn main() {
    let conn = Connection::open("dinner.sqlite3").expect("Failed to open db");

    //Treat the data in the database like a circular linked list
    let style = find_next(&conn, "style").expect("Can't find style");
    let main = find_next(&conn, "main").expect("Can't find main");
    let side1 = find_next(&conn, "side1").expect("Can't find side1");
    let side2 = find_next(&conn, "side2").expect("Can't find side2");

    mark_used(&conn, "style", &style).expect("Couldn't mark style value as used");
    mark_used(&conn, "main", &main).expect("Couldn't mark main value as used");
    mark_used(&conn, "side1", &side1).expect("Couldn't mark side1 value as used");
    mark_used(&conn, "side2", &side2).expect("Couldn't mark side2 value as used");

    println!("Todays dinner is {} style {} with sides of {} and {}", 
        style.name, main.name, side1.name, side2.name);
}
