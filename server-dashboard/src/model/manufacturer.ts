export type Medicine = {
    id: number;
    name: string;
    description: string;
    medicine_type_id: number;
    dosage_form: string;
};

export type Manufacturer = {
    id: number;
    name: string;
    region: string;
    x_position: number;
    y_position: number;
    medicine: Medicine[];
};
