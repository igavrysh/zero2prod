# Commands used for app lifecycle

## API

### Healthcheck
```
curl http://127.0.0.1:8000/health_check
```

### Subscribe
```
curl \
    --request POST \
    --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
    127.0.0.1:8000/subscriptions --verbose
```

## To prepare sqlx to work in offline mode (static compilation)
It must be invoked as a cargo subcommand
All options after `--` are passed to cargo itself
We need to point it at our library since it contains 
all our SQL queries.
```
cargo sqlx prepare -- --lib
```

## Docker

### Build
To build docker image: build a docker image tagged as "zero2prod" according to teh recipe 
specified in `Dockerfile`

```
docker build --tag zero2prod --file Dockerfile .
```

### Run
```
docker run zero2prod
```

### Kill Running Image (with -p flag) to Launch It Again With Command
```
docker run -p 8000:8000 zero2prod
```

### Docker image size
```
docker images zero2prod
```

### To clean up Docker system
```
docker  system prune
```

### View Docker resources
```
docker container ls
docker image ls
docker volume ls
docker network ls
docker info
```

### View / Delete Docker images
```
docker images  or docker images -a

docker rmi <image id>

docker rmi <image id 1> <image id 2>

docker rmi $(docker images -q) 

# view dangling images
docker images -f dangling=true

# remove dangling images
docker image prune

# remove docker images with filter
docker image prune -a -a --filter "until=12h"

# ... with label
docker image prune --filter="label=unused"
```

### View / Delete Docker containers
```
# displays a list of active containers, including their IDs, names, and other details
docker container ls -a

# stop container
docker container stop [id]

docker container stop $(docker container ls -aq)

# Remove a stopped container
docker container rm [id]

# Remove stopped containers
docker container rm $(docker container ls -aq) 

# Remove one or more containers
docker rm ID_or_Name ID_or_Name 

# Remove all Docker containers
docker container stop $(docker container ls -aq) && docker system prune -af --volumes  

# Removing container with filters
docker container prune --filter="label=maintainer=john"  
```

### Removing Docker volumes
```
# To get a list of available Docker volumes, use the following command:
docker volume ls  

# Remove one or more specific volumes 
docker volume rm vol_name vol_name2  

# Remove dangling volumes 
docker volume ls -f dangling=true 

# With docker volume prune, you can get rid of dangling volumes.
docker volume prune
```

###  Removing Docker networks
```
# Removing a single network
docker network rm [networkID] 
```

### Check Docker image size
```
docker images zero2prod
```

## Digital Ocean

### Setup Token

Create token at https://cloud.digitalocean.com/account/api/tokens & enter it when prompted by command below:
```
doctl auth init
```

Check correct setup by:
```
doctl account get
```

### Create App Based on spec.yaml
> Auth github before: GitHub user not authenticated: 
Auth github via https://cloud.digitalocean.com/apps/github/install
by https://www.digitalocean.com/community/questions/how-to-properly-link-github-repositories-in-app-platform

```
doctl apps create --spec spec.yaml
```

### View app
```
doctl apps list
```

### Update spec and deploy
```
doctl apps update <app uuid> --spec spec.yaml
# e.g.
doctl apps update ee43fcea-9230-4fb5-88aa-0bac02066448 --spec spec.yaml
```