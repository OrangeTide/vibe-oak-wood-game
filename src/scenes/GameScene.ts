import Phaser from 'phaser';

export class GameScene extends Phaser.Scene {
  private player!: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody;
  private cursors!: Phaser.Types.Input.Keyboard.CursorKeys;

  constructor() {
    super('GameScene');
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
    // Step 1: Parallax backgrounds
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer1').setScrollFactor(0);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer2').setScrollFactor(0.3);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer3').setScrollFactor(0.6);

    // Step 2: Ground tilemap
    const map = this.make.tilemap({
      tileWidth: 24,
      tileHeight: 24,
      width: 14,
      height: 8,
    });
    const tileset = map.addTilesetImage(
      'oak_woods_tileset', 'oak_woods_tileset', 24, 24, 0, 0,
    )!;
    const groundLayer = map.createBlankLayer('ground', tileset)!;
    groundLayer.setDepth(5);

    // Tile indices from the 21-column tileset (verified from reference image)
    // Platform 9-patch at cols 0-3, rows 0-2:
    const TL = 0;   // top-left corner (grass surface + left edge)
    const TC = 1;   // top center (grass surface, repeating)
    const TC2 = 2;  // top center variant (for visual variety)
    const TR = 3;   // top-right corner (grass surface + right edge)
    const ML = 21;  // middle left edge (stone wall)
    const MC = 22;  // middle center (solid dark stone fill)
    const MR = 24;  // middle right edge (stone wall)
    const BL = 42;  // bottom-left corner
    const BR = 45;  // bottom-right corner

    // Lower ground (left side): surface at row 6, cols 0-7
    // Left edge goes off screen → use TC, no TL needed
    for (let x = 0; x < 7; x++) {
      groundLayer.putTileAt(x % 2 === 0 ? TC : TC2, x, 6);
    }
    groundLayer.putTileAt(TR, 7, 6); // right end before step

    // Lower ground fill (row 7)
    for (let x = 0; x < 7; x++) {
      groundLayer.putTileAt(MC, x, 7);
    }
    groundLayer.putTileAt(MR, 7, 7);

    // Upper platform (right side): surface at row 5, cols 8-13
    groundLayer.putTileAt(TL, 8, 5); // left corner at step
    for (let x = 9; x < 14; x++) {
      groundLayer.putTileAt(x % 2 === 0 ? TC : TC2, x, 5);
    }

    // Upper platform fill (rows 6-7)
    for (let y = 6; y < 8; y++) {
      groundLayer.putTileAt(ML, 8, y); // left wall at step
      for (let x = 9; x < 14; x++) {
        groundLayer.putTileAt(MC, x, y);
      }
    }

    // Set collision on all placed tiles
    groundLayer.setCollisionBetween(0, 315);

    // Decorations (origin bottom-center so they sit on the ground surface)
    const leftGround = 6 * 24;  // y=144
    const rightGround = 5 * 24; // y=120

    this.add.image(20, leftGround, 'deco_lamp').setOrigin(0.5, 1).setDepth(6);
    this.add.image(90, leftGround, 'deco_fence1').setOrigin(0.5, 1).setDepth(6);
    this.add.image(50, leftGround, 'deco_grass1').setOrigin(0.5, 1).setDepth(6);
    this.add.image(140, leftGround, 'deco_grass2').setOrigin(0.5, 1).setDepth(6);
    this.add.image(270, rightGround, 'deco_sign').setOrigin(0.5, 1).setDepth(6);
    this.add.image(250, rightGround, 'deco_grass3').setOrigin(0.5, 1).setDepth(6);

    // Step 3: Character
    this.createAnimations();

    this.player = this.physics.add.sprite(80, 100, 'char_blue');
    this.player.setDepth(10);
    this.player.setSize(16, 28);
    this.player.setOffset(20, 24);
    this.player.play('char_blue_idle');
    this.player.setCollideWorldBounds(true);

    this.physics.world.setBounds(0, 0, 336, 192);
    this.physics.add.collider(this.player, groundLayer);

    this.cursors = this.input.keyboard!.createCursorKeys();
  }

  private createAnimations() {
    this.anims.create({
      key: 'char_blue_idle',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 0, end: 5 }),
      frameRate: 8,
      repeat: -1,
    });

    this.anims.create({
      key: 'char_blue_run',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 16, end: 23 }),
      frameRate: 10,
      repeat: -1,
    });

    this.anims.create({
      key: 'char_blue_jump',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 24, end: 31 }),
      frameRate: 10,
      repeat: 0,
    });

    this.anims.create({
      key: 'char_blue_fall',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 32, end: 39 }),
      frameRate: 10,
      repeat: -1,
    });

    this.anims.create({
      key: 'char_blue_attack',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 8, end: 13 }),
      frameRate: 12,
      repeat: 0,
    });

    this.anims.create({
      key: 'char_blue_death',
      frames: this.anims.generateFrameNumbers('char_blue', { start: 40, end: 52 }),
      frameRate: 8,
      repeat: 0,
    });
  }

  update() {
    const speed = 100;
    const jumpVelocity = -280;
    const body = this.player.body;
    const onGround = body.blocked.down;

    // Horizontal movement
    if (this.cursors.left.isDown) {
      this.player.setVelocityX(-speed);
      this.player.setFlipX(true);
    } else if (this.cursors.right.isDown) {
      this.player.setVelocityX(speed);
      this.player.setFlipX(false);
    } else {
      this.player.setVelocityX(0);
    }

    // Jump
    if ((this.cursors.space.isDown || this.cursors.up.isDown) && onGround) {
      this.player.setVelocityY(jumpVelocity);
    }

    // Animation state
    if (!onGround) {
      if (body.velocity.y < 0) {
        this.player.play('char_blue_jump', true);
      } else {
        this.player.play('char_blue_fall', true);
      }
    } else if (body.velocity.x !== 0) {
      this.player.play('char_blue_run', true);
    } else {
      this.player.play('char_blue_idle', true);
    }
  }
}
