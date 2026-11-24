import type { ReactNode } from "react";

type Props = { x: number; y: number; title: string; detail: string; children: ReactNode };

export default function Marker({ x, y, title, detail, children }: Props) {
    return (
        <div
            className="group pointer-events-auto absolute -translate-x-1/2 -translate-y-1/2 hover:z-30"
            style={{ left: `${x / 10}%`, top: `${(800 - y) / 8}%` }}
        >
            {children}
            <div className="pointer-events-none absolute bottom-full left-1/2 mb-2 hidden -translate-x-1/2 border border-ash/60 bg-slate px-3 py-2 whitespace-nowrap text-white group-hover:block">
                <p className="text-xs font-semibold tracking-wider uppercase">{title}</p>
                <p className="font-mono text-[10px] text-steel uppercase">{detail}</p>
                <p className="font-mono text-[10px] text-ash">
                    X {x.toFixed(2)} / Y {y.toFixed(2)}
                </p>
            </div>
        </div>
    );
}
