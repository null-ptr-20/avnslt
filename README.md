# AVNSLT

> [!WARNING]
> (Still in developement) This project is far from finished

## What is Avnslt ?

- Very
- Nice
- Simple
- Little
- Text-Editor

## Why Rust ?

- For speed and safety
- Because I'm learning it
- Because its hip

## Current state:

### Idea:

- Create a text editor in rust

  - Build basic functionalities read, write, save.
  - Saving to a default file extension if not specified.

- Incorporate motions like vim but with a twist!

  - Instead of hjkl -> wasd like in FPS
  - KIV..

## Roadmap

### Starting stages

- [x] Create a REPL like interface
- [x] User is prompted to enter title, date and body
- [x] When user saves, it saves to a specified file format (.txt)
- [x] Last save flag in the file is marked (e.g --Ver 1.0) [a flag of --EO(1)]
- [ ] When user open files, it all fields are editable
- [ ] Edit saved flag to iterate based on how many times its saved (e.g --EO(1) first save, --EO(2) second save)

### Next steps

- [ ] Create terminal alternate window, raw mode, undo, redo..
- [ ] Data structure to explore: Piece Table

## Installation

> [!NOTE]
> This is just cloning the github repo to your local machine.

1. Clone the repository into your local machine:

`git clone https://github.com/null-ptr-20/avnslt.git`

2. Navigate into the rs_avnslt folder:

`cd ~/avnslt/rs_avnslt`

3. Cargo run:

`cargo run`
