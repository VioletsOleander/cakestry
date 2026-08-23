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
