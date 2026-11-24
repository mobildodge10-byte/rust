export type WarehouseInventoryItem = {
    medicine_id: number;
    medicine_name: string;
    quantity: number;
    reorder_threshold: number | null;
};

export type Warehouse = {
    id: number;
    name: string;
    region: string;
    capacity: number;
    x_position: number;
    y_position: number;
    inventory: WarehouseInventoryItem[];
};
