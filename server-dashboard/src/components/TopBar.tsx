type Props = { stats: { label: string; value: string | number }[]; error: string | null; syncedAt: Date | null };

export default function TopBar({ stats, error, syncedAt }: Props) {
    return (
        <header className="flex h-20 shrink-0 items-stretch bg-slate text-white">
            <div className="flex w-64 flex-col justify-center px-6">
                <div className="flex items-center gap-3">
                    <span className="size-3 rotate-45 bg-signal" />
                    <span className="text-lg font-bold tracking-[0.25em]">RHODEV</span>
                </div>
                <p className="mt-1 pl-6 font-mono text-[10px] tracking-[0.3em] text-steel">ISLAND // CONTROL</p>
            </div>

            {stats.map((stat) => (
                <div key={stat.label} className="flex flex-col justify-center border-l border-ash/40 px-6">
                    <p className="font-mono text-[10px] tracking-[0.3em] text-steel uppercase">{stat.label}</p>
                    <p className="text-2xl font-bold tracking-wider">{stat.value}</p>
                </div>
            ))}

            <div className="ml-auto flex items-center gap-4 px-6">
                <div className="text-right">
                    <p className="font-mono text-[10px] tracking-[0.3em] text-steel">SERVER LINK</p>
                    <p className={`font-mono text-sm ${error ? "text-signal" : "text-white"}`}>
                        {error
                            ? "OFFLINE"
                            : syncedAt
                              ? `LIVE ${syncedAt.toLocaleTimeString([], { hour12: false })}`
                              : "CONNECTING"}
                    </p>
                </div>
                <span className={`size-3 rotate-45 ${error ? "bg-ash" : "bg-signal"}`} />
            </div>
        </header>
    );
}
