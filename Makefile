.PHONY: all begin check build test report clean test

all: begin front back test report

dev: front back docker_dev

prod: front back docker_prod

# Variables
PROJECT_NAME := $(shell basename $(PWD))

begin: 
	@echo "Executing make for $(PROJECT_NAME)"

back:
	@echo "Executing make for the backend"
	$(MAKE) -C backend_kronos

front:
	@echo "Executing make for the frontend"
	$(MAKE) -C frontend_kronos

test:
	cd integration_tests_kronos && ./basic_tests.sh

report:
	@echo ""
	@echo "make complete. Frontend and backend are rebuilt."
	@echo ""

docker_prod:
	@echo docker-compose build

clean:
	@echo "make clean no implemented"