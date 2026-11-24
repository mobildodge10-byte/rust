log

1. Repository (port/outbound/repository_trait.rs dan adapter/outbound/repository.rs): tambahkan fungsi insert.


// repository_trait.rs
fn insert_activity_log(&self, activity_log: ActivityLogActiveModel) -> impl Future<Output = Result<(), DbErr>> + Send;

// repository.rs
async fn insert_activity_log(&self, activity_log: ActivityLogActiveModel) -> Result<(), DbErr> {
    ac(StatusCode::INTERNAL_SERVER_ERROR),
        }

Helper-nya ditaruh di luar impl ServiceTrait, karena fungsi ini bukan bagian dari trait. Kalau ditaruh di dalam, rustc akan error E0407 seperti sebelumnya.

Helper: sisipkan di antara new_service (baris 35-37) dan impl ... ServiceTrait (baris 39):


impl<R: RepositoryTrait + Send + Sync> Service<R> {
    async fn log_activity(&self, description: String) {
        let activity = ActivityLogActiveModel {
            description: ActiveValue::Set(description),
            ..Default::default()
        };
        // Gagal mencatat log tidak boleh membatalkan aksi utama, cukup dicetak ke console
        if let Err(err) = self.repository.insert_activity_log(activity).await {
            eprintln!("Failed to write activity log: {err}");
        }
    }
}


Ada empat tempat. Baris di bawah sesuai service.rs 
1. dispatch_operator, baris 226, sebelum Ok(()):


        self.log_activity(format!("Operator #{} dispatched to order item #{}", operator_id, order_item_id)).await;

        Ok(())
2. complete_task, baris 375, sebelum Ok(()):


        self.log_activity(format!("Operator #{} delivered order item #{}", operator_id, order_item_id)).await;

        Ok(())
