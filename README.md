## File Watcher

This tool will watch given files and runs command on file changes.

## Config

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
**port**: Server Port.
**ext**: File extensions like .go, .js ect...
**dir**: directory to watch
**command**: Command to run on file changes
**command_path**: command will be run in specific path or folder
**ignore_folders (Optional)**: folder names to ignore for command to run.