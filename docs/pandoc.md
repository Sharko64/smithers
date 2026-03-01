```sh
pandoc docs/src/man.md \
  --from markdown+include \
  -t man \
  -o smithers.1
```