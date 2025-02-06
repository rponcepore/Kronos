# Kronos


# Summary
Kronos is a task management system designed to reduce administrative load on the operations staff, primarily in a garrison environemnt, but with extensibility to the tactical environment after proof-of-concept. Kronos provides a centralized Operations Management System that streamlines tasking, planning, and time/manpower allocation for Army units, Brigade to Company level. The system is designed to improve efficiency, reduce over-tasking, enhance situational awareness , and provide better data-driven decision-making for leaders.

# Collaborators
- Remington Ponce-Pore
- Olivia Beattie
- Jacob Horton

# Configuration
This repo employs multiple makefiles based on what you are trying to do. Run "make <args>" to acocmplish the following:

- `front` will rebuild the front
- `back` will check, build, test, and check vulnerabilities through cargo
- `make` will conduct a full project rebuild, and is equivalent to running `make front` and `make back`
- `clean` will remove any generated binaries

# Testing
To launch a test-only instance of the frontend without backend DB support, run:
```
    # in frontend_kronos directory
    make
    npm run dev
```
To launch a backend instance, run:
```
    # in backend_kronos directory
    make
    cargo run
```

# Prototype
A (mostly functional) prototype that we will emulate (partially) is packaged as a linux container on docker registry. Pull and run the prototype with:
```
    docker pull rponcepore/kronos_prototype:kronos_prototype
    docker run -d -p 8000:8000 rponcepore/kronos_prototype:kronos_prototype
```
Then, navigate to:
```
    http://localhost:8000/tasks/plans