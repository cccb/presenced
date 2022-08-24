# presenced

track presence of people and club status (open, closed) using
MQTT.

## Configuration

All configuration is done in the environment

    PRESENCED_MQTT_URL=mqtt://localhost:1883


## Building and running

    export PRESENCED_MQTT_URL=mqtt://localhost:1883

    cargo build

    cargo run
