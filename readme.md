Usage
-----

```plain
    POST    / 

        accepts raw data in the body of the request and responds with a URL
        of a page containing the body's content

    GET     /<id>

        retrieves the content for the paste with id `<id>`

    GET     /p/<id>

        retrieves the HTML page with syntax-highlighted content for the paste with id `<id>`
```

Examples
--------

```plain
    Paste a file named 'file.txt' using cURL:

        curl --data-binary @file.txt https://bin.wantguns.dev

    Paste from stdin using cURL:

        echo "Hello, world." | curl --data-binary @- https://bin.wantguns.dev

    Add this to your .zshrc to implement a quicker usage.

        function paste() {
          local file=${1:-/dev/stdin}
          curl --data-binary @${file} https://bin.wantguns.dev
        }

    If the uploaded data binary is parsed as "text/*", then the paste will be syntax
    highlighted
```

Deployment
----------

### Traefik

```yaml
## (... Traefik service configuration ...) ##

   pastebin:
     image: wantguns/bin
     restart: always
     container_name: pastebin
     ports:
       - 127.0.0.1:6162:6162
     environment:
       - ROCKET_PORT=6162
       - THEME=
       - HOST_URL=${BIN_DOMAIN}
     volumes:
       - /path/to/local/upload:/app/upload
     labels:
       - "traefik.enable=true"
       - "traefik.http.routers.pastebin.rule=Host(`${BIN_DOMAIN}`)"
       - "traefik.http.routers.pastebin.entrypoints=secure"
       - "traefik.http.routers.pastebin.tls.certresolver=le"
       - "traefik.http.services.pastebin.loadbalancer.server.port=6162"
     networks:
       - bin_network

networks:
  - bin_network
 ```

