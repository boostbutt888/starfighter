# Art provenance and final prompt set

All selected images were generated with the **built-in imagegen tool**, copied into assets/, and retain generated transparency where requested. Atlas cells are sampled directly by Rust. The original front-facing Imperial cells are unused; a corrected overhead sheet replaces them.

## title.png

Cinematic widescreen game title background, no text. Rebel X-wing with four open S-foils flying toward camera, banking above Tatooine dunes, twin suns on horizon, distant Imperial Star Destroyer and small TIE fighters in space overhead. Premium painted cinematic concept art, dramatic amber sunlight and deep midnight teal space, detailed weathered spacecraft. X-wing on right, dark negative space on left for title. No words, UI or watermark.

## rebel-capital-atlas.png

Transparent photorealistic spacecraft atlas, exact 3×3 grid, equal cells, overhead orthographic top-down view, noses up, no labels. Row one: T-65 X-wing with open S-foils, BTL Y-wing with twin engines, red-white RZ-1 A-wing. Row two: TIE fighter, interceptor, bomber. Row three: TIE Advanced, Imperial Star Destroyer, Executor. Film-accurate proportions, realistic metal, intricate greebles, weathered hulls, physically based 3D rendering. Only Rebel and capital-ship cells are consumed from this sheet.

## imperial-top-atlas.png

Transparent 2×2 atlas: TIE fighter, interceptor, bomber and Vader's TIE Advanced x1. Strictly directly above, orthographic plan view looking down at top surfaces, flight toward top. Not front elevation; no circular front cockpit window. Vertical solar panels appear as thin rails, with cockpit top dome and rear engine housing visible. Bomber has two elongated pods. Realistic movie-prop metal, transparent padding, no text.

## Planetary backgrounds

Shared prompt: High-resolution production terrain background. [SCENE]. Strict orthographic straight-down aerial satellite view, no horizon, camera tilt, spacecraft or text. Photorealistic physically based cinematic rendering with intricate sharp surface detail. Fill the frame with terrain. Portrait 1024×1536 or higher, for a scrolling shooter.

- **tatooine.png**: golden dunes, jagged sandstone canyons and small desert settlements.
- **hoth.png**: white snowfields, blue ice crevasses and frozen mountain ridges.
- **dagobah.png**: dark green swamp canopy, muddy waterways and fine mist.
- **coruscant.png**: futuristic night city, dense skyscraper rooftops, amber traffic lanes and illuminated towers.

## space-flight.png

High-resolution photorealistic portrait starfield. Black and midnight-blue space, thousands of tiny sharp distant stars, restrained diffuse faint blue nebula dust. No planets, moons, spacecraft, words or bright large objects. Even unobtrusive distribution, no focal point. Film-quality astronomical rendering, 1024×1536 or higher.

## endor-moon.png

Isolated photorealistic forest moon Endor from space. Complete circular planet with transparent padding. Dense dark-green forests, deep-blue rivers and lakes, wisps of white clouds, thin blue atmospheric rim. Light from upper left, softly shadowed lower right. Intricate cinematic surface detail. No ships, text, stars or backdrop. True transparency, high-resolution square sprite.

## death-stars.png

Transparent atlas, two equal square cells. Complete Death Star I on left; incomplete Death Star II with exposed skeletal construction on right. Film-accurate superlaser dish, intricate panels, equatorial trench and gray metal. Orthographic physically based 3D render, upper-left lighting, transparent padding, no labels or backdrop.

## jabba-barge.png

Jabba's Khetanna sail barge isolated on transparency, directly overhead orthographic plan, bow up. Weathered brown armored desert barge, red-orange sails either side of exposed deck. Recognizable green Jabba reclining on rear command deck. Six metallic turret mounts, detailed rivets and planks, movie-prop materials, realistic cinematic 3D render. Whole craft in frame, no text or ground. Runtime rotating guns overlay the hull, which is rotated to face down.

## turrets.png

Transparent three-cell horizontal atlas, all directly overhead with barrels pointing up. Imperial twin turbolaser with gray octagonal housing and circular base; rustic brown single-barrel sail-barge laser turret; dark-gray Death Star quad turbolaser. Intricate physically based movie-prop metal and greebles, isolated centered objects, no labels or text.
# Menu artwork refresh — v0.3.0

## Campaign ending artwork — v0.5.0

Both assets were created with the built-in imagegen tool.

`rebel-starbase.png` prompt:

> Use case: stylized-concept. Asset type: high resolution portrait 2:3 game ending landing backdrop. A Rebel starbase hangar seen directly from overhead, orthographic top-down. Huge empty central circular landing platform centered at 50 percent width, 48 percent height, warm luminous guidance rings and amber landing strips, industrial gray hangar walls, small maintenance bays, Rebel Alliance banners, tiny crews waiting safely near left and right margins. Deep space and starfield visible through open hangar doorway along bottom edge. Cinematic detailed realistic sci-fi render. Keep central platform entirely empty, no spacecraft, no pilot portrait, no text, no UI, no watermark.

`rebel-celebration.png` prompt (reference: `menu-action.png`):

> Use case: illustration-story. Create a NEW portrait 2:3 high resolution game victory ending illustration. The reference supplies the identity and anime-realistic art style of the adult female Rebel pilot, brown eyes and brown tied hair, orange flight suit white vest and helmet. Show this SAME pilot standing confidently on a Rebel starbase hangar platform, smiling with relief, helmet under one arm, being joyfully celebrated by a crowd of Rebel pilots, technicians, droids and officers applauding around her. Her X-wing is safely parked behind them, wings closed; hangar overlooks a peaceful starfield and a blue planet. Hopeful victory scene, warm golden hangar light, blue ambient space, cinematic polished anime key visual with detailed realistic spacecraft and architecture. Composition: pilot face upper-middle, welcoming crowd around both sides, full scene fits frame, dark quieter lower 20 percent for game text. No text, no lettering, no watermark.

## Menu artwork

Built-in imagegen generated `menu-action.png` with this prompt:

> Use case: illustration-story. Asset: high resolution portrait 2:3 game main menu background, no text. A confident adult anime-style female Rebel starfighter pilot in orange flight suit with white vest, helmet tucked under arm, upper left foreground; detailed X-wing banking dynamically across upper right and center, green laser bolts, TIE fighters and luminous blue nebula in space. Polished cinematic anime key visual with realistic mechanical spaceship detailing, dramatic rim light. Pilot and X-wing fully visible within top two-thirds, lower third dark space with subtle stars for UI overlay. No lettering, logos, frames, watermarks.

Boss sprites are now extracted into individual textures at load time. Only the largest connected alpha silhouette is kept, removing disconnected pieces of neighboring sprites in the original atlas. Death Stars remain unrotated; capital hulls face down; Jabba's portrait is counter-rotated independently.

