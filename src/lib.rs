#[cfg(feature = "accounts-manager")]
pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}

#[cfg(feature = "accounts-manager-persistence")]
pub mod accounts_manager_persistence_grpc {
    tonic::include_proto!("accounts_manager_persistence");
}

#[cfg(feature = "authentication-integration")]
pub mod authentication_integration_grpc {
    tonic::include_proto!("authentication_integration");
}

#[cfg(feature = "candles")]
pub mod candles_grpc {
    tonic::include_proto!("candles_grpc");
}

#[cfg(feature = "comments-flows")]
pub mod comments_flows_grpc {
    tonic::include_proto!("comment_flows");
}

#[cfg(feature = "deposit-flows")]
pub mod deposit_flows_grpc {
    tonic::include_proto!("deposit_flows");
}

#[cfg(feature = "deposit-log")]
pub mod deposit_log_grpc {
    tonic::include_proto!("deposit_log");
}

#[cfg(feature = "documents-flows")]
pub mod documents_flows_grpc {
    tonic::include_proto!("docs");
}

#[cfg(feature = "email-bridge")]
pub mod email_bridge_grpc {
    tonic::include_proto!("email_bridge");
}

#[cfg(feature = "favorite_instruments_flows")]
pub mod favorite_instruments_flows_grpc {
    tonic::include_proto!("favorite_instruments_flows");
}

#[cfg(feature = "keyvalue")]
pub mod keyvalue_grpc {
    tonic::include_proto!("keyvalue");
}

#[cfg(feature = "kyc-log")]
pub mod kyc_log_grpc {
    tonic::include_proto!("kyclog");
}

#[cfg(feature = "kyc")]
pub mod kyc_grpc {
    tonic::include_proto!("kyc");
}

#[cfg(feature = "manager-access-flows")]
pub mod manager_access_flows_grpc {
    tonic::include_proto!("manager_access_flows");
}

#[cfg(feature = "pci-dss-bank-cards")]
pub mod pci_dss_bank_cards_grpc {
    tonic::include_proto!("pci_dss_bank_cards");
}

#[cfg(feature = "personal-data-flows")]
pub mod personal_data_flows_grpc {
    tonic::include_proto!("pd");
}

#[cfg(feature = "psp-redirect-integration")]
pub mod psp_redirect_integration_grpc {
    tonic::include_proto!("psp_redirect_integration");
}

#[cfg(feature = "report-grpc")]
pub mod report_grpc_grpc {
    tonic::include_proto!("report_grpc");
}

#[cfg(feature = "reset-password-codes")]
pub mod reset_password_codes_grpc {
    tonic::include_proto!("reset_password_codes");
}

#[cfg(feature = "status-flows")]
pub mod status_flows_grpc {
    tonic::include_proto!("status_flows");
}

#[cfg(feature = "trade-log")]
pub mod trade_log_grpc {
    tonic::include_proto!("trade_log");
}

#[cfg(feature = "trader-credentials")]
pub mod trader_credentials_grpc {
    tonic::include_proto!("trader_credentials");
}

#[cfg(feature = "trading-executor")]
pub mod trading_executor_grpc {
    tonic::include_proto!("trading_executor");
}

#[cfg(feature = "verifications")]
pub mod verifications_grpc {
    tonic::include_proto!("verifications");
}

#[cfg(feature = "withdrawals-flows")]
pub mod withdrawals_flows_grpc {
    tonic::include_proto!("withdrawals_flows");
}

#[cfg(feature = "withdrawal-log")]
pub mod withdrawal_log_grpc {
    tonic::include_proto!("withdrawal_log");
}

#[cfg(feature = "position-manager")]
pub mod position_manager_grpc {
    tonic::include_proto!("position_manager");
}

#[cfg(feature = "position-manager-persistence")]
pub mod position_manager_persistence_grpc {
    tonic::include_proto!("position_manager_persistence");
}
