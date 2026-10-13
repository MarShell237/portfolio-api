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
sea-orm-cli generate entity \
-o entities/src \
--entity-format dense
;
diff entities-backup/src entities/src
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
### requetes http avec xh
- se login
```bash
xh --session=my-session POST http://127.0.0.1:8080/api/v2/users/login email=user@example.com password=user@example.com
```

- utiliser la session sur une route proteger par l'authentification
```bash
xh --session=my-session VERBE_HTTP http://127.0.0.1:8080/api/v2/auth/[url] 
```
# Fécondé par la haine de MarShell
