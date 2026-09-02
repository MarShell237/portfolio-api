# portfolio-api
## SeaOrm commands
### Migrations

- fresh database migrations
```bash
sea-orm-cli migrate fresh -d migrations
```

- generate migration table
```bash
sea-orm-cli migrate generate create_[]_table -d migrations
```

### Entities
- generate entities
```bash
sea-orm-cli generate entity -o entities/src
```

### Seeders
- execute seeders
```bash
sea-orm-cli migrate up -d seeders
```

- clear seeders
```bash
sea-orm-cli migrate down -d seeders
```

- generate migration table
```bash
sea-orm-cli migrate generate []_seeder -d seeders
```

- fresh database and seed
```bash
sea-orm-cli migrate fresh -d migrations ; sea-orm-cli migrate up -d seeders
```
