# greyna_bevy_tower_defense aka 'Side Tower Defense' for Bevy Game Jam #3
Tower Defense game using Rust programming language and Bevy game engine made by greyna.

# License
The game as well as all assets and code are free and licensed under "Licence Creative Commons Attribution 4.0 International".

# TODO
- [x] Terrain
- [x] Enemies
- [x] Shooting
- [x] Damages & death
- [x] Lives & Game over
- [x] Clean enemies & reset lives
- [x] Increasing difficulty over time
- [X] Gold: resource, turret price & enemy giving on death
- [x] Slot/turret levels & upgrade
- [x] Damage types
- [x] Authorize turret builds only on first line
- [x] Turret side effect: divide slot level and turret damage type levels
- [x] Menu & UI
- [x] Wasm deployment
- [x] Itch.io upload
- [x] Balancing !
- [x] Fixed window size
- [x] Score
- [x] Jam submission + github license

# TODO POLISH
- [ ] Buttons and UI for building and upgrading turrets
- [ ] Bugfix turret orientation/targetting
- [ ] Shoot feedback on turret
- [x] z-fight enemy bugfix
- [ ] turret slot shouldn't orientates
- [ ] improve turret targeting
- [ ] non-linear difficulty progression
- [ ] waves of enemies
- [ ] New Art integration
- [ ] Projectiles

# MY WEB BUILD TUTORIAL
- cargo build --release --target wasm32-unknown-unknown
- wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/greyna_bevy_tower_defense.wasm
- copy assets/ folder into ./out/
- copy index.html and restart-audio-context.js from tools/ to ./out/
- zip out/ folder and upload it on itch.io