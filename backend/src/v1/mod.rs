use rocket::Route;

pub(crate) mod receipts;
pub(crate) mod greeting; 

pub fn receipt_routes() -> Vec<Route> {
    routes![
        receipts::upload_receipt,
        receipts::get_receipts,
        receipts::post_receipt,
        receipts::get_receipt,
        receipts::get_receipt_file,
    ]
}