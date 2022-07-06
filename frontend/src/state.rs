use std::rc::Rc;
use ulid::Ulid;
use yew::prelude::*;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, Display};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub receipts: Vec<Receipt>,
    pub filter: Filter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Receipt {
    pub id: Ulid,
    pub description: String,
    pub state: ReceiptState,
}


#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum ReceiptState {
    Inbox,
    Valid,
    Payed,
    Declined,
    Process,
    Done
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Inbox,
    Valid,
    Process
}

impl Filter {
    pub fn fits(&self, receipt: &Receipt) -> bool {
        match *self {
            Filter::All => true,
            Filter::Inbox => receipt.state == ReceiptState::Inbox,
            Filter::Valid => receipt.state == ReceiptState::Valid,
            Filter::Process => receipt.state == ReceiptState::Process,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/all",
            Filter::Inbox => "#/inbox",
            Filter::Valid => "#/valid",
            Filter::Process => "#/process",
        }
    }
}

pub enum Action {
    UploadReceipt,
    MarkBillAsValid,
    DeclineBill,
    MarkBillAsPayed,
    AddRecipient,
    SetBillAmount,
    SetBillCategory,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::UploadReceipt => todo!(),
            Action::MarkBillAsValid => todo!(),
            Action::DeclineBill => todo!(),
            Action::MarkBillAsPayed => todo!(),
            Action::AddRecipient => todo!(),
            Action::SetBillAmount => todo!(),
            Action::SetBillCategory => todo!(),
        }
    }

}