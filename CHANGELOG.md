## [0.1.0] - 2026-08-22

### Summary

The first release, establishing the very basic architecture of UI, interaction events and HTTP
events.

Currently, only limited keyboard and mouse interaction, and naive streaming chat interaction are
supported.

### Features

- Prompt sign for request
- Experimental support for scroll
- Implement basic edit function for textarea
- Finish basic document rendering
- Introduce custom textwrap function
- Support primiteive cursor render
- Specify terminal cursor shape to steady bar
- Support left arrow and right arrow key to move cursor
- Support CTRL-W, CTRL-U for removing input
- Support very basic configuration load
- Implement prototype client
- Support rendering statusline
- Support basic conversion
- Implement basic streaming service
- Basic service event handling

### Bug Fixes

- Panic issue when delete prev char in line start
- Make query prefix symbols correctly rendered according to scroll
- Panic when press DELETE in the line end
- Panic when clear user input
- Screen cursor position rendered with wrong displacement

### Other

- Initial commit

### Refactor

- Adapt terminal init
- Improve docstring
- Isolate implementation of various widget
- Move up textarea state management to session
- Avoid redundant line wrap computation for render
- Move widgets into session folder
- Let session take a whole frame
- Use inline method call to compact code
- Remove redundant pre-materialization
- Delegate line wrapping to widget itself
- Naming modules by concept instead of standalone struct name
- Follow comment style guide
- Improve user input and cursor render logic
- Remove period in .expect() invocation
- Initialize tracing subscriber to main function
- Use full string for query and reply
- Isolate render and data storage
- Use more concise iter().find()
- Implement Terminal type for TUI and terminal interaction
- Introduce service to replace client
- Make session.user_input the only source of truth

### Documentation

- Add comment style guide
- Add simple README

### Performance

- Disable smawk feature of textwrap, use faster first fit wrapping

### Styling

- Limit line width to 100
- Update style guide, and accordingly refactor

### Miscellaneous Tasks

- Update gitignore
- Update gitignore
- Correct typos
- Add typos configuration
- Add some simple metadata
- Fix typos
- Add license
