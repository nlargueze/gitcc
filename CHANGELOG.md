# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.0] - 2023-05-02

### New features

- feat: added release notes generation [e04c14c](https://github.com/nlargueze/gitcc/commit/e04c14cd1c28fc246e7eb140d2fce898dea168ee)
- feat(changelog): added config for types included in the changelog [682534c](https://github.com/nlargueze/gitcc/commit/682534cbd1249c407050928e45866eef931a779e)
- feat(release): added allow_dirty option to release [e68c766](https://github.com/nlargueze/gitcc/commit/e68c7669bbeb45dbaa7c5ffd6c26f0dac654535b)
- feat(release): added option to commit the release explicitly [37a6017](https://github.com/nlargueze/gitcc/commit/37a601726076fd54330454cde1a8b6adb12ba734)
- feat: added message to hook scripts [fd5382d](https://github.com/nlargueze/gitcc/commit/fd5382db3e556feef23f08d4aed544602d16a95c)
- feat(hooks): added custom hooks [a5db2b6](https://github.com/nlargueze/gitcc/commit/a5db2b6a825305e5cfb450499e7756df578356d6)
- feat(release): added push workflow to release [4191740](https://github.com/nlargueze/gitcc/commit/4191740882ac93ea5d95415b55aed8665bbc8203)
- feat(release): added release command [c61f149](https://github.com/nlargueze/gitcc/commit/c61f149ae8d3c1cda3ab5eae8100e5b135e32715)
- feat(bump): added info line indicating not tagged [ebdd1f3](https://github.com/nlargueze/gitcc/commit/ebdd1f346cf8f47ff14fc6a02345ff3706dd49c8)
- feat: added change log commit url [f3195d3](https://github.com/nlargueze/gitcc/commit/f3195d35ad0645ded0f334e044598c452c7bf919)
- feat: added changelog generatiom [560f1d7](https://github.com/nlargueze/gitcc/commit/560f1d7cb76d87c0694e7d361f5603a824a12a76)
- feat: added lowercase checks [9cba0cc](https://github.com/nlargueze/gitcc/commit/9cba0ccd57060df83c529048286053566ebed0e6)
- feat: added commit parsing [7ba6a17](https://github.com/nlargueze/gitcc/commit/7ba6a171fea1d8b87e4da7a30e5441b7ff39996c)
- feat(commit): added ammend option [b2c2b0e](https://github.com/nlargueze/gitcc/commit/b2c2b0ee9dbc2c09441dfa47bf71531b5f0185f5)
- feat(commit): removed displayed commit message on push [eaef3ea](https://github.com/nlargueze/gitcc/commit/eaef3ea5b4c4bbe25de54dc252a8de0a9db36446)
- feat(commit): added push option to commit [1e4020c](https://github.com/nlargueze/gitcc/commit/1e4020ce88eed72c992a212f1d09f449b2888b21)

### Bug fixes

- fix: misc fixes [3536dee](https://github.com/nlargueze/gitcc/commit/3536deeaf659465ce0719874f2b5e4eb507bbc31)
- fix: fixed release notes format [d6b40d8](https://github.com/nlargueze/gitcc/commit/d6b40d8a356061c61dd80086fc7ed735f7e8d9b4)
- fix: changed type descriptions [386d686](https://github.com/nlargueze/gitcc/commit/386d68676ecf40e3ee53dfe5b32094e3d5d1c040)
- fix: added &#x27;v&#x27; prefix to git tags [5a4edbd](https://github.com/nlargueze/gitcc/commit/5a4edbd8c1bc303c0b322ae3949daf0b052455c5)
- fix: moved commit scope prompt [24b79cd](https://github.com/nlargueze/gitcc/commit/24b79cd1595aef5c74860a3c6cf854e65cb8ad1c)
- fix: fixed commit order in changelog [14ecefe](https://github.com/nlargueze/gitcc/commit/14ecefe36f0616dbb98ac7c016851d5af5dc0c47)
- fix: fixed get_tags when there is no tag in the repo [4b3273e](https://github.com/nlargueze/gitcc/commit/4b3273eb197499a69c91895f0a686cb1e01b0320)
- fix(hooks): fixed install-hooks script [3480daa](https://github.com/nlargueze/gitcc/commit/3480daa99da8839374a568aa34abf2d1c295485b)
- fix(bump): set current directory of custom bump commands [4a9d5bf](https://github.com/nlargueze/gitcc/commit/4a9d5bf871448061f21549381e7fdb078f342853)
- fix(bump): fixing bump script [3613d5b](https://github.com/nlargueze/gitcc/commit/3613d5b8f593aae0eaab603cc9513ebc0d6106c8)
- fix(bump): fixing bumping script [4182323](https://github.com/nlargueze/gitcc/commit/4182323dbadf78c81a3b22d0d799fa7a7bea020d)
- fix(bump): fixing cargo bump script [452e920](https://github.com/nlargueze/gitcc/commit/452e920b605ec512d0c1720d01d5deb209496512)
- fix: fixed Cargo.lock file [1625754](https://github.com/nlargueze/gitcc/commit/16257542d1ee8159359ce842b45367549c2bdf60)
- fix(bump): fixedcustom bumop scripts execution [c80f9e2](https://github.com/nlargueze/gitcc/commit/c80f9e2b2373bd7e27e94eaaebe6633e7908f05e)
- fix(bump): added cargo bump to repo gitx config [54bc79e](https://github.com/nlargueze/gitcc/commit/54bc79ea7a08f72e7cf35d21b1c8111fb44abdd9)
- fix(release): fixed typo [351e8b2](https://github.com/nlargueze/gitcc/commit/351e8b277eaefab2d6386145e3250a8eeafe9613)
- fix: fixed commit body not separated from the subject when fetching [ad664a0](https://github.com/nlargueze/gitcc/commit/ad664a09a140a65173223e265c21f7202ca78f25)
- fix(tags): fixed annotated tags not picking up the commit hashes [814a107](https://github.com/nlargueze/gitcc/commit/814a107c7a56985df823bf01a97a024adaf95b2a)
- fix(config): config looked up recursively and not created automatically [0640b88](https://github.com/nlargueze/gitcc/commit/0640b882c2cfff4d85268438b89ede8a05d0d8eb)
- fix(publish): added extra info to manifest for publishing [66a631d](https://github.com/nlargueze/gitcc/commit/66a631d2c0094d54a8bc2144283d8fcfb3829e61)
- fix(hooks): removed file extension on scripts [24e1901](https://github.com/nlargueze/gitcc/commit/24e1901f3332dc3d7aa86f5a5799cfdd4897b248)
- fix: fixed install-hooks [b96db05](https://github.com/nlargueze/gitcc/commit/b96db05851ea2a3581a37d7460f609860cb2032c)
- fix(release): added git add to release workflow [fbff27d](https://github.com/nlargueze/gitcc/commit/fbff27d0f6573514f341192c57faf4146da804ee)
- fix: fixed changelog [c5356ec](https://github.com/nlargueze/gitcc/commit/c5356ecf6747b95622468da90d2700299c27a1a9)
- fix(bump): modified bump option &#x27;print-only&#x27; -&gt; &#x27;tag&#x27; [815e439](https://github.com/nlargueze/gitcc/commit/815e4390e0bd14436f06496cfc383e3fdd74f4ef)
- fix(bump): fixed bump abort if no commits [1dcaaee](https://github.com/nlargueze/gitcc/commit/1dcaaeeb0c233a63f2bcecad587cc0aa4ab8189a)
- fix(bump): changed bump option &#x27;set&#x27; to &#x27;print-only&#x27; [01441ff](https://github.com/nlargueze/gitcc/commit/01441ffdd7750f879ee7209ed6a8804ff29675e5)
- fix(changelog): fixed incorrect group title [f21b492](https://github.com/nlargueze/gitcc/commit/f21b4926e4b0fc2a9cb3e7718bbe1bbdadf563c3)
- fix: fixed lint issue [1071470](https://github.com/nlargueze/gitcc/commit/1071470fda7e26a005194c415ab5021cf763e35c)
- fix: modified config object [aec8464](https://github.com/nlargueze/gitcc/commit/aec84640d7349d82958fd442912d57a87a3e9bec)
- fix: added misc changes [aeba006](https://github.com/nlargueze/gitcc/commit/aeba0061f8f31bea84068fc45afa34d6cb85a561)
- fix: added misc stuff [eed1ef3](https://github.com/nlargueze/gitcc/commit/eed1ef301ca9edd4cea2ebcbbbc06a25e37d5ddf)
- fix: misc changes [b799a06](https://github.com/nlargueze/gitcc/commit/b799a06fbbd0adbc1a5248c27c25816a7557fca4)
- fix: remove gitt file from root [b869a54](https://github.com/nlargueze/gitcc/commit/b869a54446ca1a46d377113b0b8edacc82747bae)
- fix: misc fixes [20903f5](https://github.com/nlargueze/gitcc/commit/20903f5d49378822dcbdcd5d3a1f59c8ad627179)
- fix: added stdout for git add wrapper [c3a868d](https://github.com/nlargueze/gitcc/commit/c3a868da0762255a6090daabb69ca86c8ef73785)
- fix: added unix exit codes [55d7982](https://github.com/nlargueze/gitcc/commit/55d7982fccb748b2291877053817a2c6d2387d8b)
- fix(commit): removed displayed commit message on push [3dd7532](https://github.com/nlargueze/gitcc/commit/3dd7532b9225bc5bbc183904999589c15a7762d5)
- fix(commit): removed commit message on push [b8ea739](https://github.com/nlargueze/gitcc/commit/b8ea739cc4bdc1096f3f203d0a2c14d3bf7f776c)
- fix(commit): removed print message [e8cc423](https://github.com/nlargueze/gitcc/commit/e8cc423394d4e677c6ecb8e43c76819505db5082)

### Documentation

- docs: fixed documentation [45734a2](https://github.com/nlargueze/gitcc/commit/45734a2e382597ce5784c81a9ea1fac43f84c224)

### Tooling

- cd: removed publish&#x3D;false in Cargo.toml [6a8189d](https://github.com/nlargueze/gitcc/commit/6a8189d7d3f39dd3f01918713078c8f96bb49254)
- cd: fixed CD workflow [0f7ee1b](https://github.com/nlargueze/gitcc/commit/0f7ee1b23fc5176df706a1483c52cd8e0162a05f)
- cd: fixed Github Actions jobs dependencies [a50aeb5](https://github.com/nlargueze/gitcc/commit/a50aeb5f8fd875fcbc988c62092745f53d1fb5e1)
- cd: removed Dagger setup, moved back to Github Actions [a749198](https://github.com/nlargueze/gitcc/commit/a749198717ab740efc3760e77352cf1660d95bb3)
- cd: fixed CD workflow [f39e6e7](https://github.com/nlargueze/gitcc/commit/f39e6e7760107a9977f83fc94b5e466be2abf5fb)
- cd: fixed release version not being picked up [19b618a](https://github.com/nlargueze/gitcc/commit/19b618a9a23b4c41c2f5a2899f24e688eee0dbec)
- cd: ignored .DS_Store file [1d42794](https://github.com/nlargueze/gitcc/commit/1d42794ce7b9ba6ea28189750c2e72c5bce704f4)
- cd: fixed CD deployment [f9955cf](https://github.com/nlargueze/gitcc/commit/f9955cf5dd0d7c632fb52e9120e403d2d4336324)
- ci: added CI dagger jobs and Github Actions [dad72c9](https://github.com/nlargueze/gitcc/commit/dad72c91752280a87a2aa6b746fe2753c0384a5d)

### Uncategorized

- test commit [944bbc1](https://github.com/nlargueze/gitcc/commit/944bbc1c13ce24213f1462dc8bf5d268892b736c)
- test commit [2611585](https://github.com/nlargueze/gitcc/commit/2611585feb89972debfc57b32ac5c321183c7270)
- chore!: refactoring [e88dae6](https://github.com/nlargueze/gitcc/commit/e88dae6d48fd85b094f58eab029a883969436101)
- chore(dev test): test for development [99e35b6](https://github.com/nlargueze/gitcc/commit/99e35b6af06f4062d754bb26a48cff24059666a6)
- Reset [b50e8e3](https://github.com/nlargueze/gitcc/commit/b50e8e3bc80023aff94a46e0d584b8029acf7785)
- refactor(git_log): moved git_log() to another file [d5d56c7](https://github.com/nlargueze/gitcc/commit/d5d56c7b4d214741a421499a0e9ecae50a481c47)
- Merge branch &#x27;main&#x27; of https://github.com/nlargueze/gitt [6f18189](https://github.com/nlargueze/gitcc/commit/6f181890f59c6a513adbb941bac8d018e2514082)
- refactor: Refactored code [c59010b](https://github.com/nlargueze/gitcc/commit/c59010b3545d67aaa399a3e14caf63375990dccd)
- refactor: Refactored code [7065dfa](https://github.com/nlargueze/gitcc/commit/7065dfa1096ed172af9499fff75fff66e354ae22)
- test [f94c13a](https://github.com/nlargueze/gitcc/commit/f94c13afa981a598243cc471666b64febdd4c5a8)
- feat: ABCD [66d51bb](https://github.com/nlargueze/gitcc/commit/66d51bbcfc9a85ea1c3f15d30b0e5031afe9b21c)
- Merge branch &#x27;main&#x27; of https://github.com/nlargueze/gitt [a178085](https://github.com/nlargueze/gitcc/commit/a178085eb233fa59f411b627d5f0e822ae5c157b)
- first commit [b9861b6](https://github.com/nlargueze/gitcc/commit/b9861b644673c21ab5718ca3c411299b51da99e0)
