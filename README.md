# Rust Name Wish

Name Wishing Website Built using Rust - Send warm and personalized greeting wishes to your friends and family members. Enter your name to receive a custom greeting wishes for the festive season.  

## Requirements

- Rust: <https://www.rust-lang.org/>  
- Base Concept: <https://github.com/mskian/greeting-wishes/tree/termux>  

## Installation

- Download or clone the repo

```sh
git clone https://github.com/sanwebinfo/rust-name-wish.git
cd rust-name-wish
```

- Test the site

```sh
cargo run

## Home Page
http://localhost:6022/

## Wish Page
http://localhost:6022/wish?name=your-name
```

- Production build and usage

```sh
cargo build --release
```

```sh
rust-name-wish/
├── start.sh          # Shell script to start the Rust project
├── static/           # Folder for static files (CSS, JS, images, etc.)
├── templates/        # Folder for HTML or other template files
└── rust-name-wish    # Production Build
```

- For Server use Update bind IP to ```0.0.0.0```

```rs
.bind("0.0.0.0:6022")?
```

## LICENSE

MIT
