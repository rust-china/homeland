
<div align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="400">
  <p align="center">Open source discussion website.</p>
  <p align="center">开源的论坛／社区网站系统，基于 <a href="https://rust-china.cn">Rust China</a> 发展而来。</p>
</div>

[![ci-backend](https://github.com/rust-china/homeland/actions/workflows/ci-backend.yml/badge.svg)](https://github.com/rust-china/homeland/actions/workflows/ci-backend.yml)
[![GitHub license](https://img.shields.io/github/license/rust-china/homeland)](https://github.com/rust-china/homeland)
[![GitHub latest SemVer tag)](https://img.shields.io/github/v/tag/rust-china/homeland)](https://github.com/rust-china/homeland/tags)
[![GitHub release)](https://img.shields.io/github/v/release/rust-china/homeland)](https://github.com/rust-china/homeland/releases)


## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](.github/CONTRIBUTING_DOC/CODE_OF_CONDUCT.md)
[![GitHub contributors](https://img.shields.io/github/contributors/rust-china/homeland)](https://github.com/rust-china/homeland/graphs/contributors)

We welcome community contributions to this project.

Please read [Contributor Guide](.github/CONTRIBUTING_DOC/CONTRIBUTING.md) for more information on how to get started.

请阅读有关 [贡献者指南](.github/CONTRIBUTING_DOC/zh-CN/CONTRIBUTING.md) 以获取更多如何入门的信息

## RUN

```bash
# cargo install cargo-watch
# cargo install sea-orm-cli
vim ./backend/.env # edit DATABASE_URL=  & create db
sea-orm-cli migrate up # cd ./backend

pnpm install
pnpm run dev
```

## License

Released under the MIT license:

- [www.opensource.org/licenses/MIT](http://www.opensource.org/licenses/MIT)