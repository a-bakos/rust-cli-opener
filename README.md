# CLI OPENER - Rust practice project - WIP

### Plan:

Create a CLI app that acts as a central place to open often-used app and websites

Examples: weather check, gym capacity check, stock price, spotify, notepad, ...

Store config data in a json file, so these aren't embedded in the program

```JSON
{
	"item_1_name": {
		"id": "id",
		"display_name": "The website or app",
		"src": "path",
		"added": "date"
	}
}
```

**Stage 1:**

- File io, JSON reading + writing
- Simple program menu loop
- External crate integration
- Handle user input
- Add ability to read and edit config data from json from menu
- Find a better name for the app...

**Stage 2:**

- Web scraping to extract info (eg. weather, gym capacity, stock price)
- Show extracted info on an app dashboard
