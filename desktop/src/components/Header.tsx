import { useEffect, useRef, useState } from "react";
import { Operator, Role } from "../model/operator";
import OperatorIcon from "./OperatorIcon";

type Props = {
    role: Role;
    operator: Operator | null;
    options: Operator[];
    onSelect: (id: number) => void;
};

function coord(n: number) {
    return n.toFixed(2).padStart(7, "0");
}

export default function Header({ role, operator, options, onSelect }: Props) {
    const [open, setOpen] = useState(false);
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        function close(e: MouseEvent) {
            if (!ref.current?.contains(e.target as Node)) setOpen(false);
        }
        document.addEventListener("mousedown", close);
        return () => document.removeEventListener("mousedown", close);
    }, []);

    return (
        <header className="relative z-20 flex h-24 shrink-0 items-center gap-6 bg-slate px-6 text-white">
            <div ref={ref} className="relative">
                <button onClick={() => setOpen(!open)} className="group flex items-start" disabled={!operator}>
                    <div className="cut-sm bg-signal p-0.5">
                        {operator ? (
                            <OperatorIcon key={operator.id} operator={operator} size="size-16" />
                        ) : (
                            <div className="size-16 bg-ash" />
                        )}
                    </div>
                    <span className="ml-1 grid size-5 place-items-center bg-ash text-slate transition-colors group-hover:bg-signal">
                        <svg viewBox="0 0 16 16" className="size-3" fill="none" stroke="currentColor" strokeWidth="2">
                            <path d="M2 5h10M9 2l3 3-3 3M14 11H4M7 8l-3 3 3 3" />
                        </svg>
                    </span>
                </button>

                {open && (
                    <div className="absolute top-full left-0 mt-2 w-72 border border-ash/60 bg-slate shadow-2xl">
                        <p className="border-b border-ash/40 px-4 py-2 font-mono text-[10px] tracking-[0.3em] text-steel">
                            SWITCH {role.toUpperCase()}
                        </p>
                        <ul className="max-h-80 overflow-auto">
                            {options.map((option) => (
                                <li key={option.id}>
                                    <button
                                        onClick={() => {
                                            onSelect(option.id);
                                            setOpen(false);
                                        }}
                                        className={`flex w-full items-center gap-3 border-l-4 px-3 py-2 text-left hover:bg-ash/25 ${
                                            option.id === operator?.id
                                                ? "border-signal bg-ash/15"
                                                : "border-transparent"
                                        }`}
                                    >
                                        <OperatorIcon operator={option} size="size-9" />
                                        <span className="min-w-0 flex-1">
                                            <span className="block truncate text-sm font-semibold tracking-wider uppercase">
                                                {option.codename}
                                            </span>
                                            <span className="block font-mono text-[10px] text-steel">
                                                X {coord(option.x_position)} / Y {coord(option.y_position)}
                                            </span>
                                        </span>
                                    </button>
                                </li>
                            ))}
                        </ul>
                    </div>
                )}
            </div>

            <div className="min-w-0">
                <p className="font-mono text-[10px] tracking-[0.3em] text-signal">{role.toUpperCase()} TERMINAL</p>
                <h1 className="truncate text-2xl font-bold tracking-[0.15em] uppercase">{operator?.codename ?? "-"}</h1>
                <p className="truncate text-xs text-steel">{operator?.fullname}</p>
            </div>

            <div className="ml-auto flex items-stretch">
                <div className="hazard w-2" />
                <div className="border border-l-0 border-ash/60 px-5 py-2">
                    <p className="font-mono text-[10px] tracking-[0.3em] text-steel">LOCATION</p>
                    <p className="font-mono text-lg">
                        <span className="text-ash">X</span> {operator ? coord(operator.x_position) : "----"}
                        <span className="mx-3 text-ash">/</span>
                        <span className="text-ash">Y</span> {operator ? coord(operator.y_position) : "----"}
                    </p>
                </div>
            </div>
        </header>
    );
}
