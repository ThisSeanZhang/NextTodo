# NextTodo

## Usage
### Docker
```shell
docker run -d --name nexttodo \
-e HOST=0.0.0.0 \
-e PORT=52525 \
-e WEBROOT=/static \
-p 52525:52525 \
thisseanzhang/nexttodo:0.1.0
```

