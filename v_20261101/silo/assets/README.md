need to compile the gschema settings file

Bash
```
glib-compile-schemas .
```


cargo build
UST_LOG=debug GSETTINGS_SCHEMA_DIR=libs/ui-gtk/assets/. cargo run --bin silo
