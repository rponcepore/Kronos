.PHONY: all begin test lint run report clean

all: begin test lint report

# Variables
PROJECT_NAME := $(shell basename $(PWD))

begin:
	@echo "Executing make for $(PROJECT_NAME)"

test:
	@npm test

lint:
	@npm run lint

report:
	@echo ""
	@echo "make complete."
	@echo ""

clean:
	@echo "make clean no implemented"