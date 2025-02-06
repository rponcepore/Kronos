.PHONY: all begin check build test report clean

all: begin front back report

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

report:
	@echo ""
	@echo "make complete. Frontend and backend are rebuilt."
	@echo ""

clean:
	@echo "make clean no implemented"