## File Watcher

This tool will watch given files and runs command on file changes.

## Config

**create fwr.json file in root directory**

```json
{
    "port": "4321",
    "ext": ["go"],
    "dir": "server",
    "command": "go run main.go",
    "command_path": "src",
    "ignore_folders": []
}
```
**port**: Server Port. <br/>
**ext**: File extensions like .go, .js ect...<br/>
**dir**: directory to watch <br/>
**command**: Command to run on file changes <br/>
**command_path**: command will be run in specific path or folder <br/>
**ignore_folders (Optional)**: folder names to ignore for command to run. <br/>