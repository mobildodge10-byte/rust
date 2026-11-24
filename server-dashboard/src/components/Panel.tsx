import type { ReactNode } from "react";

type Props = {
    code: string;
    title: string;
    aside?: ReactNode;
    footer?: ReactNode;
    className?: string;
    children: ReactNode;
};

export default function Panel({ code, title, aside, footer, className = "", children }: Props) {
    return (
        <section className={`cut-br flex min-h-0 flex-col bg-slate text-white ${className}`}>
            <header className="flex h-11 shrink-0 items-center gap-3 border-b border-ash/40 px-4">
                <span className="h-4 w-1 bg-signal" />
                <span className="font-mono text-[10px] text-steel">{code}</span>
                <h2 className="text-sm font-semibold tracking-[0.2em] uppercase">{title}</h2>
                <div className="ml-auto">{aside}</div>
            </header>
            <div className="min-h-0 flex-1 overflow-auto">{children}</div>
            {footer && <footer className="shrink-0 border-t border-ash/40 p-4">{footer}</footer>}
        </section>
    );
}
