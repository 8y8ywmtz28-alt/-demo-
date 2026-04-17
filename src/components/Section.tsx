import type { ReactNode } from 'react';

export function Section({ title, right, children }: { title: string; right?: ReactNode; children: ReactNode }) {
  return (
    <section className="section">
      <header className="section-header">
        <h3>{title}</h3>
        {right}
      </header>
      <div>{children}</div>
    </section>
  );
}
