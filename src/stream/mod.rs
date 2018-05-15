extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate std;
extern crate rocket;

use postgres::types::ToSql;

use std::mem;

pub struct PGStream {
    cursor: String,
    pending: Option<Vec<u8>>,
    trans: postgres::transaction::Transaction<'static>,
    conn: Box<r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>>
}

impl std::io::Read for PGStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut current = 0;

        while current < buf.len() {
            let mut write: Vec<u8> = Vec::new();

            if self.pending.is_some() {
                write = self.pending.clone().unwrap();
                self.pending = None;
            } else {
                let rows = self.trans.query(&*format!("FETCH 1000 FROM {};", &self.cursor), &[]).unwrap();

                if rows.len() != 0 {
                    for row_it in 0..rows.len() {
                        let feat: String = rows.get(row_it).get(0);
                        write.append(&mut feat.into_bytes().to_vec());
                        write.push(0x0A);
                    }
                }
            }

            if write.len() == 0 {
                //No more data to fetch, close up shop
                break;
            } else if current + write.len() > buf.len() {
                //There is room to put a partial feature, saving the remaining
                //to the pending q and ending

                for it in current..buf.len() {
                    buf[it] = write[it - current];
                }

                let pending = write[buf.len() - current..write.len()].to_vec();
                self.pending = Some(pending);

                current = current + (buf.len() - current);

                break;
            } else {
                //There is room in the buff to print the whole feature
                //and iterate around to grab another

                for it in 0..write.len() {
                    buf[current + it] = write[it];
                }

                current = current + write.len();
            }
        }

        Ok(current)
    }
}

impl PGStream {
    pub fn new(conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, cursor: String, query: String, params: &[&ToSql]) -> Result<Self, rocket::response::status::Custom<String>> {
        let pg_conn = Box::new(conn);

        let trans: postgres::transaction::Transaction = unsafe { mem::transmute(pg_conn.transaction().unwrap()) };

        trans.execute(&*query, params).unwrap();

        Ok(PGStream {
            cursor: cursor,
            pending: None,
            trans: trans,
            conn: pg_conn
        })
    }
}
