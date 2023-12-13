# harbor-cli

## How to generate the API client

```sh
openapi-generator-cli generate -c openapi-generator-cli.config.json -g rust -i https://github.com/goharbor/harbor/raw/main/api/v2.0/swagger.yaml -o api-client
```
