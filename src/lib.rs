mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use pb::userdeposit::v1 as userdeposit;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams::store::{StoreGet, StoreGetBigDecimal};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::{BigDecimal, BigInt};
use std::ops::Mul;

const TRACKED_CONTRACT: [u8; 20] = hex!("ae7ab96520de3a18e5e111b5eaab095312d7fe84");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        approvals: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Approval::match_and_decode(log) {
                            return Some(contract::Approval {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                owner: event.owner,
                                spender: event.spender,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        cl_validators_updateds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ClValidatorsUpdated::match_and_decode(log) {
                            return Some(contract::ClValidatorsUpdated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                post_cl_validators: event.post_cl_validators.to_string(),
                                pre_cl_validators: event.pre_cl_validators.to_string(),
                                report_timestamp: event.report_timestamp.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        contract_version_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ContractVersionSet::match_and_decode(log) {
                            return Some(contract::ContractVersionSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        deposited_validators_changeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::DepositedValidatorsChanged::match_and_decode(log) {
                            return Some(contract::DepositedValidatorsChanged {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                deposited_validators: event.deposited_validators.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        eip_712_st_eth_initializeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Eip712StEthInitialized::match_and_decode(log) {
                            return Some(contract::Eip712StEthInitialized {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                eip_712_st_eth: event.eip712_st_eth,
                            });
                        }

                        None
                })
            })
            .collect(),
        el_rewards_receiveds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ElRewardsReceived::match_and_decode(log) {
                            return Some(contract::ElRewardsReceived {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        eth_distributeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::EthDistributed::match_and_decode(log) {
                            return Some(contract::EthDistributed {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                execution_layer_rewards_withdrawn: event.execution_layer_rewards_withdrawn.to_string(),
                                post_buffered_ether: event.post_buffered_ether.to_string(),
                                post_cl_balance: event.post_cl_balance.to_string(),
                                pre_cl_balance: event.pre_cl_balance.to_string(),
                                report_timestamp: event.report_timestamp.to_string(),
                                withdrawals_withdrawn: event.withdrawals_withdrawn.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        lido_locator_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::LidoLocatorSet::match_and_decode(log) {
                            return Some(contract::LidoLocatorSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                lido_locator: event.lido_locator,
                            });
                        }

                        None
                })
            })
            .collect(),
        proxy_deposits: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ProxyDeposit::match_and_decode(log) {
                            return Some(contract::ProxyDeposit {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                sender: event.sender,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        recover_to_vaults: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::RecoverToVault::match_and_decode(log) {
                            return Some(contract::RecoverToVault {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                token: event.token,
                                vault: event.vault,
                            });
                        }

                        None
                })
            })
            .collect(),
        resumeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Resumed::match_and_decode(log) {
                            return Some(contract::Resumed {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        script_results: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ScriptResult::match_and_decode(log) {
                            return Some(contract::ScriptResult {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                executor: event.executor,
                                input: event.input,
                                return_data: event.return_data,
                                script: event.script,
                            });
                        }

                        None
                })
            })
            .collect(),
        shares_burnts: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::SharesBurnt::match_and_decode(log) {
                            return Some(contract::SharesBurnt {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                                post_rebase_token_amount: event.post_rebase_token_amount.to_string(),
                                pre_rebase_token_amount: event.pre_rebase_token_amount.to_string(),
                                shares_amount: event.shares_amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        staking_limit_removeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::StakingLimitRemoved::match_and_decode(log) {
                            return Some(contract::StakingLimitRemoved {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        staking_limit_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::StakingLimitSet::match_and_decode(log) {
                            return Some(contract::StakingLimitSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                max_stake_limit: event.max_stake_limit.to_string(),
                                stake_limit_increase_per_block: event.stake_limit_increase_per_block.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        staking_pauseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::StakingPaused::match_and_decode(log) {
                            return Some(contract::StakingPaused {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        staking_resumeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::StakingResumed::match_and_decode(log) {
                            return Some(contract::StakingResumed {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        stoppeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Stopped::match_and_decode(log) {
                            return Some(contract::Stopped {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        submitteds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Submitted::match_and_decode(log) {
                            return Some(contract::Submitted {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                referral: event.referral,
                                sender: event.sender,
                            });
                        }

                        None
                })
            })
            .collect(),
        token_rebaseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::TokenRebased::match_and_decode(log) {
                            return Some(contract::TokenRebased {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                post_total_ether: event.post_total_ether.to_string(),
                                post_total_shares: event.post_total_shares.to_string(),
                                pre_total_ether: event.pre_total_ether.to_string(),
                                pre_total_shares: event.pre_total_shares.to_string(),
                                report_timestamp: event.report_timestamp.to_string(),
                                shares_minted_as_fees: event.shares_minted_as_fees.to_string(),
                                time_elapsed: event.time_elapsed.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        transfers: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Transfer::match_and_decode(log) {
                            return Some(contract::Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        transfer_shares: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::TransferShares::match_and_decode(log) {
                            return Some(contract::TransferShares {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                shares_value: event.shares_value.to_string(),
                                to: event.to,
                            });
                        }

                        None
                })
            })
            .collect(),
        unbuffereds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Unbuffered::match_and_decode(log) {
                            return Some(contract::Unbuffered {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        withdrawals_receiveds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::WithdrawalsReceived::match_and_decode(log) {
                            return Some(contract::WithdrawalsReceived {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn deposit_and_price(events: contract::Events, chainlink_prices: StoreGetBigDecimal) -> Result<userdeposit::UserDeposits, substreams::errors::Error> {
    
    let eth_price = chainlink_prices.get_last("price_by_symbol:ETH:USD").map_or(BigDecimal::from(0), |price| price);

    substreams::log::println(format!("{:?}", eth_price));

    let all_deposits: Vec<userdeposit::UserDeposit> = events.submitteds
    .into_iter()
    .map(|original| {

        let convert_amount = BigInt::from_str(&original.amount).unwrap_or(BigInt::from(0)).to_decimal(BigInt::from(18).to_u64());

        userdeposit::UserDeposit {
            evt_tx_hash: original.evt_tx_hash,
            evt_index: original.evt_index,
            evt_block_time: original.evt_block_time,
            evt_block_number: original.evt_block_number,
            sender : original.sender,
            amount: original.amount,
            referral: original.referral,
            usd_value: eth_price.clone().mul(convert_amount).to_string()
        }
    })
    .collect();

    Ok(userdeposit::UserDeposits { deposits: all_deposits })

}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.cl_validators_updateds.into_iter().for_each(|evt| {
        tables
            .create_row("cl_validators_updated", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("post_cl_validators", BigDecimal::from_str(&evt.post_cl_validators).unwrap())
            .set("pre_cl_validators", BigDecimal::from_str(&evt.pre_cl_validators).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap());
    });
    events.contract_version_sets.into_iter().for_each(|evt| {
        tables
            .create_row("contract_version_set", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", BigDecimal::from_str(&evt.version).unwrap());
    });
    events.deposited_validators_changeds.into_iter().for_each(|evt| {
        tables
            .create_row("deposited_validators_changed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("deposited_validators", BigDecimal::from_str(&evt.deposited_validators).unwrap());
    });
    events.eip_712_st_eth_initializeds.into_iter().for_each(|evt| {
        tables
            .create_row("eip_712_st_eth_initialized", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("eip_712_st_eth", Hex(&evt.eip_712_st_eth).to_string());
    });
    events.el_rewards_receiveds.into_iter().for_each(|evt| {
        tables
            .create_row("el_rewards_received", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.eth_distributeds.into_iter().for_each(|evt| {
        tables
            .create_row("eth_distributed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("execution_layer_rewards_withdrawn", BigDecimal::from_str(&evt.execution_layer_rewards_withdrawn).unwrap())
            .set("post_buffered_ether", BigDecimal::from_str(&evt.post_buffered_ether).unwrap())
            .set("post_cl_balance", BigDecimal::from_str(&evt.post_cl_balance).unwrap())
            .set("pre_cl_balance", BigDecimal::from_str(&evt.pre_cl_balance).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap())
            .set("withdrawals_withdrawn", BigDecimal::from_str(&evt.withdrawals_withdrawn).unwrap());
    });
    events.lido_locator_sets.into_iter().for_each(|evt| {
        tables
            .create_row("lido_locator_set", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("lido_locator", Hex(&evt.lido_locator).to_string());
    });
    events.proxy_deposits.into_iter().for_each(|evt| {
        tables
            .create_row("proxy_deposit", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("sender", Hex(&evt.sender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.recover_to_vaults.into_iter().for_each(|evt| {
        tables
            .create_row("recover_to_vault", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("token", Hex(&evt.token).to_string())
            .set("vault", Hex(&evt.vault).to_string());
    });
    events.resumeds.into_iter().for_each(|evt| {
        tables
            .create_row("resumed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.script_results.into_iter().for_each(|evt| {
        tables
            .create_row("script_result", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("executor", Hex(&evt.executor).to_string())
            .set("input", Hex(&evt.input).to_string())
            .set("return_data", Hex(&evt.return_data).to_string())
            .set("script", Hex(&evt.script).to_string());
    });
    events.shares_burnts.into_iter().for_each(|evt| {
        tables
            .create_row("shares_burnt", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string())
            .set("post_rebase_token_amount", BigDecimal::from_str(&evt.post_rebase_token_amount).unwrap())
            .set("pre_rebase_token_amount", BigDecimal::from_str(&evt.pre_rebase_token_amount).unwrap())
            .set("shares_amount", BigDecimal::from_str(&evt.shares_amount).unwrap());
    });
    events.staking_limit_removeds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_limit_removed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.staking_limit_sets.into_iter().for_each(|evt| {
        tables
            .create_row("staking_limit_set", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("max_stake_limit", BigDecimal::from_str(&evt.max_stake_limit).unwrap())
            .set("stake_limit_increase_per_block", BigDecimal::from_str(&evt.stake_limit_increase_per_block).unwrap());
    });
    events.staking_pauseds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_paused", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.staking_resumeds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_resumed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.stoppeds.into_iter().for_each(|evt| {
        tables
            .create_row("stopped", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.submitteds.into_iter().for_each(|evt| {
        tables
            .create_row("submitted", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("referral", Hex(&evt.referral).to_string())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.token_rebaseds.into_iter().for_each(|evt| {
        tables
            .create_row("token_rebased", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("post_total_ether", BigDecimal::from_str(&evt.post_total_ether).unwrap())
            .set("post_total_shares", BigDecimal::from_str(&evt.post_total_shares).unwrap())
            .set("pre_total_ether", BigDecimal::from_str(&evt.pre_total_ether).unwrap())
            .set("pre_total_shares", BigDecimal::from_str(&evt.pre_total_shares).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap())
            .set("shares_minted_as_fees", BigDecimal::from_str(&evt.shares_minted_as_fees).unwrap())
            .set("time_elapsed", BigDecimal::from_str(&evt.time_elapsed).unwrap());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.transfer_shares.into_iter().for_each(|evt| {
        tables
            .create_row("transfer_shares", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("shares_value", BigDecimal::from_str(&evt.shares_value).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.unbuffereds.into_iter().for_each(|evt| {
        tables
            .create_row("unbuffered", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.withdrawals_receiveds.into_iter().for_each(|evt| {
        tables
            .create_row("withdrawals_received", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.cl_validators_updateds.into_iter().for_each(|evt| {
        tables
            .create_row("cl_validators_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("post_cl_validators", BigDecimal::from_str(&evt.post_cl_validators).unwrap())
            .set("pre_cl_validators", BigDecimal::from_str(&evt.pre_cl_validators).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap());
    });
    events.contract_version_sets.into_iter().for_each(|evt| {
        tables
            .create_row("contract_version_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", BigDecimal::from_str(&evt.version).unwrap());
    });
    events.deposited_validators_changeds.into_iter().for_each(|evt| {
        tables
            .create_row("deposited_validators_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("deposited_validators", BigDecimal::from_str(&evt.deposited_validators).unwrap());
    });
    events.eip_712_st_eth_initializeds.into_iter().for_each(|evt| {
        tables
            .create_row("eip_712_st_eth_initialized", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("eip_712_st_eth", Hex(&evt.eip_712_st_eth).to_string());
    });
    events.el_rewards_receiveds.into_iter().for_each(|evt| {
        tables
            .create_row("el_rewards_received", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.eth_distributeds.into_iter().for_each(|evt| {
        tables
            .create_row("eth_distributed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("execution_layer_rewards_withdrawn", BigDecimal::from_str(&evt.execution_layer_rewards_withdrawn).unwrap())
            .set("post_buffered_ether", BigDecimal::from_str(&evt.post_buffered_ether).unwrap())
            .set("post_cl_balance", BigDecimal::from_str(&evt.post_cl_balance).unwrap())
            .set("pre_cl_balance", BigDecimal::from_str(&evt.pre_cl_balance).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap())
            .set("withdrawals_withdrawn", BigDecimal::from_str(&evt.withdrawals_withdrawn).unwrap());
    });
    events.lido_locator_sets.into_iter().for_each(|evt| {
        tables
            .create_row("lido_locator_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("lido_locator", Hex(&evt.lido_locator).to_string());
    });
    events.proxy_deposits.into_iter().for_each(|evt| {
        tables
            .create_row("proxy_deposit", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("sender", Hex(&evt.sender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.recover_to_vaults.into_iter().for_each(|evt| {
        tables
            .create_row("recover_to_vault", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("token", Hex(&evt.token).to_string())
            .set("vault", Hex(&evt.vault).to_string());
    });
    events.resumeds.into_iter().for_each(|evt| {
        tables
            .create_row("resumed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.script_results.into_iter().for_each(|evt| {
        tables
            .create_row("script_result", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("executor", Hex(&evt.executor).to_string())
            .set("input", Hex(&evt.input).to_string())
            .set("return_data", Hex(&evt.return_data).to_string())
            .set("script", Hex(&evt.script).to_string());
    });
    events.shares_burnts.into_iter().for_each(|evt| {
        tables
            .create_row("shares_burnt", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string())
            .set("post_rebase_token_amount", BigDecimal::from_str(&evt.post_rebase_token_amount).unwrap())
            .set("pre_rebase_token_amount", BigDecimal::from_str(&evt.pre_rebase_token_amount).unwrap())
            .set("shares_amount", BigDecimal::from_str(&evt.shares_amount).unwrap());
    });
    events.staking_limit_removeds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_limit_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.staking_limit_sets.into_iter().for_each(|evt| {
        tables
            .create_row("staking_limit_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("max_stake_limit", BigDecimal::from_str(&evt.max_stake_limit).unwrap())
            .set("stake_limit_increase_per_block", BigDecimal::from_str(&evt.stake_limit_increase_per_block).unwrap());
    });
    events.staking_pauseds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_paused", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.staking_resumeds.into_iter().for_each(|evt| {
        tables
            .create_row("staking_resumed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.stoppeds.into_iter().for_each(|evt| {
        tables
            .create_row("stopped", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.submitteds.into_iter().for_each(|evt| {
        tables
            .create_row("submitted", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("referral", Hex(&evt.referral).to_string())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.token_rebaseds.into_iter().for_each(|evt| {
        tables
            .create_row("token_rebased", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("post_total_ether", BigDecimal::from_str(&evt.post_total_ether).unwrap())
            .set("post_total_shares", BigDecimal::from_str(&evt.post_total_shares).unwrap())
            .set("pre_total_ether", BigDecimal::from_str(&evt.pre_total_ether).unwrap())
            .set("pre_total_shares", BigDecimal::from_str(&evt.pre_total_shares).unwrap())
            .set("report_timestamp", BigDecimal::from_str(&evt.report_timestamp).unwrap())
            .set("shares_minted_as_fees", BigDecimal::from_str(&evt.shares_minted_as_fees).unwrap())
            .set("time_elapsed", BigDecimal::from_str(&evt.time_elapsed).unwrap());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.transfer_shares.into_iter().for_each(|evt| {
        tables
            .create_row("transfer_shares", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("shares_value", BigDecimal::from_str(&evt.shares_value).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.unbuffereds.into_iter().for_each(|evt| {
        tables
            .create_row("unbuffered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.withdrawals_receiveds.into_iter().for_each(|evt| {
        tables
            .create_row("withdrawals_received", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });

    Ok(tables.to_entity_changes())
}
