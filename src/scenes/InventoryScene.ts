import Phaser from 'phaser';

export class InventoryScene extends Phaser.Scene {
  constructor() {
    super('InventoryScene');
  }

  create() {
    // Semi-transparent overlay
    this.add.rectangle(160, 90, 280, 140, 0x000000, 0.8).setOrigin(0.5);

    // Title
    this.add.text(160, 30, 'Inventory', {
      fontSize: '14px',
      color: '#ffffff',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Empty slots grid (4x2)
    for (let row = 0; row < 2; row++) {
      for (let col = 0; col < 4; col++) {
        const x = 80 + col * 40;
        const y = 60 + row * 40;
        this.add.rectangle(x, y, 32, 32, 0x333333).setOrigin(0.5).setStrokeStyle(1, 0x666666);
      }
    }

    // Close hint
    this.add.text(160, 150, 'Press I to close', {
      fontSize: '8px',
      color: '#888888',
    }).setOrigin(0.5);

    // Close on I key
    this.input.keyboard!.once('keydown-I', () => {
      this.scene.resume('GameScene');
      this.scene.stop();
    });
    this.input.keyboard!.once('keydown-ESC', () => {
      this.scene.resume('GameScene');
      this.scene.stop();
    });
  }
}
