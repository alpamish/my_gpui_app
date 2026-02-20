use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

// Additional models for the new schema

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub base_currency_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompany {
    pub name: String,
    pub address: Option<String>,
    pub base_currency_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurrency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub partner_type: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartner {
    pub company_id: Uuid,
    pub name: String,
    pub partner_type: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub attributes: Option<serde_json::Value>,
    pub cost_price: f64,
    pub selling_price: f64,
    pub inventory_account_id: Option<Uuid>,
    pub cogs_account_id: Option<Uuid>,
    pub revenue_account_id: Option<Uuid>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductVariant {
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub attributes: Option<serde_json::Value>,
    pub cost_price: f64,
    pub selling_price: f64,
    pub inventory_account_id: Option<Uuid>,
    pub cogs_account_id: Option<Uuid>,
    pub revenue_account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarehouse {
    pub company_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub company_id: Uuid,
    pub variant_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: f64,
    pub movement_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_cost: Option<f64>,
    pub movement_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStockMovement {
    pub company_id: Uuid,
    pub variant_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: f64,
    pub movement_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub unit_cost: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLedger {
    pub variant_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrder {
    pub id: Uuid,
    pub company_id: Uuid,
    pub customer_id: Uuid,
    pub order_date: chrono::NaiveDate,
    pub currency_id: Uuid,
    pub status: String,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalesOrder {
    pub company_id: Uuid,
    pub customer_id: Uuid,
    pub order_date: chrono::NaiveDate,
    pub currency_id: Uuid,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrderLine {
    pub id: Uuid,
    pub sales_order_id: Uuid,
    pub variant_id: Uuid,
    pub quantity: f64,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSalesOrderLine {
    pub sales_order_id: Uuid,
    pub variant_id: Uuid,
    pub quantity: f64,
    pub unit_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub company_id: Uuid,
    pub vendor_id: Uuid,
    pub order_date: chrono::NaiveDate,
    pub currency_id: Uuid,
    pub status: String,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePurchaseOrder {
    pub company_id: Uuid,
    pub vendor_id: Uuid,
    pub order_date: chrono::NaiveDate,
    pub currency_id: Uuid,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderLine {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub variant_id: Uuid,
    pub quantity: f64,
    pub unit_cost: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePurchaseOrderLine {
    pub purchase_order_id: Uuid,
    pub variant_id: Uuid,
    pub quantity: f64,
    pub unit_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartOfAccount {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChartOfAccount {
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub company_id: Uuid,
    pub entry_date: chrono::NaiveDate,
    pub reference: Option<String>,
    pub currency_id: Option<Uuid>,
    pub exchange_rate: Option<f64>,
    pub posted: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalEntry {
    pub company_id: Uuid,
    pub entry_date: chrono::NaiveDate,
    pub reference: Option<String>,
    pub currency_id: Option<Uuid>,
    pub exchange_rate: Option<f64>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub id: Uuid,
    pub journal_entry_id: Uuid,
    pub account_id: Uuid,
    pub partner_id: Option<Uuid>,
    pub debit: f64,
    pub credit: f64,
    pub currency_amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalLine {
    pub journal_entry_id: Uuid,
    pub account_id: Uuid,
    pub partner_id: Option<Uuid>,
    pub debit: f64,
    pub credit: f64,
    pub currency_amount: Option<f64>,
    pub description: Option<String>,
}
