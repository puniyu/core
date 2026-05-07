set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

test:
    just crates/puniyu_macros/
    cargo test

    
