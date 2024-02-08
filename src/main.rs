use ring::digest::{Context, SHA256};
use std::io::Write;
use std::{fs,fs::File, io};


fn main() {
    encrypt_pwd("123", 2);
}

fn save_pwd(name: &str, data:&str) -> std::io::Result<()>{
    let path = format!("./Rust/examples/password-safe/src/out/{}", name);
    let mut file = File::create(path)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}

fn del_pwd(name:&str) -> std::io::Result<()>{
    let path = format!("./Rust/examples/password-safe/src/out/{}", name);
    let _ = fs::remove_file(path)?;

    Ok(())
}

fn encrypt_pwd(data: &str, iter: i32) {
    let pwd = data.to_owned();
    let mut hasher = Context::new(&SHA256);

    let _ = hasher.update(pwd.as_bytes());
    let mut pwd_crypted = hasher.finish();

    for _ in 1..iter {
        hasher = Context::new(&SHA256);
        let _ = hasher.update(pwd_crypted.as_ref());
        pwd_crypted = hasher.finish();
    }

    println!("{:?}", pwd_crypted);
}
