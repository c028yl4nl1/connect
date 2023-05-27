// Um codigo em rust pra RECEBER ARGUMENTOS

// vamos ter que usar a lib env

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
fn convert_int(port: String) -> i32 {
    let port = port.parse().unwrap();

    return port;
}

fn main() {
    println!("Sejam muito bem vindo ao Dos em Rust"); // Não é um dos , Eu estavo criando so que dasanimei
    let args: Vec<String> = env::args().collect(); // primeiro eu recebir o valor no argumento depois coletei  e transformei em um Vec.

    let host: &str = &args[1];
    let referecia_da_porta = &args[2];
    let port: i32 = convert_int(referecia_da_porta.to_string());

    conect_host(host, port);
}

fn conect_host(host: &str, port: i32) {
    let mut buffer = [0; 1024]; // receber bytes

    let values = format!("{host}:{port}"); // format String or Values
    let mut conect = TcpStream::connect(values).expect("Não foi possivel se conectar"); //  Connect
    let string_send = "GET / HTTP/1.1\r\n\r\n".as_bytes();

    conect.write(string_send);
    conect.read(&mut buffer).unwrap();

    let resposta_do_servidor = String::from_utf8_lossy(&buffer).into_owned();

    println!("{}", resposta_do_servidor);
}
