import { Role } from "../model/operator";

type Props = { active: Role; onChange: (role: Role) => void };

const menu: { role: Role; code: string; caption: string }[] = [
    { role: "Dispatcher", code: "01", caption: "Order control" },
    { role: "Courier", code: "02", caption: "Field delivery" },
];

export default function Sidebar({ active, onChange }: Props) {
    return (
        <aside className="flex w-60 shrink-0 flex-col bg-slate text-white">
            <div className="flex h-24 flex-col justify-center px-6">
                <div className="flex items-center gap-3">
                    <span className="size-3 rotate-45 bg-signal" />
                    <span className="text-lg font-bold tracking-[0.25em]">RHODEV</span>
                </div>
                <p className="mt-1 pl-6 font-mono text-[10px] tracking-[0.3em] text-steel">ISLAND // LOGISTIC</p>
            </div>
            <div className="hazard h-1" />

            <nav className="mt-6 flex flex-col gap-1 pr-4">
                {menu.map((item) => {
                    const selected = item.role === active;
                    return (
                        <button
                            key={item.role}
                            onClick={() => onChange(item.role)}
                            className={`cut-tr flex items-center gap-4 py-3 pl-6 text-left transition-colors ${
                                selected ? "bg-signal text-slate" : "text-steel hover:bg-ash/25 hover:text-white"
                            }`}
                        >
                            <span className="font-mono text-xs">{item.code}</span>
                            <span>
                                <span className="block text-sm font-semibold tracking-[0.2em] uppercase">
                                    {item.role}
                                </span>
                                <span
                                    className={`block font-mono text-[10px] ${selected ? "text-slate/70" : "text-ash"}`}
                                >
                                    {item.caption}
                                </span>
                            </span>
                        </button>
                    );
                })}
            </nav>

            <div className="mt-auto px-6 pb-6 font-mono text-[10px] leading-relaxed text-ash">
                <div className="mb-3 h-px bg-ash/40" />
                TERRA LOGISTIC NETWORK
                <br />
                SIMULATION MODE
            </div>
        </aside>
    );
}
