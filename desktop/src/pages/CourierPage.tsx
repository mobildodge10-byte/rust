import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Operator } from "../model/operator";
import { Location, OrderItem, distance, formatTime, statusLabel } from "../model/order-item";
import Button from "../components/Button";
import Panel from "../components/Panel";
import StatTile from "../components/StatTile";

type Props = { courier: Operator; items: OrderItem[] };

const STEPS = ["ASSIGNED", "IN_TRANSIT", "DELIVERED"];

function Waypoint({ label, location, range }: { label: string; location: Location; range: number }) {
    return (
        <div className="border-l-2 border-ash/60 pl-3">
            <p className="font-mono text-[10px] tracking-[0.2em] text-ash">{label}</p>
            <p className="font-semibold">{location.name}</p>
            <p className="font-mono text-[11px] text-steel">
                X {location.x_position.toFixed(2)} / Y {location.y_position.toFixed(2)}
                <span className="ml-2 text-signal">{range.toFixed(0)} u</span>
            </p>
        </div>
    );
}

export default function CourierPage({ courier, items }: Props) {
    const [message, setMessage] = useState<string | null>(null);

    const list = items.filter((i) => i.courier_id === courier.id);
    const active = list.find((t) => t.status === "ASSIGNED" || t.status === "IN_TRANSIT") ?? null;
    const history = list
        .filter((t) => t.status === "DELIVERED")
        .sort((a, b) => (b.delivery_time ?? "").localeCompare(a.delivery_time ?? ""));

    async function act() {
        if (!active) return;
        try {
            const command = active.status === "ASSIGNED" ? "pickup_medicine" : "complete_task";
            await invoke(command, { operator_id: courier.id, order_item_id: active.id });
            setMessage(`Order item #${active.id} ${active.status === "ASSIGNED" ? "picked up" : "delivered"}`);
        } catch (err) {
            setMessage(String(err));
        }
    }

    return (
        <div className="grid h-full grid-cols-[400px_1fr] grid-rows-[auto_1fr] gap-4">
            {/* Job stats */}
            <div className="col-span-2 grid grid-cols-4 gap-4">
                <StatTile label="Completed Jobs" value={courier.completed_job_count} />
                <StatTile label="Units Delivered" value={history.reduce((sum, t) => sum + t.quantity, 0)} unit="PCS" />
                <StatTile label="Status" value={courier.availability === 1 ? "Available" : "On Duty"} />
                <StatTile label="Specialty" value={courier.region_specialty} />
            </div>

            <Panel
                code="C-01"
                title="Active Order"
                footer={
                    active && (
                        <div className="space-y-2">
                            <Button className="w-full" onClick={act}>
                                {active.status === "ASSIGNED" ? "Pick Up Medicine" : "Deliver Medicine"}
                            </Button>
                            {message && <p className="font-mono text-[11px] text-steel">{message}</p>}
                        </div>
                    )
                }
            >
                {active ? (
                    <div className="space-y-6 p-5">
                        <div>
                            <p className="font-mono text-[10px] tracking-[0.2em] text-ash">
                                ORDER #{active.order_id} / ITEM #{active.id}
                            </p>
                            <p className="mt-1 text-2xl font-bold">{active.medicine_name}</p>
                            <p className="font-mono text-sm text-signal">{active.quantity} PCS</p>
                        </div>

                        {/* Progress */}
                        <div className="grid grid-cols-3 gap-1">
                            {STEPS.map((step, index) => (
                                <div key={step}>
                                    <div
                                        className={`h-1.5 ${index <= STEPS.indexOf(active.status) ? "bg-signal" : "bg-ash/40"}`}
                                    />
                                    <p className="mt-1 font-mono text-[10px] text-steel">{statusLabel(step)}</p>
                                </div>
                            ))}
                        </div>

                        <div className="space-y-4">
                            <Waypoint
                                label="PICKUP / MANUFACTURER"
                                location={active.pickup}
                                range={distance(
                                    courier.x_position,
                                    courier.y_position,
                                    active.pickup.x_position,
                                    active.pickup.y_position,
                                )}
                            />
                            <Waypoint
                                label="DESTINATION / WAREHOUSE"
                                location={active.destination}
                                range={distance(
                                    active.pickup.x_position,
                                    active.pickup.y_position,
                                    active.destination.x_position,
                                    active.destination.y_position,
                                )}
                            />
                        </div>
                    </div>
                ) : (
                    <div className="grid h-full place-items-center p-10 text-center">
                        <div>
                            <div className="hazard mx-auto mb-4 h-1.5 w-16" />
                            <p className="font-mono text-xs tracking-widest text-ash">NO ACTIVE ORDER</p>
                            <p className="mt-1 text-xs text-steel">Waiting for dispatcher assignment</p>
                            {message && <p className="mt-4 font-mono text-[11px] text-steel">{message}</p>}
                        </div>
                    </div>
                )}
            </Panel>

            <Panel
                code="C-02"
                title="Order History"
                aside={<span className="font-mono text-[10px] text-steel">{history.length} DELIVERED</span>}
            >
                <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-slate font-mono text-[10px] tracking-[0.2em] text-ash">
                        <tr className="text-left">
                            <th className="px-4 py-2 font-normal">ID</th>
                            <th className="px-2 py-2 font-normal">MEDICINE</th>
                            <th className="px-2 py-2 text-right font-normal">QTY</th>
                            <th className="px-2 py-2 font-normal">ROUTE</th>
                            <th className="px-2 py-2 font-normal">PICKED UP</th>
                            <th className="px-4 py-2 font-normal">DELIVERED</th>
                        </tr>
                    </thead>
                    <tbody>
                        {history.map((t) => (
                            <tr key={t.id} className="border-t border-ash/20 hover:bg-ash/15">
                                <td className="px-4 py-2.5 font-mono text-steel">#{t.id}</td>
                                <td className="px-2 py-2.5 font-semibold">{t.medicine_name}</td>
                                <td className="px-2 py-2.5 text-right font-mono">{t.quantity}</td>
                                <td className="px-2 py-2.5 text-steel">
                                    {t.pickup.name} <span className="text-signal">→</span> {t.destination.name}
                                </td>
                                <td className="px-2 py-2.5 whitespace-nowrap font-mono text-xs text-steel">
                                    {formatTime(t.pickup_time)}
                                </td>
                                <td className="px-4 py-2.5 whitespace-nowrap font-mono text-xs text-steel">
                                    {formatTime(t.delivery_time)}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
                {history.length === 0 && (
                    <p className="p-10 text-center font-mono text-xs tracking-widest text-ash">NO DELIVERY YET</p>
                )}
            </Panel>
        </div>
    );
}
