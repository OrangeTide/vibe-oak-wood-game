import Phaser from 'phaser';

export class DialogScene extends Phaser.Scene {
  private lines: string[] = [];
  private lineIndex = 0;
  private dialogText!: Phaser.GameObjects.Text;

  constructor() {
    super('DialogScene');
  }

  init(data: { lines?: string[] }) {
    this.lines = data.lines || ['...'];
    this.lineIndex = 0;
  }

  create() {
    // Dialog box at bottom of screen
    this.add.rectangle(160, 155, 300, 44, 0x000000, 0.85).setOrigin(0.5).setStrokeStyle(1, 0xffffff);

    this.dialogText = this.add.text(20, 138, this.lines[0], {
      fontSize: '9px',
      color: '#ffffff',
      wordWrap: { width: 280 },
    });

    // Advance prompt
    const prompt = this.add.text(295, 168, '>', {
      fontSize: '8px',
      color: '#cccccc',
    }).setOrigin(0.5);

    this.tweens.add({
      targets: prompt,
      alpha: 0.3,
      duration: 500,
      yoyo: true,
      repeat: -1,
    });

    // Advance or close on key press
    this.input.keyboard!.on('keydown-SPACE', () => this.advance());
    this.input.keyboard!.on('keydown-ENTER', () => this.advance());
    this.input.keyboard!.on('keydown-ESC', () => this.close());
  }

  private advance() {
    this.lineIndex++;
    if (this.lineIndex >= this.lines.length) {
      this.close();
    } else {
      this.dialogText.setText(this.lines[this.lineIndex]);
    }
  }

  private close() {
    this.scene.resume('GameScene');
    this.scene.stop();
  }
}
