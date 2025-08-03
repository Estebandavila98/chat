use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    let mut reader = BufReader::new(stream.try_clone()?);

    println!("Cliente conectado desde {}", peer);

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;

        if bytes == 0 {
            println!("El cliente {} se desconectó", peer);
            break;
        }

        let msg = line.trim_end();
        println!("Recibido de {}: {}", peer, msg);

        let response = format!("Echo: {}\n", msg);
        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    println!("Servidor escuchando en 0.0.0.0:9000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error en hilo de cliente: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Error aceptando conexión: {}", e),
        }
    }

    Ok(())
}
