use tracewake_core::runtime::{RuntimeReceipt, RuntimeReceiptKind};

pub fn extract_one_tick_wait_internals(receipt: &RuntimeReceipt) {
    if let RuntimeReceiptKind::OneTickAdvanced(result) = receipt.kind() {
        let _ = result.prior_tick;
        let _ = result.resulting_tick;
        let _ = result.appended_event_ids;
        let _ = result.due_work_summary;
        let _ = result.actor_step_summaries;
    }
}
