import { useState } from "react";
import { Operator } from "../model/operator";

type Props = { operator: Operator; size?: string };

export default function OperatorIcon({ operator, size = "size-10" }: Props) {
    const [broken, setBroken] = useState(false);

    return (
        <div className={`${size} shrink-0 overflow-hidden bg-ash`}>
            {broken ? (
                <div className="grid size-full place-items-center font-mono text-sm font-bold text-slate">
                    {operator.codename.slice(0, 2).toUpperCase()}
                </div>
            ) : (
                <img
                    src={`/operator/${operator.codename.toLowerCase()}.png`}
                    alt={operator.codename}
                    onError={() => setBroken(true)}
                    className="size-full object-cover"
                    draggable={false}
                />
            )}
        </div>
    );
}
