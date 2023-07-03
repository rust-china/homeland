<p align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="400">
  <p align="center">Open source discussion website.</p>
  <p align="center">开源的论坛／社区网站系统，基于 <a href="https://rust-china.cn">Rust China</a> 发展而来。</p>
  <p align="center">
    <a href="https://github.com/rust-china/homeland/actions">
      <img src="https://github.com/rust-china/homeland/workflows/Test/badge.svg">
    </a>
  </p>
</p>

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