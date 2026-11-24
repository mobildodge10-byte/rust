type Props = { label: string; value: string | number; unit?: string };

export default function StatTile({ label, value, unit }: Props) {
    return (
        <div className="cut-br relative bg-slate px-5 py-4 text-white">
            <span className="absolute top-0 left-0 h-1 w-10 bg-signal" />
            <p className="font-mono text-[10px] tracking-[0.3em] text-steel uppercase">{label}</p>
            <p className="mt-2 truncate text-3xl font-bold tracking-wider">
                {value}
                {unit && <span className="ml-2 font-mono text-xs text-ash">{unit}</span>}
            </p>
        </div>
    );
}
