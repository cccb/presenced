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

## MQTT

### Topics
#### `/presence/eta`

Message structure:

    {
        "name": "franka nord",
        "time": "1970-01-01T00:00:00.000Z",
        "note": "around 7ish, am hungry"
    }

Required: "name"
Optional: "time", "note"

#### `/presence/etd`
Message structure:

    {
        "name": "franka nord",
        "time": "1970-01-01T00:00:00.000Z"
    }

Required: "name"
Optional: "time"

#### `/presence/status`
One of:
    open | closed | thursday

#### `/presence/state`

Message structure:

    {
        "status": "open"|"closed"|"thursday",
        "people": [
            ["franka nord", "+23"],
            ["maunz", "ca. 19:30, hab hunger"],
            ["hans acker", null]
        ]
    }
