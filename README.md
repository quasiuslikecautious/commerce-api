<a name="readme-top"></a>


[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/quasiuslikecautious/commerce-api">
    <img src="images/quasius.dev.icon.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">commerce-api<h3>

  <p align="center">
	A RESTful API written in Rust (specifically using axum) to serve as the backend for a commerce web application. All data is stored in a PostgreSQL database and accessed through diesel. Supports custom session cookie based user auth. 
	<br />
	<a href="https://github.com/quasiuslikecautious/commerce-api">
	  <strong>Explore the docs</strong>
	</a>
	<br />
	<br />
	<a href="https://github.com/quasiuslikecautious/commerce-api">View Demo</a>
	.
	<a href="https://github.com/quasiuslikecautious/commerce-api/issues">Report Bug</a>
	.
	<a href="https://github.com/quasiuslikecautious/commerce-api/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

[![commerce-api Screen Shot][product-screenshot]](https://github.com/quasiuslikecautious/commerce-api)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

* [![Rust][Rust.rs]][Rust-url]
* [![Diesel][Diesel.rs]][Diesel-url]
* [![Axum][Axum.rs]][Axum-url]
* [![PostgreSQL][PostgreSQL.psql]][PostgreSQL-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started
To get a local copy up and running follow these simple steps.

### Prerequisites

To run this api, you will need to have cargo installed, and PostgreSQL setup
* Cargo <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html">installation docs</a>
* PostgreSQL <a href="https://www.postgresql.org/download/">download page</a>

After installing postgres, make sure you setup a database to be used with the api, e.g.

```sql
CREATE DATABASE commerce; -- Where commerce cand be any name you want
```

<br />
<strong>Side Note:</strong>
I also highly recommend <a href="https://crates.io/crates/cargo-watch">cargo-watch</a> for allowing live reloads on the server as changes are saved. If you do go this route, don't forget to ignore the log folder or cargo will just keep restarting the server!
<br />
<br />

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/quasiuslikecautious/commerce-api.git
   ```
1. Install cargo crates
   ```sh
   cargo build
   ```
1. Install the diesel CLI and initialize diesel in the project
   ```sh
   # run this command in the project root e.g. .../commerce-api/
   cargo install diesel_cli
   diesel setup
   ```

1. Setup your .env file with the database path and secrets
    ```sh
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
    echo JWT_SECRET=<Some Secret Value>
    echo SESSION_SECRET=<Some Secret Value>
    echo NONCE_SECRET=<Some Secret Value>
    ```

1. Initialize your database with the tables this project will use
    ``` sh
    diesel migration run
    ```

1. Finally, generate a certificate for the server to use for https. I won't get into the weeds on how to generate the certificate, but I highly recommend <a href="https://www.baeldung.com/openssl-self-signed-cert">this blog</a> if you need any help. The only requirements the server has on the certificate, is that they are stored in .../self_signed_certs/localhost.crt, and .../self_signed_certs/localhost.key path.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

To use this API, simply run 

```sh
cargo run # default run command
# OR
cargo watch -x run -i log # if you have cargo-watch installed and want live reloads
```

in the project's root, and the server will start up. 

By default, the server runs http redirection on port 7878, and the https api on port 8000, though this can be changed by specifying the <strong>`HTTP_PORT`</strong> variable for the http redirection port, and <strong>`HTTPS_PORT`</strong> variable for the https api port in the .env file.

Additionally, as this API employs logging, log files will be generated in the .../log path, storing up to 10 50kb log files with commerce.log being the most recent log file, and commerce10.log being the oldest. The log level (default is INFO) and other logging settings can be edited in .../logging_config.yaml

_Example Auth Flow_
```sh
    # start up server
    cargo run

    # to make requests to the server, as the certs will be self signed we
    # need to set the --insecure flag. Additionally, as session management
    # is cookie based, we must store and use cookies, hence the '-b' and 'c'
    # need to be set to the same file.

    curl --insecure https://127.0.0.1:8000/api/v1/auth/nonce \
        -c cookies.txt -b cookies.txt
    # user receives 401 status code and nonce in response, e.g.
    # 401 Unauthorized
    # { "nonce": <Some nonce> }

    curl --insecure https://127.0.0.1:8000/api/v1/auth/signin \
        -H 'Content-Type: application/json' \
        -d '{"email": "commerce-api@example.com", "password": "password", "nonce": <nonce returned from /nonce request>}' \
        -c cookies.txt -b cookies.txt
    # user session id is stored in db and user session cookie is now auth'd
```

_For more examples, please refer to the [Documentation](https://example.com) TODO will add link to API docs here_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] Add a README
  - [ ] Link API docs once created to usage section
- [ ] Add digest Authentication
  - [x] Use session-cookie based user auth instead of JWTs
    - [ ] Create protected tower layer for session guarding
  - [ ] Add nonce for authentication
    - [x] Fix custom implemented session store to save session if new
  - [ ] Add client nonce (?)
  - [ ] Set up extractors on routes for grabbing/guarding routes
- [ ] Add pagination for /items route and future multi item return routes
- [ ] Reuse JWT for external API authentication
- [ ] Encrypt data at rest
- [ ] Add db cleanup jobs for session based user auth
  - [ ] Add truncate table function
  - [ ] Determine how often/triggers for running cleanup jobs
- [ ] Add documentation to crate
- [ ] Add unit tests
- [ ] Add api documentation (openapi)
- [x] Move db ops out of main loop and into struct files

See the [open issues](https://github.com/quasiuslikecautious/commerce-api/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@zquasius](https://twitter.com/zquasius) - zach@quasius.dev@gmail.com

Project Link: [https://github.com/quasiuslikecautious/commerce-api](https://github.com/quasiuslikecautious/commerce-api)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/quasiuslikecautious/commerce-api.svg?style=for-the-badge
[contributors-url]: https://github.com/quasiuslikecautious/commerce-api/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/quasiuslikecautious/commerce-api.svg?style=for-the-badge
[forks-url]: https://github.com/quasiuslikecautious/commerce-api/network/members
[stars-shield]: https://img.shields.io/github/stars/quasiuslikecautious/commerce-api.svg?style=for-the-badge
[stars-url]: https://github.com/quasiuslikecautious/commerce-api/stargazers
[issues-shield]: https://img.shields.io/github/issues/quasiuslikecautious/commerce-api.svg?style=for-the-badge
[issues-url]: https://github.com/quasiuslikecautious/commerce-api/issues
[license-shield]: https://img.shields.io/github/license/quasiuslikecautious/commerce-api.svg?style=for-the-badge
[license-url]: https://github.com/quasiuslikecautious/commerce-api/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/zach-quasius-076740165
[product-screenshot]: images/screenshot.png
[Rust.rs]:  	https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Diesel.rs]: https://img.shields.io/badge/diesel-535379?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGNsYXNzPSJsb2dvIiB2ZXJzaW9uPSIxLjEiIHZpZXdCb3g9IjAgMCA4NSA3NiI+PGcgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSI+PHBhdGggZD0iTTgzLjg0IDU2Ljc2NWMwIDIuNDctMS45OTQgNC40NzMtNC40NTQgNC40NzMtMi40NjEgMC00LjQ1Ni0yLjAwMy00LjQ1Ni00LjQ3MyAwLTIuNDcgNC40NTYtNy4zNDQgNC40NTYtNy4zNDRzNC40NTQgNC44NzMgNC40NTQgNy4zNDQiIGZpbGw9IiNGREJENDEiLz48cGF0aCBkPSJNODMuODQgNTYuNzY1YzAgMi40Ny0xLjk5NCA0LjQ3My00LjQ1NCA0LjQ3My0yLjQ2MSAwLTQuNDU2LTIuMDAzLTQuNDU2LTQuNDczIDAtMi40NyA0LjQ1Ni03LjM0NCA0LjQ1Ni03LjM0NHM0LjQ1NCA0Ljg3MyA0LjQ1NCA3LjM0NFoiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLXdpZHRoPSIyLjIzNSIvPjxwYXRoIGQ9Im03OS40MDYgNDUuOTQ4Ljk0NS0uOTUzYTEuMDA1IDEuMDA1IDAgMCAwLS4wMDItMS40MTZsLTUuOTU1LTUuOTYxYS45NzYuOTc2IDAgMCAwLS43MDYtLjI3M2gtOS4zNzl2My4wMTVoOC4wMDVjLjI2NiAwIC41Mi4yNDguNzA4LjQzOGw0Ljk2OSA1LjA3OWMuMzkuMzkyIDEuMDI0LjQ2NCAxLjQxNS4wNyIgZmlsbD0iI0ZGRiIvPjxwYXRoIGQ9Im03OS40MDYgNDUuOTQ4Ljk0NS0uOTUzYTEuMDA1IDEuMDA1IDAgMCAwLS4wMDItMS40MTZsLTUuOTU1LTUuOTYxYS45NzYuOTc2IDAgMCAwLS43MDYtLjI3M2gtOS4zNzl2My4wMTVoOC4wMDVjLjI2NiAwIC41Mi4yNDguNzA4LjQzOGw0Ljk2OSA1LjA3OWMuMzkuMzkyIDEuMDI0LjQ2NCAxLjQxNS4wN1oiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLXdpZHRoPSIyIi8+PHBhdGggZD0ibTYzLjA4NSA0NC44OTUuMTQyLTEyLjEwNCAzLjAyOSAxLjVjLjI3LjE0Ny40MzcuNDMyLjQzNC43NGwtLjA2NSA3LjU3N2EuODM2LjgzNiAwIDAgMS0uNDQ3LjczM2wtMy4wOTMgMS41NTRaIiBmaWxsPSIjOUI5QjlCIi8+PHBhdGggZD0ibTYzLjA4NSA0NC44OTUuMTQyLTEyLjEwNCAzLjAyOSAxLjVjLjI3LjE0Ny40MzcuNDMyLjQzNC43NGwtLjA2NSA3LjU3N2EuODM2LjgzNiAwIDAgMS0uNDQ3LjczM2wtMy4wOTMgMS41NTRaIiBzdHJva2U9IiMwMDAiIHN0cm9rZS13aWR0aD0iMi4yMzUiLz48cGF0aCBkPSJNNTcuNDc0IDI2LjYyNWMtLjk3NS45OC0yLjgyOC45Ny0zLjgyLS4wMjdMMzguNDE4IDExLjMwMmMtLjk5Mi0uOTk2LS45MDUtMi43Ni4wNy0zLjc0bC4xNjgtLjE2OWMuOTc1LS45NzggMi41NjctLjkgMy41Ni4wOTdMNTcuNDUgMjIuNzg2Yy45OTMuOTk2IDEuMTY4IDIuNjkuMTkzIDMuNjdsLS4xNjkuMTdabTUuNjMgMTkuODI3LjIwNi0yMi43NTZjLS4wMDItLjc0My0uMjczLTEuNDk3LS43OTYtMi4wMjJMNDMuODggMi45NjRjLTEuOTgzLTEuOTktNS4yODEtMS45NDQtNy4yNjQuMDQ2TDIuNDg3IDM3LjI3NmMtMS45ODMgMS45OS0xLjc2NyA1LjAzOS4yMTcgNy4wM2wyOC4yNDQgMjguMzU3YzEuOTgyIDEuOTkgNS4wMzUgMi4yMjYgNy4wMTguMjM2bDI0LjMzNy0yNC40MzZjLjUyNi0uNTI4LjgwMy0xLjI2NC44MDEtMi4wMTFaIiBmaWxsPSIjRkYyNzI3Ii8+PHBhdGggZD0ibTYyLjk0NSA0NS42OTguMDUyLTYuNDY2LTI2LjYzNSAyNi43ODVhMy4wNDIgMy4wNDIgMCAwIDEtNC4zMiAwTDIuNDg1IDM2LjI5MmwtLjIzNS4yMzZjLTIuMDExIDIuMDIyLTIuMDExIDUuMzMxIDAgNy4zNTNsMjguMTggMjguMzRhNS4xNjMgNS4xNjMgMCAwIDAgNy4zMSAwbDI0LjM4NS0yNC41MjNjLjUyNy0uNTMuODIzLTEuMjUuODIxLTIiIGZpbGw9IiM5QjAwMDAiLz48cGF0aCBkPSJNNTcuNTkzIDI2Ljc0NWMtLjk3NS45NzgtMi45NDguODUtMy45NC0uMTQ3TDM4LjQxOSAxMS4zMDJjLS45OTMtLjk5Ny0uNzg2LTIuNjQxLjE4OC0zLjYybC4xNjktLjE3Yy45NzUtLjk3OCAyLjQ0Ny0xLjAxOSAzLjQ0LS4wMjJMNTcuNDUgMjIuNzg1Yy45OTMuOTk2IDEuMjg3IDIuODEuMzEyIDMuNzlsLS4xNy4xN1ptNS42NTctMi45OWMtLjAwMi0uNzQyLS4zMzItMS40MzctLjg1NS0xLjk2Mkw0My43NiAzLjA4M2MtMS45ODMtMS45OS01LjA0My0xLjk0NC03LjAyNi4wNDZMMi42MDYgMzcuMzk1Yy0xLjk4NCAxLjk5MS0yLjAwNCA1LjAzOC0uMDIxIDcuMDNsMjguMjQzIDI4LjM1OGMxLjk4MyAxLjk5IDUuMTU1IDIuMTA4IDcuMTM4LjExN2wyNC4zMzctMjQuNDM2Yy41MjctLjUyOC44MDMtMS4yNjQuODAxLTIuMDExbC4xNDYtMjIuNjk3WiIgc3Ryb2tlPSIjMDAwIiBzdHJva2Utd2lkdGg9IjIuMjM1Ii8+PHBhdGggZD0iTTMxLjUxIDUwLjEzNmMuMjMyLjIzNC40NjIuMzc0LjY4OC40Mi4yMjcuMDQ1LjQ0OC4wMzkuNjYxLS4wMi4yMTQtLjA1OC40MTUtLjE2My42MDMtLjMxMi4xODctLjE1LjM1My0uMjk2LjQ5NC0uNDM5bDMuMTA5LTMuMTJjLjQxNC0uNDE3LjgyOS0uNjg0IDEuMjQ0LS44YTMuMDY4IDMuMDY4IDAgMCAxIDEuMTY1LS4xMThjLjM2Mi4wNC42OS4xMzQuOTgxLjI4NC4yOTEuMTUuNTE1LjI4OS42Ny40MmwuMDU5LS4wNmE0LjE1NSA0LjE1NSAwIDAgMS0uNDI3LS42NjMgMi41NzUgMi41NzUgMCAwIDEtLjI5Mi0uOTc1Yy0uMDQtLjM2NC4wMTMtLjc2Ny4xNTUtMS4yMS4xNDItLjQ0Mi40NTQtLjkwNC45MzItMS4zODRsMi45NTQtMi45NjVhNi4yIDYuMiAwIDAgMCAuNDU3LS41MTdjLjE0OC0uMTg5LjI1Mi0uMzkuMzEtLjYwNWExLjQzIDEuNDMgMCAwIDAgLjAyLS42NjNjLS4wNDYtLjIyNy0uMTg1LS40NTgtLjQxOC0uNjkybC0xLjA0OS0xLjA1NCAyLjE3Ni0yLjE4NCAyLjQ2NiAyLjQ3N2MuMjIxLjIyMS40MTUuNTI3LjU4NC45MTcuMTY4LjM5LjI2NS44MTIuMjkgMS4yNjhhMy45NCAzLjk0IDAgMCAxLS4yMDMgMS40MzNjLS4xNjIuNS0uNDYzLjk3Mi0uOTAzIDEuNDE0bC0zLjY1MyAzLjY2OGMtLjMyNC4zMjQtLjUzNS42NjYtLjYzIDEuMDIzYTIuNDc2IDIuNDc2IDAgMCAwIC4yMTMgMS44OTNjLjE0OC4yNjYuMy40NzcuNDU2LjYzM2wtMS42OSAxLjY5N2EyLjg5NyAyLjg5NyAwIDAgMC0uNjMxLS40NTggMi44MTQgMi44MTQgMCAwIDAtLjg1NS0uMzEyIDIuMjkgMi4yOSAwIDAgMC0uOTYyLjAzYy0uMzM3LjA3Ny0uNjU0LjI2NS0uOTUyLjU2NGwtMy43NjkgMy43ODVjLS40NC40NDEtLjkxLjc0NC0xLjQwOC45MDYtLjUuMTYzLS45NzUuMjMxLTEuNDI4LjIwNWEzLjY1OCAzLjY1OCAwIDAgMS0xLjI2Mi0uMjkyYy0uMzktLjE3LS42OTMtLjM2NC0uOTE0LS41ODZsLTIuNDY3LTIuNDc2IDIuMTc2LTIuMTg1IDEuMDUgMS4wNTNabTQuNy0yMi4xMmMtLjIzNC0uMjM0LS40NjQtLjM3NC0uNjktLjQxOWExLjQwNCAxLjQwNCAwIDAgMC0uNjYuMDIgMS42NTIgMS42NTIgMCAwIDAtLjYwMy4zMTJjLS4xODguMTUtLjM2LjMwMi0uNTE1LjQ1OGwtMi45NTMgMi45NjVjLS40NzkuNDgtLjk0Ljc5My0xLjM3OS45MzUtLjQ0LjE0NC0uODQyLjE5Ni0xLjIwNS4xNTdhMi41NTQgMi41NTQgMCAwIDEtLjk3LS4yOTIgNC4xMTQgNC4xMTQgMCAwIDEtLjY2MS0uNDNsLS4wNTkuMDU5Yy4xMy4xNTYuMjY4LjM4LjQxOC42NzMuMTQ4LjI5Mi4yNDIuNjIuMjgxLjk4NC4wNC4zNjUgMCAuNzU1LS4xMTYgMS4xNy0uMTE3LjQxNy0uMzgyLjgzMy0uNzk3IDEuMjVsLTMuMTA4IDMuMTJhNS43NjIgNS43NjIgMCAwIDAtLjQzNy40OTcgMS42NiAxLjY2IDAgMCAwLS4zMS42MDUgMS40MiAxLjQyIDAgMCAwLS4wMi42NjRjLjA0NS4yMjcuMTg0LjQ1OC40MTguNjkybDEuMDQ5IDEuMDUzLTIuMTc3IDIuMTg1LTIuNDY2LTIuNDc3Yy0uMjIxLS4yMjItLjQxNS0uNTI4LS41ODQtLjkxN2EzLjY5NyAzLjY5NyAwIDAgMS0uMjktMS4yNjggMy45MjQgMy45MjQgMCAwIDEgLjIwMy0xLjQzNGMuMTYyLS41LjQ2NC0uOTcyLjkwMy0xLjQxNGwzLjc3LTMuNzg0Yy4yOTgtLjMuNDg0LS42MTguNTYzLS45NTZhMi4zMyAyLjMzIDAgMCAwIC4wMjktLjk2NiAyLjg0OCAyLjg0OCAwIDAgMC0uMzExLS44NTcgMi44MiAyLjgyIDAgMCAwLS40MzctLjYxNWwxLjY5LTEuNjk3Yy4xNDMuMTQzLjM0Ni4yOS42MTIuNDRhMi40NDYgMi40NDYgMCAwIDAgMS44ODUuMjE0Yy4zNTUtLjA5OC42OTUtLjMwOSAxLjAxOS0uNjM0bDMuNjUzLTMuNjY3Yy40NC0uNDQzLjkxLS43NDUgMS40MDgtLjkwN2EzLjg4IDMuODggMCAwIDEgMS40MjgtLjIwNSAzLjY2IDMuNjYgMCAwIDEgMS4yNjIuMjkzYy4zODkuMTcuNjkzLjM2NC45MTQuNTg1bDIuNDY3IDIuNDc3LTIuMTc2IDIuMTg0LTEuMDQ4LTEuMDUzWiIgZmlsbD0iIzlCMDAwMCIvPjwvZz48L3N2Zz4=
[Diesel-url]: https://diesel.rs/
[Axum.rs]: https://img.shields.io/badge/axum-000000?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAD8AAAA4AQMAAABwlLIkAAAABlBMVEX///8AAABVwtN+AAAACXBIWXMAAA7EAAAOxAGVKw4bAAAA4ElEQVQYlVXRPYrDMBAFYAXDupQPsGD2HgFdacsUxpNULn2lQC4SxxdwOhcPTaSnH2JVH/phNG+MelHd1RuFC5AA36tucUcD3oSFLMQMWYkJ8iAekCkhnBILxBJvSE9sGBN2DC5hGITA+UKs/nzRmfj9Jxbt/oiXM6c9Avba8GO+bW53Pr/2W0M83d6mWoJcS5Brqc/AGBEWRpNWAo8y6uXyfCsIJeyhqP9p7zdGZ83pKd9fffmuA7uofcVO59x7CWF0x3xqYjXD9Sve6Zj8VGZRp9NDOK86QV9mGocLQogPHV1XySZN2nYAAAAASUVORK5CYII=
[Axum-url]: https://github.com/tokio-rs/axum
[PostgreSQL.psql]:  	https://img.shields.io/badge/PostgreSQL-FFFFFF?style=for-the-badge&logo=postgresql&logoColor=blue
[PostgreSQL-url]: https://www.postgresql.org/
