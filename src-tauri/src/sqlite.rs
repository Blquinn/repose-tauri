use serde_json::Value;
use sqlite::{Connection, Value as SValue};
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

    pub fn handle_command(&self, cmd: Commands) -> sqlite::Result<Responses> {
        match cmd {
            Commands::Select(s) => {
                self.select(s).map(|r| { Responses::Select(r) })
            }
            Commands::SelectOne(s) => {
                self.select_one(s).map(|r| { Responses::SelectOne(r) })
            }
            Commands::Exec(e) => {
                self.exec(e).map(|r| { Responses::Exec(r) })
            }
        }
    }

    pub fn select(&self, query: StatementDTO) -> sqlite::Result<SelectResponseDTO> {
        let mut curs = self.conn.prepare(query.stmt)?.cursor();

        let args: Vec<SValue> = query.args.iter()
            .map(|v| { value_to_s(v) })
            .collect::<sqlite::Result<Vec<SValue>>>()?;

        curs.bind(&args)?;

        let mut rows: Vec<Vec<Value>> = Vec::new();

        let ct = curs.count();
        while let Some(row) = curs.next()? {
            let mut jrow: Vec<Value> = Vec::new();
            for i in 0..ct {
                jrow.push(s_to_value(&row[i]));
            }
            rows.push(jrow);
        }

        Ok(SelectResponseDTO { rows })
    }

    pub fn select_one(&self, query: StatementDTO) -> sqlite::Result<SelectOneResponseDTO> {
        let mut res = self.select(query)?;

        if res.rows.len() > 0 {
            Ok(SelectOneResponseDTO { row: Some(res.rows.remove(0)) })
        } else {
            Ok(SelectOneResponseDTO { row: None })
        }
    }

    // TODO: Return last inserted row_id.
    pub fn exec(&self, query: StatementDTO) -> sqlite::Result<ExecResponseDTO> {
        let mut stmt = self.conn.prepare(query.stmt)?;

        for (i, arg) in query.args.iter().enumerate() {
            bind_value(&mut stmt, i+1, arg)?;
        }

        stmt.next()?;

        Ok(ExecResponseDTO { num_rows_effected: self.conn.changes() })
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

fn bind_value(stmt: &mut sqlite::Statement, i: usize, val: &Value) -> sqlite::Result<()> {
    match val {
        Value::Null => {
            stmt.bind(i, 0)?;
            Ok(())
        }
        Value::Bool(b) => {
            stmt.bind(i, b.to_owned() as i64)?;
            Ok(())
        }
        Value::Number(num) => {
            let n = num.to_owned();

            if let Some(ii) = n.as_i64() {
                stmt.bind(i, ii)?;
            } else if let Some(ii) = n.as_u64() {
                stmt.bind(i, ii as i64)?;
            } else if let Some(f) = n.as_f64() {
                stmt.bind(i, f)?;
            } else {
                stmt.bind(i, 0)?;
            }

            Ok(())
        }
        Value::String(s) => {
            let ss = s.to_owned();
            stmt.bind(i, &ss[..])?;
            Ok(())
        }
        Value::Array(_) => {
            Err(sqlite::Error{
                code: None,
                message: Some("Cannot encode json array.".to_owned())
            })
        }
        Value::Object(_) => {
            Err(sqlite::Error{
                code: None,
                message: Some("Cannot encode json object.".to_owned())
            })
        }
    }
}

fn value_to_s(val: &Value) -> sqlite::Result<SValue> {
    match val {
        Value::Null => { Ok(SValue::Null) }
        Value::Bool(b) => { Ok(SValue::Integer(b.to_owned() as i64)) }
        Value::Number(num) => {
            let n = num.to_owned();

            let val = if let Some(i) = n.as_i64() {
                SValue::Integer(i)
            } else if let Some(i) = n.as_u64() {
                SValue::Integer(i as i64)
            } else if let Some(f) = n.as_f64() {
                SValue::Float(f)
            } else {
                SValue::Null
            };

            Ok(val)
        }
        Value::String(s) => { Ok(SValue::String(s.to_owned())) }
        Value::Array(_) => {
            Err(sqlite::Error{
                code: None,
                message: Some("Cannot encode json array.".to_owned())
            })
        }
        Value::Object(_) => {
            Err(sqlite::Error{
                code: None,
                message: Some("Cannot encode json object.".to_owned())
            })
        }
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
