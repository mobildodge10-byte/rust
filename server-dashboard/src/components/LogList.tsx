type Entry = { id: number; time: string; tag?: string; text: string };

export default function LogList({ entries, empty }: { entries: Entry[]; empty: string }) {
    if (entries.length === 0) {
        return <p className="p-10 text-center font-mono text-xs tracking-widest text-ash">{empty}</p>;
    }

    return (
        <ul>
            {entries.map((entry) => (
                <li key={entry.id} className="grid grid-cols-[72px_1fr] gap-3 border-b border-ash/20 px-4 py-2.5">
                    <span className="font-mono text-[11px] text-steel">
                        {new Date(entry.time).toLocaleTimeString([], { hour12: false })}
                    </span>
                    <span className="text-sm leading-snug">
                        {entry.tag && <span className="mr-2 font-mono text-[10px] text-signal">{entry.tag}</span>}
                        {entry.text}
                    </span>
                </li>
            ))}
        </ul>
    );
}
