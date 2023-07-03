# web-site-http-proxy

Example of docker-compose.yaml to run:

```yaml
version: '2'
services:
  yft-web-site:
    image: myjettools/web-site-http-proxy:0.1.0
    container_name: yft-web-site
    environment:
      - DEBUG=1
    volumes:
      - ./.web-site-http-proxy:/root/.web-site-http-proxy
    ports:
      - "6565:8000"
    mem_limit: 2048Mb
    mem_reservation: 10Mb
    cpu_percent: 50
    logging:
      options:
        max-size: 10m

networks:
  default:
    external:
      name: docker_net
```


With the file **.web-site-http-proxy**
```yaml
cache_seconds: 300
base_url: https://raw.githubusercontent.com/your-fintech/web-site/main
root_page: index.html
```
