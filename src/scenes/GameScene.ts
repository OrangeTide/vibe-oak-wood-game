import Phaser from 'phaser';

export class GameScene extends Phaser.Scene {
  private player!: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody;
  private cursors!: Phaser.Types.Input.Keyboard.CursorKeys;
  private attackKey!: Phaser.Input.Keyboard.Key;
  private isAttacking = false;
  private bgLayer1!: Phaser.GameObjects.TileSprite;
  private bgLayer2!: Phaser.GameObjects.TileSprite;
  private bgLayer3!: Phaser.GameObjects.TileSprite;

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
    // Parallax backgrounds — fixed to camera, scrolled manually in update()
    this.bgLayer1 = this.add.tileSprite(160, 90, 320, 180, 'bg_layer1')
      .setScrollFactor(0).setDepth(0);
    this.bgLayer2 = this.add.tileSprite(160, 90, 320, 180, 'bg_layer2')
      .setScrollFactor(0).setDepth(1);
    this.bgLayer3 = this.add.tileSprite(160, 90, 320, 180, 'bg_layer3')
      .setScrollFactor(0).setDepth(2);

    // Ground tilemap — wide enough for effectively infinite scrolling
    const MAP_W = 2000;
    const map = this.make.tilemap({
      tileWidth: 24,
      tileHeight: 24,
      width: MAP_W,
      height: 8,
    });
    const tileset = map.addTilesetImage(
      'oak_woods_tileset', 'oak_woods_tileset', 24, 24, 0, 0,
    )!;
    const groundLayer = map.createBlankLayer('ground', tileset)!;
    groundLayer.setDepth(5);

    // Tile indices (verified from tileset reference image)
    const TL = 0;   // top-left corner (grass + left edge)
    const TC = 1;   // top center (grass surface)
    const TC2 = 2;  // top center variant
    const TR = 3;   // top-right corner (grass + right edge)
    const ML = 21;  // middle left edge
    const MC = 22;  // middle center (solid fill)
    const MR = 24;  // middle right edge

    // 1. Fill entire lower ground across full width (rows 6-7)
    groundLayer.fill(TC, 0, 6, MAP_W, 1);
    groundLayer.fill(MC, 0, 7, MAP_W, 1);

    // Add surface variety (alternate TC2 on odd columns)
    for (let x = 1; x < MAP_W; x += 2) {
      groundLayer.putTileAt(TC2, x, 6);
    }

    // 2. Carve out the raised step section near center
    const STEP_L = 1008; // upper platform left column
    const STEP_R = 1013; // upper platform right column

    // Right edge of lower ground before step
    groundLayer.putTileAt(TR, STEP_L - 1, 6);
    groundLayer.putTileAt(MR, STEP_L - 1, 7);

    // Upper platform body (overwrite lower ground at rows 6-7)
    groundLayer.putTileAt(ML, STEP_L, 6);
    groundLayer.putTileAt(ML, STEP_L, 7);
    for (let x = STEP_L + 1; x < STEP_R; x++) {
      groundLayer.putTileAt(MC, x, 6);
      groundLayer.putTileAt(MC, x, 7);
    }
    groundLayer.putTileAt(MR, STEP_R, 6);
    groundLayer.putTileAt(MR, STEP_R, 7);

    // Left edge of lower ground resuming after step
    groundLayer.putTileAt(TL, STEP_R + 1, 6);
    groundLayer.putTileAt(ML, STEP_R + 1, 7);

    // Upper platform surface (row 5)
    groundLayer.putTileAt(TL, STEP_L, 5);
    for (let x = STEP_L + 1; x < STEP_R; x++) {
      groundLayer.putTileAt(x % 2 === 0 ? TC : TC2, x, 5);
    }
    groundLayer.putTileAt(TR, STEP_R, 5);

    groundLayer.setCollisionBetween(0, 315);

    // Decorations positioned near center (offset = 1000 * 24 = 24000)
    const O = 1000 * 24; // world offset for center section
    const leftGround = 6 * 24;  // y=144
    const rightGround = 5 * 24; // y=120

    this.add.image(O + 20, leftGround, 'deco_lamp').setOrigin(0.5, 1).setDepth(6);
    this.add.image(O + 90, leftGround, 'deco_fence1').setOrigin(0.5, 1).setDepth(6);
    this.add.image(O + 50, leftGround, 'deco_grass1').setOrigin(0.5, 1).setDepth(6);
    this.add.image(O + 140, leftGround, 'deco_grass2').setOrigin(0.5, 1).setDepth(6);
    this.add.image(O + 270, rightGround, 'deco_sign').setOrigin(0.5, 1).setDepth(6);
    this.add.image(O + 250, rightGround, 'deco_grass3').setOrigin(0.5, 1).setDepth(6);

    // Character
    this.createAnimations();

    this.player = this.physics.add.sprite(O + 80, 100, 'char_blue');
    this.player.setDepth(10);
    this.player.setSize(16, 28);
    this.player.setOffset(20, 24);
    this.player.play('char_blue_idle');

    this.physics.add.collider(this.player, groundLayer);

    // Camera follows player horizontally with smooth lerp
    this.cameras.main.startFollow(this.player, true, 0.1, 0);
    this.cameras.main.roundPixels = true;

    this.cursors = this.input.keyboard!.createCursorKeys();
    this.attackKey = this.input.keyboard!.addKey(Phaser.Input.Keyboard.KeyCodes.Z);
    this.player.on('animationcomplete-char_blue_attack', () => {
      this.isAttacking = false;
    });
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
    // Parallax: shift background tilePosition based on camera scroll
    const camX = this.cameras.main.scrollX;
    this.bgLayer1.tilePositionX = camX * 0;    // fixed sky
    this.bgLayer2.tilePositionX = camX * 0.3;  // slow trees
    this.bgLayer3.tilePositionX = camX * 0.6;  // closer trees

    const speed = 100;
    const jumpVelocity = -280;
    const body = this.player.body;
    const onGround = body.blocked.down;

    // Horizontal movement (stop during attack)
    if (this.isAttacking) {
      this.player.setVelocityX(0);
    } else if (this.cursors.left.isDown) {
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

    // Attack
    if (Phaser.Input.Keyboard.JustDown(this.attackKey) && onGround && !this.isAttacking) {
      this.isAttacking = true;
      this.player.play('char_blue_attack', true);
    }

    // Animation state (skip if attacking)
    if (!this.isAttacking) {
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
}
