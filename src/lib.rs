//! # r2d2-mysql
//! MySQL support for the r2d2 connection pool (Rust) . see [`r2d2`](http://github.com/sfackler/r2d2.git)  .
//!
//! #### Install
//! Just include another `[dependencies.*]` section into your Cargo.toml:
//!
//! ```toml
//! [dependencies.r2d2_mysql]
//! git = "https://github.com/outersky/r2d2-mysql"
//! version="*"
//! ```
//! #### Sample
//!
//! ```
//! extern crate mysql;
//! extern crate r2d2_mysql;
//! extern crate r2d2;
//! use std::sync::Arc;
//! use std::thread;
//! use mysql::{Opts,OptsBuilder};
//! use r2d2_mysql::MysqlConnectionManager;
//!
//! fn main() {
//! 	let db_url =  "mysql://root:12345678@localhost:3306/test";
//!     let opts = Opts::from_url(db_url).unwrap();
//!     let builder = OptsBuilder::from_opts(opts);
//!     let manager = MysqlConnectionManager::new(builder);
//!     let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
//!
//!     let mut tasks = vec![];
//!
//!     for i in 0..3 {
//!         let pool = pool.clone();
//!         let th = thread::spawn(move || {
//!             let mut conn = pool.get().unwrap();
//!             conn.query("select user()").unwrap();
//!             println!("thread {} end!" , i );
//!         });
//!         tasks.push(th);
//!     }
//!
//!     for th in tasks {
//!         let _ = th.join();
//!     }
//! }
//! ```
//!

#![doc(html_root_url = "http://outersky.github.io/r2d2-mysql/doc/v0.2.0/r2d2_mysql/")]
#![crate_name = "r2d2_mysql"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate rustc_serialize as serialize;
pub extern crate mysql;
pub extern crate r2d2;

pub mod pool;
pub use pool::MysqlConnectionManager;

#[cfg(test)]
mod test {
    use mysql::{Opts, OptsBuilder};
    use r2d2;
    use std::sync::Arc;
    use std::thread;
    use super::MysqlConnectionManager;

    const DB_URL: &'static str = "mysql://root:12345678@localhost:3306/test";

    #[test]
    fn query_pool() {
        let opts = Opts::from_url(DB_URL).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
        let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let th = thread::spawn(move || {
                let mut conn = pool.get()
                    .map_err(|err| {
                        println!(
                            "get connection from pool error in line:{} ! error: {:?}",
                            line!(),
                            err
                        )
                    })
                    .unwrap();
                conn.query("select user()")
                    .map_err(|err| {
                        println!("execute query error in line:{} ! error: {:?}", line!(), err)
                    })
                    .unwrap();
            });
            tasks.push(th);
        }

        for th in tasks {
            let _ = th.join();
        }
    }
}
