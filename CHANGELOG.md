# Changelog

All notable changes to `jql` are documented here.

## [unreleased]

### Breaking

- runner: mark JqlRunnerError non-exhaustive
- parser: mark JqlParserError non-exhaustive
- bin: evaluate every document in the input
- parser: resolve JSON escape sequences in key selectors

### Features

- runner: lazy tape-based evaluator for drill-down queries
- runner: evaluate selection operators on the tape
- runner: evaluate a single pipe on the tape; fuzz both evaluators
- runner: evaluate lens selectors on the tape

### Bug fixes

- runner: report the first failing group, deterministically
- bin: read the file given without a query
- parser: accept the same whitespace in array and object ranges

### Performance

- runner: drop rayon where it costs more than it saves
- runner: make the tape path pay only for what it uses

### Refactor

- parser: share one selector combinator between fragments and lenses

### Documentation

- add CLAUDE.md
- readme: multi-document input is evaluated on the tape
- runner: correct the lazy runner's documentation
- readme: correct the parallelism and rounding claims

### Testing

- runner: assert the tape is the correctly rounded side

### Miscellaneous

- performance: benchmark equal work on both sides
- runner: benchmark the lazy tape evaluator
## [8.3.0] - 2026-09-07

### Performance

- bin: drop tokio, use blocking std I/O ([#317](https://github.com/yamafaktory/jql/pull/317))
## [8.1.3] - 2026-09-07

### Miscellaneous

- update dependencies, toolchain config and CI actions ([#314](https://github.com/yamafaktory/jql/pull/314))
## [8.1.2] - 2026-03-18

### Bug fixes

- loongarch64: don't use unsupported mimalloc ([#312](https://github.com/yamafaktory/jql/pull/312))
## [8.1.1] - 2026-03-18

### Features

- bin: use mimalloc for perf ([#310](https://github.com/yamafaktory/jql/pull/310))
## [8.1.0] - 2026-03-18

### Performance

- reduce allocations and serial-threshold Rayon for small inputs ([#308](https://github.com/yamafaktory/jql/pull/308))
## [8.0.10] - 2026-01-21

### Miscellaneous

- cargo: update dependencies ([#305](https://github.com/yamafaktory/jql/pull/305))
## [8.0.9] - 2025-10-10

### Miscellaneous

- cargo: update dependencies ([#303](https://github.com/yamafaktory/jql/pull/303))
## [8.0.8] - 2025-09-03

### Miscellaneous

- cargo: update dependencies ([#301](https://github.com/yamafaktory/jql/pull/301))
## [8.0.7] - 2025-06-24

### Miscellaneous

- cargo: update dependencies and switch to latest edition ([#299](https://github.com/yamafaktory/jql/pull/299))
## [8.0.6] - 2025-05-02

### Miscellaneous

- cargo: update dependencies ([#297](https://github.com/yamafaktory/jql/pull/297))
## [8.0.5] - 2025-04-07

### Miscellaneous

- cargo: update dependencies ([#295](https://github.com/yamafaktory/jql/pull/295))
## [8.0.4] - 2025-03-03

### Miscellaneous

- cargo: update dependencies ([#292](https://github.com/yamafaktory/jql/pull/292))
## [8.0.3] - 2025-02-03

### Miscellaneous

- cargo: update dependencies ([#290](https://github.com/yamafaktory/jql/pull/290))
## [8.0.2] - 2024-12-06

### Miscellaneous

- cargo: update dependencies ([#288](https://github.com/yamafaktory/jql/pull/288))
## [8.0.1] - 2024-11-07

### Miscellaneous

- cargo: update dependencies ([#285](https://github.com/yamafaktory/jql/pull/285))
## [7.2.0] - 2024-09-23

### Bug fixes

- cargo: try to fix release

### Miscellaneous

- cargo: update dependencies ([#280](https://github.com/yamafaktory/jql/pull/280))
- lock: update lock
- lock: update lock
## [7.1.13] - 2024-07-10

### Miscellaneous

- cargo: update dependencies ([#278](https://github.com/yamafaktory/jql/pull/278))
## [7.1.12] - 2024-06-21

### Miscellaneous

- cargo: update dependencies ([#276](https://github.com/yamafaktory/jql/pull/276))
## [7.1.11] - 2024-06-01

### Bug fixes

- key-order: adjust tests and code to properly keep ordered keys ([#273](https://github.com/yamafaktory/jql/pull/273))
## [7.1.10] - 2024-05-28

### Miscellaneous

- release: update lock file and add loongarch64 binary ([#271](https://github.com/yamafaktory/jql/pull/271))
## [7.1.8] - 2024-04-24

### Miscellaneous

- cargo: update dependencies ([#266](https://github.com/yamafaktory/jql/pull/266))
## [7.1.7] - 2024-04-01

### Miscellaneous

- cargo: update dependencies ([#263](https://github.com/yamafaktory/jql/pull/263))
## [7.1.6] - 2024-03-01

### Miscellaneous

- cargo: update dependencies ([#260](https://github.com/yamafaktory/jql/pull/260))
## [7.1.5] - 2024-02-16

### Bug fixes

- release: don't use cross for Darwin and Windows builds ([#258](https://github.com/yamafaktory/jql/pull/258))
## [7.1.4] - 2024-02-16

### Miscellaneous

- cargo: update dependencies ([#256](https://github.com/yamafaktory/jql/pull/256))
## [7.1.3] - 2024-01-19

### Miscellaneous

- cargo: update dependencies ([#252](https://github.com/yamafaktory/jql/pull/252))
## [7.1.2] - 2023-12-20

### Miscellaneous

- cargo: update dependencies ([#250](https://github.com/yamafaktory/jql/pull/250))
## [7.1.1] - 2023-12-06

### Miscellaneous

- jql-parser: improve parser implementation based on @epage 's feedback ([#247](https://github.com/yamafaktory/jql/pull/247))
- cargo: update dependencies ([#248](https://github.com/yamafaktory/jql/pull/248))
## [7.1.0] - 2023-12-03

### Miscellaneous

- jql-parser: switch to winnow ([#245](https://github.com/yamafaktory/jql/pull/245))
## [7.0.7] - 2023-11-25

### Miscellaneous

- cargo: update dependencies ([#242](https://github.com/yamafaktory/jql/pull/242))
## [7.0.6] - 2023-11-04

### Miscellaneous

- github: update workflows ([#240](https://github.com/yamafaktory/jql/pull/240))
## [7.0.5] - 2023-11-03

### Miscellaneous

- cargo: update dependencies ([#238](https://github.com/yamafaktory/jql/pull/238))
## [7.0.4] - 2023-10-02

### Miscellaneous

- cargo: update dependencies and fix clippy hints ([#234](https://github.com/yamafaktory/jql/pull/234))
## [7.0.2] - 2023-07-18

### Miscellaneous

- cargo: update dependencies and fix clippy hint ([#229](https://github.com/yamafaktory/jql/pull/229))
## [7.0.1] - 2023-07-13

### Miscellaneous

- cargo: update dependencies ([#227](https://github.com/yamafaktory/jql/pull/227))
## [7.0.0] - 2023-07-01

### Miscellaneous

- lenses: allow subset of selectors in lenses ([#225](https://github.com/yamafaktory/jql/pull/225))
## [6.0.9] - 2023-06-12

### Miscellaneous

- cargo: bump up dependencies ([#222](https://github.com/yamafaktory/jql/pull/222))
## [6.0.8] - 2023-05-18

### Miscellaneous

- cargo: bump up dependencies ([#220](https://github.com/yamafaktory/jql/pull/220))
## [6.0.7] - 2023-05-07

### Bug fixes

- win: fix Windows release ([#218](https://github.com/yamafaktory/jql/pull/218))
## [6.0.4] - 2023-04-22

### Bug fixes

- array+release: only return single index and adjust release artifacts ([#209](https://github.com/yamafaktory/jql/pull/209))
## [6.0.3] - 2023-04-21

### Miscellaneous

- release: adjust release workflow
## [6.0.2] - 2023-04-21

### Bug fixes

- version: try to align to publish runner

### Miscellaneous

- version: final alignment
- release: prepare for release
- release: prepare for release
- release: add tag
## [5.1.7] - 2023-03-05

### Miscellaneous

- cargo: bump up dependencies ([#201](https://github.com/yamafaktory/jql/pull/201))
## [5.1.6] - 2023-01-27

### Bug fixes

- bumpalo: update lock file to fix bumpalo use-after-free vulnerability ([#199](https://github.com/yamafaktory/jql/pull/199))
## [5.1.5] - 2023-01-27

### Miscellaneous

- cargo: bump up dependencies ([#197](https://github.com/yamafaktory/jql/pull/197))
## [5.1.4] - 2022-12-18

### Bug fixes

- badge: adjust readme ([#194](https://github.com/yamafaktory/jql/pull/194))

### Documentation

- design: update design guidelines ([#193](https://github.com/yamafaktory/jql/pull/193))
## [5.1.3] - 2022-11-26

### Miscellaneous

- cargo+contribute-design: bump up dependencies + add contribute design ([#191](https://github.com/yamafaktory/jql/pull/191))
## [5.1.1] - 2022-10-15

### Miscellaneous

- cargo: bump up dependencies ([#186](https://github.com/yamafaktory/jql/pull/186))
## [5.1.0] - 2022-10-04

### Testing

- group_walker: Add tests ([#182](https://github.com/yamafaktory/jql/pull/182))

### Miscellaneous

- cargo: bump up dependencies ([#184](https://github.com/yamafaktory/jql/pull/184))
## [5.0.2] - 2022-09-10

### Miscellaneous

- docs: fix typos in README.md ([#179](https://github.com/yamafaktory/jql/pull/179))
- cargo: bump up dependencies ([#180](https://github.com/yamafaktory/jql/pull/180))
## [5.0.1] - 2022-09-02

### Miscellaneous

- cargo: update dependencies ([#177](https://github.com/yamafaktory/jql/pull/177))
## [4.0.7] - 2022-08-09

### Miscellaneous

- cargo: bump up dependencies ([#172](https://github.com/yamafaktory/jql/pull/172))
## [4.0.6] - 2022-07-08

### Testing

- apply_filter: Add tests ([#167](https://github.com/yamafaktory/jql/pull/167))

### Miscellaneous

- cargo+doc: update doc for Alpine and dependencies ([#168](https://github.com/yamafaktory/jql/pull/168))
## [4.0.5] - 2022-06-23

### Miscellaneous

- cargo: update dependencies ([#165](https://github.com/yamafaktory/jql/pull/165))
## [4.0.4] - 2022-05-24

### Miscellaneous

- dependencies: bump up versions ([#163](https://github.com/yamafaktory/jql/pull/163))
## [4.0.3] - 2022-04-30

### Bug fixes

- perf: adjust toolchain ([#161](https://github.com/yamafaktory/jql/pull/161))
## [4.0.2] - 2022-04-28

### Miscellaneous

- fmt+actions: update rustfmt config and GitHub workflows ([#159](https://github.com/yamafaktory/jql/pull/159))
- cargo: bump up dependencies' versions ([#160](https://github.com/yamafaktory/jql/pull/160))
## [4.0.1] - 2022-04-14

### Bug fixes

- lens: fix issue with filter and add test ([#157](https://github.com/yamafaktory/jql/pull/157))
## [3.2.1] - 2022-04-03

### Miscellaneous

- deps: bump up clap version
## [3.2.0] - 2022-03-29

### Features

- flag `--from-file` ([#136](https://github.com/yamafaktory/jql/pull/136))

### Miscellaneous

- pre-release: adjust code and update docs ([#138](https://github.com/yamafaktory/jql/pull/138))
## [3.1.2] - 2022-03-14

### Bug fixes

- array_walker: add fix and bump up deps ([#130](https://github.com/yamafaktory/jql/pull/130))

### Miscellaneous

- readme: update aur package maintainer ([#128](https://github.com/yamafaktory/jql/pull/128))
## [3.1.1] - 2022-02-19

### Miscellaneous

- cargo: update dependencies ([#126](https://github.com/yamafaktory/jql/pull/126))
## [3.1.0] - 2022-02-06

### Features

- lib: expose more methods ([#124](https://github.com/yamafaktory/jql/pull/124))
## [3.0.9] - 2022-02-01

### Bug fixes

- serde: use feature and adjust deserializer accordingly to disable recursion limit ([#121](https://github.com/yamafaktory/jql/pull/121))
## [3.0.8] - 2022-01-29

### Bug fixes

- broken-pipe: add workaround, CI test and update dependencies ([#118](https://github.com/yamafaktory/jql/pull/118))

### Miscellaneous

- docs: add brew formulae
## [3.0.7] - 2022-01-15

### Miscellaneous

- edition: move to edition 2021 and update dependencies ([#116](https://github.com/yamafaktory/jql/pull/116))
## [3.0.6] - 2021-12-18

### Features

- lib: make clap and colored_json optional ([#114](https://github.com/yamafaktory/jql/pull/114))
## [3.0.5] - 2021-12-11

### Testing

- get_selectioin: Add tests ([#110](https://github.com/yamafaktory/jql/pull/110))
## [3.0.4] - 2021-11-10

### Testing

- range_selector: Add tests ([#105](https://github.com/yamafaktory/jql/pull/105))
- array_walker: Add tests ([#106](https://github.com/yamafaktory/jql/pull/106))

### Miscellaneous

- cargo: update dependencies ([#107](https://github.com/yamafaktory/jql/pull/107))
## [3.0.3] - 2021-11-02

### Miscellaneous

- cargo: update dependencies
## [3.0.2] - 2021-10-14

### Bug fixes

- check: make flag exclusive and add test ([#101](https://github.com/yamafaktory/jql/pull/101))
## [3.0.0] - 2021-09-20

### Miscellaneous

- git: switch from master to main
## [2.9.5] - 2021-08-24

### Miscellaneous

- cargo: update dependencies
## [2.9.4] - 2021-04-09

### Miscellaneous

- cargo: bump up anyhow version ([#87](https://github.com/yamafaktory/jql/pull/87))
## [2.9.3] - 2021-02-03

### Miscellaneous

- cargo: bump up dependencies ([#84](https://github.com/yamafaktory/jql/pull/84))
## [2.9.2] - 2021-01-18

### Miscellaneous

- bump up dependencies
- fix clippy hint
## [2.8.2] - 2020-12-09

### Miscellaneous

- bump up action version ([#76](https://github.com/yamafaktory/jql/pull/76))
## [2.8.1] - 2020-12-09

### Miscellaneous

- fix bad ouput in readme example ([#74](https://github.com/yamafaktory/jql/pull/74))
## [2.7.4] - 2020-10-15

### Miscellaneous

- update dependencies ([#67](https://github.com/yamafaktory/jql/pull/67))
## [2.7.3] - 2020-09-30

### Miscellaneous

- remove lock on criterion version and update ([#65](https://github.com/yamafaktory/jql/pull/65))
## [2.7.1] - 2020-08-25

### Miscellaneous

- fix exit code on errors ([#60](https://github.com/yamafaktory/jql/pull/60))
## [2.7.0] - 2020-08-07

### Miscellaneous

- add raw-ouput flag ([#53](https://github.com/yamafaktory/jql/pull/53))
## [2.6.6] - 2020-08-06

### Miscellaneous

- use license instead of license-file ([#51](https://github.com/yamafaktory/jql/pull/51))
- revert version after cargo login failure
## [2.6.5] - 2020-07-02

### Miscellaneous

- fix the release workflow
## [2.6.4] - 2020-07-01

### Miscellaneous

- update - again - release workflow
## [2.6.3] - 2020-07-01

### Miscellaneous

- update deps & fix release workflow
## [2.6.2] - 2020-07-01

### Miscellaneous

- test
- another test - [skip ci]
- another test
- test again
- test
- test
- trying a different approach
- update readme ([#46](https://github.com/yamafaktory/jql/pull/46))
## [2.6.1] - 2020-05-27

### Miscellaneous

- readme: add Archlinux install instructions ([#42](https://github.com/yamafaktory/jql/pull/42))
## [2.6.0] - 2020-05-26

### Testing

- do not run benches on master only
- second try...
- try another syntax
- log context
- fix context

### Miscellaneous

- cargo: bump up version
- cargo: merge master
- update dependencies and add license 📖 ([#31](https://github.com/yamafaktory/jql/pull/31))
- adjust cargo configuration for license
- bump up version 🚀
- Put license specification back ([#34](https://github.com/yamafaktory/jql/pull/34))
- pre version: update deps & bump up version
- update dependencies 🚀
- drop travis.yml, update readme ([#37](https://github.com/yamafaktory/jql/pull/37))
- clean-up & drop unused shell script
- fix bench path & bump up version
- add lock file
- update dependencies and keywords ([#38](https://github.com/yamafaktory/jql/pull/38))
- add truncate grammar and logic ([#40](https://github.com/yamafaktory/jql/pull/40))

