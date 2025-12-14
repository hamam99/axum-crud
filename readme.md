
# Simple Axum CRUD

## Libraries
```
axum = "0.8.7"
chrono = { version = "0.4.42", features = ["serde"] }
dotenvy = "0.15.7"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
sqlx = { version = "0.8.6", features = ["runtime-async-std-native-tls", "mysql", "chrono", "uuid", "postgres","runtime-tokio-native-tls","macros"] }
tokio = { version = "1.48.0", features = ["full"] }
tower-http = { version = "0.6.8", features = ["cors"] }
uuid = { version = "1.19.0", features = ["serde", "v4"] }
```


## Api URL
### Get All User
```
curl --location 'http://localhost:3000/axum-crud/users
```

### Create New User
```
curl --location 'http://localhost:3000/axum-crud/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username":"test",
    "email":"test1@gmail.com"
}'
```