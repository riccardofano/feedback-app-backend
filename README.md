# Feedback App backend

Backend for my version of the [Frontend Mentor Product feedback app](https://www.frontendmentor.io/challenges/product-feedback-app-wbvUYqjR6) challenge.

[Live demo](https://rf-feedback-app.netlify.app/)  
[Frontend repo](https://github.com/riccardofano/feedback-app-frontend)

Built in Rust with Axum and Shuttle-rs.

## Usage

Install [cargo-shuttle](https://github.com/shuttle-hq/shuttle) using cargo.

```
$ cargo install cargo-shuttle
```

Running the development server with shuttle:

```
$ docker docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 --name postgres postgres
$ cargo shuttle run
```

Running tests:

```
$ docker docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 --name postgres postgres
$ SQLX_OFFLINE=true cargo t
```

Deploying to shuttle:  
_Make sure to add `--no-test` otherwise the tests will fail on the production server and the deployment will crash_

```
$ cargo shuttle login
$ cargo shuttle deploy --no-test
```