3. pickup_medicine, baris 439-442. Ganti Ok(_) => Ok(()), dengan:


        match pickup_result {
            Ok(_) => {
                self.log_activity(format!("Operator #{} picked up order item #{}", operator_id, order_item_id)).await;
                Ok(())
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
4. restock_warehouse (sekitar baris 843-849), log dihapus dan dikembalikan seperti semula:


match create_result {
    Ok(_) => Ok(()),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
}












sort

Langkah 1: App.tsx, ganti pemanggilan DispatcherPage:
<DispatcherPage couriers={couriers} items={items} anchor={current} />


Langkah 2: pages/DispatcherPage.tsx, import, pastikan baris pertama memakai useMemo:
import { useMemo, useState } from "react";


Langkah 3: pages/DispatcherPage.tsx, tipe props, ganti baris type Props:
type Props = { couriers: Operator[]; items: OrderItem[]; anchor: Operator | null };
Langkah 4: pages/DispatcherPage.tsx, signature komponen:
export default function DispatcherPage({ couriers, items, anchor }: Props) {


Langkah 5: pages/DispatcherPage.tsx, state, setelah useState untuk message:
    const [sortByProximity, setSortByProximity] = useState(true);


Langkah 6: pages/DispatcherPage.tsx, daftar terurut dan fungsi jarak, setelah baris const courier = ...:
    const orderedCouriers = useMemo(() => {
        if (!sortByProximity || anchor === null) return couriers;
        const origin = anchor;
        const gap = (c: Operator) => distance(c.x_position, c.y_position, origin.x_position, origin.y_position);
        return [...couriers].sort((a, b) => gap(a) - gap(b));
    }, [couriers, sortByProximity, anchor]);

    const distanceFromAnchor = (c: Operator) =>
        anchor ? Math.round(distance(c.x_position, c.y_position, anchor.x_position, anchor.y_position)) : 0;

Langkah 7: pages/DispatcherPage.tsx, tombol toggle di panel Couriers, ganti isi aside={...} pada panel code="D-02":


                aside={
                    <span className="flex items-center gap-3 font-mono text-[10px] text-steel">
                        <button
                            onClick={() => setSortByProximity((v) => !v)}
                            className={`border px-2 py-0.5 tracking-wider transition-colors ${
                                sortByProximity ? "border-signal text-signal" : "border-ash text-steel hover:text-white"
                            }`}
                        >
                            {sortByProximity ? "NEAREST FIRST" : "UNSORTED"}
                        </button>
                        <span>
                            {couriers.filter((c) => c.availability === 1).length}/{couriers.length} READY
                        </span>
                    </span>
                }
Langkah 8: pages/DispatcherPage.tsx, daftar kurir, ganti couriers.map di panel Couriers:


                    {orderedCouriers.map((c) => {
Langkah 9: pages/DispatcherPage.tsx, jarak di setiap baris kurir, tambahkan setelah <StatusMark ... /> di baris kurir:


                                        {anchor && <span className="ml-2 font-mono text-[10px] text-ash">{distanceFromAnchor(c)} u</span>}









                footer={
                    <div className="space-y-3">
                        <div className="grid grid-cols-2 gap-px bg-ash/40 font-mono text-xs">
                            <div className="bg-slate pr-3">
                                <p className="text-[10px] text-ash">ORDER ITEM</p>
                                <p className="truncate">
                                    {item ? `#${item.id} ${item.medicine_name}` : "Select ready item"}
                                </p>
                            </div>
                            <div className="bg-slate pl-3">
                                <p className="text-[10px] text-ash">COURIER</p>
                                <p className="truncate uppercase">{courier ? courier.codename : "Select courier"}</p>
                            </div>
                        </div>
                        <Button className="w-full" disabled={!item || !courier} onClick={assign}>
                            Assign Courier
                        </Button>
                        {message && <p className="font-mono text-[11px] text-steel">{message}</p>}
                    </div>
                }




optimize n+1
OPT #1: get_order_item di server/src/adapter/outbound/repository.rs, ganti seluruh isi fungsi (sekitar baris 159-173):


    async fn get_order_item(&self, order_item_id: i32) -> Result<OrderItemModelEx, DbErr> {
        let found = OrderItem::load()
            .filter(order_item::Column::Id.eq(order_item_id))
            .with((Medicine, Manufacturer))
            .with((Order, Warehouse))
            .with(OrderStatus)
            .with(Operator)
            .one(&self.database_connection)
            .await?;

        found.ok_or_else(|| DbErr::RecordNotFound(format!("Order item with ID {} not found", order_item_id)))
    }
OPT #3: helper mapping gudang. Letakkan di server/src/core/service/service.rs, sebelum baris impl<R: RepositoryTrait + Send + Sync> ServiceTrait for Service<R> {:


fn to_warehouse_list_item(warehouse: WarehouseModelEx, inventory: Vec<WarehouseInventoryItem>) -> WarehouseResponse {
    let region = match warehouse.region.into_option() {
        Some(region) => region.name,
        None => String::new(),
    };

    WarehouseResponse {
        id: warehouse.id,
        name: warehouse.name,
        region,
        capacity: warehouse.capacity,
        x_position: warehouse.x_position,
        y_position: warehouse.y_position,
        inventory,
    }
}
OPT #3: view_warehouse_list, ganti seluruh fungsi (sekitar baris 549-598):


    async fn view_warehouse_list(&self) -> Result<Vec<WarehouseResponse>, StatusCode> {
        let warehouses = self.repository.load_warehouses().await;
        if warehouses.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        // Semua stok diambil sekali, lalu dibagi per gudang di memori (tanpa query per gudang)
        let stock_rows = self.repository.load_all_warehouse_inventory().await;
        if stock_rows.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let mut stock_by_warehouse: HashMap<i32, Vec<WarehouseInventoryItem>> = HashMap::new();
        for row in stock_rows.unwrap() {
            let medicine_name = match row.medicine.into_option() {
                Some(medicine) => medicine.name,
                None => "Unnamed Medicine".to_string(),
            };

            stock_by_warehouse
                .entry(row.warehouse_id)
                .or_default()
                .push(WarehouseInventoryItem {
                    medicine_id: row.medicine_id,
                    medicine_name,
                    quantity: row.quantity,
                    reorder_threshold: Some(row.reorder_threshold),
                });
        }

        let mut warehouse_list = Vec::new();
        for warehouse in warehouses.unwrap() {
            let inventory = stock_by_warehouse.remove(&warehouse.id).unwrap_or_default();
            warehouse_list.push(to_warehouse_list_item(warehouse, inventory));
        }

        Ok(warehouse_list)
    }
Import di baris paling atas service.rs, tambahkan:


use std::collections::HashMap;
