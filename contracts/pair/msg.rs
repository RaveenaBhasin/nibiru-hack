use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum TokenInfo {
    CW20Token {
        contract_addr : Addr,
    },
    NativeToken {
        denom: String,
    }
}

impl TokenInfo {
    pub fn get_as_bytes(&self) -> &[u8]  {
        match self {
            TokenInfo::CW20Token { contract_addr } => contract_addr.as_bytes(),
            TokenInfo::NativeToken { denom } => denom.as_bytes()
        }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_info: [TokenInfo; 2],
    pub lp_token_decimal: u8,
}

#[cw_serde]
pub struct Token {
    pub info: TokenInfo,
    pub amount: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    SwapAsset {
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u64,
        min_amount_out: u64,
    },
    AddLiquidity{
        assets: [Token; 2],
        min_liquidity_amt : u64,
    },
    RemoveLiquidity {
        lp_token: Token,
    },
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    PoolInfo{}
}