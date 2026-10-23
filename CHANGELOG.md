## [0.3.0] - 2026-10-23

### Summary

The codebase is significantly refactored (again) for better maintainability.

Tool calling support is introduced, as well as a simple, proof-of-concept tool cakestry-clock.

### Features

- *(tool)* Add cakestry-clock ([ba29128](https://github.com/VioletsOleander/cakestry/commit/ba29128b21c3f0de96bd49f8359e562116d6d5ed))
- Finish preliminary tool calling support ([5e9b7ba](https://github.com/VioletsOleander/cakestry/commit/5e9b7baceea79fdf545c76e6d9bbec1764185302))

### Bug Fixes

- Re-add Cursor::new for test using ([c72d650](https://github.com/VioletsOleander/cakestry/commit/c72d65025bb22306966530d7431a752ca96bd2de))

### Refactor

- Improve event handling, state storage and rendering to prepare for tool incorporation ([243d400](https://github.com/VioletsOleander/cakestry/commit/243d400b5d0780dc1f3119c7fc35d9175d798312))
- Migrate to workspace structure to prepare for tool incorporation ([200c686](https://github.com/VioletsOleander/cakestry/commit/200c68699b68fcf50c5b1ba2bbf884a0565addc8))

### Documentation

- Add const order to style ([70ae3b7](https://github.com/VioletsOleander/cakestry/commit/70ae3b730b8b686334c72805cc3ddebcf899d236))
- Update release workflow for workspace ([bf702df](https://github.com/VioletsOleander/cakestry/commit/bf702dfc857a8106f409ffeda7443852baaf9673))

### Miscellaneous Tasks

- Update unreachable message ([c18e5f6](https://github.com/VioletsOleander/cakestry/commit/c18e5f669d63f3cba715d504a0cce75232b9efe7))
## [0.2.0] - 2026-10-17

### Summary

The codebase is significantly refactored for better maintainability.

Modal-based keyboard shortcuts is introduced. Currently support three types of mode: prompt mode,
command mode and scroll mode.

UI is slightly improved by introducing a spinner animation as waiting indicator.

### Features

- Support tracing event filter by CAKESTRY_LOG env var ([d8fe10e](https://github.com/VioletsOleander/cakestry/commit/d8fe10eeb812bf91053d2a94056c59c06ede19de))
- Add simple thinking indicator ([4c96bb3](https://github.com/VioletsOleander/cakestry/commit/4c96bb3a7dfe492dce6b811ecf9c06d55ee45164))
- Add naive command parse and execute logic ([e0ac90e](https://github.com/VioletsOleander/cakestry/commit/e0ac90e12ad151cd3fe73072ac9df2bf0912f9d0))
- Add separator line between statusline and chatstream ([f0f7d30](https://github.com/VioletsOleander/cakestry/commit/f0f7d3082f7765ff9f112925103600419adc1dc2))
- Add tick spinner during waiting ([c3ca7dc](https://github.com/VioletsOleander/cakestry/commit/c3ca7dcfdfffd05118961066baa1fb4612d61255))
- Support up and down arrow action for text buffer ([1d75cf9](https://github.com/VioletsOleander/cakestry/commit/1d75cf98c164c636c81e2153723bf56b221c12bc))
- Add scroll mode ([28834e8](https://github.com/VioletsOleander/cakestry/commit/28834e88fccd9aa4d47c6cb7140f16b0cd4f17cd))
- Support command and scroll mode inter-switch ([a89e4ff](https://github.com/VioletsOleander/cakestry/commit/a89e4ff9a30ee78cedf7886382a7899fd08282ab))
- Support command line cursor rendering ([981c9f4](https://github.com/VioletsOleander/cakestry/commit/981c9f418b97ef80c99a68ea154b585d42b5a3f4))
- Add CTRL-j/k shortcut in scroll mode ([cb498c8](https://github.com/VioletsOleander/cakestry/commit/cb498c8aa5e25f0718348f3438e320e62197fc29))
- Let "i" switch to prompt mode in scroll mode ([3a5ac7a](https://github.com/VioletsOleander/cakestry/commit/3a5ac7a19df11b1ee906d0aff88d225eac464b73))

### Bug Fixes

- Correct the index used in buffer.delete_line_backward ([c5a9edd](https://github.com/VioletsOleander/cakestry/commit/c5a9edd22ae602711107f84602ec7428df404ac0))
- Resolve scroll and cursor rendering issue ([2b5b098](https://github.com/VioletsOleander/cakestry/commit/2b5b0988fa38d72bb2720e8c5a13721cbb86ade6))

### Refactor

- Introduce actions as event handling feedback ([862134b](https://github.com/VioletsOleander/cakestry/commit/862134b2c2e952ad0f4304d734b3521837482364))
- Add arg, config module, introduce error propagation ([86121cd](https://github.com/VioletsOleander/cakestry/commit/86121cd45f53cc545557dca753cd464f462e54af))
- Implement TerminalEventListener ([f19ba72](https://github.com/VioletsOleander/cakestry/commit/f19ba72f27a483ef59e0eace3509dc00c8fad6a6))
- Consolidate `make_request` and `make_response` into one `launch_request` ([36d4178](https://github.com/VioletsOleander/cakestry/commit/36d417854c1f2f192f2c5a7b75561fd2d738e22e))
- Unify service abstraction, introduce event handler and state ([4d8f1ed](https://github.com/VioletsOleander/cakestry/commit/4d8f1ed4d35e22b76b3256171c6128eb42a2f7e1))
- Implement editing functionality for `TextBuffer` ([58dfec3](https://github.com/VioletsOleander/cakestry/commit/58dfec3428ce3dc84b4627d3cd191576b72b2aeb))
- Aggregate more states to State ([e9b0e4d](https://github.com/VioletsOleander/cakestry/commit/e9b0e4d32257858c7c405fafe4df0f606a498200))
- Make fields pub for passive data structure ([e66762c](https://github.com/VioletsOleander/cakestry/commit/e66762c23fc01a0a1077f9359a0ec4728d64a94f))
- Implement basic response event handler ([b96c989](https://github.com/VioletsOleander/cakestry/commit/b96c989b31af3a0661f8f211c1acb264a82708cc))
- Implement statusline and partial chatstream rendering ([0ccea92](https://github.com/VioletsOleander/cakestry/commit/0ccea92fbf264d60b1b90f187fec01a4ec087db7))
- Implement bufferview rendering logic ([f01e204](https://github.com/VioletsOleander/cakestry/commit/f01e20446855a345b8867bd995cbfa977e4a2957))
- Reduce repeat pattern in chatstream rendering loop ([7edf976](https://github.com/VioletsOleander/cakestry/commit/7edf97687feafd8748efdcdb7b646045bf9b269b))
- Set scroll stepsize to 3 to avoid sluggish ([897d9d1](https://github.com/VioletsOleander/cakestry/commit/897d9d1a75c22b50bb7b101f5fbbb06c1fc066f5))
- Let command buffer not memorize previous input, allow backspace to exit command mode ([d350a7c](https://github.com/VioletsOleander/cakestry/commit/d350a7cdeeba9498ac84d45b2e683753f364bfc7))
- Do not render cursor when in scroll mode ([d93daa5](https://github.com/VioletsOleander/cakestry/commit/d93daa5d9aa01e602433d93941a23a7f9298104b))

### Documentation

- Add release workflow ([917643d](https://github.com/VioletsOleander/cakestry/commit/917643d4216740551d234f80d1bf07adc62e034d))
- Add description of CAKESTRY_LOG ([e2971ad](https://github.com/VioletsOleander/cakestry/commit/e2971ad0eea6683bf74d9baac5330119ad28e131))
- Add separate impl blocks style and default/new style guide ([2190805](https://github.com/VioletsOleander/cakestry/commit/2190805511c9d6277f1a09d9baa49c798ce10488))
- Add code style examples ([a2c794a](https://github.com/VioletsOleander/cakestry/commit/a2c794a3ea648a4f8c27607482e636afdf50b173))
- Add group order style guide ([aac9de6](https://github.com/VioletsOleander/cakestry/commit/aac9de6f53928b30ec99c1bbbf4fe6ac36ef3f09))
- Add area separation style ([d9ae49f](https://github.com/VioletsOleander/cakestry/commit/d9ae49f25fa1df642a0252ab14974ffc0161cc89))
- Add &str, &String type comparsion in style ([e578ee5](https://github.com/VioletsOleander/cakestry/commit/e578ee53fadbb8ea6050451cc9bdab73e16d9229))
- Add visibility style guide ([26c94b4](https://github.com/VioletsOleander/cakestry/commit/26c94b4a33abf55e55d793c2c770291a145a80fd))
- Add trait order in style guide ([63f09cb](https://github.com/VioletsOleander/cakestry/commit/63f09cb2d29a189ce2bd3070e521199c42a0ca6d))

### Miscellaneous Tasks

- Add commit url in CHANGELOG ([191be2b](https://github.com/VioletsOleander/cakestry/commit/191be2be03d90fbb1676ffbee4900956a90dd3d6))
- Add release workflow based on cargo dist ([d460810](https://github.com/VioletsOleander/cakestry/commit/d4608103b75aa2e94435e03adac9a5a70efb184b))
- Temporarily enable dispatch release ([976ef74](https://github.com/VioletsOleander/cakestry/commit/976ef74ecbc10ac30277cecf4bc85fd2771df991))
- Make test function style more unified ([f4aba16](https://github.com/VioletsOleander/cakestry/commit/f4aba166c60e80e7d36686b3d7b03e530f6446c6))
- Fix typos ([f585ef7](https://github.com/VioletsOleander/cakestry/commit/f585ef7e0884f85e848e2d499668ee045cd85df9))
- Add test workflow ([87e4b06](https://github.com/VioletsOleander/cakestry/commit/87e4b060783f95e70ad6d822b7dd60244b73f0e3))
- Update cargo dist ([cde9618](https://github.com/VioletsOleander/cakestry/commit/cde96180aaef56d81af610bf06e395c4044ced10))
- Release v0.2.0 ([b6908dc](https://github.com/VioletsOleander/cakestry/commit/b6908dc2a75fc98b6d45dc8a29c168d07ebefaf9))

### Revert

- "ci: temporarily enable dispatch release" ([cfb9371](https://github.com/VioletsOleander/cakestry/commit/cfb937145aaf298b3853a7ade52319a086e43323))
## [0.1.0] - 2026-08-22

### Summary

The first release, establishing the very basic architecture of UI, interaction events and HTTP
events.

Currently, only limited keyboard and mouse interaction, and naive streaming chat interaction are
supported.

### Features

- Prompt sign for request ([0033da9](https://github.com/VioletsOleander/cakestry/commit/0033da93fcb30769d052718783d38bc492241a17))
- Experimental support for scroll ([895ad09](https://github.com/VioletsOleander/cakestry/commit/895ad09ef0d96d4db73497600166fc970da8c6d4))
- Implement basic edit function for textarea ([adb598f](https://github.com/VioletsOleander/cakestry/commit/adb598f119875e7b27995a818ab9b002d577e570))
- Finish basic document rendering ([c283283](https://github.com/VioletsOleander/cakestry/commit/c283283fb25d96f1720e9a7c09ef02d7c50c92f1))
- Introduce custom textwrap function ([043eba4](https://github.com/VioletsOleander/cakestry/commit/043eba4b4c40f37d4f212e6a62340fbd963dfd9e))
- Support primiteive cursor render ([b999d1d](https://github.com/VioletsOleander/cakestry/commit/b999d1d49dcb98cc04f41379da152a241ed467b3))
- Specify terminal cursor shape to steady bar ([d07a926](https://github.com/VioletsOleander/cakestry/commit/d07a9269d4fb9162ae054a72913a944d7379324a))
- Support left arrow and right arrow key to move cursor ([54bfbb9](https://github.com/VioletsOleander/cakestry/commit/54bfbb9f0fe3ddf6d106d70795379b10bea87579))
- Support CTRL-W, CTRL-U for removing input ([bc67f33](https://github.com/VioletsOleander/cakestry/commit/bc67f333548fa1b82c21432b9fe077d5c2f269ba))
- Support very basic configuration load ([144c9a9](https://github.com/VioletsOleander/cakestry/commit/144c9a9347c71792a782e4a82491305070da0957))
- Implement prototype client ([8c8c1d5](https://github.com/VioletsOleander/cakestry/commit/8c8c1d52d0e5f7060ff3db1cc794377da0a04289))
- Support rendering statusline ([9f05490](https://github.com/VioletsOleander/cakestry/commit/9f054906f5e2db440988272ac5158a3090dc7566))
- Support basic conversion ([df9112a](https://github.com/VioletsOleander/cakestry/commit/df9112a2564eef7088604e5b1df8c27ccff3cb90))
- Implement basic streaming service ([51b4c2a](https://github.com/VioletsOleander/cakestry/commit/51b4c2acfe1246113aabbf755641682f07ca01ff))
- Basic service event handling ([7b4506b](https://github.com/VioletsOleander/cakestry/commit/7b4506be97c589c65e5c62c7126cf141256f04d9))

### Bug Fixes

- Panic issue when delete prev char in line start ([9e177b6](https://github.com/VioletsOleander/cakestry/commit/9e177b6f73cb4c0f6b2e3c9eb8ebba77d3a6fd59))
- Make query prefix symbols correctly rendered according to scroll ([88f01cd](https://github.com/VioletsOleander/cakestry/commit/88f01cd417db1d9603b7c12f4123e21dca33a691))
- Panic when press DELETE in the line end ([68e5c90](https://github.com/VioletsOleander/cakestry/commit/68e5c902b69518156406d1dd99781fdfecb57874))
- Panic when clear user input ([966bb9d](https://github.com/VioletsOleander/cakestry/commit/966bb9d802b1f03edd4738abd254c651217ff4ac))
- Screen cursor position rendered with wrong displacement ([6004b16](https://github.com/VioletsOleander/cakestry/commit/6004b161396d1929919c50e19358fe94e6912635))

### Other

- Initial commit ([8b13e50](https://github.com/VioletsOleander/cakestry/commit/8b13e503f85f5124a3ef32c99e7d98e42f4635e0))

### Refactor

- Adapt terminal init ([8ca0e49](https://github.com/VioletsOleander/cakestry/commit/8ca0e496a767880ba0d5a130faaeeecb083de6b9))
- Improve docstring ([4c92d43](https://github.com/VioletsOleander/cakestry/commit/4c92d43830d80256c4929ae5a22e5ad30a32997b))
- Isolate implementation of various widget ([c2f7c44](https://github.com/VioletsOleander/cakestry/commit/c2f7c44e15667b7b55e1551b73df9c3d63096244))
- Move up textarea state management to session ([789051f](https://github.com/VioletsOleander/cakestry/commit/789051f9062281b2f41dc3ffef36058a09f53243))
- Avoid redundant line wrap computation for render ([9c43783](https://github.com/VioletsOleander/cakestry/commit/9c43783afce5f102da906f5c155e7d2b0602415a))
- Move widgets into session folder ([a562cfa](https://github.com/VioletsOleander/cakestry/commit/a562cfa756d2414312c177e6ac0bebe84e80c60f))
- Let session take a whole frame ([773e65e](https://github.com/VioletsOleander/cakestry/commit/773e65e426771873238efc197c59192f7b0b8614))
- Use inline method call to compact code ([4941398](https://github.com/VioletsOleander/cakestry/commit/4941398bc605175c26f46344bbff6a9df7f7fff4))
- Remove redundant pre-materialization ([849db76](https://github.com/VioletsOleander/cakestry/commit/849db76e275f0532dbb4fa719da0fb99b39d40f2))
- Delegate line wrapping to widget itself ([bf1e0e6](https://github.com/VioletsOleander/cakestry/commit/bf1e0e63ab6d2b0ed8ec511c674d300c91ac965a))
- Naming modules by concept instead of standalone struct name ([2289596](https://github.com/VioletsOleander/cakestry/commit/22895967601fb28cf102d2c2794f710f3a67a187))
- Follow comment style guide ([8903631](https://github.com/VioletsOleander/cakestry/commit/89036315dd84f2b42e56c69b2dbd7188e044f476))
- Improve user input and cursor render logic ([45e466a](https://github.com/VioletsOleander/cakestry/commit/45e466ad2310df4467f130705f7d85d43a8addfe))
- Remove period in .expect() invocation ([c43e20c](https://github.com/VioletsOleander/cakestry/commit/c43e20c9ecda2d3f8ad55d4a1200702e00450047))
- Initialize tracing subscriber to main function ([2f5d8d8](https://github.com/VioletsOleander/cakestry/commit/2f5d8d85601b92e0e24c1a584bf5977980df0f4c))
- Use full string for query and reply ([a325a9f](https://github.com/VioletsOleander/cakestry/commit/a325a9f1c35e0cee188744b90c7962f6c39ebc3e))
- Isolate render and data storage ([5969290](https://github.com/VioletsOleander/cakestry/commit/5969290de056b9d24b4540ffc6486ea9ccab02a8))
- Use more concise iter().find() ([e15aacc](https://github.com/VioletsOleander/cakestry/commit/e15aacc3b3914004607afe8ebbc816f5ac6923f7))
- Implement Terminal type for TUI and terminal interaction ([0677743](https://github.com/VioletsOleander/cakestry/commit/0677743305a327bc17b29393967ee33ed50026a7))
- Introduce service to replace client ([54df4c7](https://github.com/VioletsOleander/cakestry/commit/54df4c72d600c436ed5a241699c7f8a82a017492))
- Make session.user_input the only source of truth ([c1ed37c](https://github.com/VioletsOleander/cakestry/commit/c1ed37ca558084edb38c858773bca14c99ea02fe))

### Documentation

- Add comment style guide ([83b18dc](https://github.com/VioletsOleander/cakestry/commit/83b18dca9a9ac07929868502b77aaafa60a9dcfc))
- Add simple README ([1597dd0](https://github.com/VioletsOleander/cakestry/commit/1597dd08c0b99e55d9698944e1c4ea8cd43e92a4))

### Performance

- Disable smawk feature of textwrap, use faster first fit wrapping ([cd2d1bb](https://github.com/VioletsOleander/cakestry/commit/cd2d1bbd3e3ce19c2471149bdddc136235bf6060))

### Styling

- Limit line width to 100 ([6e31fa4](https://github.com/VioletsOleander/cakestry/commit/6e31fa442ff088d373d69c0703afdec4a7bc992b))
- Update style guide, and accordingly refactor ([2191a55](https://github.com/VioletsOleander/cakestry/commit/2191a553505316fcf0116feffeb8a275bcb35e23))

### Miscellaneous Tasks

- Update gitignore ([b24e637](https://github.com/VioletsOleander/cakestry/commit/b24e637303295c2b2eb1c57b284aa3b82bbfc8d3))
- Update gitignore ([c6c16d0](https://github.com/VioletsOleander/cakestry/commit/c6c16d0dd0fa10263b238f9a2c415b036515c20b))
- Correct typos ([8253295](https://github.com/VioletsOleander/cakestry/commit/82532955775feeaf19c73b5e4b997d2821cac3f5))
- Add typos configuration ([62d24bc](https://github.com/VioletsOleander/cakestry/commit/62d24bc1cfab7c2708ecfa8052e046029f9122d7))
- Add some simple metadata ([b2960b9](https://github.com/VioletsOleander/cakestry/commit/b2960b9aaf569d9fa467bcbf332715b38f32fadf))
- Fix typos ([ffda344](https://github.com/VioletsOleander/cakestry/commit/ffda344d8c983cbfdc6652c38ecbee8bea518c43))
- Add license ([44a1914](https://github.com/VioletsOleander/cakestry/commit/44a1914d88d1cadb3ff05381e09d7b791f7914ac))
- Release v0.1.0 ([f23775d](https://github.com/VioletsOleander/cakestry/commit/f23775d02f1dfe37e0823406ac46d04e6083566a))
