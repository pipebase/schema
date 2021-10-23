<div align="center">
<img src=".github/assets/banner.png"></img>
</div>
<br />

YAML validation / auto complete in vscode for [`pipebase`] manifest

### Setup
install [`YAML`] vscode extension, and config `.vscode/settings.json` in working directory
```
{
    "yaml.schemas": {
        "https://raw.githubusercontent.com/pipebase/schema/main/app/app.json": [ "PATH/TO/PIPE/YAML" ]
    }
}
```

### Demo
in demo, fix four missing fields (red error lines) with auto complete

![screencast](https://raw.githubusercontent.com/pipebase/schema/main/.github/assets/demo.gif)

[`pipebase`]: https://github.com/pipebase/pipebase
[`YAML`]: https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml
