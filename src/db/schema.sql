-- =====================================================
-- EXTENSIONS
-- =====================================================
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =====================================================
-- ENUM TYPES
-- =====================================================

CREATE TYPE partner_type_enum AS ENUM ('customer', 'vendor', 'both');

CREATE TYPE movement_type_enum AS ENUM ('in', 'out', 'adjustment');

CREATE TYPE account_type AS ENUM ('ASSET','LIABILITY','EQUITY','INCOME','EXPENSE');

CREATE TYPE journal_status AS ENUM ('DRAFT','POSTED','CANCELLED');

CREATE TYPE payment_status AS ENUM ('UNPAID','PARTIALLY_PAID','PAID');

CREATE TYPE order_status AS ENUM ('DRAFT','PENDING','CONFIRMED','CANCELLED','COMPLETED');

CREATE TYPE transaction_type AS ENUM (
    'PAYMENT_RECEIVED',
    'PAYMENT_MADE',
    'REFUND_GIVEN',
    'REFUND_RECEIVED'
);

CREATE TYPE payment_method AS ENUM (
    'CASH',
    'BANK_TRANSFER',
    'CHECK',
    'CREDIT_CARD'
);

CREATE TYPE user_role AS ENUM (
    'admin',
    'accountant',
    'inventory',
    'sales',
    'purchasing'
);

-- =====================================================
-- COMPANIES
-- =====================================================
CREATE TABLE companies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    address TEXT,
    -- fiscal_year_start DATE NOT NULL DEFAULT CURRENT_DATE,
    base_currency_id UUID REFERENCES currencies(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- ================================
-- USERS
-- ================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'inventory',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- =====================================================
-- CURRENCIES
-- =====================================================
CREATE TABLE currencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) UNIQUE NOT NULL,
    name TEXT NOT NULL,
    symbol TEXT,
    is_active BOOLEAN DEFAULT true
);

CREATE TABLE currency_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    currency_id UUID REFERENCES currencies(id) ON DELETE CASCADE,
    rate NUMERIC(18,8) NOT NULL,
    rate_date DATE NOT NULL,
    UNIQUE(company_id, currency_id, rate_date)
);

-- =====================================================
-- PARTNERS (Customers / Vendors)
-- =====================================================
CREATE TABLE partners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type VARCHAR(20) CHECK (type IN ('customer','vendor','both')),
    email TEXT,
    phone TEXT,
    created_at TIMESTAMP DEFAULT now()
);

-- =====================================================
-- PRODUCTS
-- =====================================================
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    sku TEXT UNIQUE,
    barcode TEXT,
    attributes JSONB,
    cost_price NUMERIC(18,4) DEFAULT 0,
    selling_price NUMERIC(18,4) DEFAULT 0,
    inventory_account_id UUID,
    cogs_account_id UUID,
    revenue_account_id UUID,
    is_active BOOLEAN DEFAULT true
);

-- =====================================================
-- WAREHOUSES
-- =====================================================
CREATE TABLE warehouses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

-- =====================================================
-- STOCK MOVEMENTS
-- =====================================================
CREATE TABLE stock_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id),
    warehouse_id UUID REFERENCES warehouses(id),
    quantity NUMERIC(18,4) NOT NULL,
    movement_type VARCHAR(20) CHECK (movement_type IN ('in','out','adjustment')),
    reference_type TEXT,
    reference_id UUID,
    unit_cost NUMERIC(18,4),
    movement_date TIMESTAMP DEFAULT now()
);

CREATE INDEX idx_stock_variant ON stock_movements(variant_id);
CREATE INDEX idx_stock_warehouse ON stock_movements(warehouse_id);

-- =====================================================
-- STOCK LEDGER (Snapshot Table)
-- =====================================================
CREATE TABLE stock_ledger (
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    warehouse_id UUID REFERENCES warehouses(id) ON DELETE CASCADE,
    quantity NUMERIC(18,4) NOT NULL DEFAULT 0,
    PRIMARY KEY (variant_id, warehouse_id)
);

-- =====================================================
-- SALES
-- =====================================================
CREATE TABLE sales_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES partners(id),
    order_date DATE NOT NULL,
    currency_id UUID REFERENCES currencies(id),
    status VARCHAR(20),
    total_amount NUMERIC(18,4) DEFAULT 0,
    created_at TIMESTAMP DEFAULT now(),
    created_by UUID REFERENCES users(id)
);

CREATE TABLE sales_order_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sales_order_id UUID REFERENCES sales_orders(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id),
    quantity NUMERIC(18,4) NOT NULL,
    unit_price NUMERIC(18,4) NOT NULL,
    subtotal NUMERIC(18,4) NOT NULL
);

CREATE INDEX idx_so_lines_variant ON sales_order_lines(variant_id);

-- =====================================================
-- PURCHASE
-- =====================================================
CREATE TABLE purchase_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    vendor_id UUID REFERENCES partners(id),
    order_date DATE NOT NULL,
    currency_id UUID REFERENCES currencies(id),
    status VARCHAR(20),
    total_amount NUMERIC(18,4) DEFAULT 0,
    created_at TIMESTAMP DEFAULT now(),
    created_by UUID REFERENCES users(id)
);

CREATE TABLE purchase_order_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    purchase_order_id UUID REFERENCES purchase_orders(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id),
    quantity NUMERIC(18,4) NOT NULL,
    unit_cost NUMERIC(18,4) NOT NULL,
    subtotal NUMERIC(18,4) NOT NULL
);

CREATE INDEX idx_po_lines_variant ON purchase_order_lines(variant_id);

-- =====================================================
-- CHART OF ACCOUNTS
-- =====================================================
CREATE TABLE chart_of_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    code VARCHAR(20) NOT NULL,
    name TEXT NOT NULL,
    account_type VARCHAR(20) CHECK (
        account_type IN ('asset','liability','equity','income','expense')
    ),
    is_active BOOLEAN DEFAULT true,
    UNIQUE(company_id, code)
);

-- =====================================================
-- JOURNAL ENTRIES
-- =====================================================
CREATE TABLE journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    entry_date DATE NOT NULL,
    reference TEXT,
    currency_id UUID REFERENCES currencies(id),
    exchange_rate NUMERIC(18,8),
    posted BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT now(),
    created_by UUID REFERENCES users(id)
);

CREATE TABLE journal_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_entry_id UUID REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id UUID REFERENCES chart_of_accounts(id),
    partner_id UUID REFERENCES partners(id),
    debit NUMERIC(18,4) DEFAULT 0,
    credit NUMERIC(18,4) DEFAULT 0,
    currency_amount NUMERIC(18,4),
    description TEXT,
    CHECK (debit >= 0 AND credit >= 0)
);

CREATE INDEX idx_journal_account ON journal_lines(account_id);

ALTER TABLE journal_lines
ADD CONSTRAINT debit_credit_check
CHECK (debit >= 0 AND credit >= 0);

-- =====================================================
-- DOCUMENT POSTINGS (Link business docs to accounting)
-- =====================================================
CREATE TABLE document_postings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    document_type TEXT NOT NULL,
    document_id UUID NOT NULL,
    journal_entry_id UUID REFERENCES journal_entries(id),
    created_at TIMESTAMP DEFAULT now(),
    created_by UUID REFERENCES users(id)
);

-- =====================================================
-- REVALUATION SYSTEM
-- =====================================================
CREATE TABLE revaluation_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    currency_id UUID REFERENCES currencies(id),
    revaluation_date DATE NOT NULL,
    journal_entry_id UUID REFERENCES journal_entries(id),
    created_at TIMESTAMP DEFAULT now(),
    created_by UUID REFERENCES users(id)
);