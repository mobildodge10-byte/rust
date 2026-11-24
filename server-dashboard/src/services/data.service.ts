import { api } from "../lib/axios";
import type { ActivityLog } from "../model/activity";
import type { EventLog } from "../model/event";
import type { Manufacturer } from "../model/manufacturer";
import type { Operator } from "../model/operator";
import type { Warehouse } from "../model/warehouse";

export async function fetchOperatorData(): Promise<Operator[]> {
    const response = await api.get("/dispatcher/get-operator-list");
    if (!response.status || response.status !== 200) {
        throw new Error(`Failed to fetch operator data: ${response.statusText}`);
    }

    const data: Operator[] = response.data;
    return data;
}

export async function fetchWarehouseData(): Promise<Warehouse[]> {
    const response = await api.get("/warehouse/get-warehouse-list");
    if (!response.status || response.status !== 200) {
        throw new Error(`Failed to fetch warehouse data: ${response.statusText}`);
    }

    const data: Warehouse[] = response.data;
    return data;
}

export async function fetchManufacturerData(): Promise<Manufacturer[]> {
    const response = await api.get("/manufacturer/get-manufacturer-list");
    if (!response.status || response.status !== 200) {
        throw new Error(`Failed to fetch manufacturer data: ${response.statusText}`);
    }

    const data: Manufacturer[] = response.data;
    return data;
}

export async function fetchEventLogData(): Promise<EventLog[]> {
    const response = await api.get("/dispatcher/get-event-log");
    if (!response.status || response.status !== 200) {
        throw new Error(`Failed to fetch event log data: ${response.statusText}`);
    }

    const data: EventLog[] = response.data;
    return data;
}

export async function fetchActivityLogData(): Promise<ActivityLog[]> {
    const response = await api.get("/dispatcher/get-activity-log");
    if (!response.status || response.status !== 200) {
        throw new Error(`Failed to fetch activity log data: ${response.statusText}`);
    }

    const data: ActivityLog[] = response.data;
    return data;
}
