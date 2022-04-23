# run cargo tests for api code.
test:
	docker-compose -f docker-compose.test.yml build && docker-compose -f docker-compose.test.yml up

# serve development environment
start:
	docker-compose -f docker-compose.dev.yml up
