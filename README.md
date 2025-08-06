las dos opciones requieren el siguiente paso:

1. **Construir la imagen**

   ```
   docker build -t chat-rust .
   ```


## Opción recomendada: DNS + red customizada 

0. notar client.rs ya està modificado para que conecte a `echo-server:9000` en lugar de `127.0.0.1:9000`:

   ```rust
   let mut stream = TcpStream::connect("echo-server:9000")?;
   ```


1. **Crea una red bridge custom** (permitirá resolución de nombres de contenedor):

   ```
   docker network create echo-net
   ```

2. **Arranca el server** en esa red, nombrándolo “echo-server”: // reemplaza -d por -t para ver su terminal

   ```
   docker run -d --name echo-server --network echo-net -p 9000:9000 chat-rust
   ```

3. **Arranca el client** en la misma red:

   ```
   docker run -it --rm --network echo-net chat-rust ./client
   ```

Al usar `--network echo-net`, Docker inyecta un DNS interno donde “echo-server” resuelve a la IP del contenedor server. Así el client puede ver y hablar al server sin exponer puertos al host si no quieres.

full docker prune:
```
docker stop $(docker ps -q)
docker rm -f $(docker ps -aq)
docker system prune -a --volumes -f
```



### automatizaciòn con el docker-compose.yml //use profile en client en el .yml


* `docker compose up` → arranca sólo **server**. (buildea y levanta client)
* (`docker compose up --profile client` → arranca ambos.) (no usar)
* `docker compose run client` → arranca sólo **client**, interactivo. (abri otra terminal y corre)
//"docker compose down" en otra terminal para bajar client

