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

2. **Arranca el server** en esa red, nombrándolo “echo-server”:

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

ADICIONAL:

## otra Opción: Conectar vía Host-Network (no usar) (descomentar la ip de client.rs y comentar la otra)

Si arrancas el contenedor del client en modo **host** (sólo Linux), éste comparte la pila de red del host:
# 1) Arranca el server publicando puerto
```
docker run -d --name echo-server -p 9000:9000 chat-rust
```
# 2) Arranca el client usando la network del host
```
docker run -it --rm --network host chat-rust ./client
```

Dentro del contenedor, `127.0.0.1:9000` ahora sí conecta al server que escucha en tu máquina (porque es “la misma” red).

> **Contras**: no funciona en Docker Desktop Mac/Win, y pierde aislamiento de red.
