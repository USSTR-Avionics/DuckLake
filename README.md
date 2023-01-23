# DuckLake
Full fledged admin dashboard for the rocket

# Tech Stack
- Svelte
- Python
- Rust
- C/C++

# Frontend

The GUI side of things, locally hosted reactive javascript

# Backend

## collector

Python framework to collect data from the USB Serial that the arduino transmits, and stores them
in a database

## emitter

Rust REST server that gets data from the database and emits to the frontend

# arduinoRX

An arduino receiver that receives radio signals from the rocket and transmits them over USB Serial
