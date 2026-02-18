import type { PhotoMode } from '../types';

interface ModeSelectProps {
  modes: PhotoMode[];
  onSelect: (mode: PhotoMode) => void;
  onBack: () => void;
}

const modeIcons: Record<string, string> = {
  cartoon: '🎨',
  movie: '🎬',
  anime: '🌸',
  cyberpunk: '🤖',
  traditional: '👘',
  age: '⏳',
};

function ModeSelect({ modes, onSelect, onBack }: ModeSelectProps) {
  return (
    <div className="mode-select">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">选择拍照风格</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      <div className="grid grid-cols-3">
        {modes.map((mode) => (
          <div
            key={mode.id}
            className="card mode-card"
            onClick={() => onSelect(mode)}
            style={{
              cursor: 'pointer',
              textAlign: 'center',
              transition: 'transform 0.2s, box-shadow 0.2s',
            }}
          >
            <div style={{ fontSize: '4rem', marginBottom: '1rem' }}>
              {modeIcons[mode.id] || '📷'}
            </div>
            <h3 className="heading-2">{mode.name}</h3>
            <p className="text-light">{mode.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
}

export default ModeSelect;
