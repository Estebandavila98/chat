use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

fn main() -> io::Result<()> {
    // Conectar al servidor
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    // Clonar el socket para lectura independiente
    let mut reader = BufReader::new(stream.try_clone()?);

    let stdin = io::stdin();
    let mut input = String::new();

    println!("Cliente conectado. Escribe 'exit' o presiona Ctrl+D para salir.");

    loop {
        input.clear();
        // Mostrar prompt
        print!("> ");
        io::stdout().flush()?;

        // Leer línea del usuario (EOF o Ctrl+D termina el bucle)
        if stdin.read_line(&mut input)? == 0 {
            println!("\nTerminando cliente por EOF.");
            break;
        }

        let msg = input.trim_end();
        if msg.eq_ignore_ascii_case("exit") {
            println!("Terminando cliente por comando 'exit'.");
            break;
        }

        // Enviar mensaje al servidor
        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?; // si tu protocolo lo requiere

        // Leer respuesta del servidor (hasta salto de línea)
        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("Servidor: {}", response.trim_end());
    }

    Ok(())
}
