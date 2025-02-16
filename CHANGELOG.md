# Changelog

## Unreleased

## 2.0.12 - 2025-02-17

- Add a test to ensure that `Error` satisfies `Send` and `Sync`.
- Publish tests. [`#179`](https://github.com/rust-syndication/rss/pull/179)

## 2.0.11 - 2024-11-22

- Fix `]]>` escaping in `CDATA` sections. [`#174`](https://github.com/rust-syndication/rss/pull/174)

## 2.0.10 - 2024-11-16

- Remove ambiguous statements about escaping from documentation. [`#171`](https://github.com/rust-syndication/rss/pull/171)
- Update `quick-xml` to 0.37. [`#172`](https://github.com/rust-syndication/rss/pull/172)

## 2.0.9 - 2024-08-28

- Fix Clippy Warnings. [`#164`](https://github.com/rust-syndication/rss/pull/164)
- Add `From<String>` constructor for `Category`. [`#165`](https://github.com/rust-syndication/rss/pull/165)
- Update `quick-xml` to 0.36. [`#166`](https://github.com/rust-syndication/rss/pull/166)

## 2.0.8 - 2024-05-11

- Update quick-xml and derive_builder dependencies. [`#162`](https://github.com/rust-syndication/rss/pull/162)

## 2.0.7 - 2024-01-13

- Update `chrono` to 0.4.31 [`#160`](https://github.com/rust-syndication/rss/pull/160)
- Change how iTunes extension is detected. Use case insensitive comparison of a namespace [`#159`](https://github.com/rust-syndication/rss/pull/159)

## 2.0.6 - 2023-08-12

- Take into account namespaces declared locally [`#155`](https://github.com/rust-syndication/rss/pull/155)

## 2.0.5 - 2023-07-26

- Upgrade `quick_xml` to `0.30` [`#153`](https://github.com/rust-syndication/rss/pull/153)

## 2.0.4 - 2023-05-29

- Fix iTunes extension writing [`#151`](https://github.com/rust-syndication/rss/pull/151)

## 2.0.3 - 2023-03-27

- Upgrade `quick_xml` to `0.28` [`#146`](https://github.com/rust-syndication/rss/pull/146)
- Switch to Rust 2021 [`#147`](https://github.com/rust-syndication/rss/pull/147)

## 2.0.2 - 2023-01-14

- Upgrade `quick_xml` to `0.27`, `derive_builder` to `0.12`, and `atom_syndication` to `0.12` [`#143`](https://github.com/rust-syndication/rss/pull/143)
- Correct serialization of atom extension [`#144`](https://github.com/rust-syndication/rss/pull/144)
- Read non-blank links only [`#145`](https://github.com/rust-syndication/rss/pull/145)

## 2.0.1 - 2022-04-17

- check if update_period and frequency are valid [`#135`](https://github.com/rust-syndication/rss/pull/135)

## 2.0.0 - 2021-10-21

- Disable clock feature of chrono to mitigate RUSTSEC-2020-0159 [`#130`](https://github.com/rust-syndication/rss/pull/130)
- Update quick_xml to 0.22 [`0daf20b`](https://github.com/rust-syndication/rss/commit/0daf20b6f19411450f79090d687d796414193327)
- Fix issues found by clippy [`f3283a1`](https://github.com/rust-syndication/rss/commit/f3283a13808f41f0c10cd64720e493f18a286967)
- Replace HashMap with BTreeMap to have a stable order of tags/attributes [`8b088b1`](https://github.com/rust-syndication/rss/commit/8b088b147c0801a950b5197d6faa475ca766f257)
- Update atom_syndication to 0.10.0 [`975e4aa`](https://github.com/rust-syndication/rss/commit/975e4aa9914985ff4af7ee9834294c15691f0b92)
- Infallible builders [`f736a24`](https://github.com/rust-syndication/rss/commit/f736a2480b3d13114f223048b36f47641bf64858)

## 1.10.0 - 2021-01-07

- Add the itunes channel "itunes:type" tag. [`#101`](https://github.com/rust-syndication/rss/pull/101)
- Add itunes season tag [`#100`](https://github.com/rust-syndication/rss/pull/100)
- Fix typo in item.rs [`#97`](https://github.com/rust-syndication/rss/pull/97)
- fix benches [`#96`](https://github.com/rust-syndication/rss/pull/96)
- make fields public [`#94`](https://github.com/rust-syndication/rss/pull/94)
- change badges [`#95`](https://github.com/rust-syndication/rss/pull/95)
- fix clippy warnings [`#93`](https://github.com/rust-syndication/rss/pull/93)
- migrate to github actions [`#91`](https://github.com/rust-syndication/rss/pull/91)
- remove from_url feature [`#88`](https://github.com/rust-syndication/rss/pull/88)
- remove deprecated Error description and cause [`#89`](https://github.com/rust-syndication/rss/pull/89)
- implement Atom extension [`6b6eac1`](https://github.com/rust-syndication/rss/commit/6b6eac1699ec63a7274f8ca0ad2088d5d4b38804)
- reformat all code [`72dbe42`](https://github.com/rust-syndication/rss/commit/72dbe42c42c49670b88a96957445d13bfed3bce7)
- initial github actions [`3557c96`](https://github.com/rust-syndication/rss/commit/3557c9606422a7e1670ab0f7e4188661b0c77955)

## 1.9.0 - 2020-01-23

- Add a default builders feature that can be disabled [`#83`](https://github.com/rust-syndication/rss/pull/83)
- Remove dependency on failure [`#82`](https://github.com/rust-syndication/rss/pull/82)
- migrate to 2018 edition [`a836e15`](https://github.com/rust-syndication/rss/commit/a836e158759f8a13763bd1078395a5f23207f194)
- Update dependencies [`e76ec24`](https://github.com/rust-syndication/rss/commit/e76ec245509be124d791f12c4eca28796cc96ac7)
- work with clippy and rustfmt [`b1a3ee3`](https://github.com/rust-syndication/rss/commit/b1a3ee3982244899ee77407104bee3791400444c)

## 1.8.0 - 2019-05-25

- Syndication support [`#78`](https://github.com/rust-syndication/rss/pull/78)
- Bump quick-xml to 0.14 [`#77`](https://github.com/rust-syndication/rss/pull/77)
- Add support for RSS syndication module [`baa9b36`](https://github.com/rust-syndication/rss/commit/baa9b3636364f47648b287d352f010ed01fb87cd)
- Parse via namespace and prefix [`91c0c03`](https://github.com/rust-syndication/rss/commit/91c0c03d58d26b775a6e9c634f482be18163b8ee)
- Static analysis fixes [`4b975c7`](https://github.com/rust-syndication/rss/commit/4b975c76c78f0eb21f43429d329f57c821ab5124)

## 1.7.0 - 2019-03-26

- read url to bytes buffer instead of a string [`#76`](https://github.com/rust-syndication/rss/pull/76)
- Prepare for 1.7.0 release [`3a551a6`](https://github.com/rust-syndication/rss/commit/3a551a664354f5bdcd7b396e634a131c796f062c)

## 1.6.1 - 2018-11-18

- Update derive_builder and bump version [`#73`](https://github.com/rust-syndication/rss/pull/73)
- Add badge from deps.rs and prepare for 1.6.1 release [`59bc1ab`](https://github.com/rust-syndication/rss/commit/59bc1ab40f2677fe891cfa8f277dee2b55686868)

## 1.6.0 - 2018-10-13

- Update to reqwest version 0.9.2 [`#72`](https://github.com/rust-syndication/rss/pull/72)
- Update dependencies and prepare for 1.6 release [`705fa6f`](https://github.com/rust-syndication/rss/commit/705fa6f616cdd621fe366ef688532cee2f4341dd)
- updated reqwest version [`8fc6a83`](https://github.com/rust-syndication/rss/commit/8fc6a83b658cb180eb602d7ead20b446371a2111)
- fixed typo [`e566647`](https://github.com/rust-syndication/rss/commit/e5666477612e46ada6ab6574073851f3a1588917)

## 1.5.0 - 2018-04-18

- Prepare for 1.5.0 release. [`97ffcd4`](https://github.com/rust-syndication/rss/commit/97ffcd474eec332523634debfe3aa2efa27032f7)
- Merge #68 [`2cee413`](https://github.com/rust-syndication/rss/commit/2cee4136629dc57ee00fe8a41ad31a42d62b6a0e)
- Channel: Add items_owned() method [`b4a5ff5`](https://github.com/rust-syndication/rss/commit/b4a5ff563bf3ab6f1c83b5c79e625ff0292aa480)

## 1.4.0 - 2018-03-10

- Make the output prettier. [`#66`](https://github.com/rust-syndication/rss/pull/66)
- rustfmt. [`67724b3`](https://github.com/rust-syndication/rss/commit/67724b3b361357b016a23421718fde91c8a1d1dc)
- RustFmt the code. [`fbd8334`](https://github.com/rust-syndication/rss/commit/fbd83344ecbd200d2e81f4d241676fc49f4074a4)
- Bump quick_xml: 0.11 -&gt; 0.12. [`b22649e`](https://github.com/rust-syndication/rss/commit/b22649e9d51c8c9f0ceb5eb5d6898ba0da14abb2)

## 1.3.0 - 2018-02-11

- Add opt-in serde serialization. [`#55`](https://github.com/rust-syndication/rss/pull/55)

## 1.2.1 - 2017-11-27

- Update quick-xml to 0.10.0 [`#64`](https://github.com/rust-syndication/rss/pull/64)

## 1.2.0 - 2017-11-27

- Prepare for 1.2.0 release. [`80824a1`](https://github.com/rust-syndication/rss/commit/80824a186d6140afe88134e4953546d8bd9af0c6)

## 1.1.0 - 2017-09-10

- Merge #62 [`ac6ea2c`](https://github.com/rust-syndication/rss/commit/ac6ea2c51eeb8b413fa330ea31e5ea36a45116d6)
- Add mutable slice variants of all slice-returning methods. [`6311dac`](https://github.com/rust-syndication/rss/commit/6311daca6fdec5e9a6574e7ac8f0641f147e97a1)

### 1.0.0 - 2017-08-28

- Merge #56 [`#32`](https://github.com/rust-syndication/rss/issues/32)
- Merge #60 [`5709f23`](https://github.com/rust-syndication/rss/commit/5709f2324d9f874cad5911ae33369d0deef64b5d)
- Merge #58 #59 [`0d21138`](https://github.com/rust-syndication/rss/commit/0d21138a257e7785192d8116611cbac4dc0b4bfe)
- Add regression test for escaped test. [`321d0e9`](https://github.com/rust-syndication/rss/commit/321d0e966145d3c3ec14d917aa6a73b2709c6e89)

## 0.7.0 - 2017-07-12

- Added setters to structs [`#52`](https://github.com/rust-syndication/rss/pull/52)
- Format using rustfmt-nightly [`#50`](https://github.com/rust-syndication/rss/pull/50)
- Update readme to match lib doc [`#48`](https://github.com/rust-syndication/rss/pull/48)
- Added constructors to builders that consume their immutable counterparts [`#45`](https://github.com/rust-syndication/rss/pull/45)
- Fix some typos [`#47`](https://github.com/rust-syndication/rss/pull/47)
- Added constructors to builders that consume their immutable counterparts [`#41`](https://github.com/rust-syndication/rss/issues/41)
- Add setters to structs [`260623c`](https://github.com/rust-syndication/rss/commit/260623c15570176d14e37caaf5b7bf74b8a48666)
- Reformatted with rustfmt 0.9.0 [`f3a3de8`](https://github.com/rust-syndication/rss/commit/f3a3de8675859698c24892175196fb4d9a042e5a)
- Added validation module [`9a755f9`](https://github.com/rust-syndication/rss/commit/9a755f93eab2ac20b73942fbd14d66f9aa4af350)

## 0.6.0 - 2017-05-30

- Added read support for RSS 0.90, 0.91, 0.92, 1.0 [`#40`](https://github.com/rust-syndication/rss/pull/40)
- Shortened some really long example strings, more validation docs [`#43`](https://github.com/rust-syndication/rss/pull/43)
- Add rating for from xml and to xml [`#44`](https://github.com/rust-syndication/rss/pull/44)
- Revised docs and made builders a bit nicer to use [`#42`](https://github.com/rust-syndication/rss/pull/42)
- Doc Updates [`#39`](https://github.com/rust-syndication/rss/pull/39)
- Documentation cleanup, removed new() methods on builders in favor of default() [`c0760d9`](https://github.com/rust-syndication/rss/commit/c0760d909f0386edee96fe6b599428ebe36195ca)
- Changed finalize to return T instead of Result&lt;T, E&gt; [`a62f972`](https://github.com/rust-syndication/rss/commit/a62f97251e343ee7129236e5a1c6a5bab4afa482)
- Changed all builder methods to take Into&lt;T&gt; [`83e478b`](https://github.com/rust-syndication/rss/commit/83e478b382b24731abf1e7b96854c111e37bd7da)

## 0.5.1 - 2017-05-28

- Upgrade to quick-xml 0.7.3 [`#36`](https://github.com/rust-syndication/rss/pull/36)
- upgrade to quick-xml 0.6.0 [`e951c6b`](https://github.com/rust-syndication/rss/commit/e951c6b1a15626b364884110b4769fdde6f232ae)
- refactor fromxml [`d7e73fc`](https://github.com/rust-syndication/rss/commit/d7e73fc7e24b4eaae44f06489fdeee0dfa9620e9)
- rustfmt [`9705dcf`](https://github.com/rust-syndication/rss/commit/9705dcf66a5c0be4e803d8a48b6d9803426b2579)

## 0.5.0 - 2017-05-22

- Cleanup following #35 [`#38`](https://github.com/rust-syndication/rss/pull/38)
- Merge feed functionality into rss. [`#35`](https://github.com/rust-syndication/rss/pull/35)
- Updates to extensions [`3feed5f`](https://github.com/rust-syndication/rss/commit/3feed5f78a1b6f6e66b053113e5aa62b0e736079)
- Use rustfmt defaults [`593b7aa`](https://github.com/rust-syndication/rss/commit/593b7aa8ef84e340b6f2311290a5531bcf10e9f1)
- Updates to Channel, [`902df10`](https://github.com/rust-syndication/rss/commit/902df101939b2d1ee04fb2560fa6607ea0558425)

## 0.4.0 - 2016-09-05

- Store and write namespaces [`#27`](https://github.com/rust-syndication/rss/pull/27)
- Fixed writing and parsing bug with &lt;rss&gt; tag [`#26`](https://github.com/rust-syndication/rss/pull/26)
- Writing support [`#25`](https://github.com/rust-syndication/rss/pull/25)
- Replace project codebase with rss-rs [`#24`](https://github.com/rust-syndication/rss/pull/24)
- Address suggestions made by rust-clippy [`#20`](https://github.com/rust-syndication/rss/pull/20)
- Add category read test [`#19`](https://github.com/rust-syndication/rss/pull/19)
- Removed previous files [`6c8aa8e`](https://github.com/rust-syndication/rss/commit/6c8aa8ea4791fdaa4a1690c814255318b5ec2cf5)
- Full implementation of RSS 2.0 specification [`fc1eee8`](https://github.com/rust-syndication/rss/commit/fc1eee84e7d971b22d0fbdb82d1cad3fbfa59e85)

## 0.3.1 - 2015-11-15

- Remove unnecessary unwraps [`#18`](https://github.com/rust-syndication/rss/pull/18)
- Get docs building again (requires sudo) [`#17`](https://github.com/rust-syndication/rss/pull/17)

## 0.3.0 - 2015-11-14

- Implement &lt;image&gt; struct [`#16`](https://github.com/rust-syndication/rss/pull/16)
- Implement &lt;image&gt; struct [`#13`](https://github.com/rust-syndication/rss/issues/13)

## 0.2.3 - 2015-10-31

- Implement item/guid spec [`#14`](https://github.com/rust-syndication/rss/pull/14)

## 0.2.2 - 2015-10-29

- Fix inaccurate reading of &lt;channel&gt; properties [`#12`](https://github.com/rust-syndication/rss/pull/12)
- Fix inaccurate reading of &lt;channel&gt; properties [`#11`](https://github.com/rust-syndication/rss/issues/11) [`#12`](https://github.com/rust-syndication/rss/issues/12)

## 0.2.1 - 2015-10-16

- Fix non sized warning for ViaXml [`#9`](https://github.com/rust-syndication/rss/pull/9)
- Make ViaXml sized [`9409223`](https://github.com/rust-syndication/rss/commit/9409223d20cc93d120636a4ed55b034c22626cd5)

## 0.2.0 - 2015-07-29

- Replace static str errors with an error enum [`#7`](https://github.com/rust-syndication/rss/pull/7) and [`#6`](https://github.com/rust-syndication/rss/issues/6)

## 0.1.2 - 2015-07-22

- Return Error instead of panicking when parsing something that isn't an RSS feed [`#5`](https://github.com/rust-syndication/rss/pull/5)
- Return Error instead of panicking [`ce89c6b`](https://github.com/rust-syndication/rss/commit/ce89c6b17c53ff3f8dcb9dc6ec2bc44ca97b43f9)
- Also test on Rust beta channel [`0a435b9`](https://github.com/rust-syndication/rss/commit/0a435b90b031d30fccf1d4431f8f30d5d22442bb)

## 0.1.1 - 2015-06-10

- Lock on to stable channel for travis [`#4`](https://github.com/rust-syndication/rss/pull/4)
- Add Apache license headers to source files [`001dc0d`](https://github.com/rust-syndication/rss/commit/001dc0d385ce7f5ca2261e1257c2098d2e94d4e8)
- Derive Debug, Clone for Channel, Item, Category, TextInput [`d29a4f2`](https://github.com/rust-syndication/rss/commit/d29a4f288977e78ff1f064d691424552cee9dae5)
- Update reading/writing examples to have same channel title [`cb37502`](https://github.com/rust-syndication/rss/commit/cb3750266a71c0ed6a010ae045a644f1c8cb5fc1)

## 0.1.0 - 2015-05-09

- Move things into submodule [`b834d0b`](https://github.com/rust-syndication/rss/commit/b834d0bf5c9892e7f149737071291d2903036fe7)
- Add more examples [`4851f17`](https://github.com/rust-syndication/rss/commit/4851f176c972cdbefd3fe14e7e71ca36dd3a4e44)
- Replace from_reader with from_str trait [`2bc231a`](https://github.com/rust-syndication/rss/commit/2bc231a8033ff81db6c3d47ea235223cdf1e671d)

## 0.0.7 - 2015-04-25

- Shorten 'element' variables to 'elem' [`163304b`](https://github.com/rust-syndication/rss/commit/163304b479a26970d2c609d3454058e8d98a28ef)
- Add pub_date field on Item [`c695fa0`](https://github.com/rust-syndication/rss/commit/c695fa00121b41c987e98e6d90b86bdd108de67e)

## 0.0.6 - 2015-04-23

- Save &lt;textInput&gt; upon export [`37eb057`](https://github.com/rust-syndication/rss/commit/37eb057d64476f12eae1ce315e38b79fcbcc1b61)

## 0.0.5 - 2015-04-23

- Implement &lt;textInput&gt; sub-element of &lt;channel&gt; [`c6e961f`](https://github.com/rust-syndication/rss/commit/c6e961f2d61c58cf24d357c587f1070269019ba0)
- Clean up tests a little bit [`0bd8516`](https://github.com/rust-syndication/rss/commit/0bd8516b83b961919bb5e0925420cab6f8eee8fc)
- Add test ensuring PI XML node gets ignored [`298c830`](https://github.com/rust-syndication/rss/commit/298c8303b79509bbfd735a67287ee51ce925531f)

## 0.0.4 - 2015-04-16

- Add license [`c7d935e`](https://github.com/rust-syndication/rss/commit/c7d935eabecfbfbe762a3270c0c695434081db8c)
- Rename constructor [`884439a`](https://github.com/rust-syndication/rss/commit/884439aeba805a655749734a8e5ce8fe8156814b)

## 0.0.3 - 2015-04-10

- Rename method to be more descriptive [`152a168`](https://github.com/rust-syndication/rss/commit/152a168d77283bdecb683ef839482fea618430f2)
- Read Channel properties from xml [`a9be5ec`](https://github.com/rust-syndication/rss/commit/a9be5ecd02112669e8912660a7df3c85f9ab4eb7)
- Simplify a few Option::map [`f9f475b`](https://github.com/rust-syndication/rss/commit/f9f475b0ae00ecfd1fd67fefd7cde945b620a451)

## 0.0.2 - 2015-03-31

- Implement Rss.from_str [`#1`](https://github.com/rust-syndication/rss/pull/1)
- Add basic test to read RSS file [`39b23d5`](https://github.com/rust-syndication/rss/commit/39b23d5c8b62956b93029bb599046b40f455f90a)
- Swap xml-rs out with rustyxml [`f261c40`](https://github.com/rust-syndication/rss/commit/f261c4033fab8ecc110e7a855a242290d63c3437)
