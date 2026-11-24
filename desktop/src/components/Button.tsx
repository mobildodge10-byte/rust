import { ButtonHTMLAttributes } from "react";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & { variant?: "primary" | "ghost" };

const variants = {
    primary: "bg-signal text-slate hover:bg-white",
    ghost: "border border-ash text-steel hover:border-signal hover:text-signal",
};

export default function Button({ variant = "primary", className = "", ...props }: Props) {
    return (
        <button
            {...props}
            className={`cut-br h-10 px-5 font-mono text-xs font-bold tracking-[0.2em] uppercase transition-colors disabled:pointer-events-none disabled:opacity-30 ${variants[variant]} ${className}`}
        />
    );
}
