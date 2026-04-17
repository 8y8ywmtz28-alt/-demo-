interface Props {
  value: number;
}

export function ProgressLine({ value }: Props) {
  return (
    <div className="progress-line">
      <div className="progress-inner" style={{ width: `${Math.max(0, Math.min(100, value))}%` }} />
    </div>
  );
}
