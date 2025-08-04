Exacto, esos tres pasos son justos y funcionan así:

1. **Construir la imagen**

   ```
   docker build -t cal-rust .
   ```
2. **Arrancar el servidor en un contenedor**, en background y publicando el puerto 9000 al host:

   ```
   docker run -d --name echo-server -p 9000:9000 cal-rust
   ```
3. **Ejecutar el cliente desde tu máquina** apuntando a localhost:9000:

   ```
   cargo run --bin client
   ```

