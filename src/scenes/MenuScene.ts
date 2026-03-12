import Phaser from 'phaser';

export class MenuScene extends Phaser.Scene {
  private menuItems: Phaser.GameObjects.Text[] = [];
  private selectedIndex = 0;

  constructor() {
    super('MenuScene');
  }

  create() {
    this.selectedIndex = 0;

    // Background
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer1').setScrollFactor(0);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer2').setScrollFactor(0);
    this.add.tileSprite(160, 90, 320, 180, 'bg_layer3').setScrollFactor(0);

    // Title
    this.add.text(160, 40, 'Oak Woods', {
      fontSize: '20px',
      color: '#ffffff',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Menu items
    const items = ['New Game', 'Continue', 'Options'];
    this.menuItems = items.map((label, i) => {
      return this.add.text(160, 80 + i * 24, label, {
        fontSize: '12px',
        color: '#aaaaaa',
      }).setOrigin(0.5);
    });

    this.updateSelection();

    const cursors = this.input.keyboard!.createCursorKeys();
    this.input.keyboard!.on('keydown-UP', () => {
      this.selectedIndex = (this.selectedIndex - 1 + this.menuItems.length) % this.menuItems.length;
      this.updateSelection();
    });
    this.input.keyboard!.on('keydown-DOWN', () => {
      this.selectedIndex = (this.selectedIndex + 1) % this.menuItems.length;
      this.updateSelection();
    });
    this.input.keyboard!.on('keydown-ENTER', () => {
      this.selectItem();
    });
    this.input.keyboard!.on('keydown-SPACE', () => {
      this.selectItem();
    });
  }

  private updateSelection() {
    this.menuItems.forEach((item, i) => {
      if (i === this.selectedIndex) {
        item.setColor('#ffffff');
        item.setText('> ' + item.text.replace(/^> /, ''));
      } else {
        item.setColor('#aaaaaa');
        item.setText(item.text.replace(/^> /, ''));
      }
    });
  }

  private selectItem() {
    switch (this.selectedIndex) {
      case 0: // New Game
        this.scene.start('GameScene');
        break;
      case 1: // Continue
        this.scene.start('GameScene');
        break;
      case 2: // Options
        break;
    }
  }
}
