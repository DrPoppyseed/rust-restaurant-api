# Run cargo tests for api code.
test:
	make clean
	docker-compose -f docker-compose.test.yml build
	docker-compose -f docker-compose.test.yml up 

start:
	make clean
	docker-compose -f docker-compose.dev.yml up -V

# Start dev environment.
start-dev:
	make clean
	docker-compose -f docker-compose.dev.yml run api bash

# Stop and remove all docker containers with the delimiter "restaurant-".
clean:
	docker ps -aqf "name=^restaurant-" | \
	xargs docker stop | \
	xargs docker rm \

# Copies .env.backup.example file contents to .env.backup, which will also be created.
# You can alter the script to the following to override the file.
# ```bash
# cat .env.example > .env
# ```
setup_env:
	cat .env.example >> .env
