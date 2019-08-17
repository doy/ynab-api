#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: String,
    pub approved: bool,
    pub flag_color: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub import_id: Option<String>,

    pub account: Option<String>,
    pub payee: Option<String>,
    pub total_amount: i64,
    pub reimbursed: bool,
    pub selected: bool,
}

impl Transaction {
    pub fn from_transaction(
        t: &ynab_api::models::TransactionSummary,
    ) -> Self {
        let reimbursed = if let Some(color) = &t.flag_color {
            color == "green"
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: t.amount,
            memo: t.memo.clone(),
            cleared: t.cleared.clone(),
            approved: t.approved,
            flag_color: t.flag_color.clone(),
            account_id: t.account_id.clone(),
            payee_id: t.payee_id.clone(),
            category_id: t.category_id.clone(),
            import_id: t.import_id.clone(),

            account: None,
            payee: None,
            total_amount: t.amount,
            reimbursed,
            selected: false,
        }
    }

    pub fn from_sub_transaction(
        t: &ynab_api::models::TransactionSummary,
        st: &ynab_api::models::SubTransaction,
    ) -> Self {
        let reimbursed = if let Some(color) = &t.flag_color {
            color == "green"
        } else {
            false
        };
        Self {
            id: t.id.clone(),
            date: t.date.clone(),
            amount: st.amount,
            memo: t.memo.clone(),
            cleared: t.cleared.clone(),
            approved: t.approved,
            flag_color: t.flag_color.clone(),
            account_id: t.account_id.clone(),
            payee_id: t.payee_id.clone(),
            category_id: t.category_id.clone(),
            import_id: t.import_id.clone(),

            account: None,
            payee: None,
            total_amount: t.amount,
            reimbursed,
            selected: false,
        }
    }

    pub fn to_update_transaction(
        &self,
    ) -> ynab_api::models::UpdateTransaction {
        let mut ut = ynab_api::models::UpdateTransaction::new(
            self.account_id.clone(),
            self.date.clone(),
            self.amount,
        );
        ut.id = Some(self.id.clone());
        ut.payee_id = self.payee_id.clone();
        ut.category_id = self.category_id.clone();
        ut.memo = self.memo.clone();
        ut.cleared = Some(self.cleared.clone());
        ut.approved = Some(self.approved);
        ut.flag_color = self.flag_color.clone();
        ut.import_id = self.import_id.clone();

        ut
    }
}
