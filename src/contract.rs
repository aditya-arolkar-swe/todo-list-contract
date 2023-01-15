#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cosmwasm_std::to_binary;
use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:todo-list-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


// src/contract.rs
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //An overview of function parameters:

    //"deps" allows us to perform storage related actions, validate addresses and query other smart contracts
    //"_env" is mainly used to access details about the current state of the blockchain (i.e., block height, time, chain id)
    //"info" provides access to the message metadata (i.e., sender address, the amount and type of funds)
    //"msg" is the MsgInstantiateContract payload, which comprises the data received from the contract creator
    //in JSON format that conforms to the InstantiateMsg struct

    //Introduce a new variable named `state` of type `State`
    let state = State {
        //the value for count in the received message is assigned to the variable `count` of the `State` struct
        count: msg.count,
        //the sender address of the MsgInstantiateContract is assigned to the variable `owner` of the `State` struct
        owner: info.sender.clone(),
    };
    //Store the contract name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    //Store the initial state of the contract
    STATE.save(deps.storage, &state)?;

    //Form and return an Ok(Response)
    //The attributes will be included in the JSON formatted response message
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Match and route the query message to the appropriate handler
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        // Return the response in byte-array format
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    // Load the current contract state
    Ok(CountResponse { count: state.count })
    // Form and return a CountResponse
}

#[cfg(test)]
mod tests {}
