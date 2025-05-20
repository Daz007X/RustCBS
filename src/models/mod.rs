// ในไฟล์ mod.rs
pub mod header_model;  // ไฟล์ชื่อ header_model.rs
pub mod pr_slk_model;
pub mod response_model;
pub mod rs_body_model;

// Re-export types
pub use header_model::HeaderModel;
pub use pr_slk_model::PrSlkModel;
pub use response_model::ResponseModel;
pub use rs_body_model::RsBodyModel;