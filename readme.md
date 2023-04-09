# greyna-bevy-tower-defense
Tower Defense game using Rust programming language and Bevy game engine made by greyna. Bevy Game Jam #3.

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
- [ ] Turret side effect: divide slot level and turret damage type levels
- [ ] Art integration
- [ ] Menu & UI
- [x] Wasm deployment
- [x] Itch.io upload
- [ ] Balancing !
- [ ] Fixed window size

# TODO POLISH
- [ ] Shoot feedback on turret
- [ ] Projectiles
- [ ] z-fight enemy bugfix

# WEB BUILD
- cargo build --release --target wasm32-unknown-unknown
- wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/greyna_bevy_tower_defense.wasm
- copy assets/ folder into ./out/
- copy index.html and restart-audio-context.js from tools/ to ./out/
- zip out/ folder and upload it on itch.io