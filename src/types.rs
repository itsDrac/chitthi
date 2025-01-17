use crate::chitthi::Cred;

enum ConfigMessages {
    AddAccount(Cred),
}

pub enum PopupMessages {
    ShowAddPopup,
    ShowQuitPopup,
    HideAddPopup,
    HideQuitPopup,
    AddCred,
}
