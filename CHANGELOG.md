# Changelog

## [0.2.0](https://github.com/AlexandreGallais/synoptic-studio/compare/v0.1.0...v0.2.0) (2026-10-14)


### ⚠ BREAKING CHANGES

* **geometry:** `contourToPathData` is removed from the public API; `filletArc(corner, angle, radius)`, `cornerSetbacks(corners, angles)` and `turningAngles(triples)` take the turning angles or the triples.

### Features

* **eslint:** folders by business domain, never files beside folders ([214792b](https://github.com/AlexandreGallais/synoptic-studio/commit/214792b6a06390fc8d7d227eb352631fdc2e538a))
* F10 round shape and F13 every calculation named, validated ([fb01fae](https://github.com/AlexandreGallais/synoptic-studio/commit/fb01fae9114fb308929af97af968a8f120c3a9d4))
* F12 points on a curve with handles ([480586f](https://github.com/AlexandreGallais/synoptic-studio/commit/480586fbc9bc791f73697cdc1c452a31f930080c))
* **geometry:** a handle from an angle and a length, and back ([4091557](https://github.com/AlexandreGallais/synoptic-studio/commit/4091557a666aa0027dd5e94aab81e9991255250f))
* **geometry:** a shape's box is the box of its curve ([b045e36](https://github.com/AlexandreGallais/synoptic-studio/commit/b045e36e27a584881dd158e83846b17fee9bd248))
* **geometry:** a side with a handle drawn as a cubic ([c1c1469](https://github.com/AlexandreGallais/synoptic-studio/commit/c1c14694728a3cd90da5d35c24d9a76d4e0fdfc1))
* **geometry:** corners beside curves keep their radius when drawn ([3a3ed51](https://github.com/AlexandreGallais/synoptic-studio/commit/3a3ed5114e3ee90ac7e25f833cdad69845635706))
* **io:** path data writes a cubic side as C, a curved closing side too ([157b40d](https://github.com/AlexandreGallais/synoptic-studio/commit/157b40dc877402e591ab12b94713ef0db2298d35))
* **model:** a free shape's corners carry their handles ([7b00e70](https://github.com/AlexandreGallais/synoptic-studio/commit/7b00e7086a832bddf049e633156f97694feb3a8c))
* **model:** a point added on a curved side splits the curve, outline kept ([dd37378](https://github.com/AlexandreGallais/synoptic-studio/commit/dd37378a5022af1caa7e0856f18202643d6d0686))
* **model:** a point smoothed by a tension, its handles from its neighbours ([7fd66d7](https://github.com/AlexandreGallais/synoptic-studio/commit/7fd66d7dc7832375c058f28aace628a36a819ec6))
* **model:** freeShapeWithHandles, a point's handles set as drawn ([29858c4](https://github.com/AlexandreGallais/synoptic-studio/commit/29858c4459841719580b65a54575fbc3f7c9d327))
* **model:** handles as shares of the side, and curves pasted on a side ([e005078](https://github.com/AlexandreGallais/synoptic-studio/commit/e005078a43b91e1487a630217e5d98b0efeab9ba))
* **model:** the round shape, its points on the curve and its two conversions ([6255c46](https://github.com/AlexandreGallais/synoptic-studio/commit/6255c467e556dbae9a94e5512e408369bd234adf))
* **playground:** convert a round shape to its points or to a rounded rectangle ([7466eec](https://github.com/AlexandreGallais/synoptic-studio/commit/7466eec2fc72be720fdd29912ea4d581340e2d99))
* **playground:** handles in Points mode, typed as x and y ([c0f05bd](https://github.com/AlexandreGallais/synoptic-studio/commit/c0f05bdd2ecc8b11bb37b727f7eba6b246ea3e54))
* **playground:** handles on a polygon's point, the polygon converted first ([fc033e6](https://github.com/AlexandreGallais/synoptic-studio/commit/fc033e6987a791b501bb2eb4155312c4c6d18cc3))
* **playground:** handles typed as an angle and a length ([877a3fc](https://github.com/AlexandreGallais/synoptic-studio/commit/877a3fca83eba615191ed9eb5c95bfe5466142f0))
* **playground:** handles typed as dx and dy from their point ([3b83642](https://github.com/AlexandreGallais/synoptic-studio/commit/3b83642be7b114d1871aa2875d2ed8818896dd09))
* **playground:** shares of the side and a paste field for curves ([d3fafc1](https://github.com/AlexandreGallais/synoptic-studio/commit/d3fafc1ed594dcccb2a0e3afc41a6bb9f2ce2047))
* **playground:** the guided test and demo page of F10 ([57c31b1](https://github.com/AlexandreGallais/synoptic-studio/commit/57c31b1f340debccc7255538c76c656aa77df5a5))
* **playground:** the guided test of F12, one step per option kept ([50be5d9](https://github.com/AlexandreGallais/synoptic-studio/commit/50be5d9890de97d63ba98d64e80db7073a46989a))
* **playground:** the guided test of F12, thirteen steps ([2eb30f4](https://github.com/AlexandreGallais/synoptic-studio/commit/2eb30f4c79e6fac545ec9099405afc32943918c1))
* **playground:** the mode Smooth by a tension, and its field ([ec02744](https://github.com/AlexandreGallais/synoptic-studio/commit/ec0274446e724a23c73445b5499602590acde74e))
* **playground:** the Round tool draws a circle or an ellipse ([7f0cc30](https://github.com/AlexandreGallais/synoptic-studio/commit/7f0cc306bfb5bd7b536bc8b99e543a6c30077149))
* **tooling:** npm run branches:clean after each merged feature ([1b27b4b](https://github.com/AlexandreGallais/synoptic-studio/commit/1b27b4b31e1bb1d2864f61847dfa272810149dcf))
* **tooling:** the metrics report splits each turn between tools and model ([0647497](https://github.com/AlexandreGallais/synoptic-studio/commit/0647497246b8abd09cf9634125261c905b014021))
* **tooling:** the private records inside the project, ignored by Git ([9d312f3](https://github.com/AlexandreGallais/synoptic-studio/commit/9d312f34eef6967884d510d4a85cbc9b6e55c743))
* **tooling:** the turn's clock on the status line moves every second ([0264e8b](https://github.com/AlexandreGallais/synoptic-studio/commit/0264e8b811b7deabd732bf0922800fb407d93ee5))


### Bug fixes

* **eslint:** a word of a JSDoc comment no longer counts as a use ([6dc7a2c](https://github.com/AlexandreGallais/synoptic-studio/commit/6dc7a2ce6713c98f4e84aac6fe4fa476b7add3dc))
* **geometry:** fillets beside curves drawn whenever their radius is shown ([78b8dce](https://github.com/AlexandreGallais/synoptic-studio/commit/78b8dcedcae867bc5dcca601b97bbc55c05d6040))
* **geometry:** no fillet beside a side of length 0, the checks tested ([5c24e8f](https://github.com/AlexandreGallais/synoptic-studio/commit/5c24e8fb8a2802824bbfbdc549c392f6fcfcd868))
* **geometry:** sides built one by one, off handles not drawn, splits mirrored ([24d2c20](https://github.com/AlexandreGallais/synoptic-studio/commit/24d2c20032cf8cf9cd8c2f36c3e3e30f88505c51))
* **geometry:** small fillets between two curves, the foot along the tangent ([7b5b5a3](https://github.com/AlexandreGallais/synoptic-studio/commit/7b5b5a39e8f29e81e2a00f93776bb2d6f2b87642))
* **model:** automatic handles stored as computed, sources as they read ([d2d2a4e](https://github.com/AlexandreGallais/synoptic-studio/commit/d2d2a4eb8be68ea779e4136da6f27574eb25c6f9))
* **model:** pasted numbers end where SVG ends them, too large refused ([e619bb8](https://github.com/AlexandreGallais/synoptic-studio/commit/e619bb89564a528d4d32299a31f7116c6c3486ef))
* **playground:** a round shape keeps the radii of the other shapes ([809a473](https://github.com/AlexandreGallais/synoptic-studio/commit/809a473e7383c78123e37ebb9597d75db8eb705a))
* **playground:** an angle typed before its length kept per point and way ([56a5e89](https://github.com/AlexandreGallais/synoptic-studio/commit/56a5e89b8694904c44083c119f15fef6d67dcfbf))
* **playground:** no switch on a polygon that cannot be converted ([251ae73](https://github.com/AlexandreGallais/synoptic-studio/commit/251ae734bb5e05bf337fc24869510ab8e28a9a82))
* **playground:** one coordinate of a handle typed alone, as a point's ([9614b13](https://github.com/AlexandreGallais/synoptic-studio/commit/9614b13ffd6b5d8581a3432a9af65d55147a7128))
* **playground:** radii shown as drawn next to curves ([1567aa7](https://github.com/AlexandreGallais/synoptic-studio/commit/1567aa7f73db4927d64000e26abd14d564b085f7))
* **tooling:** library-html removes tags by splitting, not by a pattern ([57d453c](https://github.com/AlexandreGallais/synoptic-studio/commit/57d453cd563646da207dc9e34f0486bef918431c))
* **tooling:** the expert agents search the library first ([df24c16](https://github.com/AlexandreGallais/synoptic-studio/commit/df24c16a66a00f3f817738d022a3f52fc54cbc0c))
* **tooling:** the release's CodeQL alerts on two scripts ([d66c523](https://github.com/AlexandreGallais/synoptic-studio/commit/d66c523363fe9c45882001a9fd5c18b2a58849a2))
* **tooling:** the status line ends refused background tasks, shows four states ([c49af8e](https://github.com/AlexandreGallais/synoptic-studio/commit/c49af8eba0f491808addfb6141bdb773d28643c9))
* **tooling:** the status line shows the turn resumed by a notification ([a241ea6](https://github.com/AlexandreGallais/synoptic-studio/commit/a241ea6fb42651bffdd8a4daaa4f516b0c0c4512))
* **tooling:** the status line stops the turn's clock when the agent ends its turn ([abf3fc6](https://github.com/AlexandreGallais/synoptic-studio/commit/abf3fc6de5b535494226709335812ce1dd718ffc))
* **tooling:** the status line waits for background tasks ([6d8a9b2](https://github.com/AlexandreGallais/synoptic-studio/commit/6d8a9b20c3173802639e13aecfac7acdd46f3fec))


### Performance

* **tooling:** check:all in parallel lanes, ESLint's cache for fixes ([033c8cd](https://github.com/AlexandreGallais/synoptic-studio/commit/033c8cd819b8e7d5021dd16551507e6f6750540f))


### Refactoring

* every code folder holds modules or theme folders, not both ([fb07ae2](https://github.com/AlexandreGallais/synoptic-studio/commit/fb07ae2cf9586a0586243a1581cf3baae78c2b63))
* four modules to their domain, the folder rule fully tested ([f241a2c](https://github.com/AlexandreGallais/synoptic-studio/commit/f241a2cf08c74ae8c34b22ad1510d0bb6bf7b529))
* geometry and model folders named after the domain ([0b965d7](https://github.com/AlexandreGallais/synoptic-studio/commit/0b965d7b587c2e75ad0fa09c417b12a0960a1fb7))
* **geometry:** curves in their own folder, shared zero vector ([29c73c6](https://github.com/AlexandreGallais/synoptic-studio/commit/29c73c6dffb836156cde42b3476798966bcfff01))
* **geometry:** one path writer, one pass per corner ([dc83b1b](https://github.com/AlexandreGallais/synoptic-studio/commit/dc83b1b7110f8872d9beaf41a33677430464e0cc))
* **math:** name the calculations of math and geometry ([9619307](https://github.com/AlexandreGallais/synoptic-studio/commit/9619307d280dba1f7f6c555e242b9d546691b0c5))
* **math:** the second pass of F13 and the duplicates merged ([d76058d](https://github.com/AlexandreGallais/synoptic-studio/commit/d76058d829f0ddaa634c01328ffb9061f2523769))
* **model:** name the calculations of the model and the playground ([b37f835](https://github.com/AlexandreGallais/synoptic-studio/commit/b37f835ca588052f36fbf577599e332d97178dba))
* **model:** only x and y on the page, or an angle from the side ([48c44bc](https://github.com/AlexandreGallais/synoptic-studio/commit/48c44bc537f33dfd3ff5a5a1ab8eb4168fb244d0))
* **model:** read each corner's handles by cyclicItem, every branch tested ([d264342](https://github.com/AlexandreGallais/synoptic-studio/commit/d264342d47714ba11896f3b8374305b0b3de6b41))
* **model:** the auditor's review of EN-029 applied ([7435fd2](https://github.com/AlexandreGallais/synoptic-studio/commit/7435fd2d4b68169d1d844a891ac2cffc060532da))
* **model:** what was left of the methods not kept ([f38e3d7](https://github.com/AlexandreGallais/synoptic-studio/commit/f38e3d7b13d0b6c518982dcc015d96ca3e305900))
* **playground:** readFieldShift and readTypingWay say what they read ([656d21d](https://github.com/AlexandreGallais/synoptic-studio/commit/656d21ddf5e73b7a8319b5709edfb7d68c6745b2))
* **playground:** the playground's modules in one folder per panel ([646db76](https://github.com/AlexandreGallais/synoptic-studio/commit/646db7669cdff7f7ae7bbb26a69192273b210c6c))
* **playground:** the round shape's conversion to a rounded rectangle removed ([11b8c17](https://github.com/AlexandreGallais/synoptic-studio/commit/11b8c1742abafd2658cd76ecb152f6a05fc3e5ce))
* the library's modules in folders by theme ([8582c6f](https://github.com/AlexandreGallais/synoptic-studio/commit/8582c6f9486955ab7df61aa6206f79dd2280b666))


### Documentation

* **adr:** /run is the go-ahead, a validated feature merges by itself ([977b7b3](https://github.com/AlexandreGallais/synoptic-studio/commit/977b7b37e09d27592263e1f3e00930575e2171e5))
* **adr:** a fillet beside a cubic side, ADR-0047 and its derivation ([19f7501](https://github.com/AlexandreGallais/synoptic-studio/commit/19f750146bb338935af7b90e258cfc26abee4d25))
* **adr:** a shape's type in its folder, after the review of SP-010 ([99ee69a](https://github.com/AlexandreGallais/synoptic-studio/commit/99ee69ae30e24a78f17728c533d79bd1381bf78a))
* **adr:** cubic sides in symbols, and the stories of F12 ([d9de0d1](https://github.com/AlexandreGallais/synoptic-studio/commit/d9de0d1756484fd3fb9fa9b016947c49469ab2f8))
* **adr:** every calculation named, and a naming pass closes each epic ([09cd4dc](https://github.com/AlexandreGallais/synoptic-studio/commit/09cd4dc98f7c6a6b551729667dfa806e9d0b2db6))
* **adr:** folders by theme, 15 modules as a signal ([fe99230](https://github.com/AlexandreGallais/synoptic-studio/commit/fe99230f44721847cec546fb26885c60bffe6d03))
* **adr:** the retrospective's decisions, and the main thread kept free ([21d3f96](https://github.com/AlexandreGallais/synoptic-studio/commit/21d3f96f3194bb3feeb87a3ac59c7c3d1e08bf4a))
* **backlog:** audit of F11, AUD-010 done ([976e455](https://github.com/AlexandreGallais/synoptic-studio/commit/976e45585c81faf005c8aeb499ce7c3d30b044d5))
* **backlog:** checkpoint in the middle of F12, CHK-011 done ([abe4e6b](https://github.com/AlexandreGallais/synoptic-studio/commit/abe4e6b673c605c34d2ef4726f53a4978be0f9ca))
* **backlog:** checkpoint of F11, CHK-010 done ([7b30d05](https://github.com/AlexandreGallais/synoptic-studio/commit/7b30d054781fbff9f5808c9073804a1ac90e1b16))
* **backlog:** close F10 and F13, validated together ([0581fe0](https://github.com/AlexandreGallais/synoptic-studio/commit/0581fe0913a2f523e621976bf8a392c2c58f3d98))
* **backlog:** close RET-002 and E14, the answers recorded ([5018d1c](https://github.com/AlexandreGallais/synoptic-studio/commit/5018d1c4cccf5ec3b59a14c141ac1a60ec9822e8))
* **backlog:** close VAL-011 and F12 after the second validation ([9f925fe](https://github.com/AlexandreGallais/synoptic-studio/commit/9f925fe7cec46b452ddb618b536868fc70829a04))
* **backlog:** draft F13 and log F12's second round ([9cafacb](https://github.com/AlexandreGallais/synoptic-studio/commit/9cafacbc4bd646cbd7813b3298ddeaf8658e009c))
* **backlog:** frame F12 with its checkpoint, audit and validation ([0544e44](https://github.com/AlexandreGallais/synoptic-studio/commit/0544e4410fe352d85e5ddfb86b9c409b81e3aca6))
* **backlog:** record the review of E14 and set REV-002 done ([3a53a0c](https://github.com/AlexandreGallais/synoptic-studio/commit/3a53a0cd7cca8405092d3ba69d7b7e6f08d22a7e))
* **backlog:** refine F10 in E14 and write its stories ([4e52a91](https://github.com/AlexandreGallais/synoptic-studio/commit/4e52a91f02fa41ce906aab380a23fa199cb85e5c))
* **backlog:** refine F13 and write its stories ([fa1633a](https://github.com/AlexandreGallais/synoptic-studio/commit/fa1633a71abc4d6583b3a0aa99ffdb0f88631ad9))
* **backlog:** refinement of F11, and the session's journal ([0fce5ee](https://github.com/AlexandreGallais/synoptic-studio/commit/0fce5ee0ad264a86407d7096ab7ebfb66211ce06))
* **backlog:** refinement of F12 started, F12 before F10 in E14 ([29534c8](https://github.com/AlexandreGallais/synoptic-studio/commit/29534c8d3431eccfa865cfcfe60b14f42536a88d))
* **backlog:** retrospective of E01 done, E01 done ([705046d](https://github.com/AlexandreGallais/synoptic-studio/commit/705046d12c0e1562cdd8e52abd5b3370aa4b740c))
* **backlog:** set AUD-011 done ([83efe1a](https://github.com/AlexandreGallais/synoptic-studio/commit/83efe1a774013642e99e2e582f2f41a8127f3686))
* **backlog:** set AUD-012 done ([e9143a1](https://github.com/AlexandreGallais/synoptic-studio/commit/e9143a114f730d323735e3f2c1dd811cea371525))
* **backlog:** set EN-020 done ([85f7e78](https://github.com/AlexandreGallais/synoptic-studio/commit/85f7e78b606fecff469fc9320e6a0296fb9823c6))
* **backlog:** set EN-021 done ([23fffd0](https://github.com/AlexandreGallais/synoptic-studio/commit/23fffd0532b19135cba505f8535d876a0dd18f31))
* **backlog:** set EN-022 done, its departures from note 0012 ([ae139ba](https://github.com/AlexandreGallais/synoptic-studio/commit/ae139bab4b6ddefc5c06652512f1335039ffc9a5))
* **backlog:** set EN-024 done ([36cdd00](https://github.com/AlexandreGallais/synoptic-studio/commit/36cdd00c812118f3e1036403a5a1892b06d4c1ef))
* **backlog:** set EN-025 done ([9fc932a](https://github.com/AlexandreGallais/synoptic-studio/commit/9fc932aef62421150c2ec05cf5d52b2dbf301919))
* **backlog:** set EN-026 done ([5999e36](https://github.com/AlexandreGallais/synoptic-studio/commit/5999e360e2159841b4daa5c9ada5641440d84789))
* **backlog:** set F11's stories ready, F11 and E14 in progress ([c3a2637](https://github.com/AlexandreGallais/synoptic-studio/commit/c3a2637d724ee4d7500a5ae4e0020f4b57c85692))
* **backlog:** set F12 in progress and SP-011 ready, run F12 authorized ([e8d89a2](https://github.com/AlexandreGallais/synoptic-studio/commit/e8d89a2d05aa5631707e1bcbe6e2d446fe6ac9c1))
* **backlog:** set SP-010 done ([f5707e1](https://github.com/AlexandreGallais/synoptic-studio/commit/f5707e1096fd23b415dc96f1169e9dc2dbf3ab12))
* **backlog:** set SP-011 done ([4baabb5](https://github.com/AlexandreGallais/synoptic-studio/commit/4baabb51d0bc3d0fc556654851a6d41d781d501d))
* **backlog:** set SP-012 done, US-027 and US-028 ready ([923cddb](https://github.com/AlexandreGallais/synoptic-studio/commit/923cddba2132c99d2db038c7f344a988b16833c4))
* **backlog:** set US-022 done, its test card as the page shows it ([93a927c](https://github.com/AlexandreGallais/synoptic-studio/commit/93a927c225a5bfc2b0aa262ae47c16de295bb8a9))
* **backlog:** set US-023 done ([1d01197](https://github.com/AlexandreGallais/synoptic-studio/commit/1d0119724528b7a8fc4d3f21d33727673a7e6341))
* **backlog:** set US-024 done ([6fb121b](https://github.com/AlexandreGallais/synoptic-studio/commit/6fb121b028d8590d26d5cd01277542b7b4615533))
* **backlog:** set US-025 done ([bcb336f](https://github.com/AlexandreGallais/synoptic-studio/commit/bcb336f5501b9e858dedc4ed6db1cb1205683085))
* **backlog:** set US-026 done ([08d5984](https://github.com/AlexandreGallais/synoptic-studio/commit/08d598455ca63263901a5c7942a477e9f6a20246))
* **backlog:** set US-027 done ([3fd1177](https://github.com/AlexandreGallais/synoptic-studio/commit/3fd11774ae5b97349246dbbe8a94a25f8f8ea7b9))
* **backlog:** set US-028 done ([9f75123](https://github.com/AlexandreGallais/synoptic-studio/commit/9f75123daccb9d7868c3d85f83882f4a6db168ac))
* **backlog:** set VAL-010 and F11 done, validated by the Product Owner ([a47bf3f](https://github.com/AlexandreGallais/synoptic-studio/commit/a47bf3f0027d33defc61141709cefda4f9f81a34))
* **backlog:** the audit of F10, its findings fixed or sent to F13 ([f71ec04](https://github.com/AlexandreGallais/synoptic-studio/commit/f71ec0452b25575c590adbf0f1a42bb052825de1))
* **backlog:** the audit of F13, its findings fixed or recorded ([0f844c6](https://github.com/AlexandreGallais/synoptic-studio/commit/0f844c6f93f8eb7952ae2f37e43241e9692972ec))
* **backlog:** the checkpoint between the two passes of F13 ([57986d7](https://github.com/AlexandreGallais/synoptic-studio/commit/57986d7798a9f02d7abcb3ac98ade2e2c39627cb))
* **backlog:** the checkpoint in the middle of F10 ([ade6f3e](https://github.com/AlexandreGallais/synoptic-studio/commit/ade6f3ed99d93e6c4ac1cb79420207248a680396))
* **backlog:** the demo page and tasks of VAL-011 ([eca1f92](https://github.com/AlexandreGallais/synoptic-studio/commit/eca1f92e6d97ad77e4e3b73d4e9f549c45edabc9))
* **backlog:** the demo page of F12 as a list of its options ([7d6c1a3](https://github.com/AlexandreGallais/synoptic-studio/commit/7d6c1a31dba40f2cfb31cf05d776bc294d1c094c))
* **backlog:** the demo page of F13 ([c2688bb](https://github.com/AlexandreGallais/synoptic-studio/commit/c2688bbf92e1a9e01d37c27d1f8458c0aca48133))
* **backlog:** the idea of a point 1 alike in every shape, in E10 ([f3485d9](https://github.com/AlexandreGallais/synoptic-studio/commit/f3485d9851c72067eaddd72848335585bc7d5998))
* **backlog:** the tasks of RET-002 and the journal of the retrospective ([e0ca089](https://github.com/AlexandreGallais/synoptic-studio/commit/e0ca08940d790ca4ba2df94b51439c2ee683891d))
* **backlog:** the test card and decisions of US-022 ([da79282](https://github.com/AlexandreGallais/synoptic-studio/commit/da79282e2f12bba414cbafbf4eb77d5bf6c78133))
* **backlog:** the test card and decisions of US-023 ([4421884](https://github.com/AlexandreGallais/synoptic-studio/commit/44218845652af227092c9e4660386c519b3779b7))
* **backlog:** the test card and decisions of US-024 ([a4a9688](https://github.com/AlexandreGallais/synoptic-studio/commit/a4a96888b6dca870e7a53435d2e716803b77b756))
* **backlog:** the test card and decisions of US-025 ([b867b09](https://github.com/AlexandreGallais/synoptic-studio/commit/b867b09991d46aa11ebd4394d41f85e9b10d37db))
* **backlog:** the test card and decisions of US-026 ([37aa05e](https://github.com/AlexandreGallais/synoptic-studio/commit/37aa05e86c79acb8d98eaf5bba96a328fa591fa5))
* **backlog:** the test count of CHK-012 ([1779c52](https://github.com/AlexandreGallais/synoptic-studio/commit/1779c528a1aca062fa1b3c8e13d692c80ce00373))
* **backlog:** write the tasks and decisions of EN-024 ([200d154](https://github.com/AlexandreGallais/synoptic-studio/commit/200d1540892bb9ce1d82b0c012ff6c7b18e07dba))
* **backlog:** write the tasks and decisions of EN-025 ([386c7f2](https://github.com/AlexandreGallais/synoptic-studio/commit/386c7f256668f01c8ee420f45e73ca3d008d1d5c))
* **docs:** a folder limit asked before the themes, in the feedback log ([34753aa](https://github.com/AlexandreGallais/synoptic-studio/commit/34753aaa0a4ad83a76d7b1b48f50610ec693b018))
* **docs:** apply the review of SP-011, a seventh way, the curve's box ([3b81bfc](https://github.com/AlexandreGallais/synoptic-studio/commit/3b81bfc8ee5f34ef7c246bb7026ed33b3f627ca0))
* **docs:** demo page of F11, the figures before and after ([1dd2f05](https://github.com/AlexandreGallais/synoptic-studio/commit/1dd2f05275308b51249a14265a68c94a870c0a4d))
* **docs:** diagram labels that fit their boxes ([efa15c1](https://github.com/AlexandreGallais/synoptic-studio/commit/efa15c16b3ba69e81794fa257df4088fbad4658c))
* **docs:** journal of F11 resumed, VAL-010 prepared ([55551ed](https://github.com/AlexandreGallais/synoptic-studio/commit/55551edcaaf3eabc933c7fdae79be0623cb2c322))
* **docs:** journal of the F11 run, stopped before VAL-010 ([c4e3419](https://github.com/AlexandreGallais/synoptic-studio/commit/c4e3419f6a7ea8294ab1b45ec2456bc04eb9c2fe))
* **docs:** journal of the F12 run, stopped before VAL-011 ([46e9478](https://github.com/AlexandreGallais/synoptic-studio/commit/46e94780944e4d12f5808ada6148fd57bc1edfdb))
* **docs:** journal of the restart, and the shell loop pitfall ([e2ab5f7](https://github.com/AlexandreGallais/synoptic-studio/commit/e2ab5f7dd1479ef01ab6526d2d7e3dc0a9e2bc92))
* **docs:** lint scopes of the barrels, worktrees install their hooks ([79e302e](https://github.com/AlexandreGallais/synoptic-studio/commit/79e302e0649d86824a0b96b8c27fc45f2dc3031b))
* **docs:** log the status line critique, its cause found ([fcfeeaa](https://github.com/AlexandreGallais/synoptic-studio/commit/fcfeeaa27984b7e69255d8ed18c6f73481bd66c4))
* **docs:** metrics per caller and step, faster tools, wider sources ([92f23c7](https://github.com/AlexandreGallais/synoptic-studio/commit/92f23c74e347ee0adf20c23289b122a319bd8d4b))
* **docs:** no waiting loop on a subagent, every background task stopped at the stop ([b16c9db](https://github.com/AlexandreGallais/synoptic-studio/commit/b16c9dbf043f8b1266a757d386e146457c5c44d4))
* **docs:** points with handles as tables, F12 rules everywhere ([ac9b935](https://github.com/AlexandreGallais/synoptic-studio/commit/ac9b93544778285eea8e30a025683fe1ccd81c27))
* **docs:** research note 0013, six ways to type a handle ([476db45](https://github.com/AlexandreGallais/synoptic-studio/commit/476db450e0a11074ca831504bfa4c51611b92418))
* **docs:** research note of F11, a layout by theme proposed ([11670f6](https://github.com/AlexandreGallais/synoptic-studio/commit/11670f6e33ad3fbcae77a9887582e2fd11be2c09))
* **docs:** sources of a fillet beside a curved side, research note 0014 ([623b1a5](https://github.com/AlexandreGallais/synoptic-studio/commit/623b1a579c44fed9ae247150cdf7db59c319ed40))
* **docs:** sources of F11, folders and test placement ([637fa98](https://github.com/AlexandreGallais/synoptic-studio/commit/637fa980d1e120e14f0ba17169c11674dce7f109))
* **docs:** the angle derivation counted from the side only ([bbd0697](https://github.com/AlexandreGallais/synoptic-studio/commit/bbd0697e0b5b4a8df834894d5d6081beac3254dd))
* **docs:** the barrels and the rule aggregators told apart in lint.md ([0cce7e1](https://github.com/AlexandreGallais/synoptic-studio/commit/0cce7e1fb4e0622361d9f95787d37b818391e07b))
* **docs:** the F10 code lives in pull request 139, the tag is gone ([97feb46](https://github.com/AlexandreGallais/synoptic-studio/commit/97feb465b72b995c83917f98f636716a16732980))
* **docs:** the F11 page counts 859 tests, as check:all ([d4868f3](https://github.com/AlexandreGallais/synoptic-studio/commit/d4868f39a525d76002f52d85ab1720b6676dc15d))
* **docs:** the feedback log without the row the rebase doubled ([2c97680](https://github.com/AlexandreGallais/synoptic-studio/commit/2c97680417a52cbe93a9229d64675621370a444a))
* **docs:** the fillet derivation's notation and the note's search ([589e838](https://github.com/AlexandreGallais/synoptic-studio/commit/589e838c45a284ee54a4e479a0b53666b1a938f6))
* **docs:** the improvement log aligned after bringing develop in ([cbb71df](https://github.com/AlexandreGallais/synoptic-studio/commit/cbb71dff407dc85c01aa12d7b517c70c33b517ef))
* **docs:** the journal of the F10 and F13 runs ([f7cf8a4](https://github.com/AlexandreGallais/synoptic-studio/commit/f7cf8a4857eb3aa05e51eb62d89e503acbfaa4f7))
* **docs:** the named calculations of F13 and their sources ([c9bedf5](https://github.com/AlexandreGallais/synoptic-studio/commit/c9bedf561d86ae9040b7c5da53b46aad64cfe645))
* **docs:** the pages F11 changed, as the documentation review asked ([3df9cd3](https://github.com/AlexandreGallais/synoptic-studio/commit/3df9cd3f826da90dceaf9b259f9669cbe12fb448))
* **docs:** the record of E14's retrospective, the logs emptied ([0e10640](https://github.com/AlexandreGallais/synoptic-studio/commit/0e10640f1735dd6236e64141c6e7ce7331ef2f5a))
* **docs:** the second pass of F13 in its research note ([e57272f](https://github.com/AlexandreGallais/synoptic-studio/commit/e57272f2016c35c9a19a5ed747e7ebe389698b26))
* **docs:** the sources of F12 on curves and handles ([8b6e167](https://github.com/AlexandreGallais/synoptic-studio/commit/8b6e1675a9e95376d96df49536d729d88bb83b60))
* **docs:** the status line page says a notification resumes the turn ([a1f0644](https://github.com/AlexandreGallais/synoptic-studio/commit/a1f064497670f5bd8b4c8adff60fa3287fa53059))
* **docs:** the two methods and three modes kept, Q32 settled ([65e4bbe](https://github.com/AlexandreGallais/synoptic-studio/commit/65e4bbe38c11c814e8281c64e33b469a0962e26a))
* **geometry:** the math review of SP-013 applied ([e5b0a81](https://github.com/AlexandreGallais/synoptic-studio/commit/e5b0a8171648fe95802bbe706085b7663082e37e))
* **geometry:** the round shape as four cubic sides, its sources and ADR-0048 ([cc2ac06](https://github.com/AlexandreGallais/synoptic-studio/commit/cc2ac06b6f3c11f3888beb548958be3f6f512180))
* **model:** the audit of EN-027 applied, a box of size 0 not converted ([6f28294](https://github.com/AlexandreGallais/synoptic-studio/commit/6f28294301d3d0e3d3dec2bcfcd93534658d0d41))
* the retrospective of E14 ([d0f11bd](https://github.com/AlexandreGallais/synoptic-studio/commit/d0f11bd96ec4e7937db5b7d4af838da45381ba01))
* **tooling:** the agent's tools, the record of each call, how many sources ([c35b533](https://github.com/AlexandreGallais/synoptic-studio/commit/c35b5331ad875f5ef9bc7156d0516886fcfb99d1))


### Build and dependencies

* **deps:** knip 6.40.0, its release notes read (bug fixes, plugins) ([0e41dff](https://github.com/AlexandreGallais/synoptic-studio/commit/0e41dff823e2b8f252c7408af22a2dc2d4a9b48d))

## 0.1.0 (2026-10-13)

A consolidated base after the first epic, E01 "Draw symbol shapes from numbers".

### Features

- Rectangles and regular polygons drawn from whole numbers, each corner rounded by its own radius, reduced to fit.
- A regular polygon anchored in its box at one of nine places, or stretched to fill it.
- Free shapes: points moved, added and deleted; the rectangle is one; decimals kept where nothing was typed.
- Any shape turned by any angle, with its turned and straight boxes, and edited while turned.
- The playground: a toolbar, grouped panels, the pipeline output, a reset, and a short guided test per feature.

### Tooling

- Checks run locally on every work branch; one CI, on the pull request into `main`.
- Metrics of the agent's work, a status line and a report of each session.
- A local library of the sources read, behind an MCP server.

The history before this version is summarized in [docs/process/history.md](./docs/process/history.md).
