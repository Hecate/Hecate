use postgres::types::ToSql;
use std::io::{Error, ErrorKind};
use crate::err::HecateError;
use bytes::{Bytes, BytesMut};
use futures::task::Context;
use std::task::Poll;
use std::pin::Pin;

use std::mem;

static EOT: u8 = 0x04;

pub struct PGStream {
    eot: bool, //End of Tranmission has been sent
    cursor: String,
    pending: Option<Vec<u8>>,
    trans: postgres::Transaction<'static>,
    #[allow(dead_code)]
    conn: Box<postgres::Client>
}

impl futures::stream::Stream for PGStream {
    type Item = Result<Bytes, HecateError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
        let rows = match self.trans.query(&*format!("FETCH 1000 FROM {};", &self.cursor), &[]) {
            Ok(rows) => rows,
            Err(_err) => { return Poll::Ready(None) }
        };

        if rows.is_empty() {
            if self.eot {
                // The Stream is complete
                return Poll::Ready(None);
            } else {
                self.eot = true;
                // Write EOD Character to Stream
                let mut bytes = BytesMut::new();
                bytes.extend_from_slice(&[EOT]);

                return Poll::Ready(Some(Ok(bytes.freeze())));
            }
        }

        let mut feats = String::new();

        for row_it in 0..rows.len() {
            let feat: String = rows.get(row_it).get(0);
            feats.push_str(&*feat);
            feats.push('\n');
        }

        Poll::Ready(Some(Ok(Bytes::from(feats))))
    }
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
                let rows = match self.trans.query(&*format!("FETCH 1000 FROM {};", &self.cursor), &[]) {
                    Ok(rows) => rows,
                    Err(err) => {
                        return Err(Error::new(ErrorKind::Other, format!("{:?}", err)))
                    }
                };

                if !rows.is_empty() {
                    for row_it in 0..rows.len() {
                        let feat: String = rows.get(row_it).get(0);
                        write.append(&mut feat.into_bytes().to_vec());
                        write.push(0x0A);
                    }
                }
            }

            if write.is_empty() && !self.eot {
                write.push(0x04); //Write EOT Character To Stream
                self.eot = true;
            }

            if write.is_empty() && self.eot {
                //No more data to fetch, close up shop
                break;
            } else if current + write.len() > buf.len() {
                //There is room to put a partial feature, saving the remaining
                //to the pending q and ending
                let length = buf.len();
                buf[current..].clone_from_slice(&write[0..(length - current)]);

                let pending = write[buf.len() - current..write.len()].to_vec();
                self.pending = Some(pending);

                current += buf.len() - current;

                break;
            } else {
                //There is room in the buff to print the whole feature
                //and iterate around to grab another

                buf[current..(write.len() + current)].clone_from_slice(&write[..]);

                current += write.len();
            }
        }

        Ok(current)
    }
}

impl PGStream {
    pub fn new(pg_conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<postgres::Client>>, cursor: String, query: String, params: &[&dyn ToSql]) -> Result<Self, HecateError> {
        let conn = Box::new(pg_conn);

        let trans: postgres::Transaction = unsafe {
            mem::transmute(conn.transaction().unwrap())
        };

        match trans.execute(&*query, params) {
            Ok(_) => {
                Ok(PGStream {
                    eot: false,
                    cursor,
                    pending: None,
                    trans,
                    conn
                })
            },
            Err(err) => Err(HecateError::from_db(err))
        }
    }
}

