# World Athletics Points Calculator

A web application for calculating World Athletics performance points based on official scoring tables. The calculator supports both time-based events (track, road running, race walking) and distance-based field events.

## Features

- **Flexible Performance Input**:
  - For time-based events: Enter times in various formats (seconds: `10.50`, minutes:seconds: `1:30.25`, hours:minutes:seconds: `2:15:30.50`)
  - For field events: Enter distances/heights in meters (e.g., `8.95` for long jump)
- **Wind Adjustments**: Automatic wind speed adjustments for applicable events
- **Elevation Adjustments**: Net downhill adjustments for road running events
- **Placement Scoring**: Calculate points based on competition placement and category
- **Comprehensive Event Support**: Track & field, combined events, road running, race walking, and cross country

## Performance Input Formats

### Time-Based Events (Track, Road, Race Walking)
- **Seconds only**: `10.50`, `9.58`
- **Minutes:Seconds**: `1:30.25`, `3:45.67`
- **Hours:Minutes:Seconds**: `2:15:30.50`, `1:00:00.00`

### Distance-Based Events (Field Events)
- **Meters**: `8.95` (long jump), `2.30` (high jump), `20.50` (shot put)

## Pre-Requisites

You can add the `wasm` compilation target to rust using

```sh
rustup target add wasm32-unknown-unknown
```

In order to use tailwind, you will need to run

```sh
npm install -D tailwindcss
```

## Developing your Leptos CSR project

To develop your Leptos CSR project, running

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.

## Deploying your Leptos CSR project

To build a Leptos CSR app for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].

[Leptos]: https://github.com/leptos-rs/leptos
[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/
[deploy-csr]: https://book.leptos.dev/deployment/csr.html


## TODO
* Add missing events to the coefficient list (300mH, ...)
* Add an upper limit to events in order to avoid crazy scores.
* Add a lower limit to events in order to avoid negative scores.
