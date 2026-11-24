import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Operator, Role } from "./model/operator";
import { OrderItem } from "./model/order-item";
import Header from "./components/Header";
import Sidebar from "./components/Sidebar";
import CourierPage from "./pages/CourierPage";
import DispatcherPage from "./pages/DispatcherPage";

export default function App() {
    const [operators, setOperators] = useState<Operator[]>([]);
    const [items, setItems] = useState<OrderItem[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [role, setRole] = useState<Role>("Dispatcher");
    const [selected, setSelected] = useState<Record<Role, number | null>>({ Dispatcher: null, Courier: null });

    useEffect(() => {
        const listeners = Promise.all([
            listen<Operator[]>("operator-list", (e) => {
                setOperators(e.payload);
                setError(null);
            }),
            listen<OrderItem[]>("order-item-list", (e) => setItems(e.payload)),
            listen<string>("server-error", (e) => setError(e.payload)),
        ]);
        listeners.then(() => invoke("refresh_data"));
        return () => {
            listeners.then((unlisten) => unlisten.forEach((fn) => fn()));
        };
    }, []);

    const options = operators.filter((o) => o.role === role);
    const current = options.find((o) => o.id === selected[role]) ?? options[0] ?? null;
    const couriers = operators.filter((o) => o.role === "Courier");

    return (
        <div className="flex h-full">
            <Sidebar active={role} onChange={setRole} />

            <main className="flex min-w-0 flex-1 flex-col">
                <Header
                    role={role}
                    operator={current}
                    options={options}
                    onSelect={(id) => setSelected({ ...selected, [role]: id })}
                />
                <div className="hazard h-1 shrink-0" />

                <div className="grid-field min-h-0 flex-1 p-4">
                    {error ? (
                        <p className="cut-br bg-slate p-6 font-mono text-sm text-signal">SERVER OFFLINE // {error}</p>
                    ) : role === "Dispatcher" ? (
                        <DispatcherPage couriers={couriers} items={items} anchor={current} />
                    ) : (
                        current && <CourierPage key={current.id} courier={current} items={items} />
                    )}
                </div>
            </main>
        </div>
    );
}
