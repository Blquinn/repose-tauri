use serde_json::Value;
use sqlite::{ Connection, Value as SValue };
use serde::{Deserialize, Serialize};
use base64::encode;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementDTO {
    stmt: String,
    args: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Commands {
    Select(StatementDTO),
    SelectOne(StatementDTO),
    Exec(StatementDTO)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Responses {
    Select(SelectResponseDTO),
    SelectOne(SelectOneResponseDTO),
    Exec(ExecResponseDTO)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectResponseDTO {
    rows: Vec<Vec<Value>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectOneResponseDTO {
    row: Option<Vec<Value>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecResponseDTO {
    num_rows_effected: usize,
}

pub struct DB {
    conn: Connection,
}

// TODO: Actually handle errors.
impl DB {
    pub fn new() -> Self {
        let conn = Connection::open("/tmp/repose.db").unwrap();

        DB{ conn }
    }

    pub fn handle_command(&self, cmd: Commands) -> Responses {
        match cmd {
            Commands::Select(s) => { Responses::Select(self.select(s)) }
            Commands::SelectOne(s) => { Responses::SelectOne(self.select_one(s)) }
            Commands::Exec(e) => { Responses::Exec(self.exec(e)) }
        }
    }

    pub fn select(&self, query: StatementDTO) -> SelectResponseDTO {
        let mut curs = self.conn.prepare(query.stmt).unwrap().cursor();

        let args: Vec<SValue> = query.args.iter().map(|v| { value_to_s(v) }).collect();
        curs.bind(&args).unwrap();

        let mut rows: Vec<Vec<Value>> = Vec::new();

        let ct = curs.count();
        while let Some(row) = curs.next().unwrap() {
            let mut jrow: Vec<Value> = Vec::new();
            for i in 0..ct {
                jrow.push(s_to_value(&row[i]));
            }
            rows.push(jrow);
        }

        SelectResponseDTO { rows }
    }

    pub fn select_one(&self, query: StatementDTO) -> SelectOneResponseDTO {
        let mut res = self.select(query);

        if res.rows.len() > 0 {
            SelectOneResponseDTO { row: Some(res.rows.remove(0)) }
        } else {
            SelectOneResponseDTO { row: None }
        }
    }

    // TODO: Return last inserted row_id.
    pub fn exec(&self, query: StatementDTO) -> ExecResponseDTO {
        let mut stmt = self.conn.prepare(query.stmt).unwrap();

        for (i, arg) in query.args.iter().enumerate() {
            bind_value(&mut stmt, i, arg);
        }

        stmt.next().unwrap();

        ExecResponseDTO {
            num_rows_effected: self.conn.changes(),
        }
    }
}

fn s_to_value(val: &SValue) -> Value {
    match val {
        SValue::Binary(b) => { Value::from(encode(b.to_owned())) }
        SValue::Float(f) => { Value::from(f.to_owned()) }
        SValue::Integer(i) => { Value::from(i.to_owned()) }
        SValue::String(s) => { Value::from(s.to_owned()) }
        SValue::Null => { Value::Null }
    }
}

fn bind_value(stmt: &mut sqlite::Statement, i: usize, val: &Value) {
    match val {
        Value::Null => { stmt.bind(i, 0).unwrap(); }
        Value::Bool(b) => { stmt.bind(i, b.to_owned() as i64).unwrap(); }
        Value::Number(num) => {
            let n = num.to_owned();

            if let Some(ii) = n.as_i64() {
                stmt.bind(i, ii).unwrap();
            } else if let Some(ii) = n.as_u64() {
                stmt.bind(i, ii as i64).unwrap();
            } else if let Some(f) = n.as_f64() {
                stmt.bind(i, f).unwrap();
            } else {
                stmt.bind(i, 0).unwrap();
            }
        }
        Value::String(s) => {
            let ss = s.to_owned();
            stmt.bind(i, &ss[..]).unwrap();
        }
        Value::Array(_) => { panic!("Cannot encode json array.") }
        Value::Object(_) => { panic!("Cannot encode json object.") }
    }
}

fn value_to_s(val: &Value) -> SValue {
    match val {
        Value::Null => { SValue::Null }
        Value::Bool(b) => { SValue::Integer(b.to_owned() as i64) }
        Value::Number(num) => {
            let n = num.to_owned();

            if let Some(i) = n.as_i64() {
                SValue::Integer(i)
            } else if let Some(i) = n.as_u64() {
                SValue::Integer(i as i64)
            } else if let Some(f) = n.as_f64() {
                SValue::Float(f)
            } else {
                SValue::Null
            }
        }
        Value::String(s) => { SValue::String(s.to_owned()) }
        Value::Array(_) => { panic!("Cannot encode json array.") }
        Value::Object(_) => { panic!("Cannot encode json object.") }
    }
}

#[cfg(test)]
mod tests {
    use crate::sqlite::{StatementDTO, DB};

    #[test]
    fn select() {
        let db = DB::new();
        let j = r#"{"stmt": "select ?", "args": [1]}"#;
        let s: StatementDTO = serde_json::from_str(j).unwrap();
        let res = db.select(s);
        assert_eq!(res.rows[0][0], 1);
    }
}
