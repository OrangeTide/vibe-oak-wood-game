import Phaser from 'phaser';

export class BootScene extends Phaser.Scene {
  constructor() {
    super('BootScene');
  }

  preload() {
    const base = 'assets/oak_woods';

    // Background layers
    this.load.image('bg_layer1', `${base}/background/background_layer_1.png`);
    this.load.image('bg_layer2', `${base}/background/background_layer_2.png`);
    this.load.image('bg_layer3', `${base}/background/background_layer_3.png`);

    // Tileset (load as image for tilemap use)
    this.load.image('oak_woods_tileset', `${base}/oak_woods_tileset.png`);

    // Character
    this.load.spritesheet('char_blue', `${base}/character/char_blue.png`, {
      frameWidth: 56,
      frameHeight: 56,
    });

    // Decorations
    this.load.image('deco_lamp', `${base}/decorations/lamp.png`);
    this.load.image('deco_fence1', `${base}/decorations/fence_1.png`);
    this.load.image('deco_sign', `${base}/decorations/sign.png`);
    this.load.image('deco_grass1', `${base}/decorations/grass_1.png`);
    this.load.image('deco_grass2', `${base}/decorations/grass_2.png`);
    this.load.image('deco_grass3', `${base}/decorations/grass_3.png`);
  }

  create() {
    this.scene.start('TitleScene');
  }
}
