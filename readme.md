
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

### Get Current User
```
curl --location 'http://localhost:3000/axum-crud/users/11517aca-d487-424c-9ea7-187155261481' \
--header 'Content-Type: application/json'
```

### Update User
```
curl --location --request GET 'http://localhost:3000/axum-crud/users/11517aca-d487-424c-9ea7-187155261481' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "hamam Test",
    "email": "hamam2@gmail.com"
}'
```


##Database Structure 
```
Table Name : users

Column : 
- id : uuid
- username: varchar
- email : varchar
```

