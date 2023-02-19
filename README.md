<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

English | [简体中文](./README-zh_CN.md)

<br />
<div align="center">
  <a href="https://github.com/newbee-ltd/newbee-mall-api">
    <img src="docs/images/logo.png" alt="Logo" width="120" height="120">
  </a>

<h3 align="center">newbee-mall-api-rs</h3>

<p align="center">
    Rust version of the back-end API for NewBee Mall <a href="https://github.com/newbee-ltd/newbee-mall-api">newbee-mall-api</a>.
    <br />
    <br />
    -
    <a href="https://github.com/maiqingqiang/newbee-mall-api-rs/issues">Issues</a>
    ·
    <a href="https://github.com/maiqingqiang/newbee-mall-api-rs/discussions">Discussions</a>
    -
  </p>
</div>

## About

This project is the Rust version of the back-end API for NewBee Mall [newbee-mall-api](https://github.com/newbee-ltd/newbee-mall-api),
which is based on the original version's data structures and most of its logic.

### Built With

- [Rust](https://www.rust-lang.org/)
- [Diesel](https://diesel.rs/)
- [Actix](https://actix.rs/)

#### Dependencies on Front-end Projects

- [NewBee Mall Vue2 Version newbee-mall-vue-app](https://github.com/newbee-ltd/newbee-mall-vue-app)
- [NewBee Mall Vue3 Version newbee-mall-vue3-app](https://github.com/newbee-ltd/newbee-mall-vue3-app)
- [NewBee Mall Back-end Management System Vue3 Version vue3-admin](https://github.com/newbee-ltd/vue3-admin)

## Getting Started

### 1. Configure the Project

```shell
# Clone the project
git clone https://github.com/maiqingqiang/newbee-mall-api-rs

# Go to the project directory
cd newbee-mall-api-rs

# Copy .env
cp cp .env.exmaple .env
```

### 2. Configure the Database

```sh
# Install diesel_cli. If installation fails, refer to the Diesel document: https://diesel.rs/guides/getting-started
cargo install diesel_cli

# Run the database migration. Before running, make sure `.env` is configured with `DATABASE_URL`
diesel migration run
```

### 3. Run the Project

```sh
# Run
cargo run
```

> Front-end project running is not covered here. Please go to the corresponding project to check it out.

## License

Distributed under the GNU License. See [LICENSE](LICENSE) for more information.

## Acknowledgments

- [newbee-ltd](https://github.com/newbee-ltd)
- [十三](https://github.com/ZHENFENG13)

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/maiqingqiang/newbee-mall-api-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/maiqingqiang/newbee-mall-api-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/maiqingqiang/newbee-mall-api-rs.svg?style=for-the-badge
[forks-url]: https://github.com/maiqingqiang/newbee-mall-api-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/maiqingqiang/newbee-mall-api-rs.svg?style=for-the-badge
[stars-url]: https://github.com/maiqingqiang/newbee-mall-api-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/maiqingqiang/newbee-mall-api-rs.svg?style=for-the-badge
[issues-url]: https://github.com/maiqingqiang/newbee-mall-api-rs/issues
[license-shield]: https://img.shields.io/github/license/maiqingqiang/newbee-mall-api-rs.svg?style=for-the-badge
[license-url]: https://github.com/maiqingqiang/newbee-mall-api-rs/blob/master/LICENSE.txt
