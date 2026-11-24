import { statusLabel } from "../model/order-item";

const colors: Record<string, string> = {
    PENDING: "bg-ash",
    IN_PRODUCTION: "bg-steel",
    READY_FOR_PICKUP: "bg-signal",
    ASSIGNED: "bg-moss",
    IN_TRANSIT: "bg-moss",
    DELIVERED: "bg-white",
    AVAILABLE: "bg-signal",
    ON_DUTY: "bg-ash",
};

export default function StatusMark({ status }: { status: string }) {
    return (
        <span className="inline-flex items-center gap-2 font-mono text-[11px] tracking-wider text-steel uppercase">
            <span className={`size-2 rotate-45 ${colors[status] ?? "bg-ash"}`} />
            {statusLabel(status)}
        </span>
    );
}
