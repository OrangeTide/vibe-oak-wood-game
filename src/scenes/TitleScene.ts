import Phaser from 'phaser';

export class TitleScene extends Phaser.Scene {
  constructor() {
    super('TitleScene');
  }

  create() {
    // Parallax background
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer1').setScrollFactor(0);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer2').setScrollFactor(0);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer3').setScrollFactor(0);

    // Title text
    this.add.text(160, 60, 'Oak Woods', {
      fontSize: '24px',
      color: '#ffffff',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Prompt
    const prompt = this.add.text(160, 120, 'Press any key', {
      fontSize: '10px',
      color: '#cccccc',
    }).setOrigin(0.5);

    // Blink the prompt
    this.tweens.add({
      targets: prompt,
      alpha: 0,
      duration: 800,
      yoyo: true,
      repeat: -1,
    });

    this.input.keyboard!.once('keydown', () => {
      this.scene.start('MenuScene');
    });
  }
}
