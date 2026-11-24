export type Location = {
    name: string;
    x_position: number;
    y_position: number;
};

export type OrderItem = {
    id: number;
    order_id: number;
    medicine_id: number;
    medicine_name: string;
    quantity: number;
    status: OrderStatus;
    courier_id: number | null;
    courier_codename: string | null;
    pickup: Location; // manufacturer
    destination: Location; // warehouse
    pickup_time: string | null;
    delivery_time: string | null;
};

export const ORDER_STATUSES = [
    "PENDING",
    "IN_PRODUCTION",
    "READY_FOR_PICKUP",
    "ASSIGNED",
    "IN_TRANSIT",
    "DELIVERED",
] as const;

export type OrderStatus = (typeof ORDER_STATUSES)[number];

export function statusLabel(status: string) {
    return status.split("_").join(" ");
}

export function formatTime(time: string | null) {
    return time ? new Date(time).toLocaleString([], { dateStyle: "short", timeStyle: "short" }) : "-";
}

export function distance(x1: number, y1: number, x2: number, y2: number) {
    return Math.hypot(x2 - x1, y2 - y1);
}
