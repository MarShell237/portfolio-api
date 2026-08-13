# portfolio-api
## SeaOrm commands
- fresh database migrations
```bash
sea-orm-cli migrate fresh -d migrations
```

- generate migration table
```bash
sea-orm-cli migrate generate create_[]_table -d migrations
```

- generate entities
```bash
sea-orm-cli generate entity -o entities/src
```
