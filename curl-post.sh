# to test subscriptions
curl -X POST http://localhost:8000/subscriptions -H "Content-Type: application/x-www-form-urlencoded" -d "name=ievgen123&email=ievgen_mail123"

# to run log level 
RUST_LOG=info cargo run

# to run trace level
RUST_LOG=trace cargo run
