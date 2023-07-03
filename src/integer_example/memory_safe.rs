struct BankAccount {
    name: String,
    balance: u32,
    has_credit_card: bool,
    account_history: Vec<i32>
}

pub fn struct_is_memory_safe() {
    let minsu_bank_account = BankAccount { // 민수의 계좌는 여기서부터 유효합니다
        name: String::from("minsu"),
        balance: 5000,
        has_credit_card: false,
        account_history: vec![4000, -6000, 3000]
    };

    if minsu_bank_account.balance >= 4500 &&
        minsu_bank_account.has_credit_card == true {
        println!("스벅 아이스 커피를 사먹을 수 있어요")
    } else {
        println!("스벅 아이스 커피를 사먹을 수 없어요")
    }
}
