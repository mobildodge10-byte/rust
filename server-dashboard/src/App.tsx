import { useEffect, useState } from "react";
import type { ActivityLog } from "./model/activity";
import type { EventLog } from "./model/event";
import type { Manufacturer } from "./model/manufacturer";
import type { Operator } from "./model/operator";
import type { Warehouse } from "./model/warehouse";
import {
    fetchActivityLogData,
    fetchEventLogData,
    fetchManufacturerData,
    fetchOperatorData,
    fetchWarehouseData,
} from "./services/data.service";
import LogList from "./components/LogList";
import Panel from "./components/Panel";
import TerraMap from "./components/TerraMap";
import TopBar from "./components/TopBar";

const POLL_INTERVAL = 3000;

export default function App() {
    const [operators, setOperators] = useState<Operator[]>([]);
    const [warehouses, setWarehouses] = useState<Warehouse[]>([]);
    const [manufacturers, setManufacturers] = useState<Manufacturer[]>([]);
    const [eventLogs, setEventLogs] = useState<EventLog[]>([]);
    const [activityLogs, setActivityLogs] = useState<ActivityLog[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [syncedAt, setSyncedAt] = useState<Date | null>(null);

    useEffect(() => {
        function load() {
            Promise.all([
                fetchOperatorData(),
                fetchWarehouseData(),
                fetchManufacturerData(),
                fetchEventLogData(),
                fetchActivityLogData(),
            ])
                .then(([operators, warehouses, manufacturers, eventLogs, activityLogs]) => {
                    setOperators(operators);
                    setWarehouses(warehouses);
                    setManufacturers(manufacturers);
                    setEventLogs(eventLogs);
                    setActivityLogs(activityLogs);
                    setError(null);
                    setSyncedAt(new Date());
                })
                .catch((err) => setError(String(err)));
        }

        load();
        const timer = setInterval(load, POLL_INTERVAL);
        return () => clearInterval(timer);
    }, []);

    const couriers = operators.filter((o) => o.role === "Courier");

    return (
        <div className="flex h-full flex-col">
            <TopBar
                error={error}
                syncedAt={syncedAt}
                stats={[
                    { label: "Operators", value: operators.length },
                    {
                        label: "Couriers Ready",
                        value: `${couriers.filter((c) => c.availability === 1).length}/${couriers.length}`,
                    },
                    { label: "Warehouses", value: warehouses.length },
                    { label: "Manufacturers", value: manufacturers.length },
                ]}
            />
            <div className="hazard h-1 shrink-0" />

            <main className="grid-field grid min-h-0 flex-1 grid-cols-[1fr_420px] gap-4 p-4">
                <Panel
                    code="M-01"
                    title="Terra Map"
                    aside={
                        <span className="flex items-center gap-4 font-mono text-[10px] text-steel">
                            <span className="flex items-center gap-1.5">
                                <span className="size-2 bg-signal" /> COURIER
                            </span>
                            <span className="flex items-center gap-1.5">
                                <span className="size-2 bg-white" /> DISPATCHER
                            </span>
                            <span className="flex items-center gap-1.5">
                                <img src="/map/warehouse.svg" className="size-3.5" /> WAREHOUSE
                            </span>
                            <span className="flex items-center gap-1.5">
                                <img src="/map/manufacturer.svg" className="size-3.5" /> MANUFACTURER
                            </span>
                        </span>
                    }
                >
                    <TerraMap operators={operators} warehouses={warehouses} manufacturers={manufacturers} />
                </Panel>

                <div className="grid min-h-0 grid-rows-2 gap-4">
                    <Panel
                        code="L-01"
                        title="Event Log"
                        aside={<span className="font-mono text-[10px] text-steel">{eventLogs.length} EVENTS</span>}
                    >
                        <LogList
                            empty="NO EVENT YET"
                            entries={[...eventLogs].reverse().map((log) => ({
                                id: log.id,
                                time: log.created_at,
                                tag: `T${String(log.event_type_id).padStart(2, "0")}`,
                                text: log.description,
                            }))}
                        />
                    </Panel>

                    <Panel
                        code="L-02"
                        title="Activity Log"
                        aside={<span className="font-mono text-[10px] text-steel">{activityLogs.length} ENTRIES</span>}
                    >
                        <LogList
                            empty="NO ACTIVITY YET"
                            entries={[...activityLogs].reverse().map((log) => ({
                                id: log.id,
                                time: log.timestamp,
                                text: log.description,
                            }))}
                        />
                    </Panel>
                </div>
            </main>
        </div>
    );
}
