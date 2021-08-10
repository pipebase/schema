YAML validation / auto complete in vscode for [`pipebase`] manifest
### Setup
install [`YAML`] vscode extension, and config `.vscode/settings.json`
```
{
    "yaml.schemas": {
        "https://raw.githubusercontent.com/pipebase/schema/main/app/app.json": [ "PATH/TO/PIPE/YAML" ]
    }
}
```

[`pipebase`]: https://github.com/pipebase/pipebase
[`YAML`]: https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml