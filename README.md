# api-service-rs

a simple api template using, [actix-web](https://actix.rs/), [postgres](https://www.postgresql.org/) and [sqlx](https://github.com/launchbadge/sqlx). <br>

----wip---- <br>


## project structure

```bash
├── migrations # example of sqlx migration files. 
│   ├── 20240118162858_setup.down.sql
│   └── 20240118162858_setup.up.sql
├── src
│   ├── controllers 
│   │   ├── create_trainer.rs
│   │   ├── health.rs
│   │   └── mod.rs
│   ├── lib.rs # expose modules for e2e tests
│   ├── main.rs
│   ├── models
│   │   ├── mod.rs
│   │   └── trainer.rs
│   ├── routes.rs
│   └── server.rs
└── tests # e2e tests
    ├── create_trainer_test.rs
    ├── health_test.rs
    └── utils.rs
```
migration folder can be removed and  it will be generated using the `Makefile` or `sqlx migrate add -r <new-migration>`


## requirements

- rust installed locally, [rustup](https://rustup.rs/). Later we can use docker to build the project.
- docker and docker-compose, [docker](https://docs.docker.com/get-docker/)

## getting started

- clone the repo, or `Use this template`.
- use the `Makefile` to setup the project: `make init`: this will install the dependencies and setup the database and the pgAdmin. (check docker-compose.yml for more details)

- setup the `pgAdmin` check `docker-compose.yml` for the credetials.

## run the api server
- `make run` will run it using cargo watch for development.

- `make test` for running the tests.

***Note***: check the `Makefile` for more commands.

## todo

- better env variables management
- github actions for CI/CD
- add authentication
- add pagination
- maybe add redis
- dockerize 


## contribution
if you have any suggestions or improvements, feel free to open an issue or a PR :metal:.