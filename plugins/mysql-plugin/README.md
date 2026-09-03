# podman 
run mysql in a container

podman run -d \
  --name mysql \
  -p 3306:3306 \
  -e MYSQL_ROOT_PASSWORD=replace_me \
  docker.io/library/mysql:latest

# running mysql in a container with a persistent volume

podman volume create mysql-data

podman run -d \
  --name mysql \
  -p 3306:3306 \
  -e MYSQL_ROOT_PASSWORD=replace_me \
  -e MYSQL_DATABASE=eas \
  -e MYSQL_USER=app_user \
  -e MYSQL_PASSWORD=replace_me_also_too \
  -v mysql-data:/var/lib/mysql:Z \
  docker.io/library/mysql:latest
