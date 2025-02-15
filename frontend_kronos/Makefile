.PHONY: all begin test lint build report clean

all: begin test lint build report

# Variables
PROJECT_NAME := $(shell basename $(PWD))

begin:
	@echo "Executing make for $(PROJECT_NAME)"

test:
	@npm test

lint:
	@npm run lint

# Build will output files to default vite build dir, in this case, frontend_kronos/dist
build: 
	@npm run build 

report:
	@echo ""
	@echo "make complete."
	@echo ""

clean:
	@echo "make clean no implemented"