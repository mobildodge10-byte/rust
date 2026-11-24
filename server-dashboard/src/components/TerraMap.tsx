import { useEffect, useState } from "react";
import type { Manufacturer } from "../model/manufacturer";
import type { Operator } from "../model/operator";
import type { Warehouse } from "../model/warehouse";
import Marker from "./Marker";

type Props = { operators: Operator[]; warehouses: Warehouse[]; manufacturers: Manufacturer[] };

export default function TerraMap({ operators, warehouses, manufacturers }: Props) {
    const [accent, setAccent] = useState("");
    const [region, setRegion] = useState<{ id: string; name: string; kind: string } | null>(null);

    useEffect(() => {
        fetch("/map/layers/03-accent.svg")
            .then((response) => response.text())
            .then(setAccent);
    }, []);

    const layer = "pointer-events-none absolute inset-0 size-full";

    return (
        <div className="grid size-full place-items-center p-4 [container-type:size]">
            <div className="relative aspect-[5/4] w-[min(100cqw,calc(100cqh*1.25))] bg-[#101315]">
                <img src="/map/layers/00-base.svg" className={layer} draggable={false} />
                <img src="/map/layers/01-terrain.svg" className={layer} draggable={false} />
                <img src="/map/layers/02-regions.svg" className={layer} draggable={false} />

                <div
                    className="absolute inset-0 [&_.accent]:cursor-crosshair [&_.accent]:transition-opacity [&_.accent:hover]:opacity-100 [&_svg]:size-full"
                    dangerouslySetInnerHTML={{ __html: accent }}
                    onMouseOver={(e) => {
                        const target = (e.target as Element).closest(".accent");
                        setRegion(
                            target && {
                                id: target.getAttribute("data-region-id") ?? "",
                                name: target.getAttribute("data-name") ?? "",
                                kind: target.getAttribute("data-kind") ?? "",
                            },
                        );
                    }}
                    onMouseLeave={() => setRegion(null)}
                />

                <img src="/map/layers/04-labels.svg" className={layer} draggable={false} />
                <img src="/map/hud.svg" className={layer} draggable={false} />

                <div className="pointer-events-none absolute inset-0">
                    {manufacturers.map((m) => (
                        <Marker
                            key={`m-${m.id}`}
                            x={m.x_position}
                            y={m.y_position}
                            title={m.name}
                            detail={`MANUFACTURER / ${m.region} / ${m.medicine.length} MEDICINE`}
                        >
                            <img src="/map/manufacturer.svg" className="size-6" draggable={false} />
                        </Marker>
                    ))}

                    {warehouses.map((w) => (
                        <Marker
                            key={`w-${w.id}`}
                            x={w.x_position}
                            y={w.y_position}
                            title={w.name}
                            detail={`WAREHOUSE / ${w.region} / ${w.inventory.reduce((sum, i) => sum + i.quantity, 0)} OF ${w.capacity}`}
                        >
                            <img src="/map/warehouse.svg" className="size-6" draggable={false} />
                        </Marker>
                    ))}

                    {operators.map((o) => (
                        <Marker
                            key={`o-${o.id}`}
                            x={o.x_position}
                            y={o.y_position}
                            title={o.codename}
                            detail={`${o.role} / ${o.role === "Courier" ? (o.availability === 1 ? "AVAILABLE" : "ON DUTY") : o.region_specialty}`}
                        >
                            <div
                                className={`cut-sm p-0.5 ${
                                    o.role === "Dispatcher" ? "bg-white" : o.availability === 1 ? "bg-signal" : "bg-ash"
                                }`}
                            >
                                <div className="bg-slate">
                                    <img
                                        src={`/operator/${o.codename.toLowerCase()}.png`}
                                        onError={(e) => {
                                            e.currentTarget.style.visibility = "hidden";
                                        }}
                                        className="size-6 object-cover"
                                        draggable={false}
                                    />
                                </div>
                            </div>
                        </Marker>
                    ))}
                </div>

                <div className="pointer-events-none absolute bottom-10 left-10 w-56 border-l-4 border-signal bg-slate/90 px-4 py-3 text-white">
                    <p className="font-mono text-[10px] tracking-[0.3em] text-steel">
                        {region ? `REGION ${region.id.padStart(2, "0")} / ${region.kind.toUpperCase()}` : "REGION"}
                    </p>
                    <p className="text-lg font-bold tracking-[0.15em] uppercase">{region?.name ?? "Hover a region"}</p>
                    {region && (
                        <p className="font-mono text-[10px] text-steel">
                            {warehouses.filter((w) => w.region === region.name).length} WAREHOUSE /{" "}
                            {manufacturers.filter((m) => m.region === region.name).length} MANUFACTURER
                        </p>
                    )}
                </div>
            </div>
        </div>
    );
}
